#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="kawaii"
CONFIG_FILE="$HOME/.config/kawaii/config.toml"

# Clean previous install
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "Previous install found. Cleaning..."
    rm -f "$INSTALL_DIR/$BINARY_NAME"
    # Remove old aliases
    while IFS= read -r link; do
        if [ -L "$link" ] && [ "$(readlink "$link")" = "$INSTALL_DIR/$BINARY_NAME" ]; then
            rm -f "$link"
        fi
    done < <(find "$INSTALL_DIR" -maxdepth 1 -type l 2>/dev/null)
    echo "  ✓ Cleaned old install"
fi

# Clean build artifacts
cargo clean --manifest-path "$SCRIPT_DIR/Cargo.toml" 2>/dev/null || true

# Build
echo "Building kawaii (release)..."
cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"

# Install
mkdir -p "$INSTALL_DIR"
cp "$SCRIPT_DIR/target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Ask for custom alias name
echo ""
read -rp "  Command alias (default: kawaii): " ALIAS_NAME
ALIAS_NAME="${ALIAS_NAME:-kawaii}"

if [ "$ALIAS_NAME" != "kawaii" ]; then
    ln -sf "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME"
    chmod +x "$INSTALL_DIR/$ALIAS_NAME"
    echo "  ✓ Alias '$ALIAS_NAME' → kawaii"
fi

# Create config if missing
if [ ! -f "$CONFIG_FILE" ]; then
    mkdir -p "$(dirname "$CONFIG_FILE")"
    cat > "$CONFIG_FILE" << 'TOML'
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
    echo "  ✓ Config created at $CONFIG_FILE"
fi

echo ""
echo "Done! Installed to $INSTALL_DIR/$BINARY_NAME"
echo ""
CMD="${ALIAS_NAME}"
echo "  $CMD -s <pkg>   Search"
echo "  $CMD -i <pkg>   Install"
echo "  $CMD -r <pkg>   Remove"
echo "  $CMD -u         Update"
echo "  $CMD -I <pkg>   Info"
echo "  $CMD -l         List"
echo "  $CMD -d         Doctor"
echo "  $CMD -c         Config"
echo "  $CMD -v         Version"
