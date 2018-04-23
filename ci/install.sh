set -euxo pipefail

main() {
    rustup target add $TARGET
}

main
