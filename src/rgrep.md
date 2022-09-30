# rgrep

<!--ts-->
* [rgrep](#rgrep)
   * [Cargo.toml](#cargotoml)
   * [src/error.rs: thiserror会自动转换](#srcerrorrs-thiserror会自动转换)
   * [src/lib.rs：定义结构体+实现方法+单元测试](#srclibrs定义结构体实现方法单元测试)
      * [mod引入与使用](#mod引入与使用)
      * [定义结构体](#定义结构体)
         * [专门简化复杂类型](#专门简化复杂类型)
         * [专门的结合版本grep结构体](#专门的结合版本grep结构体)
      * [给结构体实现方法](#给结构体实现方法)
      * [默认策略: default_strategy](#默认策略-default_strategy)
      * [格式化输出](#格式化输出)
      * [单元测试](#单元测试)
   * [src/main.rs](#srcmainrs)
      * [引入lib.rs中的内容](#引入librs中的内容)
      * [主函数：main()](#主函数main)
   * [使用](#使用)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Sep 30 11:06:09 UTC 2022 -->

<!--te-->

## Cargo.toml

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/Cargo.toml}}
```

## src/error.rs: thiserror会自动转换

> 它们都是需要进行转换的错误。thiserror 能够通过宏帮我们完成错误类型的转换。

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/error.rs}}
```

## src/lib.rs：定义结构体+实现方法+单元测试

### mod引入与使用

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:13:14}}
```

### 定义结构体

#### 专门简化复杂类型

> 这里其实就是传入一个指定结构的函数对象

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:16:17}}
```

#### 专门的结合版本grep结构体

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:19:27}}
```

### 给结构体实现方法

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:29:55}}
```

> 主要实现两种解析策略：

1. 默认策略：match_with_default_strategy, 使用default_strategy
2. 指定策略：match_with, 使用传入的strategy: StrategyFn

### 默认策略: default_strategy

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:57:87}}
```

### 格式化输出

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:89:103}}
```

### 单元测试

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:105:139}}
```

## src/main.rs

### 引入lib.rs中的内容

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:3:3}}
```

### 主函数：main()

```rust, editable
{{#include ../geektime_rust_codes/22_mid_term_rgrep/src/lib.rs:5:11}}
```

## 使用

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