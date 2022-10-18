# 一、所有权: 单一/共享

<!--ts-->
* [一、所有权: 单一/共享](#一所有权-单一共享)
   * [对比单一/共享所有权](#对比单一共享所有权)
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
      * [线程安全版本计数器：Arc(Rc)、Mutex/RwLock(RefCell)](#线程安全版本计数器arcrcmutexrwlockrefcell)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Oct 18 06:45:14 UTC 2022 -->

<!--te-->

## 对比单一/共享所有权

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

### 第一性原理理解单一所有权规则

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

~~~admonish info title='RefCell内部可变性示意图' collapsible=true
![使用RefCell实现可修改版本DAG](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/09%EF%BD%9C%E6%89%80%E6%9C%89%E6%9D%83%EF%BC%9A%E4%B8%80%E4%B8%AA%E5%80%BC%E5%8F%AF%E4%BB%A5%E6%9C%89%E5%A4%9A%E4%B8%AA%E6%89%80%E6%9C%89%E8%80%85%E4%B9%88%EF%BC%9F-4606429.jpg)
~~~

#### RefCell简单使用

~~~admonish info title='获得 RefCell 内部数据的可变借用' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/09_multi_owner/src/refcell.rs}}
```
~~~

~~~admonish info title='使用RefCell实现可修改版本DAG' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/09_multi_owner/src/dag_mut.rs}}
```
---
1. 首先数据结构的 downstream 需要 Rc 内部嵌套一个 RefCell
2. 这样，就可以利用 RefCell 的内部可变性，来获得数据的可变借用
3. 同时 Rc 还允许值有多个所有者。
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
