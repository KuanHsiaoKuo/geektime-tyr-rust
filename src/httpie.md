# httpie源码剖析

<!--ts-->
* [httpie源码剖析](#httpie源码剖析)
   * [example的使用](#example的使用)
   * [Step1：指令解析](#step1指令解析)
   * [Step2：添加参数验证与键值对改造](#step2添加参数验证与键值对改造)
   * [Step3：异步请求改造](#step3异步请求改造)
   * [Step4: 语法高亮打印](#step4-语法高亮打印)
   * [Step5: 添加单元测试](#step5-添加单元测试)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 24 14:02:44 UTC 2022 -->

<!--te-->

## example的使用

- [Cargo Targets >> Examples - The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=%5B%5Bexample%5D%5D#examples)

~~~admonish note title="Cargo.toml " collapsible=true
```toml
{{ #include ../geektime_rust_codes/04_httpie/Cargo.toml}}
```

```toml
{{ #include ../geektime_rust_codes/04_httpie/Cargo.toml:8:15}}
```
~~~

~~~admonish note title="1. 示例代码放在根目录的examples文件夹，与src同级" collapsible=true
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
~~~

~~~admonish note title="2. 执行指令 " collapsible=true
```shell
cargo run --example <example-name-in-cargo>
cargo run --example cli
cargo run --example cli_get
cargo run --example cli_verify
```
~~~

~~~admonish note title="3. 使用示例" collapsible=true
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

~~~admonish note title="1. 源码 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs}}
```
~~~

~~~admonish info title='clap::Parser相关资料' collapsible=true

- [clap-rs/clap: A full featured, fast Command Line Argument Parser for Rust](https://github.com/clap-rs/clap)
- [clap - Rust](https://docs.rs/clap/latest/clap/)
- [clap::parser - Rust](https://docs.rs/clap/latest/clap/parser/index.html)
~~~

~~~admonish note title="1. clap的parser派生宏会自动实现parse方法来接收指令参数" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs:9:15}}
```

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli.rs:43:46}}
```
~~~

~~~admonish note title="2. 运行效果" collapsible=true
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
~~~

## Step2：添加参数验证与键值对改造

~~~admonish note title="1. 参数验证" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:27:47}}
```
~~~

~~~admonish note title="clap 允许你为每个解析出来的值添加自定义的解析函数，我们这里定义了parse_url和parse_kv_pair检查一下。" collapsible=true

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:75:84}}
```
~~~

~~~admonish note title="2. 键值对改造 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_verify.rs:49:73}}
```
~~~

## Step3：异步请求改造

~~~admonish note title="Step3：异步请求改造 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/examples/cli_get.rs:85:112}}
```
~~~

## Step4: 语法高亮打印

~~~admonish note title="Step4: 语法高亮打印 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:112:167}}
```

```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:169:186}}
```
~~~

## Step5: 添加单元测试

~~~admonish note title="Step5: 添加单元测试 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/04_httpie/src/main.rs:189:220}}
```
~~~



