use anyhow::{bail, Context, Result};
use directories::UserDirs;
use std::env;
use std::path::PathBuf;
use std::process::Command;

mod downloads;

fn find_brew_path() -> String {
    if Command::new("which").arg("brew").output().is_ok_and(|o| o.status.success()) {
        return "brew".to_string();
    }
    if PathBuf::from("/opt/homebrew/bin/brew").exists() {
        return "/opt/homebrew/bin/brew".to_string();
    }
    if PathBuf::from("/usr/local/bin/brew").exists() {
        return "/usr/local/bin/brew".to_string();
    }
    "brew".to_string()
}

fn ensure_homebrew() -> Result<()> {
    // Check if brew can be found or resolved
    if find_brew_path() != "brew" || Command::new("which").arg("brew").output().is_ok_and(|o| o.status.success()) {
        println!("Homebrew already installed");
        return Ok(());
    }

    println!("Homebrew not found. Installing Homebrew...");
    let status = Command::new("/bin/bash")
        .arg("-c")
        .arg("NONINTERACTIVE=1 /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
        .status()
        .context("Failed to run Homebrew install script")?;

    if !status.success() {
        bail!("Homebrew installation failed");
    }
    println!("Homebrew installed successfully");
    Ok(())
}

fn ensure_brew_installed(name: &str) -> Result<()> {
    let brew_path = find_brew_path();
    let status = Command::new(&brew_path)
        .arg("list")
        .arg(name)
        .output()
        .is_ok_and(|o| o.status.success());

    if status {
        println!("{} already installed via Homebrew", name);
        return Ok(());
    }

    println!("Installing {} via Homebrew...", name);
    let status = Command::new(&brew_path)
        .arg("install")
        .arg(name)
        .status()
        .with_context(|| format!("Failed to install {} via Homebrew", name))?;

    if !status.success() {
        bail!("Failed to install {} via Homebrew", name);
    }
    println!("{} installed successfully", name);
    Ok(())
}

fn main() -> Result<()> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    println!("Current OS: {}", os);
    println!("Current ARCH: {}", arch);

    let user_dirs = UserDirs::new().context("Could not determine home directory")?;
    let data_dir: PathBuf = user_dirs.home_dir().join("Downloads/quest-dependencies");

    println!("Downloading to: {}", data_dir.display());

    if data_dir.exists() {
        println!("Directory already exists");
    } else {
        std::fs::create_dir_all(&data_dir)
            .context("Failed to create download directory")?;
        println!("Directory created successfully");
    }

    downloads::download_required_tools(&data_dir, os, arch)?;

    if os == "macos" {
        ensure_homebrew()?;
        ensure_brew_installed("git")?;
        ensure_brew_installed("glpk")?;
    }

    Ok(())
}
