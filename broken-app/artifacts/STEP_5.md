va@Lenovo:~/projects/debuging/broken-app$ cargo bench --bench criterion -- --save-baseline before
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `bench` profile [optimized + debuginfo] target(s) in 2.22s
     Running benches/criterion.rs (target/release/deps/criterion-d8e7ff052d342d7d)
Gnuplot not found, using plotters backend
sum_even_broken/50000   time:   [18.420 µs 18.556 µs 18.691 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
sum_even_broken/1000000 time:   [474.66 µs 477.48 µs 480.39 µs]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

slow_fib_broken/20      time:   [20.373 µs 20.516 µs 20.658 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
slow_fib_broken/28      time:   [965.36 µs 972.93 µs 980.76 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
slow_fib_broken/32      time:   [6.6423 ms 6.6837 ms 6.7248 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

slow_dedup_broken/10000 time:   [10.064 ms 10.200 ms 10.398 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
Benchmarking slow_dedup_broken/100000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 105.9s, or reduce sample count to 10.
slow_dedup_broken/100000
                        time:   [1.0718 s 1.0859 s 1.1019 s]
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

