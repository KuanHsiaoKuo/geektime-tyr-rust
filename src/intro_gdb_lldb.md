# gdb/lldb调试或查看内存结构

<!--ts-->

<!--te-->

## 查看hashmap内存结构

### bin配置与运行

- [Cargo Targets - The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=bin#binaries)

```toml
{{#include ../geektime_rust_codes/17_hash_table/Cargo.toml}}
```

> 如果要单独运行指定bin文件：

```shell
cargo run --bin hashmap2
```

### 目标调试代码

```rust, editable
{{#include ../geektime_rust_codes/17_hash_table/src/hashmap2.rs}}
```

### 使用gdb/lldb进行调试查看内存结构

1. gdb: 主要是linux系统

2. lldb: 主要OSX系统

#### gdb与lldb命令对照

- [GDB to LLDB command map — The LLDB Debugger](https://lldb.llvm.org/use/map.html)

#### 开始调试

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

#### b(reakpoint): 添加断点

> 在32行打断点，方便看std::mem::transmute(arr)

```shell
(lldb) b hashmap2.rs:32
Breakpoint 1: where = hashmap2`hashmap2::explain::h4091c852f38a0de4 + 406 at hashmap2.rs:32:34, address = 0x0000000100008d16
```

#### r(un):运行到断点

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

#### c(ontinue):继续单步执行

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

#### x: 打印内存地址

- [打印内存的值 | 100个gdb小技巧](https://wizardforcel.gitbooks.io/100-gdb-tips/content/examine-memory.html)

```shell
# 以12进制打印从内存地址开始的值
(lldb) x/12x 0x600001700160
0x600001700160: 0xffff6dff 0xffffffff 0xffffffff 0xffffffff
0x600001700170: 0xffff6dff 0x00000000 0x00000000 0x00000000
0x600001700180: 0x20ec913f 0x00007ff8 0x4e5ef01e 0x00000000
```

#### c(ontinue): 继续执行到下一个断点

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

## 查看闭包的结构

### 代码

```rust, editable
{{#include ../geektime_rust_codes/19_closure/examples/closure_size.rs}}
```

### 运行进入lldb

```shell
# 自动去examples目录找对应名字的代码文件
cargo run --example closure_size
```

```shell
rust-lldb ../target/debug/examples/closure_size                                                                                                  ─╯
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
(lldb) target create "../target/debug/examples/closure_size"
Current executable set to '/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/closure_size' (x86_64).
(lldb)
```

```shell
(lldb) b closure_size.rs:14
Breakpoint 1: where = closure_size`closure_size::main::h679d75437a0cd078 + 199 at closure_size.rs:14:14, address = 0x00000001000056b7
```

```shell
(lldb) r
Process 95084 launched: '/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/closure_size' (x86_64)
Process 95084 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x00000001000056b7 closure_size`closure_size::main::h679d75437a0cd078 at closure_size.rs:14:14
   11       // 如果捕获一个引用，长度为 8
   12       let c3 = || println!("hello: {}", name);
   13       // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
-> 14       let c4 = move || println!("hello: {}, {:?}", name1, table);
   15       let name2 = name.clone();
   16       // 和局部变量无关，捕获了一个 String name2，closure 长度 24
   17       let c5 = move || {
Target 0: (closure_size) stopped.
```

```shell
(lldb) frame variable
(closure_size::main::{closure_env#0}) c1 =
(closure_size::main::{closure_env#1}) c2 =
(alloc::string::String) name = "tyr" {
  vec = size=3 {
    [0] = 't'
    [1] = 'y'
    [2] = 'r'
  }
}
(alloc::string::String) name1 = "tyr" {
  vec = size=3 {
    [0] = 't'
    [1] = 'y'
    [2] = 'r'
  }
}
(std::collections::hash::map::HashMap<&str, &str, std::collections::hash::map::RandomState>) table = size=1 {
  [0] = {
    0 = "hello" {
      data_ptr = 0x0000000100043daf "helloworld, c2: , c3: , c4: , c5: \n"
      length = 5
    }
    1 = "world" {
      data_ptr = 0x0000000100043db4 "world, c2: , c3: , c4: , c5: \n"
      length = 5
    }
  }
}
(lldb) fr v
(closure_size::main::{closure_env#0}) c1 =
(closure_size::main::{closure_env#1}) c2 =
(alloc::string::String) name = "tyr" {
  vec = size=3 {
    [0] = 't'
    [1] = 'y'
    [2] = 'r'
  }
}
(alloc::string::String) name1 = "tyr" {
  vec = size=3 {
    [0] = 't'
    [1] = 'y'
    [2] = 'r'
  }
}
(std::collections::hash::map::HashMap<&str, &str, std::collections::hash::map::RandomState>) table = size=1 {
  [0] = {
    0 = "hello" {
      data_ptr = 0x0000000100043daf "helloworld, c2: , c3: , c4: , c5: \n"
      length = 5
    }
    1 = "world" {
      data_ptr = 0x0000000100043db4 "world, c2: , c3: , c4: , c5: \n"
      length = 5
    }
  }
}
```

```shell
(lldb) x/gx c1
error: memory read failed for 0x0
(lldb) x/gx &c1
0x7ff7bfefed20: 0x0000000100266000
(lldb) x/gx &c2
0x7ff7bfefed28: 0x00006000017041c0
(lldb) x/gx &c3
0x7ff7bfefed90: 0x00007ff7bfefed30
(lldb) x/gx 0x00007ff7bfefed30
0x7ff7bfefed30: 0x0000600000008010
(lldb) x/3c  0x0000600000008010
error: reading memory as characters of size 8 is not supported
(lldb) x/gx  0x0000600000008010
0x600000008010: 0x0000000000727974
```

- g表示八字节。当我们指定了字节长度后，GDB会从指内存定的内存地址开始，读写指定字节，并把其当作一个值取出来。
- 可以看出：c1是

```shell
(lldb) n
Process 95084 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100005744 closure_size`closure_size::main::h679d75437a0cd078 at closure_size.rs:15:17
   12       let c3 = || println!("hello: {}", name);
   13       // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
   14       let c4 = move || println!("hello: {}, {:?}", name1, table);
-> 15       let name2 = name.clone();
   16       // 和局部变量无关，捕获了一个 String name2，closure 长度 24
   17       let c5 = move || {
   18           let x = 1;
Target 0: (closure_size) stopped.
(lldb) x/9gx c4
error: memory read failed for 0x0
(lldb) x/9gx &c4
0x7ff7bfefed98: 0x0000600000008020 0x0000000000000003
0x7ff7bfefeda8: 0x0000000000000003 0x3e49f3270a1a0fb0
0x7ff7bfefedb8: 0x79dd9a78e6c327e7 0x0000000000000003
0x7ff7bfefedc8: 0x0000600003304080 0x0000000000000002
0x7ff7bfefedd8: 0x0000000000000001
(lldb) x/3c 0x0000600000008020
error: reading memory as characters of size 8 is not supported
(lldb) x/gx 0x0000600000008020
0x600000008020: 0x0000000000727974
(lldb) x/18gx 0x0000600000008020 - 0x80
error: memory read takes a start address expression with an optional end address expression.
warning: Expressions should be quoted if they contain spaces or other special characters.
(lldb) x/18gx '0x0000600000008020 - 0x80'
0x600000007fa0: 0x0000000000000000 0x0000000000000000
0x600000007fb0: 0x0000000000000000 0x0000000000000000
0x600000007fc0: 0x0000000000000000 0x0000000000000000
0x600000007fd0: 0x0000000000000000 0x0000000000000000
0x600000007fe0: 0x0000000000000000 0x0000000000000000
0x600000007ff0: 0x0000000000000000 0x0000000000000000
0x600000008000: 0x000000006e69616d 0x0000000000000000
0x600000008010: 0x0000000000727974 0x0000000000000000
0x600000008020: 0x0000000000727974 0x0000000000000000
```

- 0x: C语言里的0x0和0x1分别表示十六进制的数的0和1。

C语言、C++、Shell、Python、Java语言及其他相近的语言使用字首“0x”，例如“0x5A3”。开头的“0”令解析器更易辨认数，而“x”则代表十六进制（就如“O”代表八进制）。在“0x”中的“x”可以大写或小写。对于字符量C语言中则以x+两位十六进制数的方式表示，如xFF。

因此，0x0中“0x”表示的是十六进制数，0是十六进制数值0，0x,1中“0x”表示的是十六进制数，1是十六进制数值1

- C语言中的相关数值表示法：

1、在C语言里，整数有三种表示形式：十进制，八进制，十六进制。其中以数字0开头，由0~7组成的数是八进制。以0X或0x开头，由0~9，A~F或a~f 组成是十六进制。除表示正负的符号外，以1~9开头，由0~9组成是十进制。

2、十进制：除表示正负的符号外，以1~9开头，由0~9组成。如，128，+234，-278。

3、八进制：以0开头，由0~7组成的数。如，0126,050000.

4、十六进制：以0X或0x开头，由0~9，A~F或a~f 组成。如，0x12A,0x5a000。
