use std::env;
use std::process;
use directories::ProjectDirs;

fn main() {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    println!("Current OS: {}", os);
    println!("Current ARCH: {}", arch);

    // establish cross platform app directory 
    // parameters: qualifier, org, app name
    let proj_dirs = match ProjectDirs::from("com", "Sandia", "quest-bootstrap") {
        Some(dirs) => dirs, 
        None => {
            eprintln!("Error: Could not determine vaild home directory for this OS");
            process::exit(1);
        }
    }; 

    // grab app data directory
    // UNIX: ~/.local/share/quest-bootstrap
    // WINDOWS: C:\Users\User\AppData\Roaming\Sandia\quest-bootstrap
    let data_dir = proj_dirs.data_dir();

    println!("Installation target mapped to: {}", data_dir.display());

    // determine workflow based off OS
    if os == "windows" {
        println!("Need to download Windows .zip for Python..");
    } else if os == "macos" {
        println!("Need to download macOS tarball for Python..");
    } else if os == "linux" {
        println!("Need to download Linux tarball for Python..");
    } else {
        eprintln!("Unsupported OS: {}", os);
        process::exit(1);
    }

    // create directory if it doesn't exist yet
    if !data_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(data_dir) {
            eprintln!("Failed to create installation directory: {}", e);
            process::exit(1);
        } 
        println!("directory created successfully!");
    } else {
        println!("Directory already exists, Ready to launch or update");
    }

}

// goal:
// map out download URLs for python runtime (ie. python, git, glpk)
