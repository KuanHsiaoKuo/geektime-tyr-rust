# rgrep

<!--ts-->
* [rgrep](#rgrep)
   * [Cargo.toml](#cargotoml)
   * [src/error.rs: thiserror会自动转换](#srcerrorrs-thiserror会自动转换)
   * [src/lib.rs：定义结构体+实现方法+单元测试](#srclibrs定义结构体实现方法单元测试)
   * [src/main.rs](#srcmainrs)
   * [使用](#使用)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Mar 18 14:56:18 UTC 2023 -->

<!--te-->

## Cargo.toml

~~~admonish note title="Cargo.toml " collapsible=true
```toml
{{#include ../geektime_rust_codes/22_mid_term_rgrep/Cargo.toml}}
```
~~~

## src/error.rs: thiserror会自动转换

~~~admonish note title="> 它们都是需要进行转换的错误。thiserror 能够通过宏帮我们完成错误类型的转换。" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/error.rs}}
```
~~~

## src/lib.rs：定义结构体+实现方法+单元测试

~~~admonish note title="mod引入与使用" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:13:14}}
```
~~~

~~~admonish note title="定义结构体: 专门简化复杂类型" collapsible=true
> 这里其实就是传入一个指定结构的函数对象

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:16:17}}
```
~~~

~~~admonish note title="专门的结合版本grep结构体" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:19:27}}
```
~~~

~~~admonish note title="lib.rs: 给结构体实现方法" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:29:55}}
```
~~~

> 主要实现两种解析策略：

1. 默认策略：match_with_default_strategy, 使用default_strategy
2. 指定策略：match_with, 使用传入的strategy: StrategyFn

~~~admonish note title="默认策略:  default_strategy" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:57:87}}
```
~~~

~~~admonish note title="格式化输出" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:89:103}}
```
~~~

~~~admonish note title="单元测试 " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:105:139}}
```
~~~

## src/main.rs

~~~admonish note title="引入lib.rs中的内容" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:3:3}}
```
~~~

~~~admonish note title="主函数：main() " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:5:11}}
```
~~~

## 使用

~~~admonish note title='示例：cargo run --quiet -- "正则表达式" "src/*.rs"' collapsible=true
```shell
cargo run --quiet -- "Re[^\\s]+" "src/*.rs"                                                                                                                                                                                            ─╯
src/main.rs
     1:13  use anyhow::Result;
     5:14  fn main() -> Result<()> {
src/error.rs
     7:14      #[error("Regex pattern error")]
     8:5       RegexPatternError(#[from] regex::Error),
src/lib.rs
     5:12  use regex::Regex;
     8:19      io::{self, BufRead, BufReader, Write},
    17:45  pub type StrategyFn = fn(&Path, &mut dyn BufRead, &Regex, &mut dyn Write) -> Result<(), GrepError>;
    31:50      pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
    36:55      pub fn match_with(&self, strategy: StrategyFn) -> Result<(), GrepError> {
    37:21          let regex = Regex::new(&self.pattern)?;
    44:41                      let mut reader = BufReader::new(file);
    60:25      reader: &mut dyn BufRead,
    61:15      pattern: &Regex,
    63:6   ) -> Result<(), GrepError> {
   126:29          let mut reader = BufReader::new(&input[..]);
   127:23          let pattern = Regex::new(r"he\w+").unwrap();
```
~~~