#!/bin/sh

set -eu

REPOSITORY="datavil/framex"
VERSION="${FX_VERSION:-latest}"

say() {
    printf '%s\n' "framex: $*"
}

fail() {
    say "$*" >&2
    exit 1
}

command -v curl >/dev/null 2>&1 || fail "curl is required"
command -v tar >/dev/null 2>&1 || fail "tar is required"

os="$(uname -s)"
arch="$(uname -m)"

case "$os/$arch" in
    Linux/x86_64 | Linux/amd64)
        target="x86_64-unknown-linux-gnu"
        ;;
    Linux/arm64 | Linux/aarch64)
        target="aarch64-unknown-linux-gnu"
        ;;
    Darwin/arm64 | Darwin/aarch64)
        target="aarch64-apple-darwin"
        ;;
    *)
        fail "unsupported platform: $os/$arch"
        ;;
esac

archive="fx-$target.tar.gz"
if [ "$VERSION" = "latest" ]; then
    release_url="https://github.com/$REPOSITORY/releases/latest/download"
else
    VERSION="v${VERSION#v}"
    release_url="https://github.com/$REPOSITORY/releases/download/$VERSION"
fi

if [ -n "${FX_INSTALL_DIR:-}" ]; then
    install_dir="$FX_INSTALL_DIR"
else
    [ -n "${HOME:-}" ] || fail "HOME is not set; set FX_INSTALL_DIR explicitly"
    install_dir="$HOME/.local/bin"
fi

tmp_dir="$(mktemp -d 2>/dev/null || mktemp -d -t framex)"
cleanup() {
    rm -rf "$tmp_dir"
}
trap cleanup EXIT HUP INT TERM

say "downloading $archive"
curl -fLsS --retry 3 "$release_url/$archive" -o "$tmp_dir/$archive"
curl -fLsS --retry 3 "$release_url/$archive.sha256" -o "$tmp_dir/$archive.sha256"

expected_checksum="$(sed 's/[[:space:]].*//' "$tmp_dir/$archive.sha256")"
if command -v sha256sum >/dev/null 2>&1; then
    actual_checksum="$(sha256sum "$tmp_dir/$archive" | sed 's/[[:space:]].*//')"
elif command -v shasum >/dev/null 2>&1; then
    actual_checksum="$(shasum -a 256 "$tmp_dir/$archive" | sed 's/[[:space:]].*//')"
else
    fail "sha256sum or shasum is required to verify the download"
fi

[ "$expected_checksum" = "$actual_checksum" ] || fail "checksum verification failed"

tar -xzf "$tmp_dir/$archive" -C "$tmp_dir"
mkdir -p "$install_dir"
install -m 755 "$tmp_dir/fx" "$install_dir/fx"

say "installed fx to $install_dir/fx"
case ":${PATH:-}:" in
    *":$install_dir:"*) ;;
    *) say "add $install_dir to PATH to run fx" ;;
esac
