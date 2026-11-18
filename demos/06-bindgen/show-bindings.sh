#!/usr/bin/env bash
# Print part of the bindings bindgen generated, trimmed and wrapped for slides.
#
#   show-bindings.sh struct      the #[repr(C)] struct and its layout checks
#   show-bindings.sh functions   the unsafe extern "C" declarations
#   show-bindings.sh             the whole file
#
# Run `cargo build -p bindgen-demo` first: this only reads what it produced.
set -euo pipefail
cd "$(dirname "$0")/.."

# Cargo's build folder has a hash in its name; take the most recent one.
bindings="$(ls -t target/debug/build/bindgen-demo-*/out/bindings.rs | head -1)"

# bindgen emits lines over 90 columns wide; re-wrap them to fit a slide.
wrapped() { rustfmt --config max_width=72 < "$bindings"; }

case "${1:-all}" in
    struct)    wrapped | sed -n '/repr(C)/,/^};/p' ;;
    functions) wrapped | sed -n '/^unsafe extern/,$p' ;;
    all)       wrapped ;;
    *)         echo "usage: $0 [struct|functions]" >&2; exit 2 ;;
esac
