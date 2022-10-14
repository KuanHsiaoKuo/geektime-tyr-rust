# 类型系统

<!--ts-->
* [类型系统](#类型系统)
   * [Rust类型系统特点](#rust类型系统特点)
   * [类型分类](#类型分类)
      * [原生类型](#原生类型)
      * [组合类型](#组合类型)
      * [自定义组合类型](#自定义组合类型)
   * [小例子](#小例子)
      * [常量](#常量)
   * [类型推导](#类型推导)
   * [Turbofish](#turbofish)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 14 08:35:00 UTC 2022 -->

<!--te-->

## Rust类型系统特点

~~~admonish info title='类型系统分类图' collapsible=false
![类型系统分类图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F.jpg)
~~~

> 三个标准

1. 隐式转换
2. 检查时机
3. 多态支持

~~~admonish info title='Rust的类型系统有什么特点？' collapsible=false
![Rust的类型系统有什么特点？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257735.jpg)
---
强类型 + 静态类型 + 显式类型
~~~

## 类型分类

### 原生类型

~~~admonish info title='Rust原生类型' collapsible=true
![Rust原生类型](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257523.jpg)
~~~

### 组合类型

~~~admonish info title='Rust组合类型' collapsible=true
![Rust组合类型](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257587.jpg)
~~~

### 自定义组合类型

## 小例子

### 常量

~~~admonish info title='常量定义使用' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/constant.rs}}
```
~~~

## 类型推导

~~~admonish info title='Rust编译器可以从上下文自动推导类型' collapsible=true
```rust, editable
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    // 没有这一行就缺少自动推导信息
    // map.insert("hello", "world");
    println!("map: {:?}", map);
}
```
----
把第 5 行这个作用域内的 insert 语句注释去掉，Rust 编译器就会报错：“cannot infer type for type parameter K”。
~~~

~~~admonish info title='Rust编译器不能获取足够上下文信息时，就需要明确类型' collapsible=true
1. 无法自动推导collect返回什么类型
```rust, editable

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);
}
```
----
2. 给even_numbers添加类型声明即可
```rust, editable

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers: Vec<_> = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);
}
```
> 这里编译器只是无法推断出集合类型，但集合类型内部元素的类型，还是可以根据上下文得出，所以我们可以简写成 Vec<_>
3. 也可以让 collect 返回一个明确的类型
```rust, editable

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<_>>();

    println!("{:?}", even_numbers);
}
```
---
> 这里在泛型函数后使用 :: 来强制使用类型 T，这种写法被称为 turbofish
~~~

## Turbofish

~~~admonish info title='一个对 IP 地址和端口转换的例子' collapsible=true
```rust, editable

use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("addr: {:?}, port: {:?}", addr.ip(), addr.port());
}
```
~~~

~~~admonish info title='如果类型在上下文无法被推导出来，又没有 turbofish 的写法，我们就不得不先给一个局部变量赋值时声明类型，然后再返回，这样代码就变得冗余' collapsible=true
```rust
match data {
    Some(s) => v.parse::<User>()?,
    _ => return Err(...),
}
```
~~~


