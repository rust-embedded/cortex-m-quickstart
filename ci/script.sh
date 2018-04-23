set -euxo pipefail

main() {
    local td=$(mktemp -d)

    git clone . $td
    pushd $td

    cat >memory.x <<'EOF'
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
EOF

    local examples=(
        crash
        # device
        hello
        itm
        override-exception-handler
        panic
    )
    for ex in "${examples[@]}"; do
        cargo build --target $TARGET --example $ex
        cargo build --target $TARGET --example $ex --release
    done

    cargo add alloc-cortex-m

    cargo build --target $TARGET --example allocator
    cargo build --target $TARGET --example allocator --release

    examples+=( allocator )

    if [ $TARGET = thumbv7m-none-eabi ]; then
        cat >>Cargo.toml <<'EOF'
[dependencies.stm32f103xx]
features = ["rt"]
version = "0.9.0"
EOF

        cargo build --target $TARGET --example device
        cargo build --target $TARGET --example device --release

        examples+=( device )
    fi

    IFS=,;eval arm-none-eabi-size target/$TARGET/release/examples/"{${examples[*]}}"

    popd
    rm -rf $td
}

main
