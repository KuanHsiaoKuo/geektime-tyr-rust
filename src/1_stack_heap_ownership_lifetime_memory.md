# I. 从栈堆、所有权、生命周期开始内存管理

<!--ts-->

* [I. 从栈堆、所有权、生命周期开始内存管理](#i-从栈堆所有权生命周期开始内存管理)
* [内存](#内存)
    * [字符串内存使用图](#字符串内存使用图)
    * [栈](#栈)
        * [栈帧示意图](#栈帧示意图)
        * [考虑栈溢出](#考虑栈溢出)
    * [堆](#堆)
        * [使用堆引用共享数据](#使用堆引用共享数据)
        * [考虑堆溢出](#考虑堆溢出)
* [编程四大类基本概念](#编程四大类基本概念)
    * [1. 数据](#1-数据)
        * [值和类型](#值和类型)
        * [指针和引用](#指针和引用)
    * [2. 代码](#2-代码)
        * [函数 -&gt; 方法 -&gt; 闭包](#函数---方法---闭包)
            * [闭包示意图](#闭包示意图)
        * [接口与虚表](#接口与虚表)
    * [3. 运行方式](#3-运行方式)
        * [并发与并行](#并发与并行)
        * [同步和异步](#同步和异步)
    * [4. 编程范式](#4-编程范式)
        * [泛型编程](#泛型编程)
        * [函数式编程](#函数式编程)
        * [面向对象编程](#面向对象编程)
* [Rust内存管理概览](#rust内存管理概览)
    * [一、所有权: 单一/共享](#一所有权-单一共享)
    * [单一所有权：掌控生杀大权](#单一所有权掌控生杀大权)
        * [从多引用开始](#从多引用开始)
        * [Rust如何解决](#rust如何解决)
            * [方案一、单一所有权](#方案一单一所有权)
            * [方案二、Copy](#方案二copy)
        * [单一所有权规则整理](#单一所有权规则整理)
        * [单一所有权借用](#单一所有权借用)
            * [两种传参方式：传值/传址](#两种传参方式传值传址)
            * [只读借用/引用](#只读借用引用)
            * [借用的生命周期与约束](#借用的生命周期与约束)
            * [可变借用/引用](#可变借用引用)
            * [第一性原理理解单一所有权规则](#第一性原理理解单一所有权规则)
    * [共享内存-多个所有者：引用计数](#共享内存-多个所有者引用计数)
        * [Rc使用说明: 只读引用计数](#rc使用说明-只读引用计数)
            * [Rc使用Box::leak()](#rc使用boxleak)
            * [使用Rc实现DAG](#使用rc实现dag)
        * [RefCell: 提供内部可变性，可变引用计数](#refcell-提供内部可变性可变引用计数)
            * [外部可变性与内部可变性](#外部可变性与内部可变性)
            * [RefCell简单使用](#refcell简单使用)
            * [使用RefCell实现可修改版本DAG](#使用refcell实现可修改版本dag)
        * [线程安全版本计数器：Arc(Rc)、Mutex/RwLock(RefCell)](#线程安全版本计数器arcrcmutexrwlockrefcell)
    * [二、生命周期](#二生命周期)
        * [动态还是静态？](#动态还是静态)
        * [如何识别生命周期](#如何识别生命周期)
            * [两个小例子](#两个小例子)
            * [需要生命周期标注的情况](#需要生命周期标注的情况)
            * [编译器其实会自动进行生命周期标注](#编译器其实会自动进行生命周期标注)
            * [生命周期标注练习](#生命周期标注练习)
            * [生命周期标注的目的](#生命周期标注的目的)
    * [三、融会贯通，从创建到消亡](#三融会贯通从创建到消亡)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Oct  1 07:31:06 UTC 2022 -->

<!--te-->

# 内存

## 字符串内存使用图

~~~admonish info title='字符串内存使用图' collapsible=true
![字符串内存使用图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.jpg)
~~~

## 栈

### 栈帧示意图

~~~admonish info title='栈帧示意图' collapsible=true
![栈帧示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444135.jpg)
~~~

### 考虑栈溢出

## 堆

### 使用堆引用共享数据

~~~admonish info title='使用堆引用共享内存数据' collapsible=true
![使用堆引用共享数据](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444274.jpg)
~~~

### 考虑堆溢出

~~~admonish info title='堆问题示意图' collapsible=true
![堆问题](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.png)
~~~

# 编程四大类基本概念

## 1. 数据

### 值和类型

### 指针和引用

## 2. 代码

### 函数 -> 方法 -> 闭包

#### 闭包示意图

~~~admonish info title='闭包与自由变量' collapsible=true
![闭包与自由变量](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5.jpg)
~~~

### 接口与虚表

~~~admonish info title='虚表示意图' collapsible=true
![虚表](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444557.jpg)
~~~

## 3. 运行方式

### 并发与并行

~~~admonish info title='并发与并行对比' collapsible=true
![并发与并行](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444672.jpg)
~~~

### 同步和异步

## 4. 编程范式

### 泛型编程

~~~admonish info title='泛型编程更抽象，更通用' collapsible=true
![泛型编程更抽象，更通用](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444741.jpg)
~~~

### 函数式编程

### 面向对象编程

# Rust内存管理概览

## 一、所有权: 单一/共享

![单一/共享所有权对比](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4606985.jpg)

## 单一所有权：掌控生杀大权

### 从多引用开始

~~~admonish info title='多重堆引用问题' collapsible=true
![多重堆引用的问题](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/07%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E7%94%9F%E6%9D%80%E5%A4%A7%E6%9D%83%E5%88%B0%E5%BA%95%E5%9C%A8%E8%B0%81%E6%89%8B%E4%B8%8A%EF%BC%9F-4446989.jpg)
~~~

### Rust如何解决

#### 方案一、单一所有权

~~~admonish info title='单一所有权解决多重引用问题' collapsible=true
![rust所有权规则解决多重引用问题](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/07%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E7%94%9F%E6%9D%80%E5%A4%A7%E6%9D%83%E5%88%B0%E5%BA%95%E5%9C%A8%E8%B0%81%E6%89%8B%E4%B8%8A%EF%BC%9F-4446891.jpg)
~~~

#### 方案二、Copy

~~~admonish info title='使用Copy解决多重引用问题' collapsible=true
![Copy解决](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/07%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E7%94%9F%E6%9D%80%E5%A4%A7%E6%9D%83%E5%88%B0%E5%BA%95%E5%9C%A8%E8%B0%81%E6%89%8B%E4%B8%8A%EF%BC%9F-4447051.jpg)
~~~

### 单一所有权规则整理

~~~admonish info title='单一所有权规则整理' collapsible=true
![所有权规则整理](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/07%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E7%94%9F%E6%9D%80%E5%A4%A7%E6%9D%83%E5%88%B0%E5%BA%95%E5%9C%A8%E8%B0%81%E6%89%8B%E4%B8%8A%EF%BC%9F-4447111.jpg)
~~~

### 单一所有权借用

#### 两种传参方式：传值/传址

~~~admonish info title='传值 or 传址' collapsible=true
![传值/传址](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/08%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E5%80%9F%E7%94%A8%E6%98%AF%E5%A6%82%E4%BD%95%E5%B7%A5%E4%BD%9C%E7%9A%84%EF%BC%9F.jpg)
~~~

#### 只读借用/引用

~~~admonish info title='只读借用/引用' collapsible=true
![只读](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/08%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E5%80%9F%E7%94%A8%E6%98%AF%E5%A6%82%E4%BD%95%E5%B7%A5%E4%BD%9C%E7%9A%84%EF%BC%9F-4447323.jpg)
~~~

#### 借用的生命周期与约束

~~~admonish info title='三段生命周期分析' collapsible=true
![三段生命周期分析](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/08%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E5%80%9F%E7%94%A8%E6%98%AF%E5%A6%82%E4%BD%95%E5%B7%A5%E4%BD%9C%E7%9A%84%EF%BC%9F-4447702.jpg)
~~~

#### 可变借用/引用

> 同一个上下文中多个可变引用是不安全的，那如果同时有一个可变引用和若干个只读引 用就可以

#### 第一性原理理解单一所有权规则

~~~admonish info title='第一性原理理解所有权模型：单一所有权/共享所有权' collapsible=true
![第一性原理](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/08%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E5%80%BC%E7%9A%84%E5%80%9F%E7%94%A8%E6%98%AF%E5%A6%82%E4%BD%95%E5%B7%A5%E4%BD%9C%E7%9A%84%EF%BC%9F-4447883.jpg)
~~~

## 共享内存-多个所有者：引用计数

~~~admonish info title='单一所有权与多个所有者是否有冲突？' collapsible=true
1. 静态检查：单一所有权是rust的编译期默认检查内容
2. 动态检查：多个所有者主要是用于共享内存，这是专门提供Rc/Arc、Box::leak()、RefCell/Mutex/RwLock等工具。
----
> 这里其实可以看出rust如何使用'二八法则'解决问题：
- 对于常用场景，用编译期静态检查来默认解决
- 对于特别场景，用专门的语法显式表达出来，提供运行期动态检查来专门解决
~~~

### Rc使用说明: 只读引用计数

~~~admonish info title='对一个 Rc 结构进行 clone()，不会将其内部的数据复制，只会增加引用计数' collapsible=true
```rust, editable
use std::rc::Rc;
fn main() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
}
```
~~~

~~~admonish info title='上方代码Rc引用计数示意图：共享堆内存' collapsible=true
![引用计数示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4604653.jpg)
~~~

~~~admonish info title='clone源码' collapsible=true
```rust, editable
fn clone(&self) -> Rc<T> {
    // 增加引用计数
    self.inner().inc_strong();
    // 通过 self.ptr 生成一个新的 Rc 结构
    Self::from_inner(self.ptr)
}
```
~~~

#### Rc使用Box::leak()

~~~admonish info title='使用Box::leak()创建不受栈内存控制堆堆内存示意图' collapsible=true
![使用Box::leak()创建不受栈内存控制堆堆内存示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4605420.jpg)
~~~

~~~admonish info title='有了 Box::leak()，我们就可以跳出 Rust 编译器的静态检查' collapsible=true
保证 Rc 指向的堆内存，有最大的生命周期，然后我们再通过引用计数，在合适的时机，结束这段内存的生命周期。如果你对此感兴趣，可以看 [Rc::new() 的源码](https://doc.rust-lang.org/src/alloc/rc.rs.html#342-350)。
~~~

#### 使用Rc实现DAG

~~~admonish info title='DAG数据结构示意图' collapsible=true
![DAG数据结构](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4607216.jpg)
~~~

~~~admonish info title='不可修改版本' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/09_multi_owner/src/dag.rs}}
```
- new()：建立一个新的 Node。
- update_downstream()：设置 Node 的 downstream。
- get_downstream()：clone 一份 Node 里的 downstream。
~~~

### RefCell: 提供内部可变性，可变引用计数

#### 外部可变性与内部可变性

~~~admonish info title='外部可变性与内部可变性对比图' collapsible=true
![外部可变性与内部可变性](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4606188-4606221.jpg)
~~~

#### RefCell简单使用

~~~admonish info title='获得 RefCell 内部数据的可变借用' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/09_multi_owner/src/refcell.rs}}
```
~~~

#### 使用RefCell实现可修改版本DAG

~~~admonish info title='不可修改版本' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/09_multi_owner/src/dag_mut.rs}}
```
1. 首先数据结构的 downstream 需要 Rc 内部嵌套一个 RefCell
2. 这样，就可以利用 RefCell 的内部可变性，来获得数据的可变借用
3. 同时 Rc 还允许值有多个所有者。
~~~

~~~admonish info title='RefCell内部可变性示意图' collapsible=true
![使用RefCell实现可修改版本DAG](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4606429.jpg)
~~~

### 线程安全版本计数器：Arc(Rc)、Mutex/RwLock(RefCell)

~~~admonish info title='Rust实现两套不同的引用计数数据结构' collapsible=true
> Arc 内部的引用计数使用了 [Atomic Usize](https://doc.rust-lang.org/src/alloc/sync.rs.html#303-312) ，而非普通的 usize。
> 从名称上也可以感觉出来，Atomic Usize 是 usize 的原子类型，它使用了 CPU 的特殊指令，来保证多线程下的安全。
> 如果你对原子类型感兴趣，可以看 [std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic/index.html) 的文档。

Rust 实现两套不同的引用计数数据结构，完全是为了性能考虑，从这里我们也可以感受到 Rust 对性能的极致渴求:
- 如果不用跨线程访问，可以用效率非常高的 Rc； 如果要跨线程访问，那么必须用 Arc。
- 同样的，RefCell 也不是线程安全的，如果我们要在多线程中，使用内部可变性，Rust 提供了 Mutex 和 RwLock。
~~~

~~~admonish info title='Mutex/RwLock其实是并发的两个方案' collapsible=true
这两个数据结构你应该都不陌生:
1. Mutex 是互斥量，获得互斥量的线程对数据独占访问.
2. RwLock 是读写锁，获得写锁的线程对数据独占访问，但当没有写锁的时候，允许有多个读锁。 
3. 读写锁的规则和 Rust 的借用规则非常类似，我们可以类比着学。
4. Mutex 和 RwLock 都用在多线程环境下，对共享数据访问的保护上。
5. 前面构建的 DAG 如果要用在多线程环境下，需要把 Rc> 替换为 Arc> 或者 Arc>。
~~~

## 二、生命周期

### 动态还是静态？

~~~admonish info title='动态/静态生命周期定义与表示方式' collapsible=true
1. 静态生命周期: 'static str
- 如果一个值的生命周期贯穿整个进程的生命周期，那么我们就称这种生命周期为静态生命周期。
- 当值拥有静态生命周期，其引用也具有静态生命周期。
- 我们在表述这种引用的时候，可以用 'static 来表示。比如： &'static str 代表这是一个具有静态生命周期的字符串引用。
- 一般来说，全局变量、静态变量、字符串字面量（string literal (字面) ）等，都拥有静态生命周期。
- 堆内存，如果使用了 Box::leak 后，也具有静态生命周期。

2. 动态生命周期: 'a 、'b 或者 'hello 这样的小写字符或者字符串来表述
- 如果一个值是在某个作用域中定义的，也就是说它被创建在栈上或者堆上，那么其生命周期是动态的。
- 当这个值的作用域结束时，值的生命周期也随之结束。
- 对于动态生命周期，我们约定用 'a 、'b 或者 'hello 这样的小写字符或者字符串来表述。 
- ' 后面具体是什么名字不重要，它代表某一段动态的生命周期
- 其中， &'a str 和 &'b str 表示这两个字符串引用的生命周期可能不一致。
~~~

~~~admonish info title='动静态生命周期示意图' collapsible=true
![动静态生命周期示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F.jpg)

1. 分配在堆和栈上的内存有其各自的作用域，它们的生命周期是动态的。
2. 全局变量、静态变量、字符串字面量、代码等内容，在编译时，会被编译到可执行文件中的 BSS/Data/RoData/Text 段，然后在加载时，装入内存。
3. 因而，它们的生命周期和进程的生命周期一致，所以是静态的。
4. 所以，函数指针的生命周期也是静态的，因为函数在 Text 段中，只要进程活着，其内存一直存在。
~~~

### 如何识别生命周期

#### 两个小例子

~~~admonish info title='两个小例子' collapsible=true
![识别生命周期的两个小例子](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F-4607802.jpg)
1. x 引用了在内层作用域中创建出来的变量 y。由于，变量从开始定义到其作用域结束的这段时间，是它的生命周期，所以 x 的生命周期 'a 大于 y 的生命周期 'b，当 x 引用 y 时，编译器报错。
----
2. y 和 x 处在同一个作用域下， x 引用了 y，我们可以看到 x 的生命周期 'a 和 y 的生命周期 'b 几乎同时结束，或者说 'a 小于等于 'b，所以，x 引用 y 是可行的。
~~~

#### 需要生命周期标注的情况

~~~admonish info title='missing lifetime specifier' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime.rs}}
```
1. 编译器在编译 max() 函数时，无法判断 s1、s2 和返回值的生命周期。
2. 函数本身携带的信息，就是编译器在编译时使用的全部信息。
3. 这里函数本身提供的信息就告诉编译期，生命周期不一致
~~~

~~~admonish info title='添加生命周期标注即可编译通过' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime1.rs}}
```
~~~

#### 编译器其实会自动进行生命周期标注

> 编译器希望尽可能减轻开发者的负担，其实所有使用了引用的函数，都需要生命周期的标注，只不过编译器会自动做这件事，省却了开发者的麻烦

~~~admonish info title='编译器自动进行生命周期标注' collapsible=true
1. 无标注版本
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime4.rs}}
```
----
2. 自动标注

```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime3.rs}}
```
> 返回值如何标注？是 'a 还是'b 呢？这里的冲突，编译器无能为力。
~~~

~~~admonish info title='自动标注规则' collapsible=true
1. 所有引用类型的参数都有独立的生命周期 'a 、'b 等。
2. 如果只有一个引用型输入，它的生命周期会赋给所有输出。
3. 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。
~~~

#### 生命周期标注练习

~~~admonish info title='标注练习题' collapsible=true
```rust, editable

pub fn strtok(s: &mut &str, delimiter: char) -> &str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else { // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}
```
1. 按照编译器的规则， &mut &str 添加生命周期后变成 &'b mut &'a str
2. 这将导致返回的 '&str 无法选择一个合适的生命周期。
~~~

~~~admonish info title='标注练习题参考' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/strtok.rs}}
```
----
![标注练习示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F-4609161.jpg)
~~~

#### 生命周期标注的目的

~~~admonish info title='生命周期标注的目的是，在参数和返回值之间建立联系或者约束' collapsible=true
生命周期标注的目的是，在参数和返回值之间建立联系或者约束:
1. 调用函数时，传入的参数的生命周期需要大于等于标注的生命周期。
2. 当每个函数都添加好生命周期标注后，编译器，就可以从函数调用的上下文中分析出，在传参时，引用的生命周期，是否和函数签名中要求的生命周期匹配。
3. 如果不匹配，就违背了“引用的生命周期不能超出值的生命周期”，编译器就会报错。
~~~

## 三、融会贯通，从创建到消亡

## 创建

### 堆内存生命周期管理发展史

~~~admonish info title='堆内存生命周期管理发展史' collapsible=true
![堆内存生命周期管理发展史](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F.jpg)
~~~

~~~admonish info title='堆内存管理需求：动态大小 or 生命周期' collapsible=true

Rust 的创造者们，重新审视了堆内存的生命周期，发现:

- 大部分堆内存的需求在于动态大小
- 小部分需求是更长的生命周期。

> 所以它默认将堆内存的生命周期和使用它的栈内存的生命周期绑在一起，并留了个小口子 leaked 机制，让堆内存在需要的时候，可以有超出帧存活期的生命周期。

----

![Rust与其他编程语言堆内存管理对比](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4639705.jpg)
~~~

### struct/enum/vec/String创建时的内存布局

#### struct

~~~admonish info title='内存布局优化示意图' collapsible=true
![内存布局优化示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4639867.jpg)
~~~

~~~admonish info title='c语言手动优化内存布局' collapsible=true
![c语言手动优化内存布局](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4639905.jpg)
~~~

~~~admonish info title='c语言手动优化内存布局与rust自动优化内存布局对比' collapsible=true
![c语言手动优化内存布局与rust自动优化内存布局对比](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4639936.jpg)
~~~

~~~admonish info title='代码对比rust和clang的内存布局优化' collapsible=true
```c
#include <stdio.h>

struct S1 {
    u_int8_t a;
    u_int16_t b;
    u_int8_t c;
};

struct S2 {
    u_int8_t a;
    u_int8_t c;
    u_int16_t b;
};

void main() {
    printf("size of S1: %d, S2: %d", sizeof(struct S1), sizeof(struct S2));
}
```
----
```rust, editable
{{#include ../geektime_rust_codes/11_memory/examples/alignment.rs}}
```
~~~

#### enum

~~~admonish info title='enum/Option<T>/Result<T,E>内存布局对比' collapsible=true
![enum/Option<T>/Result<T,E>内存布局对比](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4640218.jpg)
~~~

~~~admonish info title='Rust 编译器会对 enum 做一些额外的优化，让某些常用结构的内存布局更紧凑。' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/11_memory/examples/size.rs}}
```
----
> 你会发现，Option 配合带有引用类型的数据结构，比如 &u8、Box、Vec、HashMap ，没有额外占用空间，这就很有意思了
```shell

Type                        T    Option<T>    Result<T, io::Error>
----------------------------------------------------------------
u8                          1        2           24
f64                         8       16           24
&u8                         8        8           24
Box<u8>                     8        8           24
&[u8]                      16       16           24
String                     24       24           32
Vec<u8>                    24       24           32
HashMap<String, String>    48       48           56
E                          56       56           64
```
----
Rust 是这么处理的:
1. 我们知道，引用类型的第一个域是个指针，而指针是不可能等于 0 的，
2. 但是我们可以复用这个指针：当其为 0 时，表示 None，否则是 Some，减少了内存占用，这是个非常巧妙的优化
~~~

#### vec<T>和String

~~~admonish info title='String其实就是Vec<u8>' collapsible=true
String 和 Vec 占用相同的大小，都是 24 个字节。其实，如果你打开 String 结构的[源码](https://doc.rust-lang.org/src/alloc/string.rs.html#279-281)，可以看到，它内部就是一个 Vec
~~~

~~~admonish info title='Vec 结构是 3 个 word 的胖指针' collapsible=true
![vec就是一个胖指针](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4640654.jpg)
----
包含：
1. 一个指向堆内存的指针 pointer
2. 分配的堆内存的容量 capacity
3. 以及数据在堆内存的长度 length
~~~

#### 引用类型的内存布局

~~~admonish info title='引用类型的内存布局' collapsible=true
![引用类型的内存布局](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F.png)
~~~

### 更多可见cheats.rs

- [Rust Language Cheat Sheet](https://cheats.rs/#data-layout)

## 使用

### copy和move

~~~admonish info title='copy和move的内部实现都只是浅层按位做内存复制' collapsible=true
![copy和move的内部实现都只是浅层按位做内存复制](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4640936.jpg)
~~~

## 销毁

### drop释放堆内存

~~~admonish info title='当一个值被释放，其实就是调用它的drop方法' collapsible=true
![当一个值被释放，其实就是调用它的drop方法](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4641072.jpg)
----
1. 变量 greeting 是一个字符串，在退出作用域时，其 drop() 函数被自动调用
2. 释放堆上包含 “hello world” 的内存
3. 然后再释放栈上的内存
~~~

~~~admonish info title='复杂结构递归调用drop' collapsible=true
![复杂结构递归调用drop](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4641173.jpg)
----
> 如果要释放的值是一个复杂的数据结构，比如一个结构体，那么:
1. 这个结构体在调用 drop() 时，会依次调用每一个域的 drop() 函数
2. 如果域又是一个复杂的结构或者集合类型，就会递归下去
3. 直到每一个域都释放干净。
----

- student 变量是一个结构体，有 name、age、scores。
- 其中 name 是 String，scores 是 HashMap，它们本身需要额外 drop()。
- 又因为 HashMap 的 key 是 String，所以还需要进一步调用这些 key 的 drop()。
> 整个释放顺序从内到外是：先释放 HashMap 下的 key，然后释放 HashMap 堆上的表结构，最后释放栈上的内存
~~~

### RAII释放其他资源

~~~admonish info title='Rust基于RAII释放文件资源' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/11_memory/examples/raii.rs}}
```
~~~

### Rust在编译时、运行时检查调用drop

![Rust在编译时、运行时检查调用drop](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4641604.jpg)
