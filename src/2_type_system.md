# II. 类型系统

<!--ts-->
* [II. 类型系统](#ii-类型系统)
   * [类型系统分类图](#类型系统分类图)
      * [三个标准](#三个标准)
      * [Rust类型系统特点](#rust类型系统特点)
   * [类型分类](#类型分类)
      * [原生类型](#原生类型)
      * [组合类型](#组合类型)
      * [自定义组合类型](#自定义组合类型)
   * [小例子](#小例子)
      * [常量](#常量)
   * [类型推导](#类型推导)
* [泛型](#泛型)
   * [实现方式](#实现方式)
   * [泛型数据结构](#泛型数据结构)
      * [逐步约束：把决策交给使用者](#逐步约束把决策交给使用者)
   * [泛型参数](#泛型参数)
      * [参数多态](#参数多态)
      * [三种使用场景](#三种使用场景)
         * [延迟绑定](#延迟绑定)
         * [延迟绑定](#延迟绑定-1)
         * [多个实现](#多个实现)
   * [泛型函数](#泛型函数)
      * [单态化](#单态化)
         * [示例](#示例)
         * [优劣](#优劣)
      * [返回值携带泛型参数](#返回值携带泛型参数)
* [Trait](#trait)
   * [基本练习](#基本练习)
      * [Self和self](#self和self)
      * [递进练习trait使用](#递进练习trait使用)
         * [基础定义trait](#基础定义trait)
         * [添加泛型参数作为泛型约束](#添加泛型参数作为泛型约束)
         * [使用关联类型+添加Result&lt;T, E&gt;](#使用关联类型添加resultt-e)
      * [泛型约束](#泛型约束)
         * [思考题](#思考题)
         * [解决方案](#解决方案)
      * [关联类型](#关联类型)
      * [支持泛型](#支持泛型)
         * [版本一：支持数字相加](#版本一支持数字相加)
      * [支持继承](#支持继承)
      * [子类型多态](#子类型多态)
   * [Trait Object](#trait-object)
      * [实现机理：ptr+vtable](#实现机理ptrvtable)
      * [使用场景](#使用场景)
         * [在函数中使用](#在函数中使用)
         * [在函数返回值中使用](#在函数返回值中使用)
            * [在数据结构中使用](#在数据结构中使用)
   * [常用trait](#常用trait)
      * [内存相关](#内存相关)
      * [标签trait](#标签trait)
      * [类型转换](#类型转换)
      * [操作符相关](#操作符相关)
   * [设计架构](#设计架构)
      * [顺手自然](#顺手自然)
      * [桥接](#桥接)
      * [控制反转](#控制反转)
      * [SOLID原则](#solid原则)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Sep 29 09:51:59 UTC 2022 -->

<!--te-->

## 类型系统分类图

![类型系统分类图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F.jpg)

### 三个标准

1. 隐式转换
2. 检查时机
3. 多态支持

### Rust类型系统特点

![Rust的类型系统有什么特点？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257735.jpg)

## 类型分类

### 原生类型

![Rust原生类型](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257523.jpg)

### 组合类型

![Rust组合类型](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F-4257587.jpg)

### 自定义组合类型

## 小例子

### 常量

```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/constant.rs}}
```

## 类型推导

# 泛型

## 实现方式

![不同语言实现泛型的方式](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F.png)

## 泛型数据结构

### 逐步约束：把决策交给使用者

```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/reader.rs}}
```

## 泛型参数

### 参数多态

### 三种使用场景

#### 延迟绑定

#### 延迟绑定

#### 多个实现

## 泛型函数

### 单态化

> 对于泛型函数，Rust 会进行单态化（Monomorphization）处理，也就是在编译时，把所有用到的泛型函数的泛型参数展开，生成若干个函数。
> 所以，下方的 id() 编译后会得到 一个处理后的多个版本

#### 示例

```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/id.rs}}
```

#### 优劣

### 返回值携带泛型参数

# Trait

![trait概览图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F.jpg)

## 基本练习

### Self和self

```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/write.rs}}
```

### 递进练习trait使用

#### 基础定义trait

```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse.rs}}
```

#### 添加泛型参数作为泛型约束

```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse1.rs}}
```

#### 使用关联类型+添加Result<T, E>

```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/parse2.rs}}
```

### 泛型约束

#### 思考题

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

~~~admonish tip title='分析编译报错原因'
主要原因是，实现 new 方法时，对泛型的约束要求要满足 W: Write，而 new 的声明返回值是 Self，也就是说 self.wirter 必须是 W: Write 类型(泛型)，但实际返回值是一个确定的类型 BufWriter<TcpStream>，这不满足要求。
~~~

#### 解决方案

~~~admonish info title='解决方案梳理'
1. 修改 new 方法的返回值
2. 对确定的类型 MyWriter<BufWriter<TcpStream>>实现 new 方法
3. 修改 new 方法的实现，使用依赖注入
~~~

```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/writer.rs}}
```

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

### 关联类型

### 支持泛型

#### 版本一：支持数字相加

```rust, editable
{{#include ../geektime_rust_codes/13_traits/src/add.rs}}
```

### 支持继承

### 子类型多态

## Trait Object

### 实现机理：ptr+vtable

![13｜类型系统：如何使用trait来定义接口？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F-4258625.jpg)

![13｜类型系统：如何使用trait来定义接口？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F-4258661.jpg)

### 使用场景

#### 在函数中使用

#### 在函数返回值中使用

##### 在数据结构中使用

## 常用trait

![14｜类型系统：有哪些必须掌握的trait？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/14%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E6%9C%89%E5%93%AA%E4%BA%9B%E5%BF%85%E9%A1%BB%E6%8E%8C%E6%8F%A1%E7%9A%84trait%EF%BC%9F.jpg)

### 内存相关

### 标签trait

### 类型转换

### 操作符相关

## 设计架构

### 顺手自然

### 桥接

### 控制反转

### SOLID原则

