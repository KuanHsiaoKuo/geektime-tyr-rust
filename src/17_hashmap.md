# 解剖哈希表数据结构

<!--ts-->
* [解剖哈希表数据结构](#解剖哈希表数据结构)
   * [bin配置与运行](#bin配置与运行)
   * [目标调试代码](#目标调试代码)
   * [使用gdb/lldb进行调试查看内存结构](#使用gdblldb进行调试查看内存结构)
      * [开始调试](#开始调试)
      * [b(reakpoint): 添加断点](#breakpoint-添加断点)
      * [r(un):运行到断点](#run运行到断点)
      * [c(ontinue):继续单步执行](#continue继续单步执行)
      * [x: 打印内存地址](#x-打印内存地址)
      * [c(tinue): 继续执行到下一个断点](#ctinue-继续执行到下一个断点)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Sep 18 05:39:46 UTC 2022 -->

<!--te-->

## bin配置与运行

- [Cargo Targets - The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=bin#binaries)

```toml
{{#include ../geektime_rust_codes/17_hash_table/Cargo.toml}}
```

> 如果要单独运行指定bin文件：

```shell
cargo run --bin hashmap2
```

## 目标调试代码

```rust, editable
{{#include ../geektime_rust_codes/17_hash_table/src/hashmap2.rs}}
```

## 使用gdb/lldb进行调试查看内存结构

1. gdb: 主要是linux系统

2. 主要OSX系统

### 开始调试

```shell
rust-lldb target/debug/hashmap2                                                                                                                  ─╯
(lldb) command script import "/Users/kuanhsiaokuo/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_lookup.py"
(lldb) command source -s 0 '/Users/kuanhsiaokuo/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_commands'
Executing commands in '/Users/kuanhsiaokuo/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_commands'.
(lldb) type synthetic add -l lldb_lookup.synthetic_lookup -x ".*" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)String$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&(mut )?str$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&(mut )?\\[.+\\]$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::ffi::([a-z_]+::)+)OsString$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Vec<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)VecDeque<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Rc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Arc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Cell<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Ref<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefMut<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefCell<.+>$" --category Rust
(lldb) type category enable Rust
(lldb) target create "target/debug/hashmap2"
Current executable set to '/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/hashmap2' (x86_64).
(lldb)
```

### b(reakpoint): 添加断点

> 在32行打断点，方便看std::mem::transmute(arr)

```shell
(lldb) b hashmap2.rs:32
Breakpoint 1: where = hashmap2`hashmap2::explain::h4091c852f38a0de4 + 406 at hashmap2.rs:32:34, address = 0x0000000100008d16
```

### r(un):运行到断点

```shell
(lldb) r
Process 69337 launched: '/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/hashmap2' (x86_64)
empty: bucket_mask 0x0, ctrl 0x100043d20, growth_left: 0, items: 0
Process 69337 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100008d16 hashmap2`hashmap2::explain::h4091c852f38a0de4(name="empty", map=<unavailable>) at hashmap2.rs:32:34
   29           "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
   30           name, arr[2], arr[3], arr[4], arr[5]
   31       );
-> 32       unsafe { std::mem::transmute(arr) }
   33   }
Target 0: (hashmap2) stopped.
```

```shell
# 最初的状态，哈希表为空
empty: bucket_mask 0x0, ctrl 0x100043d20, growth_left: 0, items: 0
```

### c(ontinue):继续单步执行

```shell
(lldb) c
Process 69337 resuming
added 1: bucket_mask 0x3, ctrl 0x600001700160, growth_left: 2, items: 1
Process 69337 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100008d16 hashmap2`hashmap2::explain::h4091c852f38a0de4(name="added 1", map=<unavailable>) at hashmap2.rs:32:34
   29           "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
   30           name, arr[2], arr[3], arr[4], arr[5]
   31       );
-> 32       unsafe { std::mem::transmute(arr) }
   33   }
Target 0: (hashmap2) stopped.
```

```shell
# 插入了一个元素后，bucket 有 4 个（0x3+1），堆地址起始位置 0x600001700160 - 4*8(0x20)
added 1: bucket_mask 0x3, ctrl 0x600001700160, growth_left: 2, items: 1
```

### x: 打印内存地址

- [打印内存的值 | 100个gdb小技巧](https://wizardforcel.gitbooks.io/100-gdb-tips/content/examine-memory.html)

```shell
# 以12进制打印从内存地址开始的值
(lldb) x/12x 0x600001700160
0x600001700160: 0xffff6dff 0xffffffff 0xffffffff 0xffffffff
0x600001700170: 0xffff6dff 0x00000000 0x00000000 0x00000000
0x600001700180: 0x20ec913f 0x00007ff8 0x4e5ef01e 0x00000000
```

### c(tinue): 继续执行到下一个断点

```shell
(lldb) c
Process 69337 resuming
added 3: bucket_mask 0x3, ctrl 0x600001700160, growth_left: 0, items: 3
Process 69337 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100008d16 hashmap2`hashmap2::explain::h4091c852f38a0de4(name="added 3", map=<unavailable>) at hashmap2.rs:32:34
   29           "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
   30           name, arr[2], arr[3], arr[4], arr[5]
   31       );
-> 32       unsafe { std::mem::transmute(arr) }
   33   }
Target 0: (hashmap2) stopped.
```

```shell
# # 插入了三个元素后，哈希表没有剩余空间，堆地址起始位置不变 0x600001700160 - 4*8(0x20)
added 3: bucket_mask 0x3, ctrl 0x600001700160, growth_left: 0, items: 3
```

```shell
(lldb) x/12x 0x600001700160
0x600001700160: 0x16ff6d66 0xffffffff 0xffffffff 0xffffffff
0x600001700170: 0x16ff6d66 0x00000000 0x00000000 0x00000000
0x600001700180: 0x20ec913f 0x00007ff8 0x4e5ef01e 0x00000000
```

```shell
(lldb) c
Process 69337 resuming
added 4: bucket_mask 0x7, ctrl 0x600002604040, growth_left: 3, items: 4
Process 69337 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100008d16 hashmap2`hashmap2::explain::h4091c852f38a0de4(name="added 4", map=<unavailable>) at hashmap2.rs:32:34
   29           "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
   30           name, arr[2], arr[3], arr[4], arr[5]
   31       );
-> 32       unsafe { std::mem::transmute(arr) }
   33   }
Target 0: (hashmap2) stopped.
```

```shell
# 插入第四个元素后，哈希表扩容，堆地址起始位置变为 0x600002604040 - 8*8(0x40)
added 4: bucket_mask 0x7, ctrl 0x600002604040, growth_left: 3, items: 4
```

```shell
(lldb) x/12x 0x600002604040
0x600002604040: 0x16446d66 0xffffffff 0xffffffff 0xffffffff
0x600002604050: 0x16446d66 0xffffffff 0x00000000 0x00000000
0x600002604060: 0x00000000 0x00000000 0x00000000 0x00000000
(lldb) x/20x 0x600002604040
0x600002604040: 0x16446d66 0xffffffff 0xffffffff 0xffffffff
0x600002604050: 0x16446d66 0xffffffff 0x00000000 0x00000000
0x600002604060: 0x00000000 0x00000000 0x00000000 0x00000000
0x600002604070: 0x00000000 0x00000000 0x00000000 0x00000000
0x600002604080: 0x00000000 0x00000000 0x00000000 0x00000000
```

```shell
(lldb) c
Process 69337 resuming
final: bucket_mask 0x7, ctrl 0x600002604040, growth_left: 4, items: 3
Process 69337 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100008d16 hashmap2`hashmap2::explain::h4091c852f38a0de4(name="final", map=<unavailable>) at hashmap2.rs:32:34
   29           "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
   30           name, arr[2], arr[3], arr[4], arr[5]
   31       );
-> 32       unsafe { std::mem::transmute(arr) }
   33   }
Target 0: (hashmap2) stopped.
```

```shell
# 删除 a 后，剩余 4 个位置。注意 ctrl bit 的变化，以及 0x61 0x1 并没有被清除
final: bucket_mask 0x7, ctrl 0x600002604040, growth_left: 4, items: 3
```

```shell
(lldb) x/12x 0x600002604040
0x600002604040: 0x1644ff66 0xffffffff 0xffffffff 0xffffffff
0x600002604050: 0x1644ff66 0xffffffff 0x00000000 0x00000000
0x600002604060: 0x00000000 0x00000000 0x00000000 0x00000000
(lldb) x/20x 0x600002604040
0x600002604040: 0x1644ff66 0xffffffff 0xffffffff 0xffffffff
0x600002604050: 0x1644ff66 0xffffffff 0x00000000 0x00000000
0x600002604060: 0x00000000 0x00000000 0x00000000 0x00000000
0x600002604070: 0x00000000 0x00000000 0x00000000 0x00000000
0x600002604080: 0x00000000 0x00000000 0x00000000 0x00000000
```
