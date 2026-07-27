### va@Lenovo:~/projects/debuging/broken-app$ cargo build --release --bin demo
   Compiling broken-app v0.1.0 (/home/va/projects/debuging/broken-app)
    Finished `release` profile [optimized + debuginfo] target(s) in 1.31s

### perf record -g ./target/release/demo

### perf report

Samples: 13  of event 'cycles:u', Event count (approx.): 624503
  Children      Self  Command  Shared Object         Symbol
+   42.38%     0.00%  demo     [unknown]             [.] 0x0000000000000040                                                                                ◆
+   42.38%     0.00%  demo     ld-linux-x86-64.so.2  [.] _dl_sysdep_start                                                                                  ▒
+   42.38%     0.00%  demo     ld-linux-x86-64.so.2  [.] dl_main                                                                                           ▒
+   32.39%    32.39%  demo     libc.so.6             [.] sysmalloc                                                                                         ▒
+   23.38%    23.38%  demo     libc.so.6             [.] memcpy@@GLIBC_2.14                                                                                ▒
+   19.00%    19.00%  demo     ld-linux-x86-64.so.2  [.] _dl_sort_maps                                                                                     ▒
+   19.00%     0.00%  demo     ld-linux-x86-64.so.2  [.] _dl_map_object_deps                                                                               ▒
+   10.93%    10.93%  demo     ld-linux-x86-64.so.2  [.] strcmp                                                                                            ▒
+    6.78%     6.78%  demo     ld-linux-x86-64.so.2  [.] _dl_start                                                                                         ▒
+    6.78%     0.00%  demo     ld-linux-x86-64.so.2  [.] _dl_start_user                                                                                    ▒
+    6.49%     6.49%  demo     ld-linux-x86-64.so.2  [.] intel_check_word.constprop.0                                                                      ▒
+    6.49%     0.00%  demo     [unknown]             [.] 0x0000000004000000                                                                                ▒
+    1.03%     0.14%  demo     ld-linux-x86-64.so.2  [.] _start                                                                                            ▒
+    0.88%     0.88%  demo     [unknown]             [k] 0xffffffff97000be0        




### cargo flamegraph \
  --release \
  --bin demo \
  --output flamegraph.svg


увеличил чило итераций в main

большую часть времени занимает
broken_app::algo::slow_fib - 89.56%