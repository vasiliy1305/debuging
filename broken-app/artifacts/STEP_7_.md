### cargo test 2>&1 | tee artifacts/after/cargo_test.txt
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/broken_app-41b486fd5cad65f8)

running 2 tests
test algo::tests::dedup_returns_sorted_unique_values ... ok
test algo::tests::fibonacci_values_are_correct ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-923475ee7ce280dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-b247f6ad4293b9a1)

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

### cargo test --release 2>&1 | tee artifacts/after/cargo_test_release.txt

   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (target/release/deps/broken_app-bc1aecd9f9eac37f)

running 2 tests
test algo::tests::dedup_returns_sorted_unique_values ... ok
test algo::tests::fibonacci_values_are_correct ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/release/deps/demo-1487c2bf77225f41)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/release/deps/integration-1bba9a0d73fb02f5)

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



### cargo +nightly miri test \
  2>&1 | tee artifacts/after/miri.txt

  va@Lenovo:~/projects/debuging/broken-app$ cargo +nightly miri test \
  2>&1 | tee artifacts/after/miri.txt
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
   Compiling criterion v0.5.1
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.17s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-278cc9ac7f7cd46d)

running 2 tests
test algo::tests::dedup_returns_sorted_unique_values ... ok
test algo::tests::fibonacci_values_are_correct ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-b779158de902388b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2638644b9d37d372)

running 8 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test sums_even_numbers_empty_slice ... ok
test use_after_free_returns_expected_value ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.95s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

### valgrind \
  --tool=memcheck \
  --leak-check=full \
  --show-leak-kinds=all \
  --track-origins=yes \
  --error-exitcode=1 \
  ./target/debug/demo \
  2>&1 | tee artifacts/after/valgrind.txt

  dedup: [1, 2, 3, 4]
==4348==
==4348== HEAP SUMMARY:
==4348==     in use at exit: 544 bytes in 1 blocks
==4348==   total heap usage: 40,010 allocs, 40,009 frees, 903,644 bytes allocated
==4348==
==4348== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==4348==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==4348==    by 0x151BF9: _RNvNtNtNtNtNtCs75vJTIYSa2J_3std3sys3pal4unix14stack_overflow11thread_info16set_current_info (alloc.rs:101)
==4348==    by 0x14F25C: _RNvNtCs75vJTIYSa2J_3std2rt19lang_start_internal (stack_overflow.rs:179)
==4348==    by 0x121C26: std::rt::lang_start (rt.rs:205)
==4348==    by 0x121B4D: main (in /home/va/projects/debuging/broken-app/target/debug/demo)
==4348==
==4348== LEAK SUMMARY:
==4348==    definitely lost: 0 bytes in 0 blocks
==4348==    indirectly lost: 0 bytes in 0 blocks
==4348==      possibly lost: 0 bytes in 0 blocks
==4348==    still reachable: 544 bytes in 1 blocks
==4348==         suppressed: 0 bytes in 0 blocks
==4348==
==4348== For lists of detected and suppressed errors, rerun with: -s
==4348== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

