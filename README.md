<div align="center">
    <img src = "images/quest-bootstrap-logo.png" width"2000" height="500" alt="API" />
</div>

# quest-bootstrap

Downloads Python, Git, and GLPK installers for your OS to prepare the QuESt prerequisites. 

On macOS:
* It automatically checks for and installs **Homebrew** if it is not already present.
* It installs **Git** and **GLPK** via Homebrew.
* It downloads the official **Python** `.pkg` installer to `~/Downloads/quest-dependencies/`.

On Windows & Linux, downloads are saved locally to `~/Downloads/quest-dependencies/` without requiring administrator privileges.

## Usage

If building from source:
```bash
cargo run
```

### Download Precompiled Binaries
You can download precompiled binaries from the **Releases** page:
* **macOS / Linux**: Download the `.tar.gz` archive. This preserves the executable permission so you can simply extract it and run the binary.
* **Windows**: Download the `.zip` archive or the `.exe` file.

If you download the raw binary directly on macOS/Linux instead of the `.tar.gz` archive, you will need to give it execute permissions manually in your terminal before running:
```bash
chmod +x quest-bootstrap-aarch64-apple-darwin
./quest-bootstrap-aarch64-apple-darwin
```

### Where files go

| OS      | Default path                              |
|---------|--------------------------------------------|
| Linux   | `~/Downloads/quest-dependencies/`         |
| macOS   | `~/Downloads/quest-dependencies/`         |
| Windows | `~\Downloads\quest-dependencies\`         |

## Downloads per platform

| Tool  | Windows                          | macOS                      | Linux                    |
|-------|-----------------------------------|----------------------------|--------------------------|
| Python| `.exe` installer                  | `.pkg` installer           | Source `.tgz`            |
| Git   | `.exe` installer                  | `brew install git`         | Source `.tar.gz`         |
| GLPK  | `.zip` (Windows GLPK)            | `brew install glpk`        | Source `.tar.gz`         |

After downloading, run each installer manually or use your system package manager to install the tools.

### Linux quick install

```bash
sudo apt install python3 python3-pip git glpk-utils
```
