use std::env;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use vcc_i18n::catalog;
use vcc_i18n::detect::{self, Detection};
use vcc_i18n::dist::{self, DistStatus};
use vcc_i18n::locale::Catalog;

struct Args {
    yes: bool,
    dry_run: bool,
    uninstall: bool,
    no_launch: bool,
    report_keys: bool,
    out: Option<PathBuf>,
    vcc_dir: Option<PathBuf>,
}

fn main() {
    enable_utf8();
    if let Err(err) = run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = parse_args()?;
    println!("VCC_I18N\n");
    println!("Standalone VCC localizer. Independent project.\n");

    let detection = detect::detect(args.vcc_dir.as_deref())?;
    print_table(&detection);

    if args.report_keys {
        report_keys(&args, &detection)?;
        return Ok(());
    }

    if args.dry_run {
        return Ok(());
    }

    let Some(vcc) = detection.vcc.as_ref() else {
        bail!("VCC was not found. Pass --vcc-dir PATH");
    };
    let dist = vcc.path.join("WebApp").join("Dist");
    if !dist.exists() {
        bail!("Dist folder is missing: {}", dist.display());
    }

    match vcc.dist_status {
        DistStatus::Foreign => {
            bail!("Dist is patched by another tool. Restore that first, then run VCC_I18N again")
        }
        DistStatus::Missing => bail!("index.html is missing in {}", dist.display()),
        DistStatus::Ours => existing_install(&args, &detection, vcc, &dist)?,
        DistStatus::Clean => fresh_install(&args, &detection, vcc, &dist)?,
    }
    Ok(())
}

fn existing_install(
    args: &Args,
    detection: &Detection,
    vcc: &vcc_i18n::detect::VccInstall,
    dist: &Path,
) -> Result<()> {
    println!("VCC_I18N is already installed.\n");
    if args.uninstall || prompt(args.yes, "Uninstall it?", false)? {
        dist::restore(dist)?;
        println!("Uninstalled. Original index.html restored.");
        return Ok(());
    }
    if prompt(args.yes, "Reapply the patch?", true)? {
        apply(detection, dist)?;
        maybe_launch(args, &vcc.path)?;
    }
    Ok(())
}

fn fresh_install(
    args: &Args,
    detection: &Detection,
    vcc: &vcc_i18n::detect::VccInstall,
    dist: &Path,
) -> Result<()> {
    if args.uninstall {
        println!("Nothing to uninstall.");
        return Ok(());
    }
    let catalog = detection.proposal.catalog;
    let question = format!(
        "Install {} ({}) into this VCC?",
        catalog.english_name(),
        catalog.id()
    );
    if !prompt(args.yes, &question, true)? {
        println!("Cancelled.");
        return Ok(());
    }
    apply(detection, dist)?;
    maybe_launch(args, &vcc.path)?;
    Ok(())
}

fn apply(detection: &Detection, dist: &Path) -> Result<()> {
    let catalog = effective_catalog(detection.proposal.catalog);
    if catalog != detection.proposal.catalog {
        println!("zh-TW catalog is empty in this version. Using zh-CN.");
    }
    let script = catalog::injector_source(catalog);
    dist::inject(dist, &script)?;
    println!("Installed {}.", catalog.id());
    Ok(())
}

fn maybe_launch(args: &Args, vcc_path: &Path) -> Result<()> {
    if args.no_launch {
        return Ok(());
    }
    let exe = if vcc_path.join("CreatorCompanion.exe").exists() {
        vcc_path.join("CreatorCompanion.exe")
    } else if vcc_path.join("CreatorCompanionBeta.exe").exists() {
        vcc_path.join("CreatorCompanionBeta.exe")
    } else {
        println!("VCC executable was not found; not launching.");
        return Ok(());
    };
    Command::new(&exe)
        .current_dir(vcc_path)
        .spawn()
        .with_context(|| format!("start {}", exe.display()))?;
    println!("Started {}", exe.display());
    Ok(())
}

fn print_table(detection: &Detection) {
    println!("Detection");
    println!("  OS            {}", detection.os);
    println!("  Arch          {}", detection.arch);
    println!(
        "  Languages     {}",
        if detection.user_languages.is_empty() {
            "(none)".to_string()
        } else {
            detection.user_languages.join(", ")
        }
    );
    let matched = detection
        .proposal
        .matched_tag
        .as_deref()
        .unwrap_or("no Chinese tag; default");
    println!(
        "  Proposed      {} ({})  [{}]",
        detection.proposal.catalog.id(),
        detection.proposal.catalog.english_name(),
        matched
    );
    match &detection.vcc {
        None => {
            println!("  VCC path      (not found)");
        }
        Some(vcc) => {
            println!("  VCC path      {}", vcc.path.display());
            println!("  VCC version   {}", vcc.version);
            if !vcc.branch.is_empty() {
                println!("  VCC branch    {}", vcc.branch);
            }
            if vcc.portable {
                println!("  VCC portable  yes");
            }
            if let Some(bundle) = &vcc.dist_bundle {
                println!("  Dist bundle   {bundle}");
            }
            println!("  Dist          {}", vcc.dist_status.as_str());
            print_missing_keys(vcc, effective_catalog(detection.proposal.catalog));
        }
    }
    println!();
}

fn effective_catalog(catalog: Catalog) -> Catalog {
    if catalog == Catalog::ZhTw && catalog::json_for(catalog).trim() == "{}" {
        Catalog::ZhCn
    } else {
        catalog
    }
}

fn print_missing_keys(vcc: &vcc_i18n::detect::VccInstall, catalog: Catalog) {
    let dist = vcc.path.join("WebApp").join("Dist");
    let Ok(bundle) = dist::index_bundle_path(&dist) else {
        return;
    };
    let Ok(js) = std::fs::read_to_string(&bundle) else {
        return;
    };
    let missing = catalog::missing_from_bundle(&js, catalog::json_for(catalog));
    if missing.is_empty() {
        println!("  Catalog        covers Dist UI strings");
        return;
    }
    println!("  Missing keys  {}", missing.len());
    for key in missing.iter().take(12) {
        println!("    - {key}");
    }
    if missing.len() > 12 {
        println!("    - ...");
    }
}

fn report_keys(args: &Args, detection: &Detection) -> Result<()> {
    let Some(vcc) = detection.vcc.as_ref() else {
        bail!("VCC was not found. Pass --vcc-dir PATH");
    };
    let dist = vcc.path.join("WebApp").join("Dist");
    let bundle = dist::index_bundle_path(&dist)?;
    let bundle_name = bundle
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| bundle.display().to_string());
    let js =
        std::fs::read_to_string(&bundle).with_context(|| format!("read {}", bundle.display()))?;
    let catalog = effective_catalog(detection.proposal.catalog);
    let coverage = catalog::coverage(&js, catalog::json_for(catalog));
    let missing_count = coverage.missing.len();
    let unused_count = coverage.unused.len();

    println!("Key coverage");
    println!("  {:<14}{bundle_name}", "bundle");
    println!("  {:<14}{}", "catalog", catalog.id());
    println!("  {:<14}{missing_count}", "missing_count");
    println!("  {:<14}{unused_count}", "unused_count");
    for (label, keys) in [("missing", &coverage.missing), ("unused", &coverage.unused)] {
        if keys.is_empty() {
            continue;
        }
        println!("  {label}:");
        for key in keys.iter().take(12) {
            println!("    - {key}");
        }
        if keys.len() > 12 {
            println!("    - ...");
        }
    }

    if let Some(path) = &args.out {
        let report = serde_json::json!({
            "bundle": bundle_name,
            "catalog": catalog.id(),
            "missing_count": missing_count,
            "unused_count": unused_count,
            "missing": coverage.missing,
            "unused": coverage.unused,
        });
        let body = serde_json::to_string_pretty(&report)?;
        std::fs::write(path, format!("{body}\n"))
            .with_context(|| format!("write {}", path.display()))?;
        println!("\nWrote {}", path.display());
    }
    Ok(())
}

fn prompt(assume_yes: bool, question: &str, default_yes: bool) -> Result<bool> {
    if assume_yes {
        return Ok(default_yes);
    }
    let hint = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{question} {hint} ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    let t = line.trim().to_ascii_lowercase();
    if t.is_empty() {
        return Ok(default_yes);
    }
    Ok(matches!(t.as_str(), "y" | "yes"))
}

fn parse_args() -> Result<Args> {
    let mut args = Args {
        yes: false,
        dry_run: false,
        uninstall: false,
        no_launch: false,
        report_keys: false,
        out: None,
        vcc_dir: None,
    };
    let mut raw = env::args().skip(1);
    while let Some(item) = raw.next() {
        match item.as_str() {
            "--yes" | "-y" => args.yes = true,
            "--dry-run" => args.dry_run = true,
            "--uninstall" => args.uninstall = true,
            "--no-launch" => args.no_launch = true,
            "--report-keys" => args.report_keys = true,
            "--out" => {
                let path = raw.next().context("--out needs a path")?;
                args.out = Some(PathBuf::from(path));
            }
            "--vcc-dir" => {
                let path = raw.next().context("--vcc-dir needs a path")?;
                args.vcc_dir = Some(PathBuf::from(path));
            }
            "--version" => {
                println!("vcc-i18n {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => bail!("unknown argument: {other}"),
        }
    }
    Ok(args)
}

fn print_help() {
    println!(
        "vcc-i18n -- standalone VCC localizer

  --dry-run       detect only
  --report-keys   print catalog coverage for the Dist bundle (read-only)
  --out PATH      with --report-keys, write the coverage JSON to PATH
  --version       print the version
  --yes, -y       accept default prompts
  --uninstall     restore original index.html
  --no-launch     do not start VCC
  --vcc-dir PATH  override install path (registry or portable)
"
    );
}

#[cfg(windows)]
fn enable_utf8() {
    use windows_sys::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};
    unsafe {
        SetConsoleOutputCP(65001);
        SetConsoleCP(65001);
    }
}

#[cfg(not(windows))]
fn enable_utf8() {}
