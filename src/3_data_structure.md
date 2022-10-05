# III. 数据结构

<!--ts-->
* [III. 数据结构](#iii-数据结构)
   * [数据结构快速一览](#数据结构快速一览)
   * [分类图](#分类图)
* [一、智能指针](#一智能指针)
   * [指针还是引用](#指针还是引用)
   * [智能指针不仅是指针](#智能指针不仅是指针)
   * [Box: 在堆上分配内存](#box-在堆上分配内存)
      * [实现内存分配器](#实现内存分配器)
      * [内存如何释放](#内存如何释放)
   * [Cow&lt;'a, B&gt;： 写时拷贝](#cowa-b-写时拷贝)
      * [定义](#定义)
      * [两个trait：ToOwned、Borrowed](#两个traittoownedborrowed)
      * [ToOwned](#toowned)
      * [匹配分发](#匹配分发)
      * [Cow在需要时才进行内存分配拷贝](#cow在需要时才进行内存分配拷贝)
   * [MutexGuard： 数据加锁](#mutexguard-数据加锁)
      * [MutexGuard与String、Box、Cow&lt;'a, B&gt;的对比](#mutexguard与stringboxcowa-b的对比)
      * [使用Mutex::lock获取](#使用mutexlock获取)
      * [定义与Deref、Drop trait实现](#定义与derefdrop-trait实现)
      * [使用Mutex_MutexGuard的例子](#使用mutex_mutexguard的例子)
   * [自定义智能指针](#自定义智能指针)
* [二、集合容器](#二集合容器)
   * [对容器进行定义](#对容器进行定义)
   * [对集合容器进行定义](#对集合容器进行定义)
   * [切片](#切片)
      * [array vs vector](#array-vs-vector)
      * [Vec 和 &amp;[T]](#vec-和-t)
      * [解引用](#解引用)
      * [切片和迭代器 Iterator](#切片和迭代器-iterator)
      * [特殊的切片：&amp;str](#特殊的切片str)
      * [Box&lt;[T]&gt;](#boxt)
      * [常用切片对比图](#常用切片对比图)
   * [哈希表](#哈希表)
      * [哈希表还是列表](#哈希表还是列表)
      * [Rust 的哈希表](#rust-的哈希表)
   * [<a target="_blank" rel="noopener noreferrer nofollow" href="https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882967.jpg"><img src="https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882967.jpg" alt="17｜数据结构：软件系统核心部件哈希表，内存如何布局？" style="max-width: 100%;"></a>](#-1)
      * [HashMap 的数据结构](#hashmap-的数据结构)
      * [HashMap 的基本使用方法](#hashmap-的基本使用方法)
      * [HashMap 的内存布局](#hashmap-的内存布局)
      * [ctrl 表](#ctrl-表)
      * [哈希表重新分配与增长](#哈希表重新分配与增长)
      * [删除一个值](#删除一个值)
      * [让自定义的数据结构做 Hash key](#让自定义的数据结构做-hash-key)
      * [HashSet / BTreeMap / BTreeSet](#hashset--btreemap--btreeset)
      * [为什么 Rust 的 HashMap 要缺省采用加密安全的哈希算法？](#为什么-rust-的-hashmap-要缺省采用加密安全的哈希算法)
* [三、错误处理](#三错误处理)
   * [错误处理包含这么几部分](#错误处理包含这么几部分)
   * [错误处理的主流方法](#错误处理的主流方法)
   * [Rust 的错误处理](#rust-的错误处理)
      * [Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型。](#rust-偷师-haskell构建了对标-maybe-的-option-类型和-对标-either-的-result-类型)
      * [? 操作符](#-操作符)
      * [函数式错误处理](#函数式错误处理)
      * [panic! 和 catch_unwind](#panic-和-catch_unwind)
      * [Error trait 和错误类型的转换](#error-trait-和错误类型的转换)
* [四、闭包结构](#四闭包结构)
   * [闭包的定义](#闭包的定义)
   * [闭包本质上是什么？](#闭包本质上是什么)
   * [不同语言的闭包设计](#不同语言的闭包设计)
   * [Rust 的闭包类型](#rust-的闭包类型)
      * [FnOnce](#fnonce)
      * [怎么理解 FnOnce 的 Args 泛型参数呢？](#怎么理解-fnonce-的-args-泛型参数呢)
      * [FnMut](#fnmut)
      * [Fn](#fn)
      * [总结一下三种闭包使用的情况以及它们之间的关系](#总结一下三种闭包使用的情况以及它们之间的关系)
   * [闭包的使用场景](#闭包的使用场景)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Oct  5 04:36:04 UTC 2022 -->

<!--te-->

## 数据结构快速一览

~~~admonish tip title="数据结构快速一览"
> 用40分钟的时间，总结了Rust的主要数据结构的内 存布局。它能厘清"数据是如何在堆和栈上存储"的思路，在这里也推荐给你。
[Visualizing memory layout of Rust's data types - YouTube](https://www.youtube.com/watch?v=rDoqT-a6UFg)
~~~

## 分类图

> 数据结构可以看作对于类型系统的进一步整理，结构化。这其实是进一步抽象，从类型中提取出日常常用的工具并分类。

~~~admonish info title='从系统/容器/原生三个纬度分类' collapsible=false
![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F.jpg)
~~~

# 一、智能指针

## 指针还是引用

~~~admonish info title='引用是特殊的指针' collapsible=true
1. 指针是一个持有内存地址的值，可以通过解引用来访问它指向的内存地址，理论上可以解引用到任意数据类型；
2. 引用是一个特殊 的指针，它的解引用访问是受限的，只能解引用到它引用数据的类型，不能用作它用。
~~~

## 智能指针不仅是指针

~~~admonish info title='智能指针=指针+额外处理能力' collapsible=true
1. 在指针和引用的基础上，Rust 偷师 C++，提供了智能指针。
2. 智能指针是一个表现行为很 像指针的数据结构，但除了指向数据的指针外，它还有元数据以提供额外的处理能力。
~~~

~~~admonish info title='智能指针=胖指针+所有权' collapsible=true
1. 智能指针一定是一个胖指针，但胖指针不一定是一个 智能指针。
2. 比如 &str 就只是一个胖指针，它有指向堆内存字符串的指针，同时还有关于字 符串长度的元数据。

![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/15%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BF%99%E4%BA%9B%E6%B5%93%E7%9C%89%E5%A4%A7%E7%9C%BC%E7%9A%84%E7%BB%93%E6%9E%84%E7%AB%9F%E7%84%B6%E9%83%BD%E6%98%AF%E6%99%BA%E8%83%BD%E6%8C%87%E9%92%88%EF%BC%9F.jpg)
---
1. String 除了多一个 capacity 字段，似乎也没有什么特殊。
2. 但 String 对 堆上的值有所有权，而 &str 是没有所有权的
3. 这是 Rust 中智能指针和普通胖指针的区 别。
~~~

~~~admonish info title='智能指针和结构体有什么区别' collapsible=true

1. String用结构体定义，其实就是Vec<u8>

```rust

pub struct String {
    vec: Vec<u8>,
}
```

2. 和普通的结构体不同的是，[String 实现了 Deref 和 DerefMut](https://doc.rust-lang.org/src/alloc/string.rs.html#2301-2316)，这使得它在解引用的时
   候，会得到 &str

```rust

impl ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}

impl ops::DerefMut for String {
    fn deref_mut(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked_mut(&mut *self.vec) }
    }
}
```

3. 另外，由于在堆上分配了数据，String 还需要为其分配的资源做相应的回收。而 String 内部使用了
   Vec，所以它可以[依赖 Vec 的能力来释放堆内存](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#2710-2720)

```rust

unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            // use a raw slice to refer to the elements of the vector as weakest necessary type;
            // could avoid questions of validity in certain cases
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
        }
        // RawVec handles deallocation
    }
}
```
~~~

~~~admonish info title='在 Rust 中，凡是需要做资源回收的数据结构，且实现了 Deref/DerefMut/Drop，都是智能指针。' collapsible=true
按照这个定义，除了 String，还有很多智能指针，比如：
1. 用于在堆上 分配内存的 Box<T> 和 Vec<T>
2. 用于引用计数的 Rc<T> 和 Arc<T> 
3. 很多其他数据结 构，如 PathBuf、Cow<'a, B>、MutexGuard<T>、RwLockReadGuard<T> 和 RwLockWriteGuard 等也是智能指针。
~~~

## Box<T>: 在堆上分配内存

~~~admonish info title='从c/c++得到Box<T>灵感' collapsible=true

我们先看 Box<T>，它是 Rust 中最基本的在堆上分配内存的方式，绝大多数其它包含堆内 存分配的数据类型，内部都是通过 Box<T> 完成的，比如 Vec<T>。

为什么有 Box<T> 的设计，我们得先回忆一下在 C 语言中，堆内存是怎么分配的。

1. C 需要使用 malloc/calloc/realloc/free 来处理内存的分配，很多时候，被分配出来的内存 在函数调用中来来回回使用，导致谁应该负责释放这件事情很难确定，给开发者造成了极 大的心智负担。

2. C++ 在此基础上改进了一下，提供了一个智能指针  unique_ptr，可以在指针退出作用 域的时候释放堆内存，这样保证了堆内存的单一所有权。这个 unique_ptr 就是 Rust 的 Box<T> 的前身。

---
[Box<T> 的定义](https://doc.rust-lang.org/src/core/ptr/unique.rs.html#36-44)里，内部就是一个 Unique<T> 用于致敬 C++，Unique<T> 是
一个私有的数据结构，我们不能直接使用，它包裹了一个 *const T 指针，并唯一拥有这个 指针。

```rust
pub struct Unique<T: ?Sized> {
    pointer: *const T,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    _marker: PhantomData<T>,
}
```
~~~

~~~admonish info title='堆上分配内存的 Box<T> 其实有一个缺省的泛型参数 A' collapsible=true

设计内存分配器的目的除了保证正确性之外，就是为了有效地利用剩余内存，并控制内存 在分配和释放过程中产生的碎片的数量。
在多核环境下，它还要能够高效地处理并发请 求。（如果你对通用内存分配器感兴趣，可以看参考资料） 堆上分配内存的 Box<T>
其实有一个缺省的泛型参数 A，就需要满足 [Allocator trait](https://doc.rust-lang.org/std/alloc/trait.Allocator.html)， 并且默认是 Global：

```rust

pub struct Box<T: ?Sized, A: Allocator = Global>(Unique<T>, A);
```

---
Allocator trait 提供很多方法：

1. allocate 是主要方法，用于分配内存，对应 C 的 malloc/calloc；

2. deallocate，用于释放内存，对应 C 的 free；

3. 还有 grow / shrink，用来扩大或缩小堆上已分配的内存，对应 C 的 realloc。
~~~

~~~admonish info title='替换默认的内存分配器' collapsible=true

如果你想替换默认的内存分配器，可以使用 #[global_allocator] 标记宏，定义你自己的全局分配器。下面的代码展示了如何在 Rust
下使用[jemalloc](https://crates.io/crates/jemallocator):

```rust

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {}
```
~~~

### 实现内存分配器

~~~admonish info title='内存分配器' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/allocator.rs}}
```

---

1. 这里 MyAllocator 就用 System allocator，然后加 eprintln!()，和我 们常用的 println!() 不同的是，eprintln!() 将数据打印到 stderr
2. 注意这里不能使用 println!() 。因为 stdout 会打印到一个由 Mutex 互斥锁保护的共享全 局 buffer 中，这个过程中会涉及内存的分配，分配的内存又会触发 println!()，最终造成 程序崩溃。而
   eprintln! 直接打印到 stderr，不会 buffer。
3. 在使用 Box 分配堆内存的时候要注意，Box::new() 是一个函数，所以传入它的数据会出现 在栈上，再移动到堆上。所以，如果我们的 Matrix 结构不是 505 个字节，是一个非常大 的结构，就有可能出问题。

```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/box.rs}}
```

- `cargo run --bin box`或者在 playground 里运行，直接栈溢出 stack overflow
- 本地使用 “cargo run --bin box —release” 编译成 release 代码运行，会正常执行！

> 这是因为 “cargo run” 或者在 playground 下运行，默认是 debug build，它不会做任 何 inline 的优化，而 Box::new() 的实现就一行代码，并注明了要 inline，在 release 模式
> 下，这个函数调用会被优化掉, 本质是编译器自动调用下列方式:

```rust

#[cfg(not(no_global_oom_handling))]
#[inline(always)]
#[doc(alias = "alloc")]
#[doc(alias = "malloc")]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn new(x: T) -> Self {
    box x
}
```

> 这里的关键字 box是 Rust 内部的关键字，用户代码无法调用，它只出现在 Rust 代码中，用于分配堆内存，box 关键字在编译时，会使用内存分配器 分配内存。
~~~

### 内存如何释放

~~~admonish info title='Box<T>默认实现的Drop trait' collapsible=true
```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        // FIXME: Do nothing, drop is currently performed by compiler.
    }
}
```
~~~

~~~admonish info title='先稳定接口，再迭代稳定实现' collapsible=true
目前 drop trait 什么都没有做，编译器会自动插入 deallocate 的代码。这是 Rust 语 言的一种策略：在具体实现还没有稳定下来之前，我先把接口稳定，实现随着之后的迭代 慢慢稳定。
~~~

## Cow<'a, B>： 写时拷贝

~~~admonish info title='写时复制（Copy-on-write）有异曲同工之妙' collapsible=false
Cow 是 Rust 下用于提供写时克隆（Clone-on-Write）的一个智能指针，它跟虚拟内存管 理的写时复制（Copy-on-write）有异曲同工之妙：
> 包裹一个只读借用，但如果调用者需 要所有权或者需要修改内容，那么它会 clone 借用的数据
~~~

### 定义

~~~admonish info title='Cow定义' collapsible=false

```rust

pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```
> 它是一个 enum，可以包含一个对类型 B 的只读引用，或者包含对类型 B 的拥有所有权的 数据。
~~~

### 两个trait：ToOwned、Borrowed

~~~admonish info title='Cow定义用到两个trait：ToOwned和Borrowed' collapsible=false

```rust

pub trait ToOwned {
    type Owned: Borrow<Self>;
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn to_owned(&self) -> Self::Owned;

    fn clone_into(&self, target: &mut Self::Owned) { ... }
}

pub trait Borrow<Borrowed> where Borrowed: ?Sized {
    fn borrow(&self) -> &Borrowed;
}
```
> 它是一个 enum，可以包含一个对类型 B 的只读引用，或者包含对类型 B 的拥有所有权的 数据。
~~~

### ToOwned

~~~admonish info title='type Owned: Borrow<Self>' collapsible=false

1. 首先，type Owned: Borrow<Self> 是一个带有关联类型的 trait. 这里 Owned 是关联类型，需要使用者定义.
2. 这里 Owned 不能是任意类型，它必须满足 Borrow<T> trait
3. [参考str对ToOwned trait的实现](https://doc.rust-lang.org/src/alloc/str.rs.html#215-227)：

```rust

impl ToOwned for str {
    type Owned = String;
    #[inline]
    fn to_owned(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }

    fn clone_into(&self, target: &mut String) {
        let mut b = mem::take(target).into_bytes();
        self.as_bytes().clone_into(&mut b);
        *target = unsafe { String::from_utf8_unchecked(b) }
    }
}
```

4. 可以看到关联类型 Owned 被定义为 String，而根据要求，String 必须定义 Borrow，那这里 Borrow 里的泛型变量 T 是谁呢？
5. ToOwned 要求是 Borrow，而此刻实现 ToOwned 的主体是 str，所以 Borrow 是 Borrow，也就是说 String 要实现 Borrow

```rust

impl Borrow<str> for String {
    #[inline]
    fn borrow(&self) -> &str {
        &self[..]
    }
}
```
~~~

~~~admonish info title='Cow 和 ToOwned / Borrow<T> 之间的关系示意图' collapsible=false
![type Owned: Borrow<Self>](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/15%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BF%99%E4%BA%9B%E6%B5%93%E7%9C%89%E5%A4%A7%E7%9C%BC%E7%9A%84%E7%BB%93%E6%9E%84%E7%AB%9F%E7%84%B6%E9%83%BD%E6%98%AF%E6%99%BA%E8%83%BD%E6%8C%87%E9%92%88%EF%BC%9F-4781304.jpg)
~~~

### 匹配分发

~~~admonish info title='为何 Borrow 要定义成一个泛型 trait 呢？' collapsible=false
1. 例子1：String不同借用方式
```rust, editable

use std::borrow::Borrow;

fn main() {
    let s = "hello world!".to_owned();

    // 这里必须声明类型，因为 String 有多个 Borrow<T> 实现
    // 借用为 &String
    let r1: &String = s.borrow();
    // 借用为 &str
    let r2: &str = s.borrow();

    println!("r1: {:p}, r2: {:p}", r1, r2);
}
```
> String 可以被借用为 &String，也可以被借用为 &str
---
2. 例子2：Cow不同解引用方式

```rust

impl<B: ?Sized + ToOwned> Deref for Cow<'_, B> {
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            Borrowed(borrowed) => borrowed,
            Owned(ref owned) => owned.borrow(),
        }
    }
}

```
---
实现的原理很简单，根据 self 是 Borrowed 还是 Owned，我们分别取其内容，生成引 用：

1. 对于 Borrowed，直接就是引用；

2. 对于 Owned，调用其 borrow() 方法，获得引用。
~~~

~~~admonish info title='匹配分发：使用match匹配实现静态、动态分发之外的第三种分发' collapsible=false
虽然 Cow 是一个 enum，但是通过 Deref 的实现，我们可以获得统一的 体验.
比如 Cow<str>，使用的感觉和 &str / String 是基本一致的。
注意，这种根据 enum 的不同状态来进行统一分发的方法是第三种分发手段，另外还可以使用泛型参数 做静态分发和使用 trait object 做动态分发
~~~

### Cow在需要时才进行内存分配拷贝

~~~admonish info title='写时拷贝' collapsible=false

那么 Cow 有什么用呢？
1. 显然，它可以在需要的时候才进行内存的分配和拷贝，在很多应用 场合，它可以大大提升系统的效率。
2. 如果 Cow<'a, B> 中的 Owned 数据类型是一个需要 在堆上分配内存的类型，如 String、Vec<T> 等，还能减少堆内存分配的次数。 
3. 相对于栈内存的分配释放来说，堆内存的分配和释放效率要低很多，其内部还 涉及系统调用和锁，减少不必要的堆内存分配是提升系统效率的关键手段。
4. 而 Rust 的 Cow<'a, B>，在帮助你达成这个效果的同时，使用体验还非常简单舒服。
~~~

~~~admonish info title='举例使用Cow进行URL解析' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/cow1.rs}}
```
---
> 在解析 URL 的时候，我们经常需要将 querystring 中的参数，提取成 KV pair 来进一步使 用。
> 绝大多数语言中，提取出来的 KV 都是新的字符串，在每秒钟处理几十 k 甚至上百 k 请求的系统中，你可以想象这会带来多少次堆内存的分配。 
> 但在 Rust 中，我们可以用 Cow 类型轻松高效处理它，在读取 URL 的过程中：
1. 每解析出一个 key 或者 value，我们可以用一个 &str 指向 URL 中相应的位置，然后用 Cow 封装它 
2. 而当解析出来的内容不能直接使用，需要 decode 时，比如 “hello%20world”，我们 可以生成一个解析后的 String，同样用 Cow 封装它。
~~~

~~~admonish info title='举例serde使用Cow进行序列化/反序列化' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/cow2.rs}}
```
~~~

## MutexGuard<T>： 数据加锁

### MutexGuard与String、Box<T>、Cow<'a, B>的对比

~~~admonish info title='Deref+Drop' collapsible=true
String、Box<T>、Cow<'a, B> 等智能指针，都是通过 Deref 来提 供良好的用户体验， 
MutexGuard<T> 是另外一类很有意思的智能指针：
1. 它不但通过 Deref 提供良好的用户体验
2. 还通过 Drop trait 来确保，使用到的内存以外的资源在退出 时进行释放。
~~~

### 使用Mutex::lock获取

~~~admonish info title='MutexGuard这个结构是在调用 Mutex::lock 时生成的' collapsible=true

```rust, editable

pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
    unsafe {
        self.inner.raw_lock();
        MutexGuard::new(self)
    }
}
```
---
[rust文档](https://doc.rust-lang.org/src/std/sync/mutex.rs.html#279-284)
1. 首先，它会取得锁资源，如果拿不到，会在这里等待；
2. 如果拿到了，会把 Mutex 结构的引 用传递给 MutexGuard。
~~~

### 定义与Deref、Drop trait实现

~~~admonish info title='MutexGuard 的定义以及它的 Deref 和 Drop 的实现' collapsible=true
```rust, editable

// 这里用 must_use，当你得到了却不使用 MutexGuard 时会报警
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
    poison: poison::Guard,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.lock.poison.done(&self.poison);
            self.lock.inner.raw_unlock();
        }
    }
}
```
---
从代码中可以看到:
1. 当 MutexGuard 结束时，Mutex 会做 unlock
2. 这样用户在使用 Mutex 时，可以不必关心何时释放这个互斥锁。
3. 因为无论你在调用栈上怎样传递 MutexGuard ，哪怕在错误处理流程上提前退出，Rust 有所有权机制，可以确保只要 MutexGuard 离开作用域，锁就会被释放
~~~

### 使用Mutex_MutexGuard的例子

~~~admonish info title='Mutex & MutexGuard example' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/guard.rs}}
```
---
> 在解析 URL 的时候，我们经常需要将 querystring 中的参数，提取成 KV pair 来进一步使 用。
> 绝大多数语言中，提取出来的 KV 都是新的字符串，在每秒钟处理几十 k 甚至上百 k 请求的系统中，你可以想象这会带来多少次堆内存的分配。 
> 但在 Rust 中，我们可以用 Cow 类型轻松高效处理它，在读取 URL 的过程中：
1. 每解析出一个 key 或者 value，我们可以用一个 &str 指向 URL 中相应的位置，然后用 Cow 封装它 
2. 而当解析出来的内容不能直接使用，需要 decode 时，比如 “hello%20world”，我们 可以生成一个解析后的 String，同样用 Cow 封装它。
~~~

~~~admonish info title='你可以把 MutexGuard 的引用传给另一个线程使用，但你无法把 MutexGuard 整个移动到另一个线程' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/guard1.rs}}
```
~~~

~~~admonish info title='MutexGuard 的智能指针有很多用途' collapsible=true
- r2d2类似实现一个数据库连接池：[源码](https://github.com/sfackler/r2d2/blob/master/src/lib.rs#L611-L638)
```rust
impl<M> Drop for PooledConnection<M>
where
    M: ManageConnection,
{
    fn drop(&mut self) {
        self.pool.put_back(self.checkout, self.conn.take().unwrap());
    }
}

impl<M> Deref for PooledConnection<M>
where
    M: ManageConnection,
{
    type Target = M::Connection;

    fn deref(&self) -> &M::Connection {
        &self.conn.as_ref().unwrap().conn
    }
}

impl<M> DerefMut for PooledConnection<M>
where
    M: ManageConnection,
{
    fn deref_mut(&mut self) -> &mut M::Connection {
        &mut self.conn.as_mut().unwrap().conn
    }
}
```
- 类似 MutexGuard 的智能指针有很多用途。比如要创建一个连接池，你可以在 Drop trait 中，回收 checkout 出来的连接，将其再放回连接池。
~~~

## 自定义智能指针

~~~admonish info title='MyString结构示意图' collapsible=true
![MyString](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/15%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BF%99%E4%BA%9B%E6%B5%93%E7%9C%89%E5%A4%A7%E7%9C%BC%E7%9A%84%E7%BB%93%E6%9E%84%E7%AB%9F%E7%84%B6%E9%83%BD%E6%98%AF%E6%99%BA%E8%83%BD%E6%8C%87%E9%92%88%EF%BC%9F-4783668.jpg)
~~~

~~~admonish info title='MyString实现代码' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/mystring.rs}}
```
---

为了让 MyString 表现行为和 &str 一致:

1. 我们可以通过实现 Deref trait 让 MyString 可以被解引用成 &str。
2. 除此之外，还可以实现 Debug/Display 和 From<T> trait，让 MyString 使用起来更方便。
3. 这个简单实现的 MyString，不管它内部的数据是纯栈上的 MiniString 版本，还是包含堆 上内存的 String 版本，使用的体验和 &str 都一致，仅仅牺牲了一点点效率和内存，就可
   以让小容量的字符串，可以高效地存储在栈上并且自如地使用。
4. [smartstring](https://github.com/bodil/smartstring) 的第三方库实现类似功能，还做了优化。
~~~

# 二、集合容器

## 对容器进行定义

~~~admonish tip title="容器数据结构如何理解" collapsible=true
提到容器，很可能你首先会想到的就是数组、列表这些可以遍历的容器，但其实只要把某 种特定的数据封装在某个数据结构中，这个数据结构就是一个容器。比如 Option<T>，它 是一个包裹了 T 存在或不存在的容器，而 Cow 是一个封装了内部数据 B 或被借用或拥有 所有权的容器。
~~~

## 对集合容器进行定义

~~~admonish tip title="把拥有相同类型对数据放在一起，统一处理" collapsible=true
集合容器，顾名思义，就是把一系列拥有相同类型的数据放在一起，统一处理，比如：

1. 我们熟悉的字符串 String、数组 [T; n]、列表 Vec<T> 和哈希表 HashMap<K, V> 等；

2. 虽然到处在使用，但还并不熟悉的切片 slice；

3. 在其他语言中使用过，但在 Rust 中还没有用过的循环缓冲区 VecDeque<T>、双向列 表 LinkedList<T> 等。

> 这些集合容器有很多共性，比如可以被遍历、可以进行 map-reduce 操作、可以从一种类 型转换成另一种类型等等。
~~~

## 切片

~~~admonish tip title="切片到底是什么" collapsible=true

在 Rust 里，切片是描述一组属于同一类型、长度不确定的、在内存中连续存放的数据结 构，用 [T] 来表述。因为长度不确定，所以切片是个 DST（Dynamically Sized Type）。

切片一般只出现在数据结构的定义中，不能直接访问，在使用中主要用以下形式：

- &[T]：表示一个只读的切片引用。

- &mut [T]：表示一个可写的切片引用。

- Box<[T]>：一个在堆上分配的切片。

~~~

~~~admonish tip title="切片与数据的关系" collapsible=true
怎么理解切片呢？我打个比方，切片之于具体的数据结构，就像数据库中的视图之于表。 你可以把它看成一种工具，让我们可以统一访问行为相同、结构类似但有些许差异的类 型。
---

```rust, editable
{{#include ../geektime_rust_codes/16_data_structure/src/slice1.rs}}
```
1. 对于 array 和 vector，虽然是不同的数据结构，一个放在栈上，一个放在堆上，但它们的 切片是类似的；
2. 而且对于相同内容数据的相同切片，比如 &arr[1…3] 和 &vec[1…3]，这 两者是等价的。
3. 除此之外，切片和对应的数据结构也可以直接比较，这是因为它们之间实 现了 PartialEq trait
![切片与具体数据的关系](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4785008.jpg)
~~~

### array vs vector

~~~admonish info title="array和vector的区别与联系" collapsible=true
对于 array 和 vector，虽然是不同的数据结构：
- 一个放在栈上
- 一个放在堆上

> 但它们的切片是类似的, 而且对于相同内容数据的相同切片
- 比如 &arr[1…3] 和 &vec[1…3]，这两者是等价的。
- 除此之外，切片和对应的数据结构也可以直接比较，这是因为它们之间实现了 PartialEq trait（源码参考资料）。
> 下图比较清晰地呈现了切片和数据之间的关系：
![切片和数据之间的关系](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4866674.jpg)
~~~

### Vec<T> 和 &[T]

~~~admonish tip title="&[T]与&Vec[T]关系" collapsible=true
![&[T]和&Vec[T]](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4785147.jpg)
~~~

### 解引用

~~~admonish info title="支持切片的具体数据类型可以根据需要解引用转换成切片类型" collapsible=true
在使用的时候，支持切片的具体数据类型，你可以根据需要，解引用转换成切片类型。
- 比如 Vec<T> 和 [T; n] 会转化成为 &[T]，这是因为 Vec<T> 实现了 Deref trait，而 array 内建了到 &[T] 的解引用。
- 我们可以写一段代码验证这一行为（代码）：
---
```rust, editable

use std::fmt;
fn main() {
    let v = vec![1, 2, 3, 4];

    // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]，符合接口定义
    print_slice(&v);
    // 直接是 &[T]，符合接口定义
    print_slice(&v[..]);

    // &Vec<T> 支持 AsRef<[T]>
    print_slice1(&v);
    // &[T] 支持 AsRef<[T]>
    print_slice1(&v[..]);
    // Vec<T> 也支持 AsRef<[T]>
    print_slice1(v);

    let arr = [1, 2, 3, 4];
    // 数组虽没有实现 Deref，但它的解引用就是 &[T]
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}

// 注意下面的泛型函数的使用
fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
```
---
> 这也就意味着，通过解引用，这几个和切片有关的数据结构都会获得切片的所有能力，包括：binary_search、chunks、concat、contains、start_with、end_with、group_by、iter、join、sort、split、swap 等一系列丰富的功能，
~~~

### 切片和迭代器 Iterator

~~~admonish info title="迭代器可以说是切片的孪生兄弟" collapsible=true
迭代器可以说是切片的孪生兄弟。切片是集合数据的视图，而迭代器定义了对集合数据的各种各样的访问操作。

Iterator trait 有大量的方法，但绝大多数情况下，只需要定义它的关联类型 Item 和 next() 方法。
- Item 定义了每次我们从迭代器中取出的数据类型；
- next() 是从迭代器里取下一个值的方法。当一个迭代器的 next() 方法返回 None 时，表明迭代器中没有数据了。
```rust

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 大量缺省的方法，包括 size_hint, count, chain, zip, map, 
    // filter, for_each, skip, take_while, flat_map, flatten
    // collect, partition 等
    ... 
}
```
~~~

~~~admonish info title="对 Vec<T> 使用 iter() 方法，并进行各种 map / filter / take 操作" collapsible=true
一个例子：对 Vec<T> 使用 iter() 方法，并进行各种 map / filter / take 操作。在函数式编程语言中，这样的写法很常见，代码的可读性很强。Rust 也支持这种写法（代码）：
```rust

fn main() {
    // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", result);
}
```
~~~

~~~admonish info title="Rust的迭代器是个懒接口，这是如何实现的？" collapsible=true
需要注意的是 Rust 下的迭代器是个懒接口（lazy interface），也就是说这段代码直到运行到 collect 时才真正开始执行，之前的部分不过是在不断地生成新的结构，来累积处理逻辑而已。你可能好奇，这是怎么做到的呢？

原来，Iterator 大部分方法都返回一个实现了 Iterator 的数据结构，所以可以这样一路链式下去，在 Rust 标准库中，这些数据结构被称为 [Iterator Adapter](https://doc.rust-lang.org/src/core/iter/adapters/mod.rs.html)。比如上面的 map 方法，它返回 Map 结构，而 Map 结构实现了 [Iterator（源码）](https://doc.rust-lang.org/src/core/iter/adapters/map.rs.html#93-133)。
整个过程是这样的（链接均为源码资料）：


1. 在 collect() 执行的时候，它[实际试图使用 FromIterator 从迭代器中构建一个集合类型](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#1744-1749)，这会不断调用 next() 获取下一个数据；
2. 此时的 Iterator 是 Take，Take 调自己的 next()，也就是它会[调用 Filter 的 next()](https://doc.rust-lang.org/src/core/iter/adapters/take.rs.html#34-41)；
3. Filter 的 next() 实际上[调用自己内部的 iter 的 find()](https://time.geekbang.org/column/article/422975)，此时内部的 iter 是 Map，find() 会使用 [try_fold()](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2312-2325)，它会[继续调用 next()](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2382-2406)，也就是 Map 的 next()；
4. Map 的 next() 会[调用其内部的 iter 取 next() 然后执行 map 函数](https://time.geekbang.org/column/article/422975)。而此时内部的 iter 来自 Vec<i32>。

所以，只有在 collect() 时，才触发代码一层层调用下去，并且调用会根据需要随时结束。这段代码中我们使用了 take(1)，整个调用链循环一次，就能满足 take(1) 以及所有中间过程的要求，所以它只会循环一次。
~~~

~~~admonish info title="Rust的函数式编程写法性能如何？" collapsible=true
你可能会有疑惑：这种函数式编程的写法，代码是漂亮了，然而这么多无谓的函数调用，性能肯定很差吧？毕竟，函数式编程语言的一大恶名就是性能差。

这个你完全不用担心， Rust 大量使用了 inline 等优化技巧，这样非常清晰友好的表达方式，性能和 C 语言的 for 循环差别不大。
~~~

~~~admonish info title="Rust的iterator除了标准库，还有itertools提供更多功能" collapsible=true
如果标准库中的功能还不能满足你的需求，你可以看看 itertools，它是和 Python 下 itertools 同名且功能类似的工具，提供了大量额外的 adapter。可以看一个简单的例子（代码）：
```rust, editable

use itertools::Itertools;

fn main() {
    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];
    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });
    // 结果应该是：vec![Ok(42), Err(err_str)]
    println!("{:?}", it.collect::<Vec<_>>());
}
```

在实际开发中，我们可能从一组 Future 中汇聚出一组结果，里面有成功执行的结果，也有失败的错误信息。如果想对成功的结果进一步做 filter/map，那么标准库就无法帮忙了，就需要用 itertools 里的 filter_map_ok()。
~~~

### 特殊的切片：&str

~~~admonish info title="String、&String和&str的区别与联系" collapsible=true
我们来看一种特殊的切片：&str。之前讲过，String 是一个特殊的 Vec<u8>，所以在 String 上做切片，也是一个特殊的结构 &str。

对于 String、&String、&str，很多人也经常分不清它们的区别，我们在之前的一篇加餐中简单聊了这个问题，在上一讲智能指针中，也对比过 String 和 &str。对于 &String 和 &str，如果你理解了上文中 &Vec<T> 和 &[T] 的区别，那么它们也是一样的：
![&String和&str](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867212.jpg)
~~~

~~~admonish info title="String在解引用时会转换成&str" collapsible=true
String 在解引用时，会转换成 &str。可以用下面的代码验证（代码）：
```rust, editable

use std::fmt;
fn main() {
    let s = String::from("hello");
    // &String 会被解引用成 &str
    print_slice(&s);
    // &s[..] 和 s.as_str() 一样，都会得到 &str
    print_slice(&s[..]);

    // String 支持 AsRef<str>
    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    // String 也实现了 AsRef<[u8]>，所以下面的代码成立
    // 打印出来是 [104, 101, 108, 108, 111]
    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
```
~~~

~~~admonish info title="字符的列表和字符串有什么关系和区别" collapsible=true
那么字符的列表和字符串有什么关系和区别？我们直接写一段代码来看看：

```rust, editable

use std::iter::FromIterator;

fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
    assert_eq!(String::from_iter(s2), s3);
}
```
---
> 可以看到，字符列表可以通过迭代器转换成 String，String 也可以通过 chars() 函数转换成字符列表，如果不转换，二者不能比较。
---
下图把数组、列表、字符串以及它们的切片放在一起比较，可以更好地理解它们的区别：
![数组、列表、字符串和各自的切片](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867332.jpg)
~~~

### Box<[T]>

~~~admonish info title="Box<[T]>和Vec<T>\&[T]对比" collapsible=true


切片主要有三种使用方式：
- 切片的只读引用 &[T]
- 切片的可变引用 &mut [T]: 和&[T]类似
- Box<[T]>

现在我们来看看 Box<[T]>。

Box<[T]> 是一个比较有意思的存在，它和 Vec<T> 有一点点差别：
- Vec<T> 有额外的 capacity，可以增长；
- 而 Box<[T]> 一旦生成就固定下来，没有 capacity，也无法增长。

Box<[T]> 和切片的引用 &[T] 也很类似：
1. 它们都是在栈上有一个包含长度的胖指针，指向存储数据的内存位置。
2. 区别是：Box<[T]> 只会指向堆，&[T] 指向的位置可以是栈也可以是堆；
3. 此外，Box<[T]> 对数据具有所有权，而 &[T] 只是一个借用。
---
![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867436.jpg)
~~~

~~~admonish info title="那么如何产生 Box<[T]> 呢？" collapsible=true
那么如何产生 Box<[T]> 呢？
目前可用的接口就只有一个：从已有的 Vec<T> 中转换。我们看代码：
```rust, editable

use std::ops::Deref;

fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8: {}", v1.capacity());

    // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());

    assert!(b2.deref() == v2);

    // Box<[T]> 可以更改其内部数据，但无法 push
    b2[0] = 2;
    // b2.push(6);
    println!("b2: {:?}", b2);

    // 注意 Box<[T]> 和 Box<[T; n]> 并不相同
    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3: {:?}", b3);

    // b2 和 b3 相等，但 b3.deref() 和 v2 无法比较
    assert!(b2 == b3);
    // assert!(b3.deref() == v2);
}
```
---
运行代码可以看到:
1. Vec<T> 可以通过 into_boxed_slice() 转换成 Box<[T]>
2. Box<[T]> 也可以通过 into_vec() 转换回 Vec<T>。

这两个转换都是很轻量的转换，只是变换一下结构，不涉及数据的拷贝。

区别是:
1. 当 Vec<T> 转换成 Box<[T]> 时，没有使用到的容量就会被丢弃，所以整体占用的内存可能会降低。
2. 而且 Box<[T]> 有一个很好的特性是，不像 Box<[T;n]> 那样在编译时就要确定大小，它可以在运行期生成，以后大小不会再改变。

所以，当我们需要在堆上创建固定大小的集合数据，且不希望自动增长，那么，可以先创建 Vec<T>，再转换成 Box<[T]>。
> tokio 在提供 broadcast channel 时，就使用了 Box<[T]> 这个特性，你感兴趣的话，可以自己看看[源码](https://github.com/tokio-rs/tokio/blob/master/tokio/src/sync/broadcast.rs#L447)。
~~~

### 常用切片对比图

~~~admonish info title="&str、[T;n]、Vec<T>、&[T]、&mut[T]的区别与联系图 " collapsible=false
下图描述了切片和数组 [T;n]、列表 Vec<T>、切片引用 &[T] /&mut [T]，以及在堆上分配的切片 Box<[T]> 之间的关系。
> 建议花些时间理解这张图，也可以用相同的方式去总结学到的其他有关联的数据结构。
---
![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867546.jpg)
~~~

## 哈希表

### 哈希表还是列表

~~~admonish info title="哈希表和列表的选择" collapsible=true
我们知道，哈希表和列表类似，都用于处理需要随机访问的数据结构。如果数据结构的输入和输出能一一对应，那么可以使用列表，如果无法一一对应，那么就需要使用哈希表。
---
![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882989.jpg)
~~~

### Rust 的哈希表

~~~admonish info title="哈希表的核心特点与解决" collapsible=true
哈希表最核心的特点就是：巨量的可能输入和有限的哈希表容量。这就会引发哈希冲突，也就是两个或者多个输入的哈希被映射到了同一个位置，所以我们要能够处理哈希冲突。

要解决冲突，首先可以通过更好的、分布更均匀的哈希函数，以及使用更大的哈希表来缓解冲突，但无法完全解决，所以我们还需要使用冲突解决机制。
~~~

如何解决冲突？

理论上，主要的冲突解决机制有链地址法（chaining）和开放寻址法（open addressing）。

- 链地址法，我们比较熟悉，就是把落在同一个哈希上的数据用单链表或者双链表连接起来。这样在查找的时候，先找到对应的哈希桶（hash bucket），然后再在冲突链上挨个比较，直到找到匹配的项。

> 冲突链处理哈希冲突非常直观，很容易理解和撰写代码，但缺点是哈希表和冲突链使用了不同的内存，对缓存不友好。
---
![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882976.jpg)
---

- 开放寻址法把整个哈希表看做一个大数组，不引入额外的内存，当冲突产生时，按照一定的规则把数据插入到其它空闲的位置。比如线性探寻（linear probing）在出现哈希冲突时，不断往后探寻，直到找到空闲的位置插入。

- 而二次探查，理论上是在冲突发生时，不断探寻哈希位置加减 n 的二次方，找到空闲的位置插入，我们看图，更容易理解：

---
![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882967.jpg)
---
> 图中示意是理论上的处理方法，实际为了性能会有很多不同的处理。

### HashMap 的数据结构

~~~admonish info title="深入Rust哈希表的数据结构：HashMap->hashbrown->RawTable" collapsible=true
我们来看看 Rust 哈希表的数据结构是什么样子的，打开标准库的 [源代码](https://doc.rust-lang.org/src/std/collections/hash/map.rs.html#206-208)：
```rust, editable

use hashbrown::hash_map as base;

#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}
```
---
可以看到，HashMap 有三个泛型参数:
1. K 和 V 代表 key / value 的类型
2. S 是哈希算法的状态，它默认是 RandomState，占两个 u64。
> RandomState 使用 SipHash 作为缺省的哈希算法，它是一个加密安全的哈希函数（cryptographically secure hashing）。

从定义中还能看到，Rust 的 HashMap 复用了 hashbrown 的 HashMap: 
hashbrown 是 Rust 下对 [Google Swiss Table](https://abseil.io/blog/20180927-swisstables) 的一个改进版实现，我们[打开 hashbrown 的代码](https://docs.rs/hashbrown/0.11.2/src/hashbrown/map.rs.html#192-195)，看它的结构：
```rust, editable
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator + Clone = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
```
---
可以看到，HashMap 里有两个域:
1. 一个是 hash_builder，类型是刚才我们提到的标准库使用的 RandomState
2. 还有一个是具体的 RawTable：

```rust, editable

pub struct RawTable<T, A: Allocator + Clone = Global> {
    table: RawTableInner<A>,
    // Tell dropck that we own instances of T.
    marker: PhantomData<T>,
}

struct RawTableInner<A> {
    // Mask to get an index from a hash value. The value is one less than the
    // number of buckets in the table.
    bucket_mask: usize,

    // [Padding], T1, T2, ..., Tlast, C1, C2, ...
    //                                ^ points here
    ctrl: NonNull<u8>,

    // Number of elements that can be inserted before we need to grow the table
    growth_left: usize,

    // Number of elements in the table, only really used by len()
    items: usize,

    alloc: A,
}
```

RawTable 中，实际上有意义的数据结构是 RawTableInner:

> 前四个字段很重要：

1. usize 的 bucket_mask，是哈希表中哈希桶的数量减一；
2. 名字叫 ctrl 的指针，它指向哈希表堆内存末端的 ctrl 区；
3. usize 的字段 growth_left，指哈希表在下次自动增长前还能存储多少数据；
4. Usize 的 items，表明哈希表现在有多少数据。

> 这里最后的 alloc 字段，和 RawTable 的 marker 一样，只是一个用来占位的类型，它用来分配在堆上的内存。
~~~

### HashMap 的基本使用方法

~~~admonish info title="HashMap基本使用方法" collapsible=true
数据结构搞清楚，我们再看具体使用方法。Rust 哈希表的使用很简单，它提供了一系列很方便的方法，使用起来和其它语言非常类似，你只要看看文档，就很容易理解。
> 我们来写段代码，尝试一下（代码）：
```rust, editable

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    // get 时需要使用引用，并且也返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    // 删除后就找不到了
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);
    // shrink 后哈希表变小
    map.shrink_to_fit();
    explain("shrinked", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
}
```
---
1. 可以看到，当 HashMap::new() 时，它并没有分配空间，容量为零
2. 随着哈希表不断插入数据，它会以 2 的幂减一的方式增长，最小是 3。
3. 当删除表中的数据时，原有的表大小不变，只有显式地调用 shrink_to_fit，才会让哈希表变小。
~~~

### HashMap 的内存布局

~~~admonish info title="ctrl表的变化：借助std::mem::transmute查看内存布局" collapsible=true
通过 HashMap 的公开接口无法看到 HashMap 在内存中是如何布局，还是需要借助 std::mem::transmute 方法，来把数据结构打出来。
> 我们把刚才的代码改一改（代码）：
```rust, editable

use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("added 1", map);
    map.insert('b', 2);
    map.insert('c', 3);

    let mut map = explain("added 3", map);

    map.insert('d', 4);

    let mut map = explain("added 4", map);

    map.remove(&'a');

    explain("final", map);
}

// HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
// 分别是 bucket_mask, ctrl, growth_left 和 items
// 我们 transmute 打印之后，再 transmute 回去
fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        name, arr[2], arr[3], arr[4], arr[5]
    );
    unsafe { std::mem::transmute(arr) }
}
```
---
运行之后，可以看到：

```shell
empty: bucket_mask 0x0, ctrl 0x1056df820, growth_left: 0, items: 0

added 1: bucket_mask 0x3, ctrl 0x7fa0d1405e30, growth_left: 2, items: 1

added 3: bucket_mask 0x3, ctrl 0x7fa0d1405e30, growth_left: 0, items: 3

added 4: bucket_mask 0x7, ctrl 0x7fa0d1405e90, growth_left: 3, items: 4

final: bucket_mask 0x7, ctrl 0x7fa0d1405e90, growth_left: 4, items: 3
```

> 发现在运行的过程中，ctrl 对应的堆地址发生了改变。

- 在我的 OS X 下，一开始哈希表为空，ctrl 地址看上去是一个 TEXT/RODATA 段的地址，应该是指向了一个默认的空表地址；
- 插入第一个数据后，哈希表分配了 4 个 bucket，ctrl 地址发生改变；
- 在插入三个数据后，growth_left 为零
- 再插入时，哈希表重新分配，ctrl 地址继续改变。

> 在探索 HashMap 数据结构时，说过 ctrl 是一个指向哈希表堆地址末端 ctrl 区的地址，所以我们可以通过这个地址，计算出哈希表堆地址的起始地址。

因为哈希表有 8 个 bucket（0x7 + 1），每个 bucket 大小是 key（char） + value（i32） 的大小，也就是 8 个字节，所以一共是 64 个字节。
对于这个例子，通过 ctrl 地址减去 64，就可以得到哈希表的堆内存起始地址。然后，我们可以用 rust-gdb / rust-lldb 来打印这个内存。

> 可以用 Linux 下的 rust-gdb 设置断点，依次查看哈希表有一个、三个、四个值，以及删除一个值的状态：
```shell

❯ rust-gdb ~/.target/debug/hashmap2
GNU gdb (Ubuntu 9.2-0ubuntu2) 9.2
...
(gdb) b hashmap2.rs:32
Breakpoint 1 at 0xa43e: file src/hashmap2.rs, line 32.
(gdb) r
Starting program: /home/tchen/.target/debug/hashmap2
...
# 最初的状态，哈希表为空
empty: bucket_mask 0x0, ctrl 0x555555597be0, growth_left: 0, items: 0

Breakpoint 1, hashmap2::explain (name=..., map=...) at src/hashmap2.rs:32
32      unsafe { std::mem::transmute(arr) }
(gdb) c
Continuing.
# 插入了一个元素后，bucket 有 4 个（0x3+1），堆地址起始位置 0x5555555a7af0 - 4*8(0x20)
added 1: bucket_mask 0x3, ctrl 0x5555555a7af0, growth_left: 2, items: 1

Breakpoint 1, hashmap2::explain (name=..., map=...) at src/hashmap2.rs:32
32      unsafe { std::mem::transmute(arr) }
(gdb) x /12x 0x5555555a7ad0
0x5555555a7ad0:  0x00000061  0x00000001  0x00000000  0x00000000
0x5555555a7ae0:  0x00000000  0x00000000  0x00000000  0x00000000
0x5555555a7af0:  0x0affffff  0xffffffff  0xffffffff  0xffffffff
(gdb) c
Continuing.
# 插入了三个元素后，哈希表没有剩余空间，堆地址起始位置不变 0x5555555a7af0 - 4*8(0x20)
added 3: bucket_mask 0x3, ctrl 0x5555555a7af0, growth_left: 0, items: 3

Breakpoint 1, hashmap2::explain (name=..., map=...) at src/hashmap2.rs:32
32      unsafe { std::mem::transmute(arr) }
(gdb) x /12x 0x5555555a7ad0
0x5555555a7ad0:  0x00000061  0x00000001  0x00000062  0x00000002
0x5555555a7ae0:  0x00000000  0x00000000  0x00000063  0x00000003
0x5555555a7af0:  0x0a72ff02  0xffffffff  0xffffffff  0xffffffff
(gdb) c
Continuing.
# 插入第四个元素后，哈希表扩容，堆地址起始位置变为 0x5555555a7b50 - 8*8(0x40)
added 4: bucket_mask 0x7, ctrl 0x5555555a7b50, growth_left: 3, items: 4

Breakpoint 1, hashmap2::explain (name=..., map=...) at src/hashmap2.rs:32
32      unsafe { std::mem::transmute(arr) }
(gdb) x /20x 0x5555555a7b10
0x5555555a7b10:  0x00000061  0x00000001  0x00000000  0x00000000
0x5555555a7b20:  0x00000064  0x00000004  0x00000063  0x00000003
0x5555555a7b30:  0x00000000  0x00000000  0x00000062  0x00000002
0x5555555a7b40:  0x00000000  0x00000000  0x00000000  0x00000000
0x5555555a7b50:  0xff72ffff  0x0aff6502  0xffffffff  0xffffffff
(gdb) c
Continuing.
# 删除 a 后，剩余 4 个位置。注意 ctrl bit 的变化，以及 0x61 0x1 并没有被清除
final: bucket_mask 0x7, ctrl 0x5555555a7b50, growth_left: 4, items: 3

Breakpoint 1, hashmap2::explain (name=..., map=...) at src/hashmap2.rs:32
32      unsafe { std::mem::transmute(arr) }
(gdb) x /20x 0x5555555a7b10
0x5555555a7b10:  0x00000061  0x00000001  0x00000000  0x00000000
0x5555555a7b20:  0x00000064  0x00000004  0x00000063  0x00000003
0x5555555a7b30:  0x00000000  0x00000000  0x00000062  0x00000002
0x5555555a7b40:  0x00000000  0x00000000  0x00000000  0x00000000
0x5555555a7b50:  0xff72ffff  0xffff6502  0xffffffff  0xffffffff
```
---
这段输出蕴藏了很多信息，结合示意图来仔细梳理。

1. 首先，插入第一个元素 ‘a’: 1 后，哈希表的内存布局如下：

![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882958.jpg)

- key ‘a’ 的 hash 和 bucket_mask 0x3 运算后得到第 0 个位置插入。
- 同时，这个 hash 的头 7 位取出来，在 ctrl 表中对应的位置，也就是第 0 个字节，把这个值写入。

要理解这个步骤，关键就是要搞清楚这个 ctrl 表是什么。
~~~

### ctrl 表

~~~admonish info title="ctrl 表的主要目的与设计" collapsible=true
ctrl 表的主要目的是快速查找。它的设计非常优雅，值得我们学习。

一张 ctrl 表里:
- 有若干个 128bit 或者说 16 个字节的分组（group）
- group 里的每个字节叫 ctrl byte，对应一个 bucket，那么一个 group 对应 16 个 bucket。
- 如果一个 bucket 对应的 ctrl byte 首位不为 1，就表示这个 ctrl byte 被使用；
- 如果所有位都是 1，或者说这个字节是 0xff，那么它是空闲的。

> 一组 control byte 的整个 128 bit 的数据，可以通过一条指令被加载进来，然后和某个值进行 mask，找到它所在的位置。这就是HashMap的 SIMD 查表。

> 我们知道，现代 CPU 都支持单指令多数据集的操作，而 Rust 充分利用了 CPU 这种能力，一条指令可以让多个相关的数据载入到缓存中处理，大大加快查表的速度。所以，Rust 的哈希表查询的效率非常高。

具体怎么操作，我们来看 HashMap 是如何通过 ctrl 表来进行数据查询的。

> 假设这张表里已经添加了一些数据，我们现在要查找 key 为 ‘c’ 的数据：

![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882948.jpg)

1. 把 h 跟 bucket_mask 做与，得到一个值，图中是 139；
2. 拿着这个 139，找到对应的 ctrl group 的起始位置，因为 ctrl group 以 16 为一组，所以这里找到 128；
3. 用 SIMD 指令加载从 128 对应地址开始的 16 个字节；
4. 对 hash 取头 7 个 bit，然后和刚刚取出的 16 个字节一起做与，找到对应的匹配，如果找到了，它（们）很大概率是要找的值；
5. 如果不是，那么以二次探查（以 16 的倍数不断累积）的方式往后查找，直到找到为止。


> 所以，当 HashMap 插入和删除数据，以及因此导致重新分配的时候，主要工作就是在维护这张 ctrl 表和数据的对应。

> 因为 ctrl 表是所有操作最先触及的内存，所以，在 HashMap 的结构中，堆内存的指针直接指向 ctrl 表，而不是指向堆内存的起始位置，这样可以减少一次内存的访问。

~~~

### 哈希表重新分配与增长

~~~admonish info title="插入三个元素后没有剩余空间的哈希表，在加入 ‘d’: 4 时，hash map是如何增长的" collapsible=true
在插入第一条数据后，哈希表只有 4 个 bucket，所以只有头 4 个字节的 ctrl 表有用。随着哈希表的增长，bucket 不够，就会导致重新分配。由于 bucket_mask 永远比 bucket 数量少 1，所以插入三个元素后就会重新分配。

根据 rust-gdb 中得到的信息，我们看插入三个元素后没有剩余空间的哈希表，在加入 ‘d’: 4 时，是如何增长的: 

1. 首先，哈希表会按幂扩容，从 4 个 bucket 扩展到 8 个 bucket。

这会导致分配新的堆内存，然后原来的 ctrl table 和对应的 kv 数据会被移动到新的内存中。
- 这个例子里因为 char 和 i32 实现了 Copy trait，所以是拷贝；
- 如果 key 的类型是 String，那么只有 String 的 24 个字节 (ptr|cap|len) 的结构被移动，String 的实际内存不需要变动。
- 在移动的过程中，会涉及哈希的重分配。
- 从下图可以看到，‘a’ / ‘c’ 的相对位置和它们的 ctrl byte 没有变化，但重新做 hash 后，‘b’ 的 ctrl byte 和位置都发生了变化：

![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882903-4882936.jpg)

~~~

### 删除一个值

~~~admonish info title="哈希表删除时内存如何释放？" collapsible=true
明白了哈希表是如何增长的，我们再来看删除的时候会发生什么。

当要在哈希表中删除一个值时，整个过程和查找类似:
1. 先要找到要被删除的 key 所在的位置。
2. 在找到具体位置后，并不需要实际清除内存，只需要将它的 ctrl byte 设回 0xff（或者标记成删除状态）。这样，这个 bucket 就可以被再次使用了

> 这里有一个问题，当 key/value 有额外的内存时，比如 String，它的内存不会立即回收，只有在下一次对应的 bucket 被使用时，让 HashMap 不再拥有这个 String 的所有权之后，这个 String 的内存才被回收。我们看下面的示意图：


![17｜数据结构：软件系统核心部件哈希表，内存如何布局？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882903.jpg)

一般来说，这并不会带来什么问题，顶多是内存占用率稍高一些。但某些极端情况下，比如在哈希表中添加大量内容，又删除大量内容后运行，这时你可以通过 shrink_to_fit / shrink_to 释放掉不需要的内存。

~~~

### 让自定义的数据结构做 Hash key

~~~admonish info title="自定义数据结构需要实现Hash、PartialEq、Eq这三个trait" collapsible=true
有时候，我们需要让自定义的数据结构成为 HashMap 的 key。此时，要使用到三个 trait：[Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)、[PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)、[Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)，不过这三个 trait 都可以通过派生宏自动生成。其中：
1. 实现了 Hash ，可以让数据结构计算哈希；
2. 实现了 PartialEq/Eq，可以让数据结构进行相等和不相等的比较。
3. Eq 实现了比较的自反性（a == a）、对称性（a == b 则 b == a）以及传递性（a == b，b == c，则 a == c）
4. PartialEq 没有实现自反性。

我们可以写个例子，看看自定义数据结构如何支持 HashMap：

```rust, editable

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

// 如果要支持 Hash，可以用 #[derive(Hash)]，前提是每个字段都实现了 Hash
// 如果要能作为 HashMap 的 key，还需要 PartialEq 和 Eq
#[derive(Debug, Hash, PartialEq, Eq)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    pub fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}
fn main() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("Tyr", 18);
    // 实现了 Hash 的数据结构可以直接调用 hash 方法
    student.hash(&mut hasher);
    let mut map = HashMap::new();
    // 实现了 Hash / PartialEq / Eq 的数据结构可以作为 HashMap 的 key
    map.insert(student, vec!["Math", "Writing"]);
    println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
}
```
~~~

### HashSet / BTreeMap / BTreeSet

~~~admonish info title="HashSet只用于确认存在，存放无序集合" collapsible=true
有时我们只需要简单确认元素是否在集合中，如果用 HashMap 就有些浪费空间了。这时可以用 HashSet，它就是简化的 HashMap，可以用来存放无序的集合，定义直接是 HashMap<K, ()>：

```rust, editable

use hashbrown::hash_set as base;

pub struct HashSet<T, S = RandomState> {
    base: base::HashSet<T, S>,
}

pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator + Clone = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}
```
> 使用 HashSet 查看一个元素是否属于集合的效率非常高。
~~~

~~~admonish info title="BTreeMap和BTreeSet都是用于查找，存放有序集合" collapsible=true
BTreeMap 是内部使用 [B-tree](https://en.wikipedia.org/wiki/B-tree) 来组织哈希表的数据结构。另外 BTreeSet 和 HashSet 类似，是 BTreeMap 的简化版，可以用来存放有序集合。
我们这里重点看下 BTreeMap，它的数据结构如下：

```rust, editable

pub struct BTreeMap<K, V> {
    root: Option<Root<K, V>>,
    length: usize,
}

pub type Root<K, V> = NodeRef<marker::Owned, K, V, marker::LeafOrInternal>;

pub struct NodeRef<BorrowType, K, V, Type> {
    height: usize,
    node: NonNull<LeafNode<K, V>>,
    _marker: PhantomData<(BorrowType, Type)>,
}

struct LeafNode<K, V> {
    parent: Option<NonNull<InternalNode<K, V>>>,
    parent_idx: MaybeUninit<u16>,
    len: u16,
    keys: [MaybeUninit<K>; CAPACITY],
    vals: [MaybeUninit<V>; CAPACITY],
}

struct InternalNode<K, V> {
    data: LeafNode<K, V>,
    edges: [MaybeUninit<BoxedNode<K, V>>; 2 * B],
}
```

> 和 HashMap 不同的是，BTreeMap 是有序的。我们看个例子（代码）:

```rust, editable

use std::collections::BTreeMap;

fn main() {
    let map = BTreeMap::new();
    let mut map = explain("empty", map);

    for i in 0..16usize {
        map.insert(format!("Tyr {}", i), i);
    }

    let mut map = explain("added", map);

    map.remove("Tyr 1");

    let map = explain("remove 1", map);

    for item in map.iter() {
        println!("{:?}", item);
    }
}

// BTreeMap 结构有 height，node 和 length
// 我们 transmute 打印之后，再 transmute 回去
fn explain<K, V>(name: &str, map: BTreeMap<K, V>) -> BTreeMap<K, V> {
    let arr: [usize; 3] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: height: {}, root node: 0x{:x}, len: 0x{:x}",
        name, arr[0], arr[1], arr[2]
    );
    unsafe { std::mem::transmute(arr) }
}
```

> 可以看到，在遍历时，BTreeMap 会按照 key 的顺序把值打印出来。如果你想让自定义的数据结构可以作为 BTreeMap 的 key，那么需要实现 PartialOrd 和 Ord，这两者的关系和 PartialEq / Eq 类似，PartialOrd 也没有实现自反性。同样的，PartialOrd 和 Ord 也可以通过派生宏来实现。

~~~

### 为什么 Rust 的 HashMap 要缺省采用加密安全的哈希算法？

~~~admonish info title="为什么 Rust 的 HashMap 要缺省采用加密安全的哈希算法？" collapsible=true
我们知道哈希表在软件系统中的重要地位，但哈希表在最坏情况下，如果绝大多数 key 的 hash 都碰撞在一起，性能会到 O(n)，这会极大拖累系统的效率。

比如 1M 大小的 session 表，正常情况下查表速度是 O(1)，但极端情况下，需要比较 1M 个数据后才能找到，这样的系统就容易被 DoS 攻击。所以如果不是加密安全的哈希函数，只要黑客知道哈希算法，就可以构造出大量的 key 产生足够多的哈希碰撞，造成目标系统 DoS。

SipHash 就是为了回应 DoS 攻击而创建的哈希算法，虽然和 sha2 这样的加密哈希不同（不要将 SipHash 用于加密！），但它可以提供类似等级的安全性。把 SipHash 作为 HashMap 的缺省的哈希算法，Rust 可以避免开发者在不知情的情况下被 DoS，就像曾经在 Web 世界发生的那样。
当然，这一切的代价是性能损耗，虽然 SipHash 非常快，但它比 hashbrown 缺省使用的 Ahash 慢了不少。如果你确定使用的 HashMap 不需要 DoS 防护（比如一个完全内部使用的 HashMap），那么可以用 Ahash 来替换。你只需要使用 Ahash 提供的 RandomState 即可：

```rust, editable

use ahash::{AHasher, RandomState};
use std::collections::HashMap;
let mut map: HashMap<char, i32, RandomState> = HashMap::default();
map.insert('a', 1);
```

~~~

# 三、错误处理

## 错误处理包含这么几部分

~~~admonish info title="错误处理包含这么几部分" collapsible=true
在一门编程语言中，控制流程是语言的核心流程，而错误处理又是控制流程的重要组成部分。

语言优秀的错误处理能力，会大大减少错误处理对整体流程的破坏，让我们写代码更行云流水，读起来心智负担也更小。


对我们开发者来说，错误处理包含这么几部分：

1. 错误捕获后，可以立刻处理
2. 也可以延迟到不得不处理的地方再处理，这就涉及到错误的传播（propagate）。
3. 最后，根据不同的错误类型，给用户返回合适的、帮助他们理解问题所在的错误消息。

---
![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895295.jpg)
---

> 作为一门极其注重用户体验的编程语言，Rust 从其它优秀的语言中，尤其是 Haskell ，吸收了错误处理的精髓，并以自己独到的方式展现出来。
~~~

## 错误处理的主流方法

~~~admonish info title="错误处理的三种主流方法以及其他语言是如何应用这些方法的。" collapsible=true
1. 使用返回值（错误码）

使用返回值来表征错误，是最古老也是最实用的一种方式，它的使用范围很广，从函数返回值，到操作系统的系统调用的错误码 errno、进程退出的错误码 retval，甚至 HTTP API 的状态码，都能看到这种方法的身影。

> 举个例子，在 C 语言中，如果 fopen(filename) 无法打开文件，会返回 NULL，调用者通过判断返回值是否为 NULL，来进行相应的错误处理。

> 我们再看个例子：

```c

size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream)
```

单看这个接口，我们很难直观了解，当读文件出错时，错误是如何返回的。从文档中，我们得知，如果返回的 size_t 和传入的 size_t 不一致，那么要么发生了错误，要么是读到文件尾（EOF），调用者要进一步通过 ferror 才能得到更详细的错误。

像 C 这样，通过返回值携带错误信息，有很多局限。返回值有它原本的语义，强行把错误类型嵌入到返回值原本的语义中，需要全面且实时更新的文档，来确保开发者能正确区别对待，正常返回和错误返回。

所以 Golang 对其做了扩展，在函数返回的时候，可以专门携带一个错误对象。比如上文的 fread，在 Golang 下可以这么定义：

```go

func Fread(file *File, b []byte) (n int, err error)
```

Golang 这样，区分开错误返回和正常返回，相对 C 来说进了一大步。

> 但是使用返回值的方式，始终有个致命的问题：在调用者调用时，错误就必须得到处理或者显式的传播。

如果函数 A 调用了函数 B，在 A 返回错误的时候，就要把 B 的错误转换成 A 的错误，显示出来。如下图所示：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895283.jpg)

这样写出来的代码会非常冗长，对我们开发者的用户体验不太好。如果不处理，又会丢掉这个错误信息，造成隐患。

另外，大部分生产环境下的错误是嵌套的。一个 SQL 执行过程中抛出的错误，可能是服务器出错，而更深层次的错误可能是，连接数据库服务器的 TLS session 状态异常。

其实知道服务器出错之外，我们更需要清楚服务器出错的内在原因。
- 因为服务器出错这个表层错误会提供给最终用户，而出错的深层原因要提供给我们自己，服务的维护者。
- 但是这样的嵌套错误在 C / Golang 都是很难完美表述的。

---

2. 使用异常

因为返回值不利于错误的传播，有诸多限制，Java 等很多语言使用异常来处理错误。

你可以把异常看成一种关注点分离（Separation of Concerns）：
1. 错误的产生和错误的处理完全被分隔开
2. 调用者不必关心错误，而被调者也不强求调用者关心错误。

- 程序中任何可能出错的地方，都可以抛出异常；
- 而异常可以通过栈回溯（stack unwind）被一层层自动传递，直到遇到捕获异常的地方，
- 如果回溯到 main 函数还无人捕获，程序就会崩溃。如下图所示：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895270.jpg)

使用异常来返回错误可以极大地简化错误处理的流程，它解决了返回值的传播问题。

然而，上图中异常返回的过程看上去很直观，就像数据库中的事务（transaction）在出错时会被整体撤销（rollback）一样。
> 但实际上，这个过程远比你想象的复杂，而且需要额外操心[异常安全（exception safety）](https://www.lighterra.com/papers/exceptionsharmful/)。
我们看下面用来切换背景图片的（伪）代码：

```c++

void transition(...) {
  lock(&mutex);
  delete background;
  ++changed;
  background = new Background(...);
  unlock(&mutex);
}
```

试想, 如果在创建新的背景时失败，抛出异常，会跳过后续的处理流程，一路栈回溯到 try catch 的代码，那么，这里锁住的 mutex 无法得到释放，而已有的背景被清空，新的背景没有创建，程序进入到一个奇怪的状态。

确实在大多数情况下，用异常更容易写代码，但当异常安全无法保证时，程序的正确性会受到很大的挑战。因此，你在使用异常处理时，需要特别注意异常安全，尤其是在并发环境下。

异常处理另外一个比较严重的问题是：开发者会滥用异常。只要有错误，不论是否严重、是否可恢复，都一股脑抛个异常。到了需要的地方，捕获一下了之。殊不知，异常处理的开销要比处理返回值大得多，滥用会有很多额外的开销。
~~~

3. 使用类型系统

第三种错误处理的方法就是使用类型系统。其实，在使用返回值处理错误的时候，我们已经看到了类型系统的雏形。

错误信息既然可以通过已有的类型携带，或者通过多返回值的方式提供，那么通过类型来表征错误，使用一个内部包含正常返回类型和错误返回类型的复合类型，通过类型系统来强制错误的处理和传递，是不是可以达到更好的效果呢？

的确如此。这种方式被大量使用在有强大类型系统支持的函数式编程语言中，如 Haskell/Scala/Swift。其中最典型的包含了错误类型的复合类型是 Haskell 的 Maybe 和 Either 类型。

- Maybe 类型允许数据包含一个值（Just）或者没有值（Nothing），这对简单的不需要类型的错误很有用。还是以打开文件为例，如果我们只关心成功打开文件的句柄，那么 Maybe 就足够了。

- 当我们需要更为复杂的错误处理时，我们可以使用 Either 类型。它允许数据是 Left a 或者 Right b 。其中，a 是运行出错的数据类型，b 可以是成功的数据类型。

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895260.jpg)

> 我们可以看到，这种方法依旧是通过返回值返回错误，但是错误被包裹在一个完整的、必须处理的类型中，比 Golang 的方法更安全。

我们前面提到，使用返回值返回错误的一大缺点是，错误需要被调用者立即处理或者显式传递。但是使用 Maybe / Either 这样的类型来处理错误的好处是，我们可以用函数式编程的方法简化错误的处理，比如 map、fold
等函数，让代码相对不那么冗余。

需要注意的是，很多不可恢复的错误，如“磁盘写满，无法写入”的错误，使用异常处理可以避免一层层传递错误，让代码简洁高效，所以大多数使用类型系统来处理错误的语言，会同时使用异常处理作为补充。

## Rust 的错误处理

由于诞生的年代比较晚，Rust 有机会从已有的语言中学习到各种错误处理的优劣。对于 Rust 来说，目前的几种方式相比而言，最佳的方法是，使用类型系统来构建主要的错误处理流程。

### Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型。

~~~admonish info title="Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型" collapsible=true
![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895260.jpg)


- Option 是一个 enum，其定义如下：

```rust, editable

pub enum Option<T> {
    None,
    Some(T),
}
```
> 它可以承载有值 / 无值这种最简单的错误类型。

- Result 是一个更加复杂的 enum，其定义如下：

```rust, editable

#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> 当函数出错时，可以返回 Err(E)，否则 Ok(T)。

我们看到，Result 类型声明时还有个 must_use 的标注，编译器会对有 must_use 标注的所有类型做特殊处理：如果该类型对应的值没有被显式使用，则会告警。这样，保证错误被妥善处理。如下图所示：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/e2100e3f17a9587c4d4bf50523c10653.png)

这里，如果我们调用 read_file 函数时，直接丢弃返回值，由于 #[must_use] 的标注，Rust 编译器报警，要求我们使用其返回值。
~~~

### ? 操作符

~~~admonish info title="? 操作符的由来" collapsible=true
这虽然可以极大避免遗忘错误的显示处理，但如果我们并不关心错误，只需要传递错误，还是会写出像 C 或者 Golang 一样比较冗余的代码。怎么办？

好在 Rust 除了有强大的类型系统外，还具备元编程的能力。早期 Rust 提供了 try! 宏来简化错误的显式处理，后来为了进一步提升用户体验，try! 被进化成 ? 操作符。

所以在 Rust 代码中，如果你只想传播错误，不想就地处理，可以用 ? 操作符，比如（代码）:

```rust, editable

use std::fs::File;
use std::io::Read;

fn read_file(name: &str) -> Result<String, std::io::Error> {
  let mut f = File::open(name)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  Ok(contents)
}
```

> 通过 ? 操作符，Rust 让错误传播的代价和异常处理不相上下，同时又避免了异常处理的诸多问题。
---
? 操作符内部被展开成类似这样的代码：

```rust, editable

match result {
  Ok(v) => v,
  Err(e) => return Err(e.into())
}
```

> 所以，我们可以方便地写出类似这样的代码，简洁易懂，可读性很强：

```rust, editable

fut
  .await?
  .process()?
  .next()
  .await?;
```

整个代码的执行流程如下：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895239.jpg)

虽然 ? 操作符使用起来非常方便，但你要注意在不同的错误类型之间是无法直接使用的，需要实现 From trait 在二者之间建立起转换的桥梁，这会带来额外的麻烦。
~~~

###  函数式错误处理

~~~admonish info title="map / map_err / and_then: 使用函数式错误处理" collapsible=true
Rust 还为 Option 和 Result 提供了大量的辅助函数，如 map / map_err / and_then，你可以很方便地处理数据结构中部分情况。如下图所示：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F.jpg)

通过这些函数，你可以很方便地对错误处理引入 [Railroad oriented programming 范式](https://www.slideshare.net/ScottWlaschin/railway-oriented-programming)。比如用户注册的流程，你需要校验用户输入，对数据进行处理，转换，然后存入数据库中。你可以这么撰写这个流程：

```rust, editable

Ok(data)
  .and_then(validate)
  .and_then(process)
  .map(transform)
  .and_then(store)
  .map_error(...)
```

> 执行流程如下图所示：

![image-20221004225336162](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20221004225336162.png)

此外，Option 和 Result 的互相转换也很方换，这也得益于 Rust 构建的强大的函数式编程的能力。

~~~

我们可以看到，无论是通过 ? 操作符，还是函数式编程进行错误处理，Rust 都力求让错误处理灵活高效，让开发者使用起来简单直观。

###  panic! 和 catch_unwind

~~~admonish info title="Rust 也提供了特殊的异常处理能力: panic!和catch_unwind" collapsible=true
使用 Option 和 Result 是 Rust 中处理错误的首选，绝大多数时候我们也应该使用，但 Rust 也提供了特殊的异常处理能力。

在 Rust 看来，一旦你需要抛出异常，那抛出的一定是严重的错误。所以，Rust 跟 Golang 一样，使用了诸如 panic! 这样的字眼警示开发者：想清楚了再使用我。在使用 Option 和 Result 类型时，开发者也可以对其 unwarp() 或者 expect()，强制把 Option<T> 和 Result<T, E> 转换成 T，如果无法完成这种转换，也会 panic! 出来。

一般而言，panic! 是不可恢复或者不想恢复的错误，我们希望在此刻，程序终止运行并得到崩溃信息。比如下面的代码，它解析[ noise protocol](https://noiseprotocol.org/noise.html#protocol-names-and-modifiers)的协议变量：

```rust, editable

let params: NoiseParams = "Noise_XX_25519_AESGCM_SHA256".parse().unwrap();
```

如果开发者不小心把协议变量写错了，最佳的方式是立刻 panic! 出来，让错误立刻暴露，以便解决这个问题。
---
有些场景下，我们也希望能够像异常处理那样能够栈回溯，把环境恢复到捕获异常的上下文。Rust 标准库下提供了 catch_unwind() ，把调用栈回溯到 catch_unwind 这一刻，作用和其它语言的 try {…} catch {…} 一样。见如下代码：
```rust, editable

use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:#?}", result);
}
```

当然，和异常处理一样，并不意味着你可以滥用这一特性，我想，这也是 Rust 把抛出异常称作 panic! ，而捕获异常称作 catch_unwind 的原因，让初学者望而生畏，不敢轻易使用。这也是一个不错的用户体验。

catch_unwind 在某些场景下非常有用:
1. 比如你在使用 Rust 为 erlang VM 撰写 NIF，你不希望 Rust 代码中的任何 panic! 导致 erlang VM 崩溃。因为崩溃是一个非常不好的体验，它违背了 erlang 的设计原则：process 可以 let it crash，但错误代码不该导致 VM 崩溃。
2. 此刻，你就可以把 Rust 代码整个封装在 catch_unwind() 函数所需要传入的闭包中。这样，一旦任何代码中，包括第三方 crates 的代码，含有能够导致 panic! 的代码，都会被捕获，并被转换为一个 Result。
~~~

### Error trait 和错误类型的转换

~~~admonish info title="使用Error trait自定义错误类型" collapsible=true
上文中，我们讲到 Result<T, E> 里 E 是一个代表错误的数据类型。为了规范这个代表错误的数据类型的行为，Rust 定义了 Error trait：

```rust, editable

pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn backtrace(&self) -> Option<&Backtrace> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
}
```

我们可以定义我们自己的数据类型，然后为其实现 Error trait。
~~~

~~~admonish info title="使用thiserror和anyhow简化步骤" collapsible=true
不过，这样的工作已经有人替我们简化了：我们可以使用 thiserror和 anyhow来简化这个步骤。

- thiserror 提供了一个派生宏（derive macro）来简化错误类型的定义，比如：

```rust, editable

use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

如果你在撰写一个 Rust 库，那么 thiserror 可以很好地协助你对这个库里所有可能发生的错误进行建模。

- anyhow 实现了 anyhow::Error 和任意符合 Error trait 的错误类型之间的转换，让你可以使用 ? 操作符，不必再手工转换错误类型。
> anyhow 还可以让你很容易地抛出一些临时的错误，而不必费力定义错误类型，当然，我们不提倡滥用这个能力。

作为一名严肃的开发者，我非常建议你在开发前，先用类似 thiserror 的库定义好你项目中主要的错误类型，并随着项目的深入，不断增加新的错误类型，让系统中所有的潜在错误都无所遁形。

~~~

# 四、闭包结构

## 闭包的定义

~~~admonish info title="闭包的基本概念" collapsible=true
闭包的基本概念：

- 闭包是将函数，或者说代码和其环境一起存储的一种数据结构。
- 闭包引用的上下文中的自由变量，会被捕获到闭包的结构中，成为闭包类型的一部分
- 闭包会根据内部的使用情况，捕获环境中的自由变量。
~~~

~~~admonish info title="Rust中闭包捕获自由变量的两个方法" collapsible=true
> 在 Rust 里，闭包可以用 |args| {code} 来表述，图中闭包 c 捕获了上下文中的 a 和 b，并通过引用来使用这两个自由变量：

![19｜闭包：FnOnce、FnMut和Fn，为什么有这么多类型？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/19%EF%BD%9C%E9%97%AD%E5%8C%85%EF%BC%9AFnOnce%E3%80%81FnMut%E5%92%8CFn%EF%BC%8C%E4%B8%BA%E4%BB%80%E4%B9%88%E6%9C%89%E8%BF%99%E4%B9%88%E5%A4%9A%E7%B1%BB%E5%9E%8B%EF%BC%9F-4897625.jpg)

> 除了用引用来捕获自由变量之外，还有另外一个方法使用 move 关键字 move |args| {code} 。

~~~

~~~admonish info title="thread::spawn的参数是一个闭包，使用move关键字" collapsible=true
比如创建新线程的 thread::spawn，它的参数就是一个闭包：

```rust, editable

pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```
> 仔细看这个接口：

1. F: FnOnce() → T，表明 F 是一个接受 0 个参数、返回 T 的闭包。FnOnce 我们稍后再说。
2. F: Send + 'static，说明闭包 F 这个数据结构，需要静态生命周期或者拥有所有权，并且它还能被发送给另一个线程。a
3. T: Send + 'static，说明闭包 F 返回的数据结构 T，需要静态生命周期或者拥有所有权，并且它还能被发送给另一个线程。

> 1 和 3 都很好理解，2 就有些费解了。一个闭包，它不就是一段代码 + 被捕获的变量么？需要静态生命周期或者拥有所有权是什么意思？

拆开看:
- 代码自然是静态生命周期
- 那么是不是意味着被捕获的变量，需要静态生命周期或者拥有所有权？

> 的确如此。在使用 thread::spawn 时，我们需要使用 move 关键字，把变量的所有权从当前作用域移动到闭包的作用域，让 thread::spawn 可以正常编译通过：

```rust, editable

use std::thread;

fn main() {
    let s = String::from("hello world");

    let handle = thread::spawn(move || {
        println!("moved: {:?}", s);
    });

    handle.join().unwrap();
}
```
~~~

> 但你有没有好奇过，加 move 和不加 move，这两种闭包有什么本质上的不同？闭包究竟是一种什么样的数据类型，让编译器可以判断它是否满足 Send + 'static 呢？我们从闭包的本质下手来尝试回答这两个问题。

## 闭包本质上是什么？

~~~admonish info title="闭包的本质是匿名结构体，move会转移自由变量的所有权" collapsible=true
在官方的 Rust reference 中，有这样的定义：
> A closure expression produces a closure value with a unique, anonymous type that cannot be written out. A closure type is approximately equivalent to a struct which contains the captured variables.
---
闭包是一种匿名类型，一旦声明，就会产生一个新的类型，但这个类型无法被其它地方使用。这个类型就像一个结构体，会包含所有捕获的变量。

所以闭包类似一个特殊的结构体？

为了搞明白这一点，我们得写段代码探索一下，建议你跟着敲一遍认真思考（代码）：

```rust, editable

use std::{collections::HashMap, mem::size_of_val};
fn main() {
    // 长度为 0
    let c1 = || println!("hello world!");
    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 如果捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}
```

> 分别生成了 5 个闭包：

1. c1 没有参数，也没捕获任何变量，从代码输出可以看到，c1 长度为 0；
2. c2 有一个 i32 作为参数，没有捕获任何变量，长度也为 0，可以看出参数跟闭包的大小无关；
3. c3 捕获了一个对变量 name 的引用，这个引用是 &String，长度为 8。而 c3 的长度也是 8；
4. c4 捕获了变量 name1 和 table，由于用了 move，它们的所有权移动到了 c4 中。c4 长度是 72，恰好等于 String 的 24 字节，加上 HashMap 的 48 字节。
5. c5 捕获了 name2，name2 的所有权移动到了 c5，虽然 c5 有局部变量，但它的大小和局部变量也无关，c5 的大小等于 String 的 24 字节。

> 可以看到，不带 move 时，闭包捕获的是对应自由变量的引用；带 move 时，对应自由变量的所有权会被移动到闭包结构中。
~~~

继续分析这段代码的运行结果。

~~~admonish info title="闭包大小只跟捕获的变量有关" collapsible=true
还知道了，闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关:
> 因为它们是在调用的时刻才在栈上产生的内存分配，说到底和闭包类型本身是无关的，所以闭包的大小跟它们自然无关。

```rust, editable

use std::{collections::HashMap, mem::size_of_val};
fn main() {
    // 长度为 0
    let c1 = || println!("hello world!");
    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 如果捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}
```

~~~

~~~admonish info title="闭包的内存布局与结构体有什么区别？" collapsible=true

那一个闭包类型在内存中究竟是如何排布的，和结构体有什么区别？

> 我们要再次结合 rust-gdb 探索，看看上面的代码在运行结束前，几个长度不为 0 闭包内存里都放了什么：

![19｜闭包：FnOnce、FnMut和Fn，为什么有这么多类型？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/19%EF%BD%9C%E9%97%AD%E5%8C%85%EF%BC%9AFnOnce%E3%80%81FnMut%E5%92%8CFn%EF%BC%8C%E4%B8%BA%E4%BB%80%E4%B9%88%E6%9C%89%E8%BF%99%E4%B9%88%E5%A4%9A%E7%B1%BB%E5%9E%8B%EF%BC%9F-4897598.png)
---
可以看到:
1. c3 的确是一个引用，把它指向的内存地址的 24 个字节打出来，是 (ptr | cap | len) 的标准结构。如果打印 ptr 对应的堆内存的 3 个字节，是 ‘t’ ‘y’ ‘r’。
2. 而 c4 捕获的 name 和 table，内存结构和下面的结构体一模一样：

```rust, editable

struct Closure4 {
    name: String,  // (ptr|cap|len)=24字节
    table: HashMap<&str, &str> // (RandomState(16)|mask|ctrl|left|len)=48字节
}
```

不过，对于 closure 类型来说，编译器知道像函数一样调用闭包 c4() 是合法的，并且知道执行 c4() 时，代码应该跳转到什么地址来执行。在执行过程中，如果遇到 name、table，可以从自己的数据结构中获取。

那么多想一步，闭包捕获变量的顺序，和其内存结构的顺序是一致的么？

的确如此，如果我们调整闭包里使用 name1 和 table 的顺序：

```rust, editable

let c4 = move || println!("hello: {:?}, {}", table, name1);
```

其数据的位置是相反的，类似于：

```rust, editable

struct Closure4 {
    table: HashMap<&str, &str> // (RandomState(16)|mask|ctrl|left|len)=48字节
    name: String,  // (ptr|cap|len)=24字节
}
```

从 gdb 中也可以看到同样的结果：

![19｜闭包：FnOnce、FnMut和Fn，为什么有这么多类型？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/19%EF%BD%9C%E9%97%AD%E5%8C%85%EF%BC%9AFnOnce%E3%80%81FnMut%E5%92%8CFn%EF%BC%8C%E4%B8%BA%E4%BB%80%E4%B9%88%E6%9C%89%E8%BF%99%E4%B9%88%E5%A4%9A%E7%B1%BB%E5%9E%8B%EF%BC%9F.png)

> 不过这只是逻辑上的位置，Rust 编译器会重排内存，让数据能够以最小的代价对齐，所以有些情况下，内存中数据的顺序可能和 struct 定义不一致。

所以回到刚才闭包和结构体的比较。

在 Rust 里，闭包产生的匿名数据类型，格式和 struct 是一样的。看图中 gdb 的输出，闭包是存储在栈上，并且除了捕获的数据外，闭包本身不包含任何额外函数指针指向闭包的代码。如果你理解了 c3 / c4 这两个闭包，c5 是如何构造的就很好理解了。

现在，你是不是可以回答为什么 thread::spawn 对传入的闭包约束是 Send + 'static 了？究竟什么样的闭包满足它呢？
> 很明显，使用了 move 且 move 到闭包内的数据结构满足 Send，因为此时，闭包的数据结构拥有所有数据的所有权，它的生命周期是 'static。
~~~

看完 Rust 闭包的内存结构，你是不是想说“就这”，没啥独特之处吧？但是对比其他语言，结合接下来我的解释，你再仔细想想就会有一种“这怎么可能”的惊讶。

## 不同语言的闭包设计

~~~admonish info title="其他语言闭包设计有什么问题？为何Rust可以保证闭包的性能与函数差不多" collapsible=true
闭包最大的问题是变量的多重引用导致生命周期不明确，所以你先想，其它支持闭包的语言（lambda 也是闭包），它们的闭包会放在哪里？

栈上么？是，又好像不是。

因为闭包这玩意，从当前上下文中捕获了些变量，变得有点不伦不类，不像函数那样清楚，尤其是这些被捕获的变量，它们的归属和生命周期处理起来很麻烦。所以，大部分编程语言的闭包很多时候无法放在栈上，需要额外的堆分配。[你可以看这个 Golang 的例子](https://github.com/golang/go/issues/43210)。


不光 Golang，Java / Swift / Python / JavaScript 等语言都是如此，这也是为什么大多数编程语言闭包的性能要远低于函数调用。

因为使用闭包就意味着：
1. 额外的堆内存分配
2. 潜在的动态分派（很多语言会把闭包处理成函数指针）
3. 额外的内存回收。

在性能上，唯有 C++ 的 lambda 和 Rust 闭包类似，不过 C++ 的闭包还有一些场景会触发堆内存分配。

Rust / Swift / Kotlin iterator 函数式编程的性能测试：
1. Kotlin 运行超时
2. Swift 很慢
3. Rust 的性能却和使用命令式编程的 C 几乎一样，除了编译器优化的效果，也因为 Rust 闭包的性能和函数差不多。

> 为什么 Rust 可以做到这样呢？

这又跟 Rust 从根本上使用所有权和借用，解决了内存归属问题有关。

在其他语言中，闭包变量因为多重引用导致生命周期不明确，但 Rust 从一开始就消灭了这个问题：

- 如果不使用 move 转移所有权，闭包会引用上下文中的变量，这个引用受借用规则的约束，所以只要编译通过，那么闭包对变量的引用就不会超过变量的生命周期，没有内存安全问题。

- 如果使用 move 转移所有权，上下文中的变量在转移后就无法访问，闭包完全接管这些变量，它们的生命周期和闭包一致，所以也不会有内存安全问题。

- 而 Rust 为每个闭包生成一个新的类型，又使得调用闭包时可以直接和代码对应，省去了使用函数指针再转一道手的额外消耗。

> 所以还是那句话，当回归到最初的本原，你解决的不是单个问题，而是由此引发的所有问题。我们不必为堆内存管理设计 GC、不必为其它资源的回收提供 defer 关键字、不必为并发安全进行诸多限制、也不必为闭包挖空心思搞优化。
~~~

## Rust 的闭包类型

现在我们搞明白了闭包究竟是个什么东西，在内存中怎么表示，接下来我们看看 FnOnce / FnMut / Fn 这三种闭包类型有什么区别。

在声明闭包的时候，我们并不需要指定闭包要满足的约束，但是当闭包作为函数的参数或者数据结构的一个域时，我们需要告诉调用者，对闭包的约束。

还以 thread::spawn 为例，它要求传入的闭包满足 FnOnce trait。

### FnOnce

~~~admonish info title="FnOnce定义->例子解析" collapsible=true
先来看 FnOnce。它的[定义](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)如下：

```rust, editable

pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

1. FnOnce 有一个关联类型 Output，显然，它是闭包返回值的类型；
2. 还有一个方法 call_once，要注意的是 call_once 第一个参数是 self，它会转移 self 的所有权到 call_once 函数中。

> 这也是为什么 FnOnce 被称作 Once ：它只能被调用一次。再次调用，编译器就会报变量已经被 move 这样的常见所有权错误了。

至于 FnOnce 的参数，是一个叫 Args 的泛型参数，它并没有任何约束。

看一个隐式的 FnOnce 的例子：

```rust, editable

fn main() {
    let name = String::from("Tyr");
    // 这个闭包啥也不干，只是把捕获的参数返回去
    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());

    println!("result: {:?}", result);

    // 无法再次调用
    let result = c("hi".to_string());
}
```

1. 这个闭包 c，啥也没做，只是把捕获的参数返回。
2. 就像一个结构体里，某个字段被转移走之后，就不能再访问一样，闭包内部的数据一旦被转移，这个闭包就不完整了，也就无法再次使用，所以它是一个 FnOnce 的闭包。

如果一个闭包并不转移自己的内部数据，那么它就不是 FnOnce，然而，一旦它被当做 FnOnce 调用，自己会被转移到 call_once 函数的作用域中，之后就无法再次调用了，我们看个例子（代码）：

```rust, editable

fn main() {
    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c1 可以被调用多次

    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));

    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    println!("result: {:?}", call_once("hi".into(), c));

    // 无法再次调用
    // let result = c("hi".to_string());

    // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}
```

~~~

### 怎么理解 FnOnce 的 Args 泛型参数呢？

~~~admonish info title="怎么理解 FnOnce 的 Args 泛型参数呢？" collapsible=true
Args 又是怎么和 FnOnce 的约束，比如 FnOnce(String) 这样的参数匹配呢？感兴趣的同学可以看下面的例子，它（不完全）模拟了 FnOnce 中闭包的使用（代码）：
```rust, editable

struct ClosureOnce<Captured, Args, Output> {
    // 捕获的数据
    captured: Captured,
    // closure 的执行代码
    func: fn(Args, Captured) -> Output,
}

impl<Captured, Args, Output> ClosureOnce<Captured, Args, Output> {
    // 模拟 FnOnce 的 call_once，直接消耗 self
    fn call_once(self, greeting: Args) -> Output {
        (self.func)(greeting, self.captured)
    }
}

// 类似 greeting 闭包的函数体
fn greeting_code1(args: (String,), captured: (String,)) -> (String, String) {
    (args.0, captured.0)
}

fn greeting_code2(args: (String, String), captured: (String, u8)) -> (String, String, String, u8) {
    (args.0, args.1, captured.0, captured.1)
}

fn main() {
    let name = "Tyr".into();
    // 模拟变量捕捉
    let c = ClosureOnce {
        captured: (name,),
        func: greeting_code1,
    };

    // 模拟闭包调用，这里和 FnOnce 不完全一样，传入的是一个 tuple 来匹配 Args 参数
    println!("{:?}", c.call_once(("hola".into(),)));
    // 调用一次后无法继续调用
    // println!("{:?}", clo.call_once("hola".into()));

    // 更复杂一些的复杂的闭包
    let c1 = ClosureOnce {
        captured: ("Tyr".into(), 18),
        func: greeting_code2,
    };

    println!("{:?}", c1.call_once(("hola".into(), "hallo".into())));
}
```

~~~

### FnMut

~~~admonish info title="FnMut的定义与例子" collapsible=true
理解了 FnOnce，我们再来看 FnMut，[它的定义如下](https://doc.rust-lang.org/std/ops/trait.FnMut.html)：

```rust, editable

pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(
        &mut self, 
        args: Args
    ) -> Self::Output;
}
```

1. 首先，FnMut “继承”了 FnOnce，或者说 FnOnce 是 FnMut 的 super trait。
2. 所以 FnMut 也拥有 Output 这个关联类型和 call_once 这个方法。
3. 此外，它还有一个 call_mut() 方法。
4. 注意 call_mut() 传入 &mut self，它不移动 self，所以 FnMut 可以被多次调用。

> 因为 FnOnce 是 FnMut 的 super trait，所以，一个 FnMut 闭包，可以被传给一个需要 FnOnce 的上下文，此时调用闭包相当于调用了 call_once()。

如果你理解了前面讲的闭包的内存组织结构，那么 FnMut 就不难理解，就像结构体如果想改变数据需要用 let mut 声明一样，如果你想改变闭包捕获的数据结构，那么就需要 FnMut。

我们看个例子（代码）：
```rust, editable


fn main() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    // 捕获 &mut name
    let mut c = || {
        name.push_str(" Tyr");
        println!("c: {}", name);
    };

    // 捕获 mut name1，注意 name1 需要声明成 mut
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once(c);
    call_once(c1);
}

// 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 想想看，为啥 call_once 不需要 mut？
fn call_once(c: impl FnOnce()) {
    c();
}

```
1. 在声明的闭包 c 和 c1 里，我们修改了捕获的 name 和 name1。
2. 不同的是 name 使用了引用，而 name1 移动了所有权，这两种情况和其它代码一样，也需要遵循所有权和借用有关的规则。
3. 所以，如果在闭包 c 里借用了 name，你就不能把 name 移动给另一个闭包 c1。

> 这里也展示了，c 和 c1 这两个符合 FnMut 的闭包，能作为 FnOnce 来调用。我们在代码中也确认了，FnMut 可以被多次调用，这是因为 call_mut() 使用的是 &mut self，不移动所有权。
~~~

### Fn

~~~admonish info title="Fn的定义与例子" collapsible=true
最后我们来看看 Fn trait。[它的定义如下](https://doc.rust-lang.org/std/ops/trait.Fn.html)：

```rust, editable

pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

1. 可以看到，它“继承”了 FnMut，或者说 FnMut 是 Fn 的 super trait。
2. 这也就意味着任何需要 FnOnce 或者 FnMut 的场合，都可以传入满足 Fn 的闭包。

我们继续看例子（代码）：

```rust, editable

fn main() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    // Fn，不移动所有权
    let mut c = |x: u64| v.len() as u64 * x;
    // Fn，移动所有权
    let mut c1 = move |x: u64| v1.len() as u64 * x;

    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));

    println!("call: {}", call(3, &c));
    println!("call: {}", call(3, &c1));

    println!("call_mut: {}", call_mut(4, &mut c));
    println!("call_mut: {}", call_mut(4, &mut c1));

    println!("call_once: {}", call_once(5, c));
    println!("call_once: {}", call_once(5, c1));
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}
```

~~~

### 总结一下三种闭包使用的情况以及它们之间的关系

~~~admonish info title="总结一下三种闭包使用的情况以及它们之间的关系" collapsible=true
![19｜闭包：FnOnce、FnMut和Fn，为什么有这么多类型？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/19%EF%BD%9C%E9%97%AD%E5%8C%85%EF%BC%9AFnOnce%E3%80%81FnMut%E5%92%8CFn%EF%BC%8C%E4%B8%BA%E4%BB%80%E4%B9%88%E6%9C%89%E8%BF%99%E4%B9%88%E5%A4%9A%E7%B1%BB%E5%9E%8B%EF%BC%9F.jpg)
~~~

## 闭包的使用场景

~~~admonish info title="在函数的参数和返回值中使用闭包" collapsible=true
thread::spawn 自不必说，我们熟悉的 Iterator trait 里面大部分函数都接受一个闭包，[比如 map](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#682-685)：
```rust, editable

fn map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}
```

1. 可以看到，Iterator 的 map() 方法接受一个 FnMut
2. 它的参数是 Self::Item
3. 返回值是没有约束的泛型参数 B。
4. Self::Item 是 Iterator::next() 方法吐出来的数据，被 map 之后，可以得到另一个结果。

所以在函数的参数中使用闭包，是闭包一种非常典型的用法。另外闭包也可以作为函数的返回值，举个简单的例子（代码）：

```rust, editable

use std::ops::Mul;

fn main() {
    let c1 = curry(5);
    println!("5 multiply 2 is: {}", c1(2));

    let adder2 = curry(3.14);
    println!("pi multiply 4^2 is: {}", adder2(4. * 4.));
}

fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
}
```
~~~

~~~admonish info title="为闭包实现某个 trait，使其也能表现出其他行为，而不仅仅是作为函数被调用。" collapsible=true
最后，闭包还有一种并不少见，但可能不太容易理解的用法：

为它实现某个 trait，使其也能表现出其他行为，而不仅仅是作为函数被调用。

比如说有些接口既可以传入一个结构体，又可以传入一个函数或者闭包。

> 我们看一个 [tonic（Rust 下的 gRPC 库）](https://github.com/hyperium/tonic)的[例子](https://docs.rs/tonic/0.5.2/src/tonic/service/interceptor.rs.html#41-53)

```rust, editable

pub trait Interceptor {
    /// Intercept a request before it is sent, optionally cancelling it.
    fn call(&mut self, request: crate::Request<()>) -> Result<crate::Request<()>, Status>;
}

impl<F> Interceptor for F
where
    F: FnMut(crate::Request<()>) -> Result<crate::Request<()>, Status>,
{
    fn call(&mut self, request: crate::Request<()>) -> Result<crate::Request<()>, Status> {
        self(request)
    }
}
```

在这个例子里，Interceptor 有一个 call 方法，它可以让 gRPC Request 被发送出去之前被修改，一般是添加各种头，比如 Authorization 头。

我们可以创建一个结构体，为它实现 Interceptor，不过大部分时候 Interceptor 可以直接通过一个闭包函数完成。为了让传入的闭包也能通过 Interceptor::call() 来统一调用，可以为符合某个接口的闭包实现 Interceptor trait。掌握了这种用法，我们就可以通过某些 trait 把特定的结构体和闭包统一起来调用，是不是很神奇。


~~~

