use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::env;

mod downloads;

// -> Result<()> allows using the '?' operator 
// for clean error handling in the main function.
fn main() -> Result<()> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    println!("Current OS: {}", os);
    println!("Current ARCH: {}", arch);

    let proj_dirs = ProjectDirs::from("com", "Sandia", "quest-bootstrap")
        .expect("Could not determine valid home directory");

    // grab app data directory
    // UNIX: ~/.local/share/quest-bootstrap
    // WINDOWS: C:\Users\User\AppData\Roaming\Sandia\quest-bootstrap
    let data_dir = proj_dirs.data_dir();

    println!("Installation target mapped to: {}", data_dir.display());

    if data_dir.exists() {
        println!("Directory already exists, ready to launch or update");
    } else {
        std::fs::create_dir_all(data_dir)
            .context("Failed to create installation directory")?;
        println!("Directory created successfully");
    }

    downloads::download_required_tools(data_dir, os, arch)?;

    Ok(())
}
