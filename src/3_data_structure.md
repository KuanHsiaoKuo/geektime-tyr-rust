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
      * [HashMap数据结构](#hashmap数据结构)
      * [HashMap基本使用方法](#hashmap基本使用方法)
      * [HashMap内存布局](#hashmap内存布局)
      * [ctrl表](#ctrl表)
* [三、错误处理](#三错误处理)
   * [错误处理主流方法](#错误处理主流方法)
   * [Rust如何处理错误](#rust如何处理错误)
* [四、闭包结构](#四闭包结构)
   * [闭包定义](#闭包定义)
   * [闭包本质](#闭包本质)
   * [闭包设计](#闭包设计)
   * [Rust闭包类型](#rust闭包类型)
   * [闭包使用场景](#闭包使用场景)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Oct  4 07:24:32 UTC 2022 -->

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

### HashMap数据结构

### HashMap基本使用方法

### HashMap内存布局

### ctrl表

# 三、错误处理

## 错误处理主流方法

## Rust如何处理错误

# 四、闭包结构

## 闭包定义

## 闭包本质

## 闭包设计

## Rust闭包类型

## 闭包使用场景