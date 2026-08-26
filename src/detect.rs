use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::dist::{self, DistStatus};
use crate::locale::{self, Proposal};

#[derive(Debug, Clone)]
pub struct Detection {
    pub os: String,
    pub arch: String,
    pub user_languages: Vec<String>,
    pub proposal: Proposal,
    pub vcc: Option<VccInstall>,
}

#[derive(Debug, Clone)]
pub struct VccInstall {
    pub path: PathBuf,
    pub version: String,
    pub branch: String,
    pub portable: bool,
    pub dist_status: DistStatus,
    pub dist_bundle: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BuildInfo {
    version: BuildVersion,
    #[serde(default)]
    branch: String,
    #[serde(default)]
    portable: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BuildVersion {
    major: u32,
    minor: u32,
    patch: u32,
    #[serde(default)]
    pre_release: Option<String>,
}

pub fn detect(vcc_dir_override: Option<&Path>) -> Result<Detection> {
    let user_languages = user_languages();
    let tags: Vec<&str> = user_languages.iter().map(String::as_str).collect();
    let proposal = locale::propose_catalog(&tags);
    let vcc = match vcc_dir_override {
        Some(path) => Some(read_vcc(path)?),
        None => match vcc_install_path() {
            Some(path) => Some(read_vcc(&path)?),
            None => None,
        },
    };
    Ok(Detection {
        os: os_label(),
        arch: env::consts::ARCH.to_string(),
        user_languages,
        proposal,
        vcc,
    })
}

fn os_label() -> String {
    env::consts::OS.to_string()
}

fn read_vcc(path: &Path) -> Result<VccInstall> {
    let dist = path.join("WebApp").join("Dist");
    let info_path = dist.join("buildInfo.json");
    let (version, branch, portable) = if info_path.exists() {
        let raw = fs::read_to_string(&info_path)
            .with_context(|| format!("read {}", info_path.display()))?;
        let info: BuildInfo =
            serde_json::from_str(&raw).with_context(|| format!("parse {}", info_path.display()))?;
        (format_version(&info.version), info.branch, info.portable)
    } else {
        ("unknown".to_string(), String::new(), false)
    };
    let dist_status = if dist.exists() {
        dist::inspect(&dist)?
    } else {
        DistStatus::Missing
    };
    let dist_bundle = dist::index_bundle_name(&dist);
    Ok(VccInstall {
        path: path.to_path_buf(),
        version,
        branch,
        portable,
        dist_status,
        dist_bundle,
    })
}

fn format_version(version: &BuildVersion) -> String {
    let base = format!("{}.{}.{}", version.major, version.minor, version.patch);
    match version.pre_release.as_deref() {
        Some(pre) if !pre.is_empty() => format!("{base}-{pre}"),
        _ => base,
    }
}

fn vcc_install_path() -> Option<PathBuf> {
    if let Some(path) = registry_install_path() {
        if looks_like_vcc(&path) {
            return Some(path);
        }
    }
    let mut found = discover_installs(&default_search_roots());
    if found.is_empty() {
        return None;
    }
    found.sort_by_key(|path| std::cmp::Reverse(version_rank(path)));
    Some(found.remove(0))
}

fn version_rank(path: &Path) -> (u32, u32, u32, u8, String) {
    read_vcc(path)
        .ok()
        .map(|vcc| {
            let mut parts = vcc.version.splitn(2, '-');
            let core = parts.next().unwrap_or("0.0.0");
            let pre = parts.next().unwrap_or("");
            let mut nums = core.split('.');
            let major = nums.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let minor = nums.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let patch = nums.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let release_first = if pre.is_empty() { 1 } else { 0 };
            (major, minor, patch, release_first, pre.to_string())
        })
        .unwrap_or((0, 0, 0, 0, String::new()))
}

pub fn looks_like_vcc(path: &Path) -> bool {
    let dist_index = path.join("WebApp").join("Dist").join("index.html");
    if !dist_index.is_file() {
        return false;
    }
    path.join("CreatorCompanion.exe").is_file() || path.join("CreatorCompanionBeta.exe").is_file()
}

pub fn discover_installs(search_roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for root in search_roots {
        if looks_like_vcc(root) {
            push_unique(&mut out, root);
        }
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && looks_like_vcc(&path) {
                push_unique(&mut out, &path);
            }
        }
    }
    out
}

fn push_unique(out: &mut Vec<PathBuf>, path: &Path) {
    if !out.iter().any(|existing| existing == path) {
        out.push(path.to_path_buf());
    }
}

fn default_search_roots() -> Vec<PathBuf> {
    let mut roots = vec![
        PathBuf::from(r"D:\Software"),
        PathBuf::from(r"C:\Program Files"),
        PathBuf::from(r"C:\Program Files (x86)"),
    ];
    if let Some(local) = env::var_os("LOCALAPPDATA") {
        roots.push(PathBuf::from(local).join("Programs"));
    }
    roots
}

#[cfg(windows)]
fn registry_install_path() -> Option<PathBuf> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Software\\VCC").ok()?;
    let path: String = key.get_value("InstallPath").ok()?;
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

#[cfg(not(windows))]
fn registry_install_path() -> Option<PathBuf> {
    None
}

#[cfg(windows)]
fn user_languages() -> Vec<String> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey("Control Panel\\International\\User Profile") {
        if let Ok(langs) = key.get_value::<Vec<String>, _>("Languages") {
            if !langs.is_empty() {
                return langs;
            }
        }
    }
    default_locale_name().into_iter().collect::<Vec<_>>()
}

#[cfg(not(windows))]
fn user_languages() -> Vec<String> {
    Vec::new()
}

#[cfg(windows)]
fn default_locale_name() -> Option<String> {
    use windows_sys::Win32::Globalization::GetUserDefaultLocaleName;
    let mut buf = [0u16; 85];
    let n = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    if n <= 1 {
        return None;
    }
    Some(String::from_utf16_lossy(&buf[..n as usize - 1]))
}
