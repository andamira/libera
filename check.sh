#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.71.1" # in sync with ./Cargo.toml & .github/workflows/check.yml
RCMD="rustup -v run $MSRV"

rustup override set $MSRV
rustup target add thumbv7m-none-eabi

cmd="$RCMD cargo c"; echo "std, safe\n$ " $cmd; $cmd
cmd="$RCMD cargo cu"; echo "std, unsafe\n$" $cmd; $cmd
cmd="$RCMD cargo cn"; echo "no-std, safe\n$" $cmd; $cmd
cmd="$RCMD cargo cNu"; echo "no-std, no-alloc, unsafe\n$" $cmd; $cmd

cmd="$RCMD cargo t"; echo "tests\n$" $cmd; $cmd
cmd="$RCMD cargo tu"; echo "tests\n$" $cmd; $cmd

cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd