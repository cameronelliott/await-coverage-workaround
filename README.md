
# await coverage work-around


At the time of publishing, in Rust 1.77 nightly, the `await` keyword often shows
as not-covered in the coverage report. Even when the await was excuted.

Here is the issue:
https://github.com/rust-lang/rust/issues/98712

The issue is not yet resolved, but there is a work-around.

This demo shows how to get 100% coverage for the `await` keyword.

The helper type will still show it's awaits as not-covered, but I
just keep the helper type outside the code I am checking coverage on.
It would be nice if the helper showed 100% coverage, and
there is an abandoned example to attempt that.
But the effort is not worth it at this time.


## install cargo-llvm-cov
```bash
cargo +nightly install cargo-llvm-cov --locked
```

## Run tests with coverage
```bash
cargo llvm-cov run --text
```
