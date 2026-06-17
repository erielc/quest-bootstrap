use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub struct DownloadItem {
    pub name: &'static str,
    pub url: &'static str,
    pub output_file: &'static str,
}

pub fn download_required_tools(data_dir: &Path, os: &str, arch: &str) -> Result<()> {
    let downloads = get_downloads_for_platform(os, arch)?;

    let client = Client::new();

    for item in downloads {
        download_file(&client, data_dir, &item)?;
    }

    println!("All required downloads completed.");

    Ok(())
}

fn get_downloads_for_platform(os: &str, arch: &str) -> Result<Vec<DownloadItem>> {
    match (os, arch) {
        ("windows", "x86_64") => Ok(vec![
            DownloadItem {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.12.4/python-3.12.4-amd64.exe",
                output_file: "python-installer.exe",
            },
            DownloadItem {
                name: "Git",
                url: "https://github.com/git-for-windows/git/releases/download/v2.45.2.windows.1/Git-2.45.2-64-bit.exe",
                output_file: "git-installer.exe",
            },
            DownloadItem {
                name: "GLPK",
                url: "https://sourceforge.net/projects/winglpk/files/latest/download",
                output_file: "glpk.zip",
            },
        ]),

        ("macos", "x86_64") => Ok(vec![
            DownloadItem {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.12.4/python-3.12.4-macos11.pkg",
                output_file: "python-installer.pkg",
            },
            DownloadItem {
                name: "Git",
                url: "https://sourceforge.net/projects/git-osx-installer/files/latest/download",
                output_file: "git-installer.dmg",
            },
            DownloadItem {
                name: "GLPK",
                url: "https://ftp.gnu.org/gnu/glpk/glpk-5.0.tar.gz",
                output_file: "glpk.tar.gz",
            },
        ]),

        ("macos", "aarch64") => Ok(vec![
            DownloadItem {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.12.4/python-3.12.4-macos11.pkg",
                output_file: "python-installer.pkg",
            },
            DownloadItem {
                name: "Git",
                url: "https://sourceforge.net/projects/git-osx-installer/files/latest/download",
                output_file: "git-installer.dmg",
            },
            DownloadItem {
                name: "GLPK",
                url: "https://ftp.gnu.org/gnu/glpk/glpk-5.0.tar.gz",
                output_file: "glpk.tar.gz",
            },
        ]),

        ("linux", "x86_64") => Ok(vec![
            DownloadItem {
                name: "Python",
                url: "https://www.python.org/ftp/python/3.12.4/Python-3.12.4.tgz",
                output_file: "python-source.tgz",
            },
            DownloadItem {
                name: "Git",
                url: "https://mirrors.edge.kernel.org/pub/software/scm/git/git-2.45.2.tar.gz",
                output_file: "git-source.tar.gz",
            },
            DownloadItem {
                name: "GLPK",
                url: "https://ftp.gnu.org/gnu/glpk/glpk-5.0.tar.gz",
                output_file: "glpk.tar.gz",
            },
        ]),

        _ => {
            bail!("Unsupported platform: os={}, arch={}", os, arch);
        }
    }
}

fn download_file(client: &Client, data_dir: &Path, item: &DownloadItem) -> Result<()> {
    let output_path = data_dir.join(item.output_file);

    println!("Downloading {}...", item.name);
    println!("URL: {}", item.url);
    println!("Output: {}", output_path.display());

    let mut response = client
        .get(item.url)
        .send()
        .with_context(|| format!("Failed to request {}", item.url))?
        .error_for_status()
        .with_context(|| format!("Download failed for {}", item.url))?;

    let mut file = File::create(&output_path)
        .with_context(|| format!("Failed to create {}", output_path.display()))?;

    copy(&mut response, &mut file)
        .with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!("Downloaded {} successfully.\n", item.name);

    Ok(())
}
