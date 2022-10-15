# 一、智能指针

<!--ts-->
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

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Oct 15 10:38:57 UTC 2022 -->

<!--te-->

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
