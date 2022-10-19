# SQL查询工具

<!--ts-->
* [SQL查询工具](#sql查询工具)
   * [workspace: 这里使用虚拟清单(virtual manifest)方式](#workspace-这里使用虚拟清单virtual-manifest方式)
      * [workspace使用方式](#workspace使用方式)
   * [queryer package](#queryer-package)
      * [cargo.toml](#cargotoml)
      * [两个使用示例](#两个使用示例)
      * [src/convert.rs](#srcconvertrs)
      * [src/dialect.rs](#srcdialectrs)
      * [src/loader.rs](#srcloaderrs)
      * [src/fetcher.rs](#srcfetcherrs)
   * [queryer-js package: 使用neon](#queryer-js-package-使用neon)
   * [queryer-py package: 使用pyo3](#queryer-py-package-使用pyo3)
   * [data-viewer package: 使用tauri](#data-viewer-package-使用tauri)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Oct 19 02:12:25 UTC 2022 -->

<!--te-->

## workspace: 这里使用虚拟清单(virtual manifest)方式

> [工作空间 Workspace - Rust语言圣经(Rust Course)](https://course.rs/cargo/reference/workspaces.html)

~~~admonish note title="Cargo.toml " collapsible=true
```toml
{{#include ../geektime_rust_codes/06_queryer/Cargo.toml.bak}}
```
~~~

~~~admonish info title="虚拟清单" collapsible=true
若一个 Cargo.toml 有 [workspace] 但是没有 [package] 部分，则它是虚拟清单类型的工作空间。

对于没有主 package 的场景或你希望将所有的 package 组织在单独的目录中时，这种方式就非常适合。
~~~

~~~admonish tip title="workspace关键点" collapsible=true
- 所有的 package 共享同一个 Cargo.lock 文件，该文件位于工作空间的根目录中
- 所有的 package 共享同一个输出目录，该目录默认的名称是 target ，位于工作空间根目录下
- 只有工作空间根目录的 Cargo.toml 才能包含 [patch], [replace] 和 [profile.*]，而成员的 Cargo.toml 中的相应部分将被自动忽略
~~~

### workspace使用方式

```shell
cargo run -p <member package>
cargo build -p queryer
```

~~~admonish info title='使用说明' collapsible=true
1. 在工作空间中，package 相关的 Cargo 命令(例如 cargo build )可以使用 -p 、 --package 或 --workspace 命令行参数来指定想要操作的 package。

2. 若没有指定任何参数，则 Cargo 将使用当前工作目录的中的 package 。若工作目录是虚拟清单类型的工作空间，则该命令将作用在所有成员上(就好像是使用了 --workspace 命令行参数)。而 default-members 可以在命令行参数没有被提供时，手动指定操作的成员
~~~

## queryer package

### cargo.toml

~~~admonish note title="cargo.toml " collapsible=true
```toml
{{#include ../geektime_rust_codes/06_queryer/queryer/cargo.toml}}
```
~~~

### 两个使用示例

~~~admonish note title="1. dialect.rs:SQL解析" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/examples/dialect.rs}}
```
~~~

~~~admonish note title="2. covid.rs: AST转换 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/examples/covid.rs}}
```
~~~

### src/convert.rs

~~~admonish note title="结构体定义:sql与对应部分结构体, 注意限于孤儿原则的再包装" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:8:28}}
```
~~~

~~~admonish note title="sql的转换" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:30:86}}
```
~~~

~~~admonish note title=" 对应部分结构体的转换" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:88:227}}
```
~~~

~~~admonish note title="单元测试" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/convert.rs:229:250}}
```
~~~

### src/dialect.rs

~~~admonish note title="定义方言结构体" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:3:4}}
```
~~~

~~~admonish note title="给方言结构体实现trait" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:6:19}}
```
~~~

~~~admonish note title="添加测试用函数" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:21:32}}
```
~~~

~~~admonish note title="单元测试 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:34:43}}
```
~~~

### src/loader.rs

~~~admonish note title="定义Loader与CsvLoader" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/loader.rs:7:14}}
```
~~~

~~~admonish note title="定义trait并给CsvLoader实现" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:24:38}}
```
~~~

~~~admonish note title="todo: 给CsvLoader添加内容检测" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:40:44}}
```
~~~

### src/fetcher.rs

~~~admonish note title="定义UrlFetcher与FileFetcher" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:6:8}}
```
~~~

~~~admonish note title="定义trait并给Fetcher与FileFetcher实现 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:10:33}}
```
~~~

~~~admonish note title="最后定义一个获取数据的方法 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:35:45}}
```
~~~

## queryer-js package: 使用neon

~~~admonish note title="Cargo.toml " collapsible=true

```toml
{{#include ../geektime_rust_codes/06_queryer/queryer-js/Cargo.toml}}
```
~~~

~~~admonish note title="build in package.json" collapsible=true
```json
{{#include ../geektime_rust_codes/06_queryer/queryer-js/package.json}}
```
~~~

~~~admonish note title="src/lib.rs " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-js/src/lib.rs}}
```
~~~

## queryer-py package: 使用pyo3

> python调用查询包

~~~admonish note title="Cargo.toml " collapsible=true
```toml
{{#include ../geektime_rust_codes/06_queryer/queryer-py/Cargo.toml}}
```
~~~

~~~admonish note title="build.rs " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-py/build.rs}}
```
~~~

~~~admonish note title="src/lib.rs" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-py/src/lib.rs}}
```
~~~

## data-viewer package: 使用tauri

~~~admonish note title="Cargo.toml" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/Cargo.toml}}
```
~~~

~~~admonish note title="build.rs" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/src/build.rs}}
```
~~~

~~~admonish note title="main.rs " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/src/main.rs}}
```
~~~
