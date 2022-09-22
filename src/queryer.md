# SQL查询工具

<!--ts-->
* [SQL查询工具](#sql查询工具)
   * [workspace: 这里使用虚拟清单(virtual manifest)方式](#workspace-这里使用虚拟清单virtual-manifest方式)
      * [workspace使用方式](#workspace使用方式)
   * [queryer package](#queryer-package)
      * [cargo.toml](#cargotoml)
      * [两个使用示例](#两个使用示例)
         * [dialect.rs:SQL解析](#dialectrssql解析)
         * [covid.rs: AST转换](#covidrs-ast转换)
      * [src/convert.rs](#srcconvertrs)
         * [结构体定义:sql与对应部分结构体, 注意限于孤儿原则的再包装](#结构体定义sql与对应部分结构体-注意限于孤儿原则的再包装)
         * [sql的转换](#sql的转换)
         * [对应部分结构体的转换](#对应部分结构体的转换)
         * [单元测试](#单元测试)
      * [src/dialect.rs](#srcdialectrs)
         * [定义方言结构体](#定义方言结构体)
         * [给方言结构体实现trait](#给方言结构体实现trait)
         * [添加测试用函数](#添加测试用函数)
         * [单元测试](#单元测试-1)
      * [src/loader.rs](#srcloaderrs)
         * [定义Loader与CsvLoader](#定义loader与csvloader)
         * [定义trait并给CsvLoader实现](#定义trait并给csvloader实现)
         * [todo: 给CsvLoader添加内容检测](#todo-给csvloader添加内容检测)
      * [src/fetcher.rs](#srcfetcherrs)
         * [定义UrlFetcher与FileFetcher](#定义urlfetcher与filefetcher)
         * [定义trait并给Fetcher与FileFetcher实现](#定义trait并给fetcher与filefetcher实现)
         * [最后定义一个获取数据的方法](#最后定义一个获取数据的方法)
   * [queryer-js package: 使用neon](#queryer-js-package-使用neon)
      * [Cargo.toml](#cargotoml-1)
      * [build in package.json](#build-in-packagejson)
      * [src/lib.rs](#srclibrs)
   * [queryer-py package: 使用pyo3](#queryer-py-package-使用pyo3)
      * [Cargo.toml](#cargotoml-2)
      * [build.rs](#buildrs)
      * [src/lib.rs](#srclibrs-1)
   * [data-viewer package: 使用tauri](#data-viewer-package-使用tauri)
      * [Cargo.toml](#cargotoml-3)
      * [build.rs](#buildrs-1)
      * [main.rs](#mainrs)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Sep 22 04:16:34 UTC 2022 -->

<!--te-->

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

#### 定义方言结构体

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:3:4}}
```

#### 给方言结构体实现trait

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:6:19}}
```

#### 添加测试用函数

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:21:32}}
```

#### 单元测试

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:34:43}}
```

### src/loader.rs

#### 定义Loader与CsvLoader

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/loader.rs:7:14}}
```

#### 定义trait并给CsvLoader实现

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:24:38}}
```

#### todo: 给CsvLoader添加内容检测

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/dialect.rs:40:44}}
```

### src/fetcher.rs

#### 定义UrlFetcher与FileFetcher

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:6:8}}
```

#### 定义trait并给Fetcher与FileFetcher实现

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:10:33}}
```

#### 最后定义一个获取数据的方法

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer/src/fetcher.rs:35:45}}
```

## queryer-js package: 使用neon

### Cargo.toml

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-js/Cargo.toml}}
```

### build in package.json

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-js/package.json}}
```

### src/lib.rs

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-js/src/lib.rs}}
```

## queryer-py package: 使用pyo3

> python调用查询包

### Cargo.toml

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-py/Cargo.toml}}
```

### build.rs

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-py/build.rs}}
```

### src/lib.rs

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/queryer-py/src/lib.rs}}
```

## data-viewer package: 使用tauri

### Cargo.toml

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/Cargo.toml}}
```

### build.rs

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/src/build.rs}}
```

### main.rs

```rust, editable
{{#include ../geektime_rust_codes/06_queryer/data-viewer/src-tauri/src/main.rs}}
```
