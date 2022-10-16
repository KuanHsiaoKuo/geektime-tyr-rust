# 泛型

<!--ts-->
* [泛型](#泛型)
   * [泛型就像定义函数](#泛型就像定义函数)
   * [泛型基本使用示例](#泛型基本使用示例)
   * [实现方式](#实现方式)
   * [泛型数据结构](#泛型数据结构)
      * [逐步约束：把决策交给使用者](#逐步约束把决策交给使用者)
   * [泛型参数](#泛型参数)
      * [参数多态](#参数多态)
      * [三种使用场景](#三种使用场景)
         * [延迟绑定](#延迟绑定)
         * [额外类型](#额外类型)
         * [多个实现](#多个实现)
   * [泛型函数](#泛型函数)
      * [单态化](#单态化)
         * [优劣](#优劣)
      * [返回值携带泛型参数](#返回值携带泛型参数)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct 16 16:20:30 UTC 2022 -->

<!--te-->

## 泛型就像定义函数

1. 函数，是把重复代码中的参数抽取出来，使其更加通用，调用函数的时候，根据参数的不同，我们得到不同的结果；
2. 而泛型，是把重复数据结构中的参数抽取出来，在使用泛型类型时，根据不同的参数，我们会得到不同的具体类型。

## 泛型基本使用示例

~~~admonish info title='泛型结构Vec<T>例子' collapsible=true
```rust, editable

pub struct Vec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

pub struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    alloc: A,
}
```
---
Vec有两个参数:
1. 一个是 T，是列表里的每个数据的类型
2. 另一个是 A，它有进一步的限制 A: Allocator 
> 也就是说 A 需要满足 Allocator trait。
> A 这个参数有默认值 Global，它是 Rust 默认的全局分配器
3. 这也是为什么 Vec 虽然有两个参数，使用时都只需要用 T。
~~~

~~~admonish info title='枚举类型Cow<T>例子' collapsible=true
```rust, editable

pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    // 借用的数据
    Borrowed(&'a B),
    // 拥有的数据
    Owned(<B as ToOwned>::Owned),
}
```
这里对 B 的三个约束分别是：
1. 生命周期 'a
2. 长度可变 ?Sized
3. 符合 ToOwned trait
~~~

~~~admonish info title='Cow' collapsible=true
Cow（Clone-on-Write）是 Rust 中一个很有意思且很重要的数据结构。它就像 Option 一样，在返回数据的时候，提供了一种可能：要么返回一个借用的数据（只读），要么返回一个拥有所有权的数据（可写）。
~~~

~~~admonish info title='?Sized代表可变大小' collapsible=true
?Sized 是一种特殊的约束写法，? 代表可以放松问号之后的约束。由于 Rust 默认的泛型参数都需要是 Sized，也就是固定大小的类型，所以这里 ?Sized 代表用可变大小的类型。
~~~

~~~admonish info title='ToOwned' collapsible=true
[ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) 是一个 trait，它可以把借用的数据克隆出一个拥有所有权的数据。
~~~

~~~admonish info title='\<B as ToOwned\>::Owned' collapsible=true
它对 B 做了一个强制类型转换，转成 ToOwned trait，然后访问 ToOwned trait 内部的 Owned 类型
----
因为在 Rust 里，子类型可以强制转换成父类型，B 可以用 ToOwned 约束，所以它是 ToOwned trait 的子类型，因而 B 可以安全地强制转换成 ToOwned。这里 B as ToOwned 是成立的。
~~~

## 实现方式

> 阅读这张图的内容，可以理出不少设计。比如trait object和trait bound其实从一开始就是不同路线的解决方案

~~~admonish info title='不同语言实现泛型的方式' collapsible=true
![不同语言实现泛型的方式](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F.png)
~~~

## 泛型数据结构

### 逐步约束：把决策交给使用者

~~~admonish info title='在不同的实现下逐步添加约束' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/reader.rs}}
```
~~~

## 泛型参数

### 参数多态

### 三种使用场景

#### 延迟绑定

#### 额外类型

#### 多个实现

## 泛型函数

### 单态化

~~~admonish info title='编译时展开泛型参数单态化' collapsible=true
> 对于泛型函数，Rust 会进行单态化（Monomorphization）处理，也就是在编译时，把所有用到的泛型函数的泛型参数展开，生成若干个函数。
> 所以，下方的 id() 编译后会得到 一个处理后的多个版本
----
```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/id.rs}}
```
~~~

~~~admonish info title='单态化的优劣' collapsible=true

1. 单态化的好处是:

- 泛型函数的调用是静态分派（static dispatch） 在编译时就一一对应
- 既保有多态的灵活性，又没有任何效率的损失，和普通函数调用一样高效。

2. 坏处：编译慢、文件大、丢失泛型信息。这反过来又是动态分派的好处

- 但是对比刚才编译会展开的代码也能很清楚看出来，单态化有很明显的坏处
- 就是编译速度很慢，一个泛型函数，编译器需要找到所有用到的不同类型，一个个编译
- 所以 Rust 编译代码的速度总被人吐槽，这和单态化脱不开干系（另一个重要因素是宏）。
- 同时，这样编出来的二进制会比较大，因为泛型函数的二进制代码实际存在 N 份。
- 还有一个可能你不怎么注意的问题：因为单态化，代码以二进制分发会损失泛型的信息。
- 如果我写了一个库，提供了如上的 id() 函数，使用这个库的开发者如果拿到的是二进制
- 那么这个二进制中必须带有原始的泛型函数，才能正确调用。但单态化之后，原本的泛型信息就被丢弃了。
~~~

#### 优劣

### 返回值携带泛型参数
