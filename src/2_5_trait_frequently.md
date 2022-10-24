# 常用trait

<!--ts-->
* [常用trait](#常用trait)
   * [内存相关](#内存相关)
      * [Copy](#copy)
      * [Drop](#drop)
   * [标签trait](#标签trait)
      * [Sized](#sized)
      * [Send/Sync](#sendsync)
   * [类型转换](#类型转换)
      * [From/Into: 值到值](#frominto-值到值)
      * [TryFrom/TryInto: 值到值，可能出现错误](#tryfromtryinto-值到值可能出现错误)
      * [AsRef/AsMut: 引用到引用](#asrefasmut-引用到引用)
   * [操作符相关: Deref/DerefMut](#操作符相关-derefderefmut)
   * [其他：Debug/Display/Default](#其他debugdisplaydefault)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 24 14:02:35 UTC 2022 -->

<!--te-->

> 基于[trait进阶使用](/2_3_1_trait_impl.html#%E8%BF%9B%E9%98%B6%E4%BD%BF%E7%94%A8)

~~~admonish info title='常用trait分类整理' collapsible=false
![14｜类型系统：有哪些必须掌握的trait？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/14%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E6%9C%89%E5%93%AA%E4%BA%9B%E5%BF%85%E9%A1%BB%E6%8E%8C%E6%8F%A1%E7%9A%84trait%EF%BC%9F.jpg)
~~~

## 内存相关

~~~admonish info title='Clone使用示例' collapsible=true
```rust, editable

pub trait Clone {
  fn clone(&self) -> Self;

  fn clone_from(&mut self, source: &Self) {
    *self = source.clone()
  }
}
```
~~~

~~~admonish info title='clone()与clone_from()' collapsible=true
Clone trait 有两个方法， clone() 和 clone_from() ，后者有缺省实现，所以平时我们只需要实现 clone() 方法即可。你也许会疑惑，这个 clone_from() 有什么作用呢？因为看起来 a.clone_from(&b) ，和 a = b.clone() 是等价的。其实不是，如果 a 已经存在，在 clone 过程中会分配内存，那么用 a.clone_from(&b) 可以避免内存分配，提高效率。b.clone() 是等价的。其实不是，如果 a 已经存在，在 clone 过程中会分配内存，那么用 a.clone_from(&b) 可以避免内存分配，提高效率。
~~~

~~~admonish info title='Clone trait 可以通过派生宏直接实现，这样能简化不少代码' collapsible=true
```rust, editable
#[derive(Clone, Debug)]
struct Developer {
  name: String,
  age: u8,
  lang: Language
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
  Rust,
  TypeScript,
  Elixir,
  Haskell
}

fn main() {
    let dev = Developer {
        name: "Tyr".to_string(),
        age: 18,
        lang: Language::Rust
    };
    let dev1 = dev.clone();
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str())
}
```
----
如果没有为 Language 实现 Clone 的话，Developer 的派生宏 Clone 将会编译出错。运行这段代码可以看到，对于 name，也就是 String 类型的 Clone，其堆上的内存也被 Clone 了一份，所以 Clone 是深度拷贝，栈内存和堆内存一起拷贝。
~~~

~~~admonish info title='clone 方法的接口是 &self' collapsible=true
值得注意的是，clone 方法的接口是 &self，这在绝大多数场合下都是适用的，我们在 clone 一个数据时只需要有已有数据的只读引用。但对 Rc 这样在 clone() 时维护引用计数的数据结构，clone() 过程中会改变自己，所以要用 Cell 这样提供内部可变性的结构来进行改变，如果你也有类似的需求，可以参考
~~~

### Copy

~~~admonish info title='不可变引用实现了 Copy，而可变引用 &mut T 没有实现 Copy' collapsible=true
不可变引用实现了 Copy，而可变引用 &mut T 没有实现 Copy。为什么是这样？因为如果可变引用实现了 Copy trait，那么生成一个可变引用然后把它赋值给另一个变量时，就会违背所有权规则：
同一个作用域下只能有一个可变引用。可见，Rust 标准库在哪些结构可以 Copy、哪些不可以 Copy 上，有着仔细的考量。
~~~

### Drop

~~~admonish info title='有两种情况你可能需要手工实现 Drop' collapsible=true
大部分场景无需为数据结构提供 Drop trait，系统默认会依次对数据结构的每个域做 drop。但有两种情况你可能需要手工实现 Drop。
1. 第一种是希望在数据结束生命周期的时候做一些事情，比如记日志。
2. 第二种是需要对资源回收的场景。编译器并不知道你额外使用了哪些资源，也就无法帮助你 drop 它们。比如说锁资源的释放，
3. 在 MutexGuard 中实现了 Drop 来释放锁资源：
```rust, editable
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
~~~

~~~admonish info title='Copy与Drop互斥' collapsible=true
需要注意的是，Copy trait 和 Drop trait 是互斥的，两者不能共存，当你尝试为同一种数据类型实现 Copy 时，也实现 Drop，编译器就会报错。
> 这其实很好理解：

- Copy 是按位做浅拷贝，那么它会默认拷贝的数据没有需要释放的资源；
- 而 Drop 恰恰是为了释放额外的资源而生的。

---
辅助理解，在代码中，强行用 Box::into_raw 获得堆内存的指针，放入 RawBuffer 结构中，这样就接管了这块堆内存的释放。
虽然 RawBuffer 可以实现 Copy trait，但这样一来就无法实现 Drop trait。
如果程序非要这么写，会导致内存泄漏，因为该释放的堆内存没有释放。

```rust, editable

use std::{fmt, slice};

// 注意这里，我们实现了 Copy，这是因为 *mut u8/usize 都支持 Copy
#[derive(Clone, Copy)]
struct RawBuffer {
    // 裸指针用 *const / *mut 来表述，这和引用的 & 不同
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释去掉，看看会出什么问题
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();

    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(buf);

    // buf 还能用
    println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    drop(buf)
}
```

----
但是这个操作不会破坏 Rust 的正确性保证：即便你 Copy 了 N 份 RawBuffer，由于无法实现 Drop trait，RawBuffer 指向的那同一块堆内存不会释放，所以不会出现 use after free 的内存安全问题

~~~

~~~admonish info title='对于代码安全来说，内存泄漏危害大？还是 use after free 危害大呢？' collapsible=true
对于代码安全来说，内存泄漏危害大？还是 use after free 危害大呢？
> 肯定是后者。
- Rust 的底线是内存安全，所以两害相权取其轻。
- 实际上，任何编程语言都无法保证不发生人为的内存泄漏
- 比如程序在运行时，开发者疏忽了，对哈希表只添加不删除，就会造成内存泄漏。
- 但 Rust 会保证即使开发者疏忽了，也不会出现内存安全问题。
~~~

## 标签trait

### Sized

~~~admonish info title='Size: Data 和处理 Data 的函数 process_data' collapsible=true
```rust, editable

struct Data<T> {
    inner: T,
}

fn process_data<T>(data: Data<T>) {
    todo!();
}
```
等价于：

```rust, editable

struct Data<T: Sized> {
    inner: T,
}

fn process_data<T: Sized>(data: Data<T>) {
    todo!();
}
```
---
大部分时候，我们都希望能自动添加这样的约束，因为这样定义出的泛型结构，在编译期，大小是固定的，可以作为参数传递给函数。如果没有这个约束，T 是大小不固定的类型， process_data 函数会无法编译。
~~~

~~~admonish info title='?Sized: 在少数情况下，需要 T 是可变类型的' collapsible=true
```rust, editable

pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    // 借用的数据
    Borrowed(&'a B),
    // 拥有的数据
    Owned(<B as ToOwned>::Owned),
}
```
----
这样 B 就可以是 [T] 或者 str 类型，大小都是不固定的。要注意 Borrowed(&'a B) 大小是固定的，因为它内部是对 B 的一个引用，而引用的大小是固定的
~~~

### Send/Sync

~~~admonish info title='这两个 trait 都是 unsafe auto trait' collapsible=true
这两个 trait 都是 unsafe auto trait:
- auto 意味着编译器会在合适的场合，自动为数据结构添加它们的实现
- 而 unsafe 代表实现的这个 trait 可能会违背 Rust 的内存安全准则
- 如果开发者手工实现这两个 trait ，要自己为它们的安全性负责。
~~~

~~~admonish info title='Send/Sync 是 Rust 并发安全的基础' collapsible=true
Send/Sync 是 Rust 并发安全的基础：
- 如果一个类型 T 实现了 Send trait，意味着 T 可以安全地从一个线程移动到另一个线程，也就是说所有权可以在线程间移动。
- 如果一个类型 T 实现了 Sync trait，则意味着 &T 可以安全地在多个线程中共享。一个类型 T 满足 Sync trait，当且仅当 &T 满足 Send trait。
~~~

~~~admonish info title='Send/Sync 在线程安全中的作用' collapsible=true
对于 Send/Sync 在线程安全中的作用，可以这么看:
1. 如果一个类型 T: Send，那么 T 在某个线程中的独占访问是线程安全的；
2. 如果一个类型 T: Sync，那么 T 在线程间的只读共享是安全的。
~~~

~~~admonish info title='绝大多数自定义的数据结构都是满足 Send / Sync 的。标准库中，不支持 Send / Sync 的数据结构主要有' collapsible=true
1. 裸指针 *const T / *mut T。
它们是不安全的，所以既不是 Send 也不是 Sync。
2. UnsafeCell 不支持 Sync。
也就是说，任何使用了 Cell 或者 RefCell 的数据结构不支持 Sync。
3. 引用计数 Rc 不支持 Send 也不支持 Sync。所以 Rc 无法跨线程。
~~~

~~~admonish info title='尝试跨线程使用 Rc / RefCell，会发生什么' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/14_sys_traits/src/send_sync.rs}}
```
~~~

~~~admonish info title='用到的std::thread::spawn' collapsible=true
```rust, editable

pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```
----
它的参数是一个闭包，这个闭包需要 Send + 'static：
1. 'static 意思是闭包捕获的自由变量必须是一个拥有所有权的类型，或者是一个拥有静态生命周期的引用；
2. Send 意思是，这些被捕获自由变量的所有权可以从一个线程移动到另一个线程。
> 从这个接口上，可以得出结论：如果在线程间传递 Rc，是无法编译通过的
~~~

~~~admonish info title='Rc不支持Send和Sync' collapsible=true

```rust, editable

// Rc 既不是 Send，也不是 Sync
fn rc_is_not_send_and_sync() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    thread::spawn(move || {
        println!("c= {:?}", c);
    });
}
```
---
![Rc不支持Send/Sync](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/14%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E6%9C%89%E5%93%AA%E4%BA%9B%E5%BF%85%E9%A1%BB%E6%8E%8C%E6%8F%A1%E7%9A%84trait%EF%BC%9F-4694742.jpg)
---
[Rc 的实现不支持 Send 和 Sync](https://doc.rust-lang.org/std/rc/struct.Rc.html#impl-Send)
~~~

~~~admonish info title='RefCell 可以在线程间转移所有权么？' collapsible=true
```rust, editable
fn refcell_is_send() {
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("a= {:?}", a);
    });
}
```
---
> [RefCell 实现了 Send，但没有实现 Sync](https://doc.rust-lang.org/std/cell/struct.RefCell.html#impl-Send)
~~~

~~~admonish info title='Arc支持Send/Sync' collapsible=true
> [Arc支持Send/Sync](https://doc.rust-lang.org/std/sync/struct.Arc.html#impl-Send)
```rust, editable

// RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync，但 RefCell 不是 Sync
fn refcell_is_not_sync() {
    let a = Arc::new(RefCell::new(1));
    let b = a.clone();
    let c = a.clone();
    thread::spawn(move || {
        println!("c= {:?}", c);
    });
}

```
---
因为 Arc 内部的数据是共享的，需要支持 Sync 的数据结构，但是 RefCell 不是 Sync，编译失败。
~~~

~~~admonish info title='在多线程情况下，我们只能使用支持 Send/Sync 的 Arc ，和 Mutex 一起，构造一个可以在多线程间共享且可以修改的类型' collapsible=true

```rust, editable

use std::{
    sync::{Arc, Mutex},
    thread,
};

// Arc<Mutex<T>> 可以多线程共享且修改数据
fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a= {:?}", a);
}

fn main() {
    arc_mutext_is_send_sync();
}
```
---
最后一个标记 trait Unpin，是用于自引用类型的，属于Future trait。
~~~

## 类型转换

~~~admonish info title='对比两种转化方式' collapsible=true

```rust, editable

// 第一种方法，为每一种转换提供一个方法
// 把字符串 s 转换成 Path
let v = s.to_path();
// 把字符串 s 转换成 u64
let v = s.to_u64();

// 第二种方法，为 s 和要转换的类型之间实现一个 Into<T> trait
// v 的类型根据上下文得出
let v = s.into();
// 或者也可以显式地标注 v 的类型
let v: u64 = s.into();
```
---
显然，第二种方法要更好，因为它符合软件开发的开闭原则（Open-Close Principle），
> “软件中的对象（类、模块、函数等等）对扩展是开放的，但是对修改是封闭的”。
1. 在第一种方式下，未来每次要添加对新类型的转换，都要重新修改类型 T 的实现
2. 而第二种方式，我们只需要添加一个对于数据转换 trait 的新实现即可。
~~~

~~~admonish info title='Rust 提供了两套不同的 trait' collapsible=true
1. 值类型到值类型的转换：From / Into / TryFrom / TryInto
2. 引用类型到引用类型的转换：AsRef / AsMut
~~~

### From/Into: 值到值

~~~admonish info title='这两种方式是等价的，怎么选呢？' collapsible=true

```rust, editable
let s = String::from("Hello world!");
let s: String = "Hello world!".into();
```
---
这两种方式是等价的，怎么选呢？
1. From 可以根据上下文做类型推导，使用场景更多；
2. 而且因为实现了 From 会自动实现 Into，反之不会。
3. 所以需要的时候，不要去实现 Into，只要实现 From 就好了。
~~~

~~~admonish info title='From 和 Into 还是自反的' collapsible=true
把类型 T 的值转换成类型 T，会直接返回。这是因为标准库有如下的实现：
```rust, editable

// From（以及 Into）是自反的
impl<T> From<T> for T {
    fn from(t: T) -> T {
        t
    }
}
```
~~~

~~~admonish info title='有了 From 和 Into，很多函数的接口就可以变得灵活' collapsible=true

```rust, editable

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

fn main() {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();
    
    // IPAddr 实现了 From<[u8; 4]，转换 IPv4 地址
    print([1, 1, 1, 1]);
    // IPAddr 实现了 From<[u16; 8]，转换 IPv6 地址
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    // IPAddr 实现了 From<Ipv4Addr>
    print(v4);
    // IPAddr 实现了 From<Ipv6Addr>
    print(v6);
}
```
---
函数如果接受一个 IpAddr 为参数，我们可以使用 Into 让更多的类型可以被这个函数使用
所以，合理地使用 From / Into，可以让代码变得简洁，符合 Rust 可读性强的风格，更符合开闭原则。

~~~

### TryFrom/TryInto: 值到值，可能出现错误

> 注意，如果你的数据类型在转换过程中有可能出现错误，可以使用 TryFrom 和 TryInto.
> 它们的用法和 From / Into 一样，只是 trait 内多了一个关联类型 Error，且返回的结果是 Result。

### AsRef/AsMut: 引用到引用

~~~admonish info title='AsRef/AsMut定义' collapsible=true

```rust, editable

pub trait AsRef<T> where T: ?Sized {
    fn as_ref(&self) -> &T;
}

pub trait AsMut<T> where T: ?Sized {
    fn as_mut(&mut self) -> &mut T;
}
```
---
在 trait 的定义上，都允许 T 使用大小可变的类型，如 str、[u8] 等。
AsMut 除了使用可变引用生成可变引用外，其它都和 AsRef 一样，所以我们重点看 AsRef

~~~

~~~admonish info title='体验一下 AsRef 的使用和实现' collapsible=true

```rust, editable

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

fn main() {
    let lang = Language::Rust;
    // &str 实现了 AsRef<str>
    print_ref("Hello world!");
    // String 实现了 AsRef<str>
    print_ref("Hello world!".to_string());
    // 我们自己定义的 enum 也实现了 AsRef<str>
    print_ref(lang);
}
```
---
~~~

~~~admonish info title='v.as_ref().clone()' collapsible=true
额外说明一下的是:
如果代码出现 v.as_ref().clone() 这样的语句，也就是说你要对 v 进行引用转换，然后又得到了拥有所有权的值，那么应该实现 From，然后做 v.into()。
~~~

## 操作符相关: Deref/DerefMut

~~~admonish info title='Deref/DerefMut定义及说明' collapsible=true

```rust, editable

pub trait Deref {
    // 解引用出来的结果类型
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```
---
可以看到，DerefMut “继承”了 Deref，只是它额外提供了一个 deref_mut 方法，用来获取可变的解引用。所以这里重点学习 Deref。
~~~

~~~admonish info title='对于普通的引用，解引用很直观' collapsible=true
> 对于普通的引用，解引用很直观，因为它只有一个指向值的地址，从这个地址可以获取到所需要的值

```rust, editable

let mut x = 42;
let y = &mut x;
// 解引用，内部调用 DerefMut（其实现就是 *self）
*y += 1;
```
~~~

~~~admonish info title='智能指针来说，拿什么域来解引用就不那么直观, 看看Rc如何实现Deref' collapsible=true

```rust, editable

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}
```

---

可以看到:
1. 它最终指向了堆上的 RcBox 内部的 value 的地址
2. 然后如果对其解引用的话，得到了 value 对应的值。
3. 以下图为例，最终打印出 v = 1。
4. 从图中还可以看到，Deref 和 DerefMut 是自动调用的，*b 会被展开为 *(b.deref())。

---

![RcBox解引用](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/14%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E6%9C%89%E5%93%AA%E4%BA%9B%E5%BF%85%E9%A1%BB%E6%8E%8C%E6%8F%A1%E7%9A%84trait%EF%BC%9F-4696029.jpg)
~~~

~~~admonish info title='在 Rust 里，绝大多数智能指针都实现了 Deref，我们也可以为自己的数据结构实现 Deref' collapsible=true

```rust, editable

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut buf = Buffer::new([1, 3, 2, 4]);
    // 因为实现了 Deref 和 DerefMut，这里 buf 可以直接访问 Vec<T> 的方法
    // 下面这句相当于：(&mut buf).deref_mut().sort()，也就是 (&mut buf.0).sort()
    buf.sort();
    println!("buf: {:?}", buf);
}
```
---
但是在这个例子里，数据结构 Buffer 包裹住了 Vec，但这样一来，原本 Vec 实现了的很多方法，现在使用起来就很不方便，需要用 buf.0 来访问。怎么办？
可以实现 Deref 和 DerefMut，这样在解引用的时候，直接访问到 buf.0，省去了代码的啰嗦和数据结构内部字段的隐藏。

~~~

~~~admonish info title='编译器默认强制做解引用' collapsible=true
在上面代码里，还有一个值得注意的地方：
写 buf.sort() 的时候，并没有做解引用的操作，为什么会相当于访问了 buf.0.sort() 呢？
这是因为 sort() 方法第一个参数是 &mut self，此时 Rust 编译器会强制做 Deref/DerefMut 的解引用，所以这相当于 (*(&mut buf)).sort()。
~~~

## 其他：Debug/Display/Default

~~~admonish info title='Debug/Display/Defalut定义' collapsible=true

```rust, editable

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```
---
可以看到，Debug 和 Display 两个 trait 的签名一样，都接受一个 &self 和一个 &mut Formatter。

那为什么要有两个一样的 trait 呢？

这是因为:
1. Debug 是为开发者调试打印数据结构所设计的
2. 而 Display 是给用户显示数据结构所设计的
3. 这也是为什么 Debug trait 的实现可以通过派生宏直接生成，而 Display 必须手工实现(手工决定，自定义如何展现给用户)。
4. 在使用的时候，Debug 用 {:?} 来打印，Display 用 {} 打印。

---

```rust, editable

pub trait Default {
    fn default() -> Self;
}
```

---

Default trait 用于为类型提供缺省值:
- 它也可以通过 derive 宏 #[derive(Default)] 来生成实现，前提是类型中的每个字段都实现了 Default trait
- 在初始化一个数据结构时，我们可以部分初始化，然后剩余的部分使用 Default::default()。
~~~

~~~admonish info title='Debug/Display/Default统一使用例子' collapsible=true

```rust, editable

use std::fmt;
// struct 可以 derive Default，但我们需要所有字段都实现了 Default
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

// enum 不能 derive Default
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

// 手工实现 Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        // 用 ..Default::default() 为剩余字段使用缺省值
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

fn main() {
    // 使用 T::default()
    let dev1 = Developer::default();
    // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();
    // 使用 T::new
    let dev3 = Developer::new("Tyr");
    println!("dev1: {}\\ndev2: {}\\ndev3: {:?}", dev1, dev2, dev3);
}
```
~~~
