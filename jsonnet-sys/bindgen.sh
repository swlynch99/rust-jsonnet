#!/usr/bin/bash

set -euo pipefail
set -x

cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"

bindgen jsonnet.h                           \
    --output src/lib.rs                     \
    --raw-line '#![allow(clippy::all)]'     \
    --raw-line '#![allow(rustdoc::all)]'    \
    --blocklist-type wchar_t                \
    --blocklist-type max_align_t            \
    --                                      \
    -I jsonnet/include

# Clean up some doxygen comments to make them render a bit better in rustdoc
sed -i -e 's/\\\\param \([[:alpha:]_]*\)/* `\1` - /g' src/lib.rs
sed -i -e 's/\\\\returns/* returns/g'                 src/lib.rs
sed -i -e 's/\\\\see \([[:alnum:]_]*\)/`\1`/g'        src/lib.rs
