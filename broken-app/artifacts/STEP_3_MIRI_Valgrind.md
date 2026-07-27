va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 8 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.90s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

va@Lenovo:~/projects/debuging/broken-app$ cargo test --no-run
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s
  Executable unittests src/lib.rs (target/debug/deps/broken_app-1bab22637453d7a8)
  Executable unittests src/bin/demo.rs (target/debug/deps/demo-b09258deb65fdbf3)
  Executable tests/integration.rs (target/debug/deps/integration-2923de752302df34)
va@Lenovo:~/projects/debuging/broken-app$ ls target/debug/deps/integration-*
target/debug/deps/integration-2923de752302df34  target/debug/deps/integration-2923de752302df34.d  target/debug/deps/integration-dc6ee3caf32d0fc7.d
va@Lenovo:~/projects/debuging/broken-app$ valgrind \
  --leak-check=full \
  --show-leak-kinds=all \
  --track-origins=yes \
  ./target/debug/deps/integration-2923de752302df34
==80361== Memcheck, a memory error detector
==80361== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==80361== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==80361== Command: ./target/debug/deps/integration-2923de752302df34
==80361==

running 8 tests
test fib_small_numbers ... ok
test dedup_preserves_uniques ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test normalize_simple ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

==80361==
==80361== HEAP SUMMARY:
==80361==     in use at exit: 544 bytes in 1 blocks
==80361==   total heap usage: 837 allocs, 836 frees, 94,377 bytes allocated
==80361==
==80361== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==80361==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==80361==    by 0x1BD8A9: _RNvNtNtNtNtNtCs75vJTIYSa2J_3std3sys3pal4unix14stack_overflow11thread_info16set_current_info (alloc.rs:101)
==80361==    by 0x1B69FC: _RNvNtCs75vJTIYSa2J_3std2rt19lang_start_internal (stack_overflow.rs:179)
==80361==    by 0x13FA96: std::rt::lang_start (rt.rs:205)
==80361==    by 0x14119D: main (in /home/va/projects/debuging/broken-app/target/debug/deps/integration-2923de752302df34)
==80361==
==80361== LEAK SUMMARY:
==80361==    definitely lost: 0 bytes in 0 blocks
==80361==    indirectly lost: 0 bytes in 0 blocks
==80361==      possibly lost: 0 bytes in 0 blocks
==80361==    still reachable: 544 bytes in 1 blocks
==80361==         suppressed: 0 bytes in 0 blocks
==80361==
==80361== For lists of detected and suppressed errors, rerun with: -s
==80361== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
va@Lenovo:~/projects/debuging/broken-app$