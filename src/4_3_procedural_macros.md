# 过程宏

<!--ts-->
* [过程宏](#过程宏)
   * [手工定义图](#手工定义图)
   * [Cargo.toml添加proc-macro声明](#cargotoml添加proc-macro声明)
   * [函数宏](#函数宏)
   * [派生宏:](#派生宏)
      * [常用派生宏](#常用派生宏)
         * [#[derive(Debug)]](#derivedebug)
      * [手工实现builder模式](#手工实现builder模式)
         * [实现效果：链式调用](#实现效果链式调用)
         * [派生宏思路](#派生宏思路)
      * [自动实现：使用syn/quote可以不用自己定义模版](#自动实现使用synquote可以不用自己定义模版)
   * [过程属性宏: proc_macro_derive(macro_name, attributes(attr_name))](#过程属性宏-proc_macro_derivemacro_name-attributesattr_name)
      * [使用syn/quote定义属性宏](#使用synquote定义属性宏)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct 23 06:38:30 UTC 2022 -->

<!--te-->

## 手工定义图

> 过程宏要比声明宏要复杂很多，不过无论是哪一种过程宏，本质都是一样的，都涉及要把 `输入的 TokenStream` 处理成`输出的 TokenStream`。

## Cargo.toml添加proc-macro声明

> 这样，编译器才允许你使用 #[proc_macro] 相关的宏。

```toml, editable
{{#include ../geektime_rust_codes/47_48_macros/Cargo.toml:8:9}}
```

## 函数宏

- \#[proc_macro]

> 和macro_rules! 功能类似，但更为强大。

~~~admonish info title="src/lib.rs:定义过程函数宏示例：可以看到，都是处理TokenStream" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/lib.rs}}
```
~~~

~~~admonish info title="TokenStream" collapsible=true
使用者可以通过 query!(...) 来调用。我们打印传入的 TokenStream，
然后把一段包含在字符串中的代码解析成 TokenStream 返回。

这里可以非常方便地用字符串的 parse() 方法来获得 TokenStream，
是因为 TokenStream 实现了  FromStr trait。
~~~

~~~admonish info title="examples/query.rs使用过冲示例" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/query.rs}}
```
~~~

1. .parse().unwrap(): 字符串自动转为TokenStream类型

```shell
cargo run --example query > examples/query_output.txt
```

~~~admonish info title="运行结果示例：可以看到打印出来的TokenStream" collapsible=true
```shell
{{#include ../geektime_rust_codes/47_48_macros/examples/query_output.txt}}
```
~~~

~~~admonish tip title='TokenStream是一个Iterator，里面包含一系列的TokenTree' collapsible=true
```rust
pub enum TokenTree {
    // 组，如果代码中包含括号，比如{} [] <> () ，那么内部的内容会被分析成一个Group（组）
    Group(Group), 
    // 标识符
    Ident(Ident),
    // 标点符号 
    Punct(Punct),
    // 字面量 
    Literal(Literal), 
}
```
~~~

~~~admonish info title="Group Example" collapsible=true
```rust
use macros::query;

fn main() {
    // query!(SELECT * FROM users WHERE age > 10);
    query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id);
    hello()
}
```
~~~

## 派生宏:

#[proc_macro_devive(DeriveMacroName)]

> 用于结构体（struct）、枚举（enum）、联合（union）类型，可为其实现函数或特征（Trait）

### 常用派生宏

#### #[derive(Debug)]

### 手工实现builder模式

#### 实现效果：链式调用

~~~admonish info title="想到达到链式调用的效果" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/mannual_command.rs:56:74}}
```
~~~

~~~admonish info title="可以这样定义" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/mannual_command.rs:1:53}}
```
~~~

> 但是有点繁琐，可以使用派生宏派生出这些代码

#### 派生宏思路

~~~admonish info title="要生成的代码模版: 把输入的 TokenStream 抽取出来，也就是把在 struct 的定义内部，每个域的名字及其类型都抽出来，然后生成对应的方法代码。" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/templates/builder.j2}}
```
---
1. 7-12: 这里的 fileds / builder_name 是我们要传入的参数，每个 field 还需要 name 和 ty 两个 属性，分别对应 field 的名字和类型
2. 25-26: 对于原本是 Option<T> 类型的域，要避免生成 Option<Option>，我们需要把是否是 Option 单独抽取出来，如果是 Option<T>，那么 ty 就是 T。所以，field 还需要一个属 性
   optional。
~~~

~~~admonish info title="构建对应数据结构" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:6:13}}
```
~~~

~~~admonish info title="src/lib.rs: 使用派生宏从TokenStream抽取出想要的信息" collapsible=true
> 对于 derive macro，要使用 proce_macro_derive 这个宏。我们把这个 derive macro 命名为 Builder。

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/lib.rs:18:23}}
```
~~~

~~~admonish info title="examples/raw_command.rs: 使用这个派生宏抽取" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/raw_command.rs}}
```
~~~

~~~admonish info title="运行，查看获取的TokenStream" collapsible=true
```shell
cargo run --example raw_command > examples/raw_command_output.txt
```

```shell
{{#include ../geektime_rust_codes/47_48_macros/examples/raw_command_output.txt}}
```
~~~

~~~admonish info title='打印信息说明' collapsible=true
1. 首先有一个 Group，包含了 #[allow(dead_code)] 属性的信息。因为我们现在拿到 的 derive 下的信息，所以所有不属于 #[derive(...)] 的属性，都会被放入 TokenStream 中。

2. 之后是 pub / struct / Command 三个 ident。

3. 随后又是一个 Group，包含了每个 field 的信息。我们看到，field 之间用逗号这个 Punct 分隔，field 的名字和类型又是通过冒号这个 Punct 分隔。而类型，可能是一个 Ident，如 String，或者一系列 Ident / Punct，如 Vec / < / String / >。
~~~

~~~admonish info title="src/raw_builder.rs: 使用anyhow与askama抽取TokenStream中的信息" collapsible=true
> 我们要做的就是，把这个 TokenStream 中的 struct 名字，以及每个 field 的名字和类型拿出来。
> 如果类型是 Option<T>，那么把 T 拿出来，把 optional 设置为 true。

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:1:21}}
```
~~~

~~~admonish info title="templates/builder.j2: 上面askama用到的jinja2模版" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/templates/builder.j2}}
```
~~~

~~~admonish info title="src/raw_builder.rs: 实现对应抽取方法" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:23:}}
```
~~~

~~~admonish tip title="提示：类比理解" collapsible=true
可以对着打印出来的 TokenStream 和刚才的分析进行理解。
核心的就是 get_struct_fields() 方法，如果觉得难懂，
可以想想如果要把一个 a=1,b=2 的字符串切成 [[a, 1], [b, 2]] 该怎么做，就很容易理解了。
~~~

### 自动实现：使用syn/quote可以不用自己定义模版

> 详见上方对比图

## 过程属性宏: proc_macro_derive(macro_name, attributes(attr_name))

> 用于属性宏， 用在结构体、字段、函数等地方，为其指定属性等功能, 类似python的计算属性

~~~admonish tip title='定义结构体时在某个字段上方使用对应attr_name' collapsible=true
```rust
#[allow(dead_code)]
#[derive(Debug, BuilderWithAttr)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env", default = "vec![]")]
    env: Vec<String>,
    current_dir: Option<String>,
}
```
~~~

### 使用syn/quote定义属性宏

> 详见上方对比图
