use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

pub struct Download {
    pub name: &'static str,
    pub url: &'static str,
    pub file: &'static str,
}

fn is_tool_installed(name: &str) -> bool {
    match name {
        "Python" => {
            Command::new("python3")
                .arg("--version")
                .output()
                .is_ok_and(|o| o.status.success())
                || Command::new("python")
                    .arg("--version")
                    .output()
                    .is_ok_and(|o| o.status.success())
        }
        "Git" => {
            Command::new("git")
                .arg("--version")
                .output()
                .is_ok_and(|o| o.status.success())
        }
        "GLPK" => {
            Command::new("glpsol")
                .arg("--version")
                .output()
                .is_ok_and(|o| o.status.success())
        }
        _ => false,
    }
}

pub fn download_required_tools(data_dir: &Path, os: &str, arch: &str) -> Result<()> {
    let items = get_downloads(os, arch)?;
    let client = Client::new();

    for item in items {
        if is_tool_installed(item.name) {
            println!("{} is already installed on the system. Skipping download.", item.name);
            continue;
        }

        let path = data_dir.join(item.file);
        println!("Downloading {}...", item.name);
        println!("  from: {}", item.url);
        println!("  to:   {}", path.display());

        let mut resp = client
            .get(item.url)
            .send()
            .with_context(|| format!("request failed: {}", item.url))?
            .error_for_status()
            .with_context(|| format!("bad status: {}", item.url))?;

        let mut f = File::create(&path)
            .with_context(|| format!("create failed: {}", path.display()))?;

        copy(&mut resp, &mut f)
            .with_context(|| format!("write failed: {}", path.display()))?;

        println!("  done ({})", item.name);
    }

    println!("All downloads saved to: {}", data_dir.display());
    Ok(())
}

fn get_downloads(os: &str, arch: &str) -> Result<Vec<Download>> {
    match (os, arch) {
        ("windows", "x86_64") => Ok(vec![
            Download {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.13.9/python-3.13.9-amd64.exe",
                file: "python-3.13.9-amd64.exe",
            },
            Download {
                name: "Git",
                url: "https://github.com/git-for-windows/git/releases/download/v2.45.2.windows.1/Git-2.45.2-64-bit.exe",
                file: "Git-2.45.2-64-bit.exe",
            },
            Download {
                name: "GLPK",
                url: "https://sourceforge.net/projects/winglpk/files/latest/download",
                file: "glpk.zip",
            },
        ]),

        ("macos", "x86_64") | ("macos", "aarch64") => Ok(vec![
            Download {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.13.9/python-3.13.9-macos11.pkg",
                file: "python-3.13.9-macos11.pkg",
            },
        ]),

        _ => bail!("unsupported platform: os={}, arch={}", os, arch),
    }
}
