# IV 宏编程

<!--ts-->
* [IV 宏编程](#iv-宏编程)
   * [资料](#资料)
   * [宏的分类](#宏的分类)
      * [声明宏(declarative macros): macro_rules!(bang)](#声明宏declarative-macros-macro_rulesbang)
      * [过程宏：深度定制与生成代码](#过程宏深度定制与生成代码)
         * [函数宏](#函数宏)
         * [属性宏](#属性宏)
         * [派生宏](#派生宏)
   * [声明宏](#声明宏)
      * [Rust常用声明宏](#rust常用声明宏)
         * [println!](#println)
      * [示例](#示例)
         * [macro_rules!定义](#macro_rules定义)
         * [使用](#使用)
      * [声明宏用到的参数类型](#声明宏用到的参数类型)
   * [过程宏](#过程宏)
      * [Cargo.toml添加proc-macro声明](#cargotoml添加proc-macro声明)
      * [使用cargo中定义的声明宏](#使用cargo中定义的声明宏)
      * [使用](#使用-1)
   * [函数宏](#函数宏-1)
   * [属性宏](#属性宏-1)
   * [派生宏](#派生宏-1)
      * [常用派生宏](#常用派生宏)
         * [#[derive(Debug)]](#derivedebug)
      * [定义](#定义)
      * [使用](#使用-2)
         * [builder](#builder)
         * [builderwithattr](#builderwithattr)
      * [不用派生宏, 只用rust的写法](#不用派生宏-只用rust的写法)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Sep 23 08:28:42 UTC 2022 -->

<!--te-->

## 资料

- [宏 - Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/ch19-06-macros.html)
- [Macros - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [Macros By Example - The Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)

## 宏的分类

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

### 示例

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

## 过程宏

> 过程宏要比声明宏要复杂很多，不过无论是哪一种过程宏，本质都是一样的，都涉及要把 `输入的 TokenStream` 处理成`输出的 TokenStream`。

### Cargo.toml添加proc-macro声明

> 这样，编译器才允许你使用 #[proc_macro] 相关的宏。

```toml, editable
{{#include ../geektime_rust_codes/47_48_macros/Cargo.toml:8:9}}
```

### 使用cargo中定义的声明宏

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

### 使用

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/query.rs:9:15}}
```

1. .parse().unwrap(): 字符串自动转为TokenStream类型

```shell
cargo run --example query
TokenStream [
    Ident {
        ident: "SELECT",
        span: #0 bytes(43..49),
    },
    Punct {
        ch: '*',
        spacing: Alone,
        span: #0 bytes(50..51),
    },
    Ident {
        ident: "FROM",
        span: #0 bytes(52..56),
    },
    Ident {
        ident: "users",
        span: #0 bytes(57..62),
    },
    Ident {
        ident: "WHERE",
        span: #0 bytes(63..68),
    },
    Ident {
        ident: "age",
        span: #0 bytes(69..72),
    },
    Punct {
        ch: '>',
        spacing: Alone,
        span: #0 bytes(73..74),
    },
    Literal {
        kind: Integer,
        symbol: "10",
        suffix: None,
        span: #0 bytes(75..77),
    },
]
    Finished dev [unoptimized + debuginfo] target(s) in 33.98s
     Running `/Users/kuanhsiaokuo/Developer/spare_projects/rust_lab/geektime-rust/geektime_rust_codes/target/debug/examples/query`
Hello world!
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

## 函数宏

## 属性宏

## 派生宏

### 常用派生宏

#### #[derive(Debug)]

### 定义

> 对于 derive macro，要使用 proce_macro_derive 这个宏

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/lib.rs:17:34}}
```

### 使用

#### builder

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/command.rs}}
```

1. #[derive(Debug, Builder)]

#### builderwithattr

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/command_with_attr.rs}}
```

### 不用派生宏, 只用rust的写法

```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/src/raw_builder.rs}}
```
