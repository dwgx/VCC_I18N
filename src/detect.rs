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
    pub dist_status: DistStatus,
}

#[derive(Debug, Deserialize)]
struct BuildInfo {
    version: BuildVersion,
    #[serde(default)]
    branch: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BuildVersion {
    major: u32,
    minor: u32,
    patch: u32,
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
    let (version, branch) = if info_path.exists() {
        let raw = fs::read_to_string(&info_path)
            .with_context(|| format!("read {}", info_path.display()))?;
        let info: BuildInfo =
            serde_json::from_str(&raw).with_context(|| format!("parse {}", info_path.display()))?;
        (
            format!(
                "{}.{}.{}",
                info.version.major, info.version.minor, info.version.patch
            ),
            info.branch,
        )
    } else {
        ("unknown".to_string(), String::new())
    };
    let dist_status = if dist.exists() {
        dist::inspect(&dist)?
    } else {
        DistStatus::Missing
    };
    Ok(VccInstall {
        path: path.to_path_buf(),
        version,
        branch,
        dist_status,
    })
}

#[cfg(windows)]
fn vcc_install_path() -> Option<PathBuf> {
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
fn vcc_install_path() -> Option<PathBuf> {
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
