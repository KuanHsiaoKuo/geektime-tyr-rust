# IV 宏编程

<!--ts-->
* [IV 宏编程](#iv-宏编程)
   * [资料](#资料)
   * [宏的分类](#宏的分类)
      * [使用泳道图](#使用泳道图)
      * [表格](#表格)
      * [声明宏的缺陷，而后有了过程宏](#声明宏的缺陷而后有了过程宏)
      * [声明宏(declarative macros): macro_rules!(bang)](#声明宏declarative-macros-macro_rulesbang)
      * [过程宏：深度定制与生成代码](#过程宏深度定制与生成代码)
         * [函数宏](#函数宏)
         * [属性宏](#属性宏)
         * [派生宏](#派生宏)
   * [声明宏](#声明宏)
      * [Rust常用声明宏](#rust常用声明宏)
         * [println!](#println)
         * [writeln!](#writeln)
         * [eprintln!](#eprintln)
      * [示例](#示例)
         * [声明宏示意图](#声明宏示意图)
         * [macro_rules!定义](#macro_rules定义)
         * [使用](#使用)
      * [声明宏用到的参数类型](#声明宏用到的参数类型)
   * [过程宏手工定义图](#过程宏手工定义图)
      * [Cargo.toml添加proc-macro声明](#cargotoml添加proc-macro声明)
   * [过程函数宏: #[proc_macro]](#过程函数宏-proc_macro)
      * [src/lib.rs:定义过程函数宏](#srclibrs定义过程函数宏)
      * [examples/query.rs:使用](#examplesqueryrs使用)
   * [过程派生宏: /#[proc_macro_devive(DeriveMacroName)]](#过程派生宏-proc_macro_devivederivemacroname)
      * [常用派生宏](#常用派生宏)
         * [#[derive(Debug)]](#derivedebug)
      * [原始实现builder模式](#原始实现builder模式)
         * [想到达到链式调用的效果](#想到达到链式调用的效果)
         * [可以这样定义](#可以这样定义)
         * [但是有点繁琐，可以使用派生宏派生出这些代码](#但是有点繁琐可以使用派生宏派生出这些代码)
      * [派生宏思路](#派生宏思路)
         * [要生成的代码模版](#要生成的代码模版)
         * [构建对应数据结构](#构建对应数据结构)
         * [src/lib.rs: 使用派生宏从TokenStream抽取出想要的信息](#srclibrs-使用派生宏从tokenstream抽取出想要的信息)
         * [examples/raw_command.rs: 使用这个派生宏抽取](#examplesraw_commandrs-使用这个派生宏抽取)
         * [运行，查看获取的TokenStream](#运行查看获取的tokenstream)
         * [src/raw_builder.rs: 使用anyhow与askama抽取TokenStream中的信息](#srcraw_builderrs-使用anyhow与askama抽取tokenstream中的信息)
         * [templates/builder.j2: 上面askama用到的jinja2模版](#templatesbuilderj2-上面askama用到的jinja2模版)
         * [src/raw_builder.rs: 实现对应抽取方法](#srcraw_builderrs-实现对应抽取方法)
      * [使用syn/quote可以不用自己定义模版](#使用synquote可以不用自己定义模版)
   * [过程属性宏: proc_macro_derive(macro_name, attributes(attr_name))](#过程属性宏-proc_macro_derivemacro_name-attributesattr_name)
      * [使用syn/quote定义属性宏](#使用synquote定义属性宏)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Sep 29 10:42:12 UTC 2022 -->

<!--te-->

## 资料

- [宏 - Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/ch19-06-macros.html)
- [Macros - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [Macros By Example - The Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)

## 宏的分类

### 使用泳道图

```plantuml
{{#include ../materials/plantumls/macros_overview.puml}}
```

### 表格

```extended-markdown-table
| Macros            | Define                                                                                 | Usage                                         | note                       | Example          |
|-------------------|----------------------------------------------------------------------------------------|-----------------------------------------------|----------------------------|------------------|
| Declarative Macro | #[macro_export]/macro_rules! macro_name{}                                              | macro_name!()                                 |                            | println!         |
|-------------------|----------------------------------------------------------------------------------------|-----------------------------------------------|----------------------------|------------------|
| Function Macro    | #[proc_macros]/pub fn macro_name                                                       | macro_name!()                                 | advanced declarative macro |                  |
|-------------------|----------------------------------------------------------------------------------------|-----------------------------------------------|----------------------------|------------------|
| Derive Macro      | #[proc_macros_derive(DeriveMacroName)]/pub  fn other_fn_name                           | DeriveMacroName!()                            |                            | #[derive(Debug)] |
|-------------------|----------------------------------------------------------------------------------------|-----------------------------------------------|----------------------------|------------------|
| Attritubte Macro  | #[proc_macros_derive(AttributeMacroName, attributes(attr_name))]/pub  fn other_fn_name | Only Diff with DeriveMacro when define struct |                            |                  |
```

### 声明宏的缺陷，而后有了过程宏

- [Macros in Rust: A tutorial with examples - LogRocket Blog](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#limitationsofdeclarativemacros)

~~~admonish tip title='为什么过程宏和声明宏那么像'
> 过程宏的缺陷
1. 缺乏对宏自动完成和扩展的支持 
2. 调试声明性宏很困难 
3. 修改能力有限 
4. 较大的二进制文件 
5. 更长的编译时间（这适用于声明性宏和过程宏）

> 过程宏是语法树级别的转换
过程宏是宏的更高级版本。过程宏允许你扩展现有的 Rust 语法。它接受任意输入并返回有效的 Rust 代码。 
过程宏是将 TokenStream 作为输入并返回另一个 Token Stream 的函数。过程宏操作输入 TokenStream 以产生输出流。
~~~

### 声明宏(declarative macros): macro_rules!(bang)

> 对代码模版做简单替换
> 声明宏可以用 macro_rules! 来描述, 如果重复性的代码无法用函数来封装，那么声明宏就是一个好的选择

### 过程宏：深度定制与生成代码

- [Rust 过程宏(1): 如何硬生生解析和手写过程宏](https://www.bilibili.com/video/BV1Za411q7LQ)

> 主要以如何使用 function-like macro 在不依赖于 syn / quote 的情况下，把 Json Schema 在编译期转换成 Rust struct。主要目的是让大家熟悉基本的处理 TokenStream 的思路

- [Rust 过程宏(2): 使用 syn/quote 撰写过程宏](https://www.bilibili.com/video/BV1Fu411m7W7)

> 主要通过一个 derive Builder 宏，来展示使用 syn/quote 如何开发过程宏。

- [Rust 过程宏(3): 使用 darling 处理 attributes](https://www.bilibili.com/video/BV1dr4y1v74n)

> 做个收尾，对上一讲的 derive macro 支持 attributes。 我们可以直接解析 attributes 相关的 TokenStream，也可以使用 darling 这个很方便的库，直接把 attributes 像
> Clap/Structopts 那样收集到一个数据结构中，然后再进一步处理。

~~~admonish info title='总结'
这三讲的内容虽然简单，但足以应付大家绝大多数宏编程的需求。
其实我们现在对 syn 库的使用还只是一个皮毛，我们还没有深入
去撰写自己的数据结构去实现 Parse trait，像 DeriveInput 
那样可以直接把 TokenStream 转换成我们想要的东西。

大家感兴趣的话，可以自行去看 syn 库的文档。
~~~

#### 函数宏

看起来像函数的宏，但在编译期进行处理.
> sqlx 用函数宏来处理SQL query、tokio使用属性宏 #[tokio::main] 来引入 runtime。
> 它们可以帮助目标代码的实现逻辑变得更加简单， 但一般除非特别必要，否则并不推荐写。
> 并没有特定的使用场景

#### 属性宏

可以在其他代码块上添加属性，为代码块提供更多功能。

#### 派生宏

为 derive属性添加新的功能。这是我们平时使用最多的宏，比如 #[derive(Debug)].
> 如果你定义的 trait 别人实现起来有固定的模式可循，那么可以考虑为其构建派生宏

## 声明宏

### Rust常用声明宏

#### println!

- [println in std - Rust](https://doc.rust-lang.org/std/macro.println.html)
- [Rust声明宏println剖析_一线coder的博客-CSDN博客_rust 声明宏](https://blog.csdn.net/jiangjkd/article/details/120994956)

```rust
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    })
}
```

#### writeln!

> 可以将内容输入到指定文件

- [writeln in std - Rust](https://doc.rust-lang.org/std/macro.writeln.html)
- [rust入门笔记---翻译rust write！宏 - 知乎](https://zhuanlan.zhihu.com/p/148945862)

```shell
cargo run --example raw_command > examples/raw_command_output.txt
```

#### eprintln!

- [eprintln in std - Rust](https://doc.rust-lang.org/std/macro.eprintln.html)

### 示例

#### 声明宏示意图

```plantuml
{{#include ../materials/plantumls/declarative_macros.puml}}
```

#### macro_rules!定义

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/rule.rs:1:20}}
```

~~~admonish info title="$($el:expr), *)"
1. 在声明宏中，条件捕获的参数使用 $ 开头的标识符来声明。
2. 每个参数都需要提供类型，这里`expr`代表表达式，所以 $el:expr 是说把匹配到的表达式命名为 $el。
3. $(...),* 告诉编译器可以匹配任意多个以逗号分隔的表达式，然后捕获到的每一个表达式可以用 $el 来访问。
4. 由于匹配的时候匹配到一个 $(...)* （我们可以不管分隔符），在执行的代码块中，我们也要相应地使用 $(...)* 展开。
5. 所以这句 $(v.push($el);)* 相当于匹配出多少个 $el就展开多少句 push 语句。
~~~

#### 使用

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/rule.rs:22:}}
```

### 声明宏用到的参数类型

~~~admonish info title='类型列表'
1. item，比如一个函数、结构体、模块等。 
2. block，代码块。比如一系列由花括号包裹的表达式和语句。 
3. stmt，语句。比如一个赋值语句。 
4. pat，模式。 
5. expr，表达式。刚才的例子使用过了。 
6. ty，类型。比如 Vec。 
7. ident，标识符。比如一个变量名。 
8. path，路径。比如：foo、::std::mem::replace、transmute::<_, int>。 
9. meta，元数据。一般是在 #[...] 和 #![...] 属性内部的数据。 
10. tt，单个的 token 树。 
11. vis，可能为空的一个 Visibility 修饰符。比如 pub、pub(crate)
~~~

## 过程宏手工定义图

> 过程宏要比声明宏要复杂很多，不过无论是哪一种过程宏，本质都是一样的，都涉及要把 `输入的 TokenStream` 处理成`输出的 TokenStream`。

### Cargo.toml添加proc-macro声明

> 这样，编译器才允许你使用 #[proc_macro] 相关的宏。

```toml, editable
{{#include ../geektime_rust_codes/47_48_macros/Cargo.toml:8:9}}
```

## 过程函数宏: #[proc_macro]

> 和macro_rules! 功能类似，但更为强大。

### src/lib.rs:定义过程函数宏

> 可以看到，都是处理TokenStream

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/lib.rs}}
```

~~~admonish info title="TokenStream"
使用者可以通过 query!(...) 来调用。我们打印传入的 TokenStream，
然后把一段包含在字符串中的代码解析成 TokenStream 返回。

这里可以非常方便地用字符串的 parse() 方法来获得 TokenStream，
是因为 TokenStream 实现了  FromStr trait。
~~~

### examples/query.rs:使用

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/query.rs}}
```

1. .parse().unwrap(): 字符串自动转为TokenStream类型

```shell
cargo run --example query > examples/query_output.txt
```

```shell
{{#include ../geektime_rust_codes/47_48_macros/examples/query_output.txt}}
```

~~~admonish tip title='TokenStream是一个Iterator，里面包含一系列的TokenTree'
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

~~~admonish info title="Group Example"
```rust
use macros::query;

fn main() {
    // query!(SELECT * FROM users WHERE age > 10);
    query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id);
    hello()
}
```
~~~

## 过程派生宏: /#[proc_macro_devive(DeriveMacroName)]

> 用于结构体（struct）、枚举（enum）、联合（union）类型，可为其实现函数或特征（Trait）

### 常用派生宏

#### #[derive(Debug)]

### 原始实现builder模式

#### 想到达到链式调用的效果

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/mannual_command.rs:56:74}}
```

#### 可以这样定义

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/mannual_command.rs:1:53}}
```

#### 但是有点繁琐，可以使用派生宏派生出这些代码

### 派生宏思路

#### 要生成的代码模版

把输入的 TokenStream 抽取出来，也就是把在 struct 的定义内部，每个域的名字及其类型都抽出来，然后生成对应的方法代码。

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/templates/builder.j2}}
```

1. 7-12: 这里的 fileds / builder_name 是我们要传入的参数，每个 field 还需要 name 和 ty 两个 属性，分别对应 field 的名字和类型
2. 25-26: 对于原本是 Option<T> 类型的域，要避免生成 Option<Option>，我们需要把是否是 Option 单独抽取出来，如果是 Option<T>，那么 ty 就是 T。所以，field 还需要一个属 性
   optional。

#### 构建对应数据结构

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:6:13}}
```

#### src/lib.rs: 使用派生宏从TokenStream抽取出想要的信息

> 对于 derive macro，要使用 proce_macro_derive 这个宏。我们把这个 derive macro 命名为 Builder。

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/lib.rs:18:23}}
```

#### examples/raw_command.rs: 使用这个派生宏抽取

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/raw_command.rs}}
```

#### 运行，查看获取的TokenStream

```shell
cargo run --example raw_command > examples/raw_command_output.txt
```

```shell
{{#include ../geektime_rust_codes/47_48_macros/examples/raw_command_output.txt}}
```

~~~admonish info title='打印信息说明'
1. 首先有一个 Group，包含了 #[allow(dead_code)] 属性的信息。因为我们现在拿到 的 derive 下的信息，所以所有不属于 #[derive(...)] 的属性，都会被放入 TokenStream 中。

2. 之后是 pub / struct / Command 三个 ident。

3. 随后又是一个 Group，包含了每个 field 的信息。我们看到，field 之间用逗号这个 Punct 分隔，field 的名字和类型又是通过冒号这个 Punct 分隔。而类型，可能是一个 Ident，如 String，或者一系列 Ident / Punct，如 Vec / < / String / >。
~~~

#### src/raw_builder.rs: 使用anyhow与askama抽取TokenStream中的信息

> 我们要做的就是，把这个 TokenStream 中的 struct 名字，以及每个 field 的名字和类型拿出来。
> 如果类型是 Option<T>，那么把 T 拿出来，把 optional 设置为 true。

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:1:21}}
```

#### templates/builder.j2: 上面askama用到的jinja2模版

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/templates/builder.j2}}
```

#### src/raw_builder.rs: 实现对应抽取方法

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs:23:}}
```

~~~admonish tip
可以对着打印出来的 TokenStream 和刚才的分析进行理解。
核心的就是 get_struct_fields() 方法，如果觉得难懂，
可以想想如果要把一个 a=1,b=2 的字符串切成 [[a, 1], [b, 2]] 该怎么做，就很容易理解了。
~~~

### 使用syn/quote可以不用自己定义模版

> 详见上方对比图

## 过程属性宏: proc_macro_derive(macro_name, attributes(attr_name))

> 用于属性宏， 用在结构体、字段、函数等地方，为其指定属性等功能, 类似python的计算属性

~~~admonish tip title='定义结构体时在某个字段上方使用对应attr_name'
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
