# Trait Object

<!--ts-->
* [Trait Object](#trait-object)
   * [子类型多态: 动态分派](#子类型多态-动态分派)
   * [实现机理：ptr+vtable](#实现机理ptrvtable)
   * [对象安全](#对象安全)
   * [使用场景](#使用场景)
      * [在函数中使用](#在函数中使用)
      * [在函数返回值中使用](#在函数返回值中使用)
      * [在数据结构中使用](#在数据结构中使用)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Oct 19 08:52:55 UTC 2022 -->

<!--te-->

## 子类型多态: 动态分派

~~~admonish info title='在运行期决定' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/formatter.rs}}
```
----

要有一种手段，告诉编译器，此处需要并且仅需要任何实现了 Formatter 接口的数据类型。在 Rust 里，这种类型叫 Trait Object，表现为 &dyn Trait 或者 Box。
1. 这里结构体只是声明了一下，并不关注其包含什么字段
~~~

## 实现机理：ptr+vtable

~~~admonish info title='Trait Object的底层逻辑就是胖指针' collapsible=true
![13｜类型系统：如何使用trait来定义接口？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F-4258625.jpg)
----
HtmlFormatter 的引用赋值给 Formatter 后，会生成一个 Trait Object，在上图中可以看到，Trait Object 的底层逻辑就是胖指针。
> 其中，一个指针指向数据本身，另一个则指向虚函数表（vtable）。
~~~

~~~admonish info title='vtable是一张静态表' collapsible=true
![13｜类型系统：如何使用trait来定义接口？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F-4258661.jpg)
----
1. vtable 是一张静态的表，Rust 在编译时会为使用了 trait object 的类型的 trait 实现生成一张表，放在可执行文件中（一般在 TEXT 或 RODATA 段）
> 在这张表里，包含具体类型的一些信息，如 size、aligment 以及一系列函数指针
> 这个接口支持的所有的方法，比如 format() ；具体类型的 drop trait，当 Trait object 被释放，它用来释放其使用的所有资源。这样，当在运行时执行 formatter.format() 时，formatter 就可以从 vtable 里找到对应的函数指针，执行具体的操作。
~~~

~~~admonish info title='vtable会为每个类型的每个trait实现一张表' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/trait_object_internal.rs}}
```
~~~

## 对象安全

~~~admonish info title='那什么样的 trait 不是对象安全的呢？' collapsible=true
1. 如果 trait 所有的方法，返回值是 Self 或者携带泛型参数，那么这个 trait 就不能产生 trait object。
2. 不允许返回 Self，是因为 trait object 在产生时，原来的类型会被抹去，所以 Self 究竟是谁不知道。
3. 比如 Clone trait 只有一个方法 clone()，返回 Self，所以它就不能产生 trait object。
4. 不允许携带泛型参数，是因为 Rust 里带泛型的类型在编译时会做单态化，而 trait object 是运行时的产物，两者不能兼容。
5. 比如 Fromtrait，因为整个 trait 带了泛型，每个方法也自然包含泛型，就不能产生 trait object。如果一个 trait 只有部分方法返回 Self 或者使用了泛型参数，那么这部分方法在 trait object 中不能调用。
~~~

## 使用场景

### 在函数中使用

### 在函数返回值中使用

### 在数据结构中使用
