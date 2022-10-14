# Trait Impl

<!--ts-->
* [Trait Impl](#trait-impl)
   * [基本练习](#基本练习)
      * [支持泛型](#支持泛型)
      * [支持继承](#支持继承)
      * [Self和self](#self和self)
   * [递进练习trait使用场景](#递进练习trait使用场景)
      * [基础使用：具体类型实现](#基础使用具体类型实现)
      * [进阶使用](#进阶使用)
         * [泛型实现+trait约束](#泛型实现trait约束)
         * [trait带有泛型参数+trait约束](#trait带有泛型参数trait约束)
      * [补充使用：使用关联类型+添加Result&lt;T, E&gt;](#补充使用使用关联类型添加resultt-e)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 14 08:27:17 UTC 2022 -->

<!--te-->

## 基本练习

### 支持泛型

~~~admonish info title='版本一：支持数字相加' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/add.rs}}
```
~~~

### 支持继承

~~~admonish info title='trait B:A' collapsible=true
在 Rust 中，一个 trait 可以“继承”另一个 trait 的关联类型和关联函数。比如 trait B: A ，是说任何类型 T，如果实现了 trait B，它也必须实现 trait A，换句话说，trait B 在定义时可以使用 trait A 中的关联类型和方法。
```rust, editable
impl<T: ?Sized> StreamExt for T where T: Stream {}
```
----
如果你实现了 Stream trait，就可以直接使用 StreamExt 里的方法了
~~~

### Self和self

> 类比python：Self对应Cls， self两边一样。

~~~admonish info title='Self和self区别使用, Self其实就是类方法' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/write.rs}}
```
~~~

~~~admonish info title="顺便区分一下类方法和静态方法" collapsible=true
1. 定义时：类方法需要加cls参数，静态方法不需要
2. 调用时：类方法默认添加cls参数, 静态方法不会传入
~~~

~~~admonish info title='self: Self, 实例来自于类型' collapsible=true
1. Self 代表当前的类型，比如 File 类型实现了 Write，那么实现过程中使用到的 Self 就指代 File。
2. self 在用作方法的第一个参数时，实际上是 self: Self 的简写，所以 &self 是 self: &Self, 而 &mut self 是 self: &mut Self。
~~~

## 递进练习trait使用场景

### 基础使用：具体类型实现

~~~admonish info title='定义Parse trait并实现使用' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse.rs}}
```
---
1. 这里的Parse Trait里面的parse方法没有传入self/Self参数，所以调用的时候使用::而不是.
2. 这种基础用法中，被实现的是具体类型
~~~

### 进阶使用

1. 这也是分离定义与实现的用处
2. 下方的[常用trait](2_3_3_trait_frequently.html#常用trait)实现也是基于进阶使用整理出来提供的工具。

#### 泛型实现+trait约束

~~~admonish info title='impl<T> Parse for T' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse1.rs}}
```
1. 对比上一个例子，这里被实现的是泛型，对于上一种用法进一步抽象
2. 这样就把被实现类型从一个具体类型，扩展为一类实现了具体trait的类型，不需要重复去实现trait
> 这个抽象点多体会一下：从抓类型到抓实现特定trait的泛型
~~~

#### trait带有泛型参数+trait约束

> 使用一个思考题来加深印象

~~~admonish info title='泛型参数impl报错' collapsible=true
```rust, editable
use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let writer = MyWriter::new("127.0.0.1:8080");
    writer.write("hello world!");
}
```
~~~

~~~admonish tip title='分析编译报错原因'
主要原因是，实现 new 方法时，对泛型的约束要求要满足 W: Write，而 new 的声明返回值是 Self，也就是说 self.wirter 必须是 W: Write 类型(泛型)，但实际返回值是一个确定的类型 BufWriter<TcpStream>，这不满足要求。
~~~

~~~admonish info title='解决方案梳理'
1. 修改 new 方法的返回值
2. 对确定的类型 MyWriter<BufWriter<TcpStream>>实现 new 方法
3. 修改 new 方法的实现，使用依赖注入
~~~

~~~admonish info title='1. 修改new方法返回值' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/writer.rs}}
```
~~~

~~~admonish info title='2. 针对实现new方法' collapsible=true
```rust, editable
impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }
}

fn main() {
    let mut writer = MyWriter::new("127.0.0.1:8080");
    writer.write("hello world!");
}
```
~~~

~~~admonish info title='3. 使用依赖注入修改new方法实现' collapsible=true
```rust, editable
impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
        }
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut writer = MyWriter::new(BufWriter::new(stream));
    writer.write("hello world!");
}
```
~~~

### 补充使用：使用关联类型+添加Result<T, E>

~~~admonish info title='关联类型自定义Error' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse2.rs}}
```
---
```rust
type Error = String;
fn parse(s: &str) -> Result<Self, Self::Error>
```
~~~
