# get hands dirty

<!--ts-->

* [get hands dirty](#get-hands-dirty)
* [httpie源码剖析](#httpie源码剖析)
    * [example的使用](#example的使用)
        * [Cargo.toml](#cargotoml)
    * [Step1：指令解析](#step1指令解析)
        * [clap::Parser](#clapparser)
    * [Step2：添加参数验证与键值对改造](#step2添加参数验证与键值对改造)
        * [参数验证](#参数验证)
        * [键值对改造](#键值对改造)
    * [Step3：异步请求改造](#step3异步请求改造)
    * [Step4: 语法高亮打印](#step4-语法高亮打印)
    * [Step5: 添加单元测试](#step5-添加单元测试)
* [thumbor图片服务](#thumbor图片服务)
    * [abi.proto](#abiproto)
    * [build.rs](#buildrs)
    * [关于rust的模块](#关于rust的模块)
    * [mod文件定义与实现分离](#mod文件定义与实现分离)
    * [pb模块](#pb模块)
        * [pb/mod.rs声明模块](#pbmodrs声明模块)
        * [pb/abi.rs里面还有子模块](#pbabirs里面还有子模块)
        * [pb/abi.rs另外定义了spec::Data里面的各个元素结构体/嵌套模块mod](#pbabirs另外定义了specdata里面的各个元素结构体嵌套模块mod)
        * [pb/abi.rs有个特殊结构体](#pbabirs有个特殊结构体)
        * [ImageSpec](#imagespec)
            * [定义：有序数组](#定义有序数组)
            * [实现：new方法、From&amp;TryFrom实现类型转化](#实现new方法fromtryfrom实现类型转化)
        * [Filter](#filter)
            * [定义：枚举体mod](#定义枚举体mod)
            * [实现：双引号的使用、模式匹配](#实现双引号的使用模式匹配)
        * [SampleFilter](#samplefilter)
            * [定义：枚举体mod](#定义枚举体mod-1)
        * [实现：mod使用双引号、From转为不同结果](#实现mod使用双引号from转为不同结果)
        * [Spec](#spec)
            * [定义：结构体](#定义结构体)
            * [实现：类似面向对象中添加类方法Self](#实现类似面向对象中添加类方法self)
        * [单元测试](#单元测试)
    * [engine模块](#engine模块)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Sep 21 07:23:04 UTC 2022 -->

<!--te-->

# httpie源码剖析

## example的使用

- [Cargo Targets >> Examples - The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=%5B%5Bexample%5D%5D#examples)

### Cargo.toml

```toml
{{ #include ../geektime_rust_codes/04_httpie/Cargo.toml}}
```

```toml
{{ #include ../geektime_rust_codes/04_httpie/Cargo.toml:8:15}}
```

~~~admonish tip title='example使用'
1. 示例代码放在根目录的examples文件夹，与src同级
```shell
tree -L 2                                                                                                       ─╯
.
├── Cargo.toml
├── examples
│   ├── cli.rs
│   ├── cli_get.rs
│   └── cli_verify.rs
└── src
    └── main.rs

2 directories, 5 files
```

2. 执行指令

```shell
cargo run --example <example-name-in-cargo>
cargo run --example cli
cargo run --example cli_get
cargo run --example cli_verify
```

3. 使用示例

```shell
cargo run --example cli                                                                                                                                                                                                                ─╯
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/cli`
httpie 1.0
Tyr Chen <tyr@chen.com>
A naive httpie implementation with Rust, can you imagine how easy it is?

USAGE:
    cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    get     feed get with an url and we will retrieve the response for you
    help    Print this message or the help of the given subcommand(s)
    post    feed post with an url and optional key=value pairs. We will post the data as JSON,
                and retrieve the response for you
```

- Run a binary or example of the local package
- SUBCOMMANDS来自代码中的注释
~~~

## Step1：指令解析

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs}}
```

### clap::Parser

~~~admonish info title='clap::Parser'

- [clap-rs/clap: A full featured, fast Command Line Argument Parser for Rust](https://github.com/clap-rs/clap)
- [clap - Rust](https://docs.rs/clap/latest/clap/)
- [clap::parser - Rust](https://docs.rs/clap/latest/clap/parser/index.html)
~~~

1. clap的parser派生宏会自动实现parse方法来接收指令参数

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs:9:15}}
```

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs:43:46}}
```

2. 运行效果

```shell
cargo run --example cli get http://jsonplaceholder.typicode.com/posts/2                                                                                                                                                                ─╯
   Compiling httpie v0.1.0 (/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/04_httpie)
    Finished dev [unoptimized + debuginfo] target(s) in 2.31s
     Running `/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/cli get 'http://jsonplaceholder.typicode.com/posts/2'`
Opts { subcmd: Get(Get { url: "http://jsonplaceholder.typicode.com/posts/2" }) }
```

```shell
cargo run --example cli post http://jsonplaceholder.typicode.com/posts/2                                                                                                                                                               ─╯
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/cli post 'http://jsonplaceholder.typicode.com/posts/2'`
Opts { subcmd: Post(Post { url: "http://jsonplaceholder.typicode.com/posts/2", body: [] }) }
```

```shell
cargo run --example cli delete http://jsonplaceholder.typicode.com/posts/2                                                                                                                                                             ─╯
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/cli delete 'http://jsonplaceholder.typicode.com/posts/2'`
error: Found argument 'delete' which wasn't expected, or isn't valid in this context

USAGE:
    cli <SUBCOMMAND>

For more information try --help

```

- opts的获取：自动以空格分隔，根据<subcommand>模式匹配，之后的参数依次赋值给<subcommand> struct里面的元素

## Step2：添加参数验证与键值对改造

### 参数验证

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:27:47}}
```

1. clap 允许你为每个解析出来的值添加自定义的解析函数，我们这里定义了parse_url和parse_kv_pair检查一下。

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:75:84}}
```

1. clap 允许你为每个解析出来的值添加自定义的解析函数，我们这里定义了个parse_url检查一下。

### 键值对改造

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:49:73}}
```

## Step3：异步请求改造

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_get.rs:85:112}}
```

## Step4: 语法高亮打印

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:112:167}}
```

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:169:186}}
```

## Step5: 添加单元测试

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:189:220}}
```

# thumbor图片服务

## abi.proto

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/abi.proto}}
```

## build.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/build.rs}}
```

- [option_env in std - Rust](https://doc.rust-lang.org/std/macro.option_env.html)

> 在编译时可选择检查环境变量。

## 关于rust的模块

> 可以参考这篇：[Rust 模块系统理解 - 知乎](https://zhuanlan.zhihu.com/p/443926839)

~~~admonish tip title='mod全认识'
1. mod(mod.rs或mod关键字)将代码分为多个逻辑模块，并管理这些模块的可见性（public / private）。
2. 模块是项（item）的集合，项可以是：函数，结构体，trait，impl块，甚至其它模块。
3. 一个目录下的所有代码，可以通过 mod.rs 声明
4. Rust模块有三种形式:
    - mod.rs: 一个目录下的所有代码，可以通过 mod.rs 声明
    - 文件/目录即模块：编译器的机制决定，除了mod.rs外，每一个文件和目录都是一个模块。不允许只分拆文件，但是不声明mod，我们通常使用pub use，在父空间直接调用子空间的函数。
    - mod关键字: 在文件内部分拆模块
5. Rust编译器只接受一个源文件，输出一个crate
6. 每一个crate都有一个匿名的根命名空间，命名空间可以无限嵌套
7. “mod mod-name { ... }“ 将大括号中的代码置于命名空间mod-name之下
8. “use mod-name1::mod-name2;" 可以打开命名空间，减少无休止的::操作符
9. “mod mod-name;“ 可以指导编译器将多个文件组装成一个文件
10. “pub use mod-nam1::mod-name2::item-name;“
    语句可以将mod-name2下的item-name提升到这条语句所在的空间，item-name通常是函数或者结构体。Rust社区通常用这个方法来缩短库API的命名空间深度
    编译器规定use语句一定要在mod语句之前
~~~

## mod文件定义与实现分离

在rust中，一般会在模块的mod.rs文件中对供外部使用的项进行实现, 项可以是：函数，结构体，trait，impl块，甚至其它模块.
这样有个好处，高内聚，可以在代码增长时，将变动局限在服务提供者内部，对外提供的api不变，不会造成破坏性更新。

## pb模块: 处理protobuf

### pb/mod.rs声明模块

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:5:6}}
```

### pb/abi.rs里面还有子模块

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:94:113}}
```

### pb/abi.rs另外定义了spec::Data里面的各个元素结构体/嵌套模块mod

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:1:87}}
```

### pb/abi.rs有个特殊结构体

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:88:93}}
```

### ImageSpec

#### 定义：有序数组

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:1:6}}
```

#### 实现：new方法、From&TryFrom实现类型转化

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:8:30}}
```

### Filter

#### 定义：枚举体mod

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:68:79}}
```

#### 实现：双引号的使用、模式匹配

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:32:42}}
```

### SampleFilter

#### 定义：枚举体mod

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:19:37}}
```

### 实现：mod使用双引号、From转为不同结果

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:45:56}}
```

### Spec

#### 定义：结构体

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:88:93}}
```

#### 实现：类似面向对象中添加类方法Self

> 注意区别Self和self的使用：
> [rust - What's the difference between self and Self? - Stack Overflow](https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self)

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:58:95}}
```

### 单元测试

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:97:110}}
```

## engine模块
