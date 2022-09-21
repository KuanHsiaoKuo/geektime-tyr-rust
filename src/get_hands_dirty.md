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
   * [pb模块: 处理protobuf](#pb模块-处理protobuf)
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
   * [engine模块: 处理图片](#engine模块-处理图片)
      * [mod.rs: 定义统一的引擎trait](#modrs-定义统一的引擎trait)
      * [photon.rs &gt; 静态变量加载](#photonrs--静态变量加载)
      * [photon.rs &gt; 具体引擎Photon的定义与转化TryFrom](#photonrs--具体引擎photon的定义与转化tryfrom)
      * [photon.rs &gt; 具体引擎Photon的trait实现](#photonrs--具体引擎photon的trait实现)
         * [Engine Trait](#engine-trait)
         * [SpecTransform Trait](#spectransform-trait)
            * [格式语义化](#格式语义化)
      * [photon.rs &gt; 在内存中对图片转换格式的方法](#photonrs--在内存中对图片转换格式的方法)
   * [main.rs](#mainrs)
      * [先引入mod，再use](#先引入mod再use)
      * [图片资源用到Lru策略缓存type定义](#图片资源用到lru策略缓存type定义)
      * [主流程main函数](#主流程main函数)
         * [建造者模式](#建造者模式)
         * [类型转换](#类型转换)
            * [数字与字符串](#数字与字符串)
            * [String 与 &amp; str](#string-与--str)
            * [智能指针](#智能指针)
      * [路由绑定的处理函数handler](#路由绑定的处理函数handler)
      * [处理函数用到的图片获取方法](#处理函数用到的图片获取方法)
      * [一个用于调试的辅助函数](#一个用于调试的辅助函数)
   * [运行与日志](#运行与日志)
* [SQL查询工具](#sql查询工具)
   * [workspace: 这里使用虚拟清单(virtual manifest)方式](#workspace-这里使用虚拟清单virtual-manifest方式)
      * [workspace使用方式](#workspace使用方式)
   * [queryer package](#queryer-package)
   * [queryer-js package](#queryer-js-package)
   * [queryer-py package](#queryer-py-package)
   * [data-viewer package](#data-viewer-package)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Sep 21 10:05:57 UTC 2022 -->

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

## engine模块: 处理图片

### mod.rs: 定义统一的引擎trait

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/mod.rs:7:19}}
```

### photon.rs > 静态变量加载

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:11:18}}
```

### photon.rs > 具体引擎Photon的定义与转化TryFrom

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:21:30}}
```

### photon.rs > 具体引擎Photon的trait实现

#### Engine Trait

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:32:47}}
```

#### SpecTransform Trait

##### 格式语义化

```rust
impl SpecTransform(&OpreationName) for SpecificEngine {
    fn transform(&mut self, _op: &OperationName) {
        transform::OperationMethod(&mut self.0)
    }
}
```

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:54:108}}
```

### photon.rs > 在内存中对图片转换格式的方法

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:111:122}}
```

## main.rs

### 先引入mod，再use

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:34:39}}
```

### 图片资源用到Lru策略缓存type定义

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:41:41}}
```

### 主流程main函数

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:43:71}}
```

#### 建造者模式

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:53:60}}
```

#### 类型转换

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:63:64}}
```

##### 数字与字符串

|         | i32                | u32                | f64                | String*       |
|---------|--------------------|--------------------|--------------------|---------------|
| i32     | \                  | x as u32           | x as f64           | x.to_string() |
| u32     | x as i32           | \                  | x as f64           | x.to_string() |
| f64     | x as i32           | x as u32           | \                  | x.to_string() |
| String* | x.parse().unwrap() | x.parse().unwrap() | x.parse().unwrap() | \             |

##### String 与 & str

| \      | String        | &str |
|--------|---------------|------|
| String | \             | &*x  |
| &str   | x.to_string() | \    |

##### 智能指针

| \        | Vec\<T\>   | &[T]    | Box<[T]>             |
|----------|------------|---------|----------------------|
| Vec\<T\> | \          | &x[...] | x.into_boxed_slice() |
| &[T]     | x.to_vec() | \       | Box::new(\*x)        |
| Box<[T]> | x.to_vec() | &\*x    | \                    |

### 路由绑定的处理函数handler

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:73:101}}
```

### 处理函数用到的图片获取方法

> 对于图片的网络请求，我们先把 URL 做个哈希，在 LRU 缓存中查找，找不到才用 reqwest 发送请求。

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:103:125}}
```

### 一个用于调试的辅助函数

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:127:137}}
```

## 运行与日志

> 将RUST_LOG级别设置为info

```shell
cargo build --release
RUST_LOG=info target/release/thumbor
```

# SQL查询工具

## workspace: 这里使用虚拟清单(virtual manifest)方式

> [工作空间 Workspace - Rust语言圣经(Rust Course)](https://course.rs/cargo/reference/workspaces.html)

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/Cargo.toml.bak}}
```

~~~admonish info title="虚拟清单"
若一个 Cargo.toml 有 [workspace] 但是没有 [package] 部分，则它是虚拟清单类型的工作空间。

对于没有主 package 的场景或你希望将所有的 package 组织在单独的目录中时，这种方式就非常适合。
~~~

~~~admonish tip title="workspace关键点"
- 所有的 package 共享同一个 Cargo.lock 文件，该文件位于工作空间的根目录中
- 所有的 package 共享同一个输出目录，该目录默认的名称是 target ，位于工作空间根目录下
- 只有工作空间根目录的 Cargo.toml 才能包含 [patch], [replace] 和 [profile.*]，而成员的 Cargo.toml 中的相应部分将被自动忽略
~~~

### workspace使用方式

```shell
cargo run -p <member package>
cargo build -p queryer
```

~~~admonish info title='使用说明'
1. 在工作空间中，package 相关的 Cargo 命令(例如 cargo build )可以使用 -p 、 --package 或 --workspace 命令行参数来指定想要操作的 package。

2. 若没有指定任何参数，则 Cargo 将使用当前工作目录的中的 package 。若工作目录是虚拟清单类型的工作空间，则该命令将作用在所有成员上(就好像是使用了 --workspace 命令行参数)。而 default-members 可以在命令行参数没有被提供时，手动指定操作的成员
~~~

## queryer package

### cargo.toml

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/cargo.toml}}
```

### 两个使用示例

#### dialect.rs:SQL解析

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/examples/dialect.rs}}
```

#### covid.rs: AST转换

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/examples/covid.rs}}
```

### src/convert.rs

#### 结构体定义:sql与对应部分结构体, 注意限于孤儿原则的再包装

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:8:28}}
```

#### sql的转换

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:30:86}}
```

#### 对应部分结构体的转换

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:88:227}}
```

#### 单元测试

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:229:250}}
```

### src/dialect.rs

### src/loader.rs

### src/fetcher.rs

## queryer-js package

## queryer-py package

## data-viewer package