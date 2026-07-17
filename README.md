# Kawaii-pkg

**One command. Every package manager.**

Kawaii-pkg is a Rust-based universal Linux package manager wrapper that provides a single unified CLI for pacman, yay, paru, apt, dnf, zypper, flatpak, snap, and more.

## Quick Start

```bash
# Install with one command
curl -sSL https://raw.githubusercontent.com/shirushimori/kawaii-pkg/main/install.sh | bash

# Search for a package
kawaii -s firefox

# Install a package
kawaii -i firefox

# Update everything
kawaii -u
```

## Installation

### One-Line Install (recommended)

```bash
curl -sSL https://raw.githubusercontent.com/shirushimori/kawaii-pkg/main/install.sh | bash
```

The installer will:
1. Install Rust if not present
2. Clone the repo and build a release binary
3. Ask for your preferred **command name** (default: `kawaii`)
4. Ask for an optional **alias** (e.g. `loda`)
5. Install to `~/.local/bin/`
6. Create config at `~/.config/kawaii/config.toml`

### From crates.io

```bash
cargo install kawaii-pkg
```

### From Source

```bash
git clone https://github.com/shirushimori/kawaii-pkg.git
cd kawaii-pkg
cargo build --release
cp target/release/kawaii ~/.local/bin/
```

## Usage

```bash
kawaii -s <package>    # Search across all managers
kawaii -i <package>    # Install (asks which manager if multiple found)
kawaii -r <package>    # Remove a package
kawaii -I <package>    # Show package info
kawaii -u              # Update all packages
kawaii -u <package>    # Update specific package
kawaii -l              # List installed packages
kawaii -C              # Clean package caches
kawaii -d              # Run system health checks
kawaii -H              # Show command history
kawaii -c              # Open config in editor
kawaii -v              # Show version info
```

## Supported Package Managers

| Manager | Type | Sudo? |
|---------|------|-------|
| pacman | System | Yes |
| yay | AUR | No |
| paru | AUR | No |
| apt | System | Yes |
| dnf | System | Yes |
| yum | System | Yes |
| zypper | System | Yes |
| xbps | System | No |
| nix | User | No |
| apk | System | Yes |
| brew | User | No |
| flatpak | Universal | No |
| snap | Universal | No |

## Configuration

Edit `~/.config/kawaii/config.toml`:

```toml
name = "kawaii"
auto_yes = false
colors = true
parallel_search = true
show_summary = true
full_log = false
search_order = [
    "pacman",
    "yay",
    "paru",
    "apt",
    "dnf",
    "flatpak",
    "snap",
]

[aliases]
```

### Options

- `name` — command name shown in prompts
- `auto_yes` — skip confirmation prompts
- `colors` — enable/disable colored output
- `parallel_search` — search managers in parallel
- `show_summary` — show install summary after operations
- `full_log` — show full command output (vs summary)
- `search_order` — priority order for manager selection
- `[aliases]` — custom command aliases

## How It Works

1. **Detection** — On startup, kawaii scans your system for installed package managers
2. **Search** — When you search, it queries all detected managers in parallel with 5s timeouts
3. **Install** — If a package is found in multiple managers, you choose which one
4. **History** — Every operation is logged to `~/.local/share/kawaii/history.json`

## License

MIT
