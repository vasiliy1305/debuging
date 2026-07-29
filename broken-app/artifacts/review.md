### Добавил тест:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn race_increment_counts_all_iterations() {
        let threads = 8;
        let iterations = 10_000;

        let result = race_increment(threads, iterations);

        assert_eq!(result, (threads * iterations) as u64);
    }
}

### Тест провалился:

va@Lenovo:~/projects/debuging/broken-app$ cargo test race_increment_counts_all_iterations
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/broken_app-41b486fd5cad65f8)

running 1 test
test concurrency::tests::race_increment_counts_all_iterations ... FAILED

failures:

---- concurrency::tests::race_increment_counts_all_iterations stdout ----

thread 'concurrency::tests::race_increment_counts_all_iterations' (56900) panicked at src/concurrency.rs:53:9:
assertion `left == right` failed
  left: 79999
 right: 80000
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    concurrency::tests::race_increment_counts_all_iterations

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.56s

error: test failed, to rerun pass `--lib`

### Поменял на:
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn read_after_sleep() -> u64 {
    // thread::sleep(Duration::from_millis(10)); 
    COUNTER.load(Ordering::SeqCst)
}


### Повторная проверка:
va@Lenovo:~/projects/debuging/broken-app$ cargo test race_increment_counts_all_iterations
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running unittests src/lib.rs (target/debug/deps/broken_app-41b486fd5cad65f8)

running 1 test
test concurrency::tests::race_increment_counts_all_iterations ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.57s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-923475ee7ce280dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-b247f6ad4293b9a1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

### поправил:

pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

### тест:
test normalize_removes_all_ws ... ok
