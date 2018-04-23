set -euxo pipefail

main() {
    rustup target add $TARGET

    cargo install cargo-edit || true
}

main
