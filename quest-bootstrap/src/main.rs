use anyhow::{bail, Context, Result};
use directories::UserDirs;
use std::env;
use std::path::PathBuf;
use std::process::Command;

mod downloads;

fn ensure_homebrew() -> Result<()> {
    if Command::new("which").arg("brew").output().is_ok_and(|o| o.status.success()) {
        println!("Homebrew already installed");
        return Ok(());
    }

    println!("Homebrew not found. Installing Homebrew...");
    let status = Command::new("/bin/bash")
        .arg("-c")
        .arg(
            "curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
        )
        .status()
        .context("Failed to run Homebrew install script")?;

    if !status.success() {
        bail!("Homebrew installation failed");
    }
    println!("Homebrew installed successfully");
    Ok(())
}

fn ensure_brew_installed(name: &str) -> Result<()> {
    let status = Command::new("brew")
        .arg("list")
        .arg(name)
        .output()
        .is_ok_and(|o| o.status.success());

    if status {
        println!("{} already installed via Homebrew", name);
        return Ok(());
    }

    println!("Installing {} via Homebrew...", name);
    let status = Command::new("brew")
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
    }

    Ok(())
}
