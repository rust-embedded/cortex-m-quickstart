set -euxo pipefail

main() {
    local td=$(mktemp -d)

    git clone . $td
    pushd $td

    cat >memory.x <<'EOF'
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
EOF

    local examples=(
        crash
        exception
        hello
        minimal
        panic
    )
    for ex in "${examples[@]}"; do
        cargo build --target $TARGET --example $ex
        cargo build --target $TARGET --example $ex --release
    done

    # ITM is not available on Cortex-M0
    if [ $TARGET != thumbv6m-none-eabi ]; then
        local ex=itm
        cargo build --target $TARGET --example $ex
        cargo build --target $TARGET --example $ex --release

        examples+=( $ex )
    fi

    # Allocator example needs an extra dependency
    cat >>Cargo.toml <<'EOF'
[dependencies.alloc-cortex-m]
version = "0.3.4"
EOF

    local ex=allocator
    cargo build --target $TARGET --example $ex --release

    examples+=( $ex )

    # Device example needs an extra dependency
    if [ $TARGET = thumbv7m-none-eabi ]; then
        cat >>Cargo.toml <<'EOF'
[dependencies.stm32f103xx]
features = ["rt"]
version = "0.10.0"
EOF

        local ex=device
        cargo build --target $TARGET --example $ex
        cargo build --target $TARGET --example $ex --release

        examples+=( $ex )
    fi

    IFS=,;eval size target/$TARGET/release/examples/"{${examples[*]}}"

    popd
    rm -rf $td
}

if [ $TARGET != x86_64-unknown-linux-gnu ]; then
    main
fi
