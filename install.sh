#!/bin/sh
set -eu

REPO="qiz-li/feeling"
BINARY="feeling"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

get_arch() {
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64) echo "x86_64" ;;
        arm64|aarch64) echo "aarch64" ;;
        *) echo "error: unsupported architecture: $arch" >&2; exit 1 ;;
    esac
}

get_os() {
    os=$(uname -s)
    case "$os" in
        Linux) echo "unknown-linux-musl" ;;
        Darwin) echo "apple-darwin" ;;
        *) echo "error: unsupported OS: $os" >&2; exit 1 ;;
    esac
}

main() {
    arch=$(get_arch)
    os=$(get_os)
    target="${arch}-${os}"

    if [ -n "${1:-}" ]; then
        tag="$1"
    else
        tag=$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
    fi

    url="https://github.com/${REPO}/releases/download/${tag}/${BINARY}-${target}.tar.gz"

    echo "Installing ${BINARY} ${tag} (${target})..."

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    curl -sSfL "$url" | tar xz -C "$tmpdir"

    mkdir -p "$INSTALL_DIR"
    mv "$tmpdir/$BINARY" "$INSTALL_DIR/$BINARY"
    chmod +x "$INSTALL_DIR/$BINARY"

    echo "Installed to ${INSTALL_DIR}/${BINARY}"

    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        echo ""
        echo "Add to your PATH:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

main "$@"
