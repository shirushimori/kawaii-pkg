#!/usr/bin/env bash
set -euo pipefail

# Helper: read from /dev/tty (works with curl | bash)
read_from_tty() {
    if [ -t 0 ]; then
        read -r
    else
        read -r < /dev/tty
    fi
}

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
    printf "  Choose (1/2/3): "
    CHOICE=$(read_from_tty)
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

# Clean previous install and all broken symlinks
rm -f "$INSTALL_DIR/$BINARY_NAME"
while IFS= read -r link; do
    rm -f "$link"
done < <(find "$INSTALL_DIR" -maxdepth 1 -type l 2>/dev/null)

# Install
mkdir -p "$INSTALL_DIR"
cp "$TMPDIR/target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo ""

printf "  Command name (default: kawaii): "
CMD_NAME=$(read_from_tty)
CMD_NAME="${CMD_NAME:-kawaii}"

printf "  Alias name (leave blank to skip): "
ALIAS_NAME=$(read_from_tty)

# Always create the primary command symlink
if [ "$CMD_NAME" != "kawaii" ]; then
    ln -sf "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$CMD_NAME"
    chmod +x "$INSTALL_DIR/$CMD_NAME"
    echo "  ✓ Command: ${CMD_NAME} → kawaii"
fi

# Create optional alias symlink
if [ -n "$ALIAS_NAME" ] && [ "$ALIAS_NAME" != "$CMD_NAME" ] && [ "$ALIAS_NAME" != "kawaii" ]; then
    ln -sf "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME"
    chmod +x "$INSTALL_DIR/$ALIAS_NAME"
    echo "  ✓ Alias: ${ALIAS_NAME} → kawaii"
fi

# Create config with command name
mkdir -p "$(dirname "$CONFIG_FILE")"
cat > "$CONFIG_FILE" << TOML
name = "${CMD_NAME}"
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

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "  ⚠ $INSTALL_DIR is not in your PATH"
    echo "  Add this to your ~/.bashrc or ~/.zshrc:"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

echo ""
echo "  ✓ Installed to ${INSTALL_DIR}/${BINARY_NAME}"
echo ""
echo "  ${CMD_NAME} -s <pkg>   Search"
echo "  ${CMD_NAME} -i <pkg>   Install"
echo "  ${CMD_NAME} -r <pkg>   Remove"
echo "  ${CMD_NAME} -u         Update"
echo "  ${CMD_NAME} -I <pkg>   Info"
echo "  ${CMD_NAME} -l         List"
echo "  ${CMD_NAME} -C         Clean"
echo "  ${CMD_NAME} -d         Doctor"
echo "  ${CMD_NAME} -c         Config"
echo "  ${CMD_NAME} -v         Version"
