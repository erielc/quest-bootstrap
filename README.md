<div align="center">
    <img src = "images/quest-bootstrap-logo.png" width"2000" height="500" alt="API" />
</div>

# quest-bootstrap

Downloads Python and GLPK installers for your OS (plus Git on Windows/Linux)
so QuESt prerequisites are ready to install. On macOS, Git is installed via
Homebrew instead. No system admin rights needed — everything goes into
`~/Downloads/quest-dependencies`.

## Usage

```bash
cargo run
```

The tool detects your OS and architecture, creates a `~/Downloads/quest-dependencies`
directory, and downloads the required installers into it.

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
| GLPK  | `.zip` (Windows GLPK)            | Source `.tar.gz`           | Source `.tar.gz`         |

After downloading, run each installer manually or use your system package
manager to install the tools.

### Linux quick install

```bash
sudo apt install python3 python3-pip git glpk-utils
```
