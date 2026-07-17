#!/usr/bin/env bash
set -euo pipefail

REPO="https://github.com/shirushimori/kawaii-pkg.git"
TMPDIR=$(mktemp -d)
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="kawaii"
CONFIG_FILE="$HOME/.config/kawaii/config.toml"

echo ""
echo "  ⬡ Kawaii-pkg Installer"
echo "  ─────────────────────────"
echo ""

# Check if already installed
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "  Previous installation found at $INSTALL_DIR/$BINARY_NAME"
    echo ""
    echo "  [1] Reinstall (update)"
    echo "  [2] Uninstall"
    echo "  [3] Cancel"
    echo ""
    read -rp "  Choose (1/2/3): " CHOICE
    case "$CHOICE" in
        2)
            echo ""
            rm -f "$INSTALL_DIR/$BINARY_NAME"
            while IFS= read -r link; do
                if [ -L "$link" ] && [ "$(readlink "$link")" = "$INSTALL_DIR/$BINARY_NAME" ]; then
                    rm -f "$link"
                fi
            done < <(find "$INSTALL_DIR" -maxdepth 1 -type l 2>/dev/null)
            echo "  ✓ Uninstalled kawaii"
            rm -rf "$TMPDIR"
            exit 0
            ;;
        3)
            echo "  Cancelled."
            rm -rf "$TMPDIR"
            exit 0
            ;;
        *)
            echo "  Reinstalling..."
            ;;
    esac
fi

# Check Rust
if ! command -v cargo &>/dev/null; then
    echo "  Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "  Cloning repository..."
git clone --depth 1 "$REPO" "$TMPDIR" &>/dev/null

echo "  Building kawaii (release)..."
cargo build --release --manifest-path "$TMPDIR/Cargo.toml" --quiet

# Clean previous install
rm -f "$INSTALL_DIR/$BINARY_NAME"
while IFS= read -r link; do
    if [ -L "$link" ] && [ "$(readlink "$link")" = "$INSTALL_DIR/$BINARY_NAME" ]; then
        rm -f "$link"
    fi
done < <(find "$INSTALL_DIR" -maxdepth 1 -type l 2>/dev/null)

# Install
mkdir -p "$INSTALL_DIR"
cp "$TMPDIR/target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Ask for command name
echo ""
read -rp "  Command name (default: kawaii): " CMD_NAME
CMD_NAME="${CMD_NAME:-kawaii}"

# Ask for alias
read -rp "  Alias name (leave blank to skip): " ALIAS_NAME

# Create symlinks
if [ "$CMD_NAME" != "kawaii" ]; then
    ln -sf "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$CMD_NAME"
    chmod +x "$INSTALL_DIR/$CMD_NAME"
    echo "  ✓ Command: $CMD_NAME → kawaii"
fi

if [ -n "$ALIAS_NAME" ] && [ "$ALIAS_NAME" != "$CMD_NAME" ]; then
    ln -sf "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME"
    chmod +x "$INSTALL_DIR/$ALIAS_NAME"
    echo "  ✓ Alias: $ALIAS_NAME → kawaii"
fi

# Create config with command name
mkdir -p "$(dirname "$CONFIG_FILE")"
cat > "$CONFIG_FILE" << TOML
name = "$CMD_NAME"
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
    "yum",
    "zypper",
    "xbps",
    "nix",
    "apk",
    "brew",
    "flatpak",
    "snap",
]

[aliases]
TOML

# Cleanup
rm -rf "$TMPDIR"

echo ""
echo "  ✓ Installed to $INSTALL_DIR/$BINARY_NAME"
echo ""
DISPLAY_NAME="${CMD_NAME}"
echo "  $DISPLAY_NAME -s <pkg>   Search"
echo "  $DISPLAY_NAME -i <pkg>   Install"
echo "  $DISPLAY_NAME -r <pkg>   Remove"
echo "  $DISPLAY_NAME -u         Update"
echo "  $DISPLAY_NAME -I <pkg>   Info"
echo "  $DISPLAY_NAME -l         List"
echo "  $DISPLAY_NAME -C         Clean"
echo "  $DISPLAY_NAME -d         Doctor"
echo "  $DISPLAY_NAME -c         Config"
echo "  $DISPLAY_NAME -v         Version"
