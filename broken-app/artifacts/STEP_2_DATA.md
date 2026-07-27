### cargo check (без исправлений)

va@Lenovo:~/projects/debuging/broken-app$ cargo check
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s


### cargo test (без исправлений)


va@Lenovo:~/projects/debuging/broken-app$ cargo test -- --nocapture
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1bab22637453d7a8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-b09258deb65fdbf3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-2923de752302df34)

running 6 tests

thread 'averages_only_positive' (49678) panicked at tests/integration.rs:36:5:
assertion failed: (broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test counts_non_zero_bytes ... ok
test averages_only_positive ... FAILED
test fib_small_numbers ... ok

thread 'sums_even_numbers' (49683) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
thread caused non-unwinding panic. aborting.
test normalize_simple ... ok
test dedup_preserves_uniques ... ok
error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/va/projects/debuging/broken-app/target/debug/deps/integration-2923de752302df34 --nocapture` (signal: 6, SIGABRT: process abort signal)
va@Lenovo:~/projects/debuging/broken-app$

### cargo +nightly miri test (без исправлений)

va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 6 tests
test averages_only_positive ... FAILED
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... error: Undefined Behavior: `assume` called with `false`
  --> src/lib.rs:11:22
   |
11 |             let v = *values.get_unchecked(idx);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: this is on thread `sums_even_numbe`
   = note: stack backtrace:
           0: broken_app::sum_even
               at src/lib.rs:11:22: 11:47
           1: sums_even_numbers
               at tests/integration.rs:7:16: 7:31
           2: sums_even_numbers::{closure#0}
               at tests/integration.rs:4:23: 4:23

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner /home/va/projects/debuging/broken-app/target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8` (exit status: 1)
note: test exited abnormally; to see the full output pass --no-capture to the harness.
va@Lenovo:~/projects/debuging/broken-app$

### AddressSanitizer (первичный)

RUSTFLAGS="-Zsanitizer=address" \ 
cargo +nightly test -Zbuild-std \
--target x86_64-unknown-linux-gnu \
--tests -- --nocapture

...

   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
   Compiling criterion v0.5.1
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 37.22s
     Running unittests src/lib.rs (target/x86_64-unknown-linux-gnu/debug/deps/broken_app-408689b81ec63103)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/x86_64-unknown-linux-gnu/debug/deps/demo-8400e77a61969a6e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/x86_64-unknown-linux-gnu/debug/deps/integration-81131c0937a803d8)

running 6 tests

thread 'averages_only_positive' (53531) panicked at tests/integration.rs:36:5:
assertion failed: (broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test counts_non_zero_bytes ... ok

thread 'sums_even_numbers' (53536) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
test dedup_preserves_uniques ... thread caused non-unwinding panic. aborting.
ok
test averages_only_positive ... FAILED
error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/va/projects/debuging/broken-app/target/x86_64-unknown-linux-gnu/debug/deps/integration-81131c0937a803d8 --nocapture` (signal: 6, SIGABRT: process abort signal)
va@Lenovo:~/projects/debuging/broken-app$


## fix sum_even + test

pub fn sum_even(values: &[i64]) -> i64 {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .sum()
}

#[test]
fn sums_even_numbers_empty_slice() {
    assert_eq!(broken_app::sum_even(&[]), 0);
}

### cargo test --test integration sums_even_numbers -- --nocapture

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:57:15
   |
57 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:54:1
   |
54 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:58:10
   |
58 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:59:11
   |
59 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

   Compiling regex v1.12.2
   Compiling ciborium v0.2.2
   Compiling oorandom v11.1.5
For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
   Compiling tinytemplate v1.2.1
   Compiling criterion v0.5.1
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.74s
     Running tests/integration.rs (target/debug/deps/integration-2923de752302df34)

running 2 tests
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

### cargo +nightly miri test --test integration sums_even_numbers

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:57:15
   |
57 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:54:1
   |
54 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:58:10
   |
58 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:59:11
   |
59 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

   Compiling clap v4.5.53
For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
   Compiling regex v1.12.2
   Compiling zerocopy-derive v0.8.31
   Compiling serde_derive v1.0.228
   Compiling half v2.7.1
   Compiling ciborium-ll v0.2.2
   Compiling tinytemplate v1.2.1
   Compiling ciborium v0.2.2
   Compiling criterion v0.5.1
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.55s
     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 2 tests
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.29s


### fix average_positive

va@Lenovo:~/projects/debuging/broken-app$ cargo test --test integration averages_only_positive -- --nocapture
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:67:15
   |
67 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:64:1
   |
64 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:68:10
   |
68 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:69:11
   |
69 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running tests/integration.rs (target/debug/deps/integration-2923de752302df34)

running 1 test
test averages_only_positive ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

va@Lenovo:~/projects/debuging/broken-app$

va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:67:15
   |
67 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:64:1
   |
64 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:68:10
   |
68 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:69:11
   |
69 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:67:15
   |
67 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:64:1
   |
64 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:68:10
   |
68 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:69:11
   |
69 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 7 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.80s

error: memory leaked: alloc48670 (Rust heap, size: 5, align: 1), allocated here:
   --> /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:466:41
    |
466 |             AllocInit::Uninitialized => alloc.allocate(layout),
    |                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: stack backtrace:
            0: alloc::raw_vec::RawVecInner::try_allocate_in
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:466:41: 466:63
            1: alloc::raw_vec::RawVecInner::with_capacity_in
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:435:15: 435:92
            2: alloc::raw_vec::RawVec::<u8>::with_capacity_in
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20: 177:77
            3: std::vec::Vec::<u8>::with_capacity_in
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:975:20: 975:61
            4: <u8 as std::slice::<impl [T]>::to_vec_in::ConvertVec>::to_vec::<std::alloc::Global>
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:448:29: 448:62
            5: std::slice::<impl [u8]>::to_vec_in::<std::alloc::Global>
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:400:16: 400:38
            6: std::slice::<impl [u8]>::to_vec
                at /home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:376:9: 376:31
            7: broken_app::leak_buffer
                at src/lib.rs:20:17: 20:31
            8: counts_non_zero_bytes
                at tests/integration.rs:13:16: 13:34
            9: counts_non_zero_bytes::{closure#0}
                at tests/integration.rs:11:27: 11:27

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

note: set `MIRIFLAGS=-Zmiri-ignore-leaks` to disable this check

error: aborting due to 1 previous error

error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner /home/va/projects/debuging/broken-app/target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8` (exit status: 1)
note: test exited abnormally; to see the full output pass --no-capture to the harness.


### error: memory leaked
### broken_app::leak_buffer
### src/lib.rs:20

va@Lenovo:~/projects/debuging/broken-app$ valgrind --leak-check=full cargo test --tests
==63648== Memcheck, a memory error detector
==63648== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==63648== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==63648== Command: cargo test --tests
==63648==
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:67:15
   |
67 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:64:1
   |
64 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:68:10
   |
68 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:69:11
   |
69 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1bab22637453d7a8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-b09258deb65fdbf3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-2923de752302df34)

running 7 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

va@Lenovo:~/projects/debuging/broken-app$ cargo test --no-run
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:67:15
   |
67 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:64:1
   |
64 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:68:10
   |
68 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:69:11
   |
69 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
  Executable unittests src/lib.rs (target/debug/deps/broken_app-1bab22637453d7a8)
  Executable unittests src/bin/demo.rs (target/debug/deps/demo-b09258deb65fdbf3)
  Executable tests/integration.rs (target/debug/deps/integration-2923de752302df34)
va@Lenovo:~/projects/debuging/broken-app$ ls target/debug/deps/integration-*
target/debug/deps/integration-2923de752302df34  target/debug/deps/integration-2923de752302df34.d
va@Lenovo:~/projects/debuging/broken-app$ valgrind \
  --leak-check=full \
  --show-leak-kinds=all \
  --track-origins=yes \
  ./target/debug/deps/integration-2923de752302df34
==63893== Memcheck, a memory error detector
==63893== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==63893== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==63893== Command: ./target/debug/deps/integration-2923de752302df34
==63893==

running 7 tests
test counts_non_zero_bytes ... ok
test sums_even_numbers ... ok
test normalize_simple ... ok
test dedup_preserves_uniques ... ok
test sums_even_numbers_empty_slice ... ok
test fib_small_numbers ... ok
test averages_only_positive ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

==63893==
==63893== HEAP SUMMARY:
==63893==     in use at exit: 597 bytes in 3 blocks
==63893==   total heap usage: 814 allocs, 811 frees, 92,267 bytes allocated
==63893==
==63893== 5 bytes in 1 blocks are definitely lost in loss record 1 of 3
==63893==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==63893==    by 0x1C7B39: _RNvMs4_NtCsgW8esjfipvk_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs8clXCmsS0Ah_5gimli (in /home/va/projects/debuging/broken-app/target/debug/deps/integration-2923de752302df34)
==63893==    by 0x187647: alloc::raw_vec::RawVecInner<A>::with_capacity_in (mod.rs:434)
==63893==    by 0x17FAF5: <T as alloc::slice::<impl [T]>::to_vec_in::ConvertVec>::to_vec (mod.rs:177)
==63893==    by 0x17FA9B: alloc::slice::<impl [T]>::to_vec (slice.rs:400)
==63893==    by 0x18500C: broken_app::leak_buffer (lib.rs:20)
==63893==    by 0x140F6C: integration::counts_non_zero_bytes (integration.rs:13)
==63893==    by 0x140276: integration::counts_non_zero_bytes::{{closure}} (integration.rs:11)
==63893==    by 0x140475: core::ops::function::FnOnce::call_once (function.rs:250)
==63893==    by 0x14154A: _RINvCs7Gciw6hfIAV_4test28___rust_begin_short_backtraceINtNtCs27Vx93FoQ6z_4core6result6ResultuNtNtCsgW8esjfipvk_5alloc6string6StringEFEBQ_EB2_ (function.rs:250)
==63893==    by 0x14DF1A: _RNCNvCs7Gciw6hfIAV_4test8run_test0B3_ (lib.rs:686)
==63893==    by 0x149633: _RINvNtNtCs75vJTIYSa2J_3std3sys9backtrace28___rust_begin_short_backtraceNCNvCs7Gciw6hfIAV_4test8run_tests_0uEB1b_ (lib.rs:637)
==63893==
==63893== 48 bytes in 1 blocks are possibly lost in loss record 2 of 3
==63893==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==63893==    by 0x1AC46B: _RNvMs_NtNtCs75vJTIYSa2J_3std6thread6threadNtB4_6Thread3new (unix.rs:14)
==63893==    by 0x1B9E7F: _RNvNtNtCs75vJTIYSa2J_3std6thread7current12init_current (current.rs:294)
==63893==    by 0x1A16E5: _RNvMNtNtNtCs75vJTIYSa2J_3std4sync4mpmc7contextNtB2_7Context3new (current.rs:251)
==63893==    by 0x141C8A: _RINvMs0_NtNtNtNtCs75vJTIYSa2J_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCs27Vx93FoQ6z_4core4cell4CellINtNtB1j_6option6OptionNtNtNtNtBe_4sync4mpmc7context7ContextEEuE16get_or_init_slowNvNvNvMB2b_B29_4with7CONTEXT27___rust_std_internal_init_fnECs7Gciw6hfIAV_4test (context.rs:43)
==63893==    by 0x158C42: _RNvMs1_NtNtNtCs75vJTIYSa2J_3std4sync4mpmc4listINtB5_7ChannelNtNtCs7Gciw6hfIAV_4test5event13CompletedTestE4recvB10_ (lazy.rs:62)
==63893==    by 0x165AB3: _RNvNtCs7Gciw6hfIAV_4test7console17run_tests_console (mod.rs:1147)
==63893==    by 0x154058: _RNvCs7Gciw6hfIAV_4test9test_main (lib.rs:160)
==63893==    by 0x1518AA: _RNvCs7Gciw6hfIAV_4test16test_main_static (lib.rs:183)
==63893==    by 0x1412D2: integration::main (integration.rs:1)
==63893==    by 0x1404EA: core::ops::function::FnOnce::call_once (function.rs:250)
==63893==    by 0x14130D: std::sys::backtrace::__rust_begin_short_backtrace (backtrace.rs:166)
==63893==
==63893== 544 bytes in 1 blocks are still reachable in loss record 3 of 3
==63893==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==63893==    by 0x1BEA49: _RNvNtNtNtNtNtCs75vJTIYSa2J_3std3sys3pal4unix14stack_overflow11thread_info16set_current_info (alloc.rs:101)
==63893==    by 0x1B7B9C: _RNvNtCs75vJTIYSa2J_3std2rt19lang_start_internal (stack_overflow.rs:179)
==63893==    by 0x140026: std::rt::lang_start (rt.rs:205)
==63893==    by 0x1412FD: main (in /home/va/projects/debuging/broken-app/target/debug/deps/integration-2923de752302df34)
==63893==
==63893== LEAK SUMMARY:
==63893==    definitely lost: 5 bytes in 1 blocks
==63893==    indirectly lost: 0 bytes in 0 blocks
==63893==      possibly lost: 48 bytes in 1 blocks
==63893==    still reachable: 544 bytes in 1 blocks
==63893==         suppressed: 0 bytes in 0 blocks
==63893==
==63893== For lists of detected and suppressed errors, rerun with: -s
==63893== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 0 from 0)
va@Lenovo:~/projects/debuging/broken-app$


### cargo test --test integration counts_non_zero_bytes

va@Lenovo:~/projects/debuging/broken-app$ cargo test --test integration counts_non_zero_bytes
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:71:15
   |
71 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:68:1
   |
68 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:72:10
   |
72 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running tests/integration.rs (target/debug/deps/integration-2923de752302df34)

running 1 test
test counts_non_zero_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s


### va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test

   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:71:15
   |
71 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:68:1
   |
68 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:72:10
   |
72 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

   Compiling clap v4.5.53
For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
   Compiling regex v1.12.2
   Compiling zerocopy-derive v0.8.31
   Compiling serde_derive v1.0.228
   Compiling half v2.7.1
   Compiling ciborium-ll v0.2.2
   Compiling tinytemplate v1.2.1
   Compiling ciborium v0.2.2
   Compiling criterion v0.5.1
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.37s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:71:15
   |
71 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:68:1
   |
68 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:72:10
   |
72 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 7 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.81s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

va@Lenovo:~/projects/debuging/broken-app$

### cargo +nightly miri test detects_use_after_free -- --nocapture

va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test detects_use_after_free -- --nocapture
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:71:15
   |
71 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:68:1
   |
68 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:72:10
   |
72 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:71:15
   |
71 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:68:1
   |
68 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:72:10
   |
72 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 1 test
test detects_use_after_free ... error: Undefined Behavior: memory access failed: alloc43162 has been freed, so this pointer is dangling
  --> src/lib.rs:73:11
   |
73 |     val + *raw
   |           ^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc43162 was allocated here:
  --> src/lib.rs:69:13
   |
69 |     let b = Box::new(42_i32);
   |             ^^^^^^^^^^^^^^^^
help: alloc43162 was deallocated here:
  --> src/lib.rs:72:5
   |
72 |     drop(Box::from_raw(raw));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this is on thread `detects_use_aft`
   = note: stack backtrace:
           0: broken_app::use_after_free
               at src/lib.rs:73:11: 73:15
           1: detects_use_after_free
               at tests/integration.rs:48:17: 48:45
           2: detects_use_after_free::{closure#0}
               at tests/integration.rs:46:28: 46:28

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/va/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner /home/va/projects/debuging/broken-app/target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8 detects_use_after_free --nocapture` (exit status: 1)
va@Lenovo:~/projects/debuging/broken-app$

### fix  use_after_free
pub fn use_after_free() -> i32 {
    let value = 42;
    value + value
}

### cargo test




va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 9 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test detects_use_after_free ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.12s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

va@Lenovo:~/projects/debuging/broken-app$


### va@Lenovo:~/projects/debuging/broken-app$ ls -lt target/debug/deps/integration-* | head
-rwxr-xr-x 1 va va 7534768 Jul 27 23:06 target/debug/deps/integration-2923de752302df34
-rw-r--r-- 1 va va     240 Jul 27 23:06 target/debug/deps/integration-2923de752302df34.d
-rw-r--r-- 1 va va     249 Jul 27 23:05 target/debug/deps/integration-dc6ee3caf32d0fc7.d
va@Lenovo:~/projects/debuging/broken-app$ valgrind \
  --leak-check=full \
  --show-leak-kinds=all \
  --track-origins=yes \
  ./target/debug/deps/integration-2923de752302df34
==73985== Memcheck, a memory error detector
==73985== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==73985== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==73985== Command: ./target/debug/deps/integration-2923de752302df34
==73985==

running 8 tests
test sums_even_numbers ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test averages_only_positive ... ok
test sums_even_numbers_empty_slice ... ok
test counts_non_zero_bytes ... ok
test use_after_free_returns_expected_value ... ok
test normalize_simple ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

==73985==
==73985== HEAP SUMMARY:
==73985==     in use at exit: 544 bytes in 1 blocks
==73985==   total heap usage: 837 allocs, 836 frees, 94,377 bytes allocated
==73985==
==73985== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==73985==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==73985==    by 0x1BD8A9: _RNvNtNtNtNtNtCs75vJTIYSa2J_3std3sys3pal4unix14stack_overflow11thread_info16set_current_info (alloc.rs:101)
==73985==    by 0x1B69FC: _RNvNtCs75vJTIYSa2J_3std2rt19lang_start_internal (stack_overflow.rs:179)
==73985==    by 0x13FA96: std::rt::lang_start (rt.rs:205)
==73985==    by 0x14119D: main (in /home/va/projects/debuging/broken-app/target/debug/deps/integration-2923de752302df34)
==73985==
==73985== LEAK SUMMARY:
==73985==    definitely lost: 0 bytes in 0 blocks
==73985==    indirectly lost: 0 bytes in 0 blocks
==73985==      possibly lost: 0 bytes in 0 blocks
==73985==    still reachable: 544 bytes in 1 blocks
==73985==         suppressed: 0 bytes in 0 blocks
==73985==
==73985== For lists of detected and suppressed errors, rerun with: -s
==73985== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
va@Lenovo:~/projects/debuging/broken-app$


### RUSTFLAGS="-Zsanitizer=address" \
cargo +nightly test \
  --target x86_64-unknown-linux-gnu

   Finished `test` profile [unoptimized + debuginfo] target(s) in 17.91s
     Running unittests src/lib.rs (target/x86_64-unknown-linux-gnu/debug/deps/broken_app-823f9bd8cd006800)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/x86_64-unknown-linux-gnu/debug/deps/demo-41f520e4cd023b42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/x86_64-unknown-linux-gnu/debug/deps/integration-6383544173c3801d)

running 8 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test fib_small_numbers ... ok
test dedup_preserves_uniques ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

### RUSTFLAGS="-Zsanitizer=leak" \
cargo +nightly test \
  --target x86_64-unknown-linux-gnu


      Finished `test` profile [unoptimized + debuginfo] target(s) in 11.28s
     Running unittests src/lib.rs (target/x86_64-unknown-linux-gnu/debug/deps/broken_app-20cac4b893548ef6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/x86_64-unknown-linux-gnu/debug/deps/demo-cef0510c350ecb24)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/x86_64-unknown-linux-gnu/debug/deps/integration-8541d5640a32d9db)

running 8 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

### RUSTFLAGS="-Zsanitizer=undefined" \
cargo +nightly test \
  --target x86_64-unknown-linux-gnu

  UndefinedBehaviorSanitizer не запускался, поскольку текущая версия Rust nightly
не поддерживает значение `undefined` для опции `-Zsanitizer`.
Проверка неопределённого поведения выполнена с помощью Miri.