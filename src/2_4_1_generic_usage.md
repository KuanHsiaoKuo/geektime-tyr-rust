# Generics: 参数多态, 编译期单态化

<!--ts-->
* [Generics: 参数多态, 编译期单态化](#generics-参数多态-编译期单态化)
   * [生命周期标注](#生命周期标注)
   * [泛型结构：struct/enum定义中](#泛型结构structenum定义中)
      * [Generic Vec](#generic-vec)
      * [Generic Cow](#generic-cow)
      * [trait impl: 在不同的实现下逐步添加约束](#trait-impl-在不同的实现下逐步添加约束)
   * [泛型参数：三种使用场景](#泛型参数三种使用场景)
      * [延迟绑定](#延迟绑定)
      * [额外类型](#额外类型)
      * [多个实现](#多个实现)
         * [简单实现](#简单实现)
         * [AsyncProstReader综合例子](#asyncprostreader综合例子)
      * [AsyncProstReader: 综合使用了泛型的三种用法，感兴趣的话你可以看源代码](#asyncprostreader-综合使用了泛型的三种用法感兴趣的话你可以看源代码)
   * [泛型函数](#泛型函数)
      * [使用泛型结构作为参数或返回值](#使用泛型结构作为参数或返回值)
      * [泛型函数编译时进行单态化](#泛型函数编译时进行单态化)
      * [返回值携带泛型参数: 选择trait object而不是trait impl](#返回值携带泛型参数-选择trait-object而不是trait-impl)
      * [复杂泛型参数处理：一步步分解](#复杂泛型参数处理一步步分解)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct 23 06:38:25 UTC 2022 -->

<!--te-->

## 生命周期标注

> 其实在 Rust 里，生命周期标注也是泛型的一部分，一个生命周期 'a 代表任意的生命周期，和 T 代表任意类型是一样的。

## 泛型结构：struct/enum定义中

### Generic Vec<T>

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
~~~

~~~admonish question title='为什么上面的例子中，Vec 虽然有两个参数，使用时都只需要用 T ？' collapsible=true
Vec有两个参数:
1. 一个是 T，是列表里的每个数据的类型
2. 另一个是 A，它有进一步的限制 A: Allocator 
- 也就是说 A 需要满足 Allocator trait。
- A 这个参数有默认值 Global，它是 Rust 默认的全局分配器, 它不需要用到T
- 这也是为什么 Vec 虽然有两个参数，使用时都只需要用 T。
~~~

### Generic Cow<T>

~~~admonish info title='Cow就像Option' collapsible=true
Cow（Clone-on-Write）是 Rust 中一个很有意思且很重要的数据结构。
它就像 Option 一样，在返回数据的时候，提供了一种可能：
- 要么返回一个借用的数据（只读）
- 要么返回一个拥有所有权的数据（可写）。
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
~~~

> 这里对 B 的三个约束分别是：

1. 生命周期 'a

~~~admonish info title="告诉编译器，Cow的生命周期也是'a" collapsible=true
- 这里 B 的生命周期是 'a，所以 B 需要满足 'a，这里和泛型约束一样，也是用  B: 'a 来表示。
- 当 Cow 内部的类型 B 生命周期为 'a 时，Cow 自己的生命周期也是 'a。
~~~

2. 长度可变 ?Sized

~~~admonish info title='?Sized代表可变大小' collapsible=true
?Sized 是一种特殊的约束写法:
- ? 代表可以放松问号之后的约束
- 由于 Rust 默认的泛型参数都需要是 Sized，也就是固定大小的类型，所以这里 ?Sized 代表用可变大小的类型。
~~~

3. where B: ToOwned

- 符合 ToOwned trait

- 这里把ToOwned放在where字句中，其实行内泛型约束的写法本身就是where子句的一个语法糖，简写。

~~~admonish info title='where子句与行内泛型约束有什么区别呢？' collapsible=true
[rust - What's the difference between `<T: Trait>` and `where T: Trait`? - Stack Overflow](https://stackoverflow.com/questions/46793494/whats-the-difference-between-t-trait-and-where-t-trait)
~~~

~~~admonish info title='ToOwned: 可以把借用的数据克隆出一个拥有所有权的数据' collapsible=true
[ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) 是一个 trait，它可以把借用的数据克隆出一个拥有所有权的数据。
~~~

4. <B as ToOwned>::Owned

~~~admonish info title='\<B as ToOwned\>::Owned' collapsible=true
1. 它对 B 做了一个强制类型转换，转成 ToOwned trait
2. 然后访问 ToOwned trait 内部的 Owned 类型
> 因为在 Rust 里，子类型可以强制转换成父类型，B 可以用 ToOwned 约束，所以它是 ToOwned trait 的子类型，因而 B 可以安全地强制转换成 ToOwned。这里 B as ToOwned 是成立的。
~~~

> [两个trait的说明：ToOwned、Borrowed](3_1_2_cow.html#两个traittoownedborrowed)

### trait impl: 在不同的实现下逐步添加约束

~~~admonish info title='逐步约束：把决策交给使用者' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/12_type_system/src/reader.rs}}
```
~~~

> 逐步添加约束，可以让约束只出现在它不得不出现的地方，这样代码的灵活性最大。

## 泛型参数：三种使用场景

### 延迟绑定

使用泛型参数延迟数据结构的绑定；

### 额外类型

使用泛型参数和 PhantomData，声明数据结构中不直接使用，但在实现过程中需要用到的类型；

~~~admonish info title='在定义数据结构时，对于额外的、暂时不需要的泛型参数，用 PhantomData 来“拥有”它们，这样可以规避编译器的报错。' collapsible=true
```rust, editable

use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let product = Product::default();

        // 两个 id 不能比较，因为他们属于不同的类型
        // assert_ne!(user.id, product.id);

        assert_eq!(user.id.inner, product.id.inner);
    }
}
```
~~~

~~~admonish info title='加深对PhantomData的理解' collapsible=true
```rust, editable

use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 plan 到 DB
    // ...
    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("Tyr".into());
        // 使用免费 feature
        customer.feature1();
        customer.feature2();
        // 用着用着觉得产品不错愿意付费
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature1();
        // 付费用户解锁了新技能
        customer.advance_feature();
    }
}
```
~~~

~~~admonish info title='在这个例子里，Customer 有个额外的类型 T:' collapsible=true

1. 通过类型 T，我们可以将用户分成不同的等级
- 比如免费用户是 Customer<FreePlan>
- 付费用户是 Customer<PersonalPlan>
- 免费用户可以转化成付费用户，解锁更多权益
2. 使用 PhantomData 处理这样的状态，可以在编译期做状态的检测，避免运行期检测的负担和潜在的错误。
~~~

### 多个实现

使用泛型参数让同一个数据结构对同一个 trait 可以拥有不同的实现。

#### 简单实现

~~~admonish question title='有时候，对于同一个 trait，我们想要有不同的实现，该怎么办？' collapsible=true
```rust, editable

use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

// 线性增长
#[derive(Debug, Default)]
pub struct Linear;

// 二次增长
#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }

        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None;
        }

        Some(self.current * self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let mut equation = Equation::<Linear>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(2), equation.next());
        assert_eq!(Some(3), equation.next());
    }

    #[test]
    fn test_quadratic() {
        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(4), equation.next());
        assert_eq!(Some(9), equation.next());
    }
}
```
~~~

#### AsyncProstReader综合例子

~~~admonish info title='来看一个真实存在的例子[AsyncProstReader](https://docs.rs/async-prost/0.2.1/src/async_prost/reader.rs.html#26-31)，它来自KV server 里用过的 [async-prost 库](https://docs.rs/async-prost/0.2.1/async_prost/)' collapsible=true
```rust, editable

/// A marker that indicates that the wrapping type is compatible with `AsyncProstReader` with Prost support.
#[derive(Debug)]
pub struct AsyncDestination;

/// a marker that indicates that the wrapper type is compatible with `AsyncProstReader` with Framed support.
#[derive(Debug)]
pub struct AsyncFrameDestination;

/// A wrapper around an async reader that produces an asynchronous stream of prost-decoded values
#[derive(Debug)]
pub struct AsyncProstReader<R, T, D> {
    reader: R,
    pub(crate) buffer: BytesMut,
    into: PhantomData<T>,
    dest: PhantomData<D>,
}
```
~~~

1. 这个数据结构使用了三个泛型参数，R,T,D.
2. 其实数据结构中真正用到的只有一个 R，它可以是一个实现了 AsyncRead 的数据结构（稍后会看到）。
3. 另外两个泛型参数 T 和 D，在**数据结构定义的时候**其实并不需要，只是**在数据结构的实现过程**中，才需要用到它们的约束。
4. 其中，T 是从 R 中读取出的数据反序列化出来的类型，在实现时用 prost::Message 约束。
5. D 是一个类型占位符，它会根据需要被具体化为 AsyncDestination 或者 AsyncFrameDestination。
6. 类型参数 D 如何使用，我们可以先想像一下:

- 实现 AsyncProstReader 的时候，我们希望在使用 AsyncDestination 时，提供一种实现
- 而在使用 AsyncFrameDestination 时，提供另一种实现

> 也就是说，这里的类型参数 D，在 impl 的时候，会被具体化成某个类型。

~~~admonish info title='拿着这个想法，来看 AsyncProstReader 在[实现 Stream](https://docs.rs/futures/0.3.17/futures/prelude/trait.Stream.html) 时，D 是如何具体化的' collapsible=true
> 这里你不用关心 Stream 具体是什么以及如何实现。
```rust, editable

impl<R, T> Stream for AsyncProstReader<R, T, AsyncDestination>
where
    T: Message + Default,
    R: AsyncRead + Unpin,
{
    type Item = Result<T, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        ...
    }
}
```
> 再看对另外一个对 D 的具体实现：

```rust, editable

impl<R, T> Stream for AsyncProstReader<R, T, AsyncFrameDestination>
where
    R: AsyncRead + Unpin,
    T: Framed + Default,
{
    type Item = Result<T, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        ...
    }
}
```
~~~

在这个例子里，除了 Stream 的实现不同外，AsyncProstReader 的其它实现都是共享的。
> 所以我们有必要为其增加一个泛型参数 D，使其可以根据不同的 D 的类型，来提供不同的 Stream 实现。

### AsyncProstReader: 综合使用了泛型的三种用法，感兴趣的话你可以看源代码

## 泛型函数

### 使用泛型结构作为参数或返回值

~~~admonish info title='在声明一个函数的时候，我们还可以不指定具体的参数或返回值的类型，而是由泛型参数来代替' collapsible=true
```rust, editable

fn id<T>(x: T) -> T {
    return x;
}

fn main() {
    let int = id(10);
    let string = id("Tyr");
    println!("{}, {}", int, string);
}
```
~~~

### 泛型函数编译时进行单态化

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

### 返回值携带泛型参数: 选择trait object而不是trait impl

~~~admonish info title='例子：使用trait object的迭代函数' collapsible=true
```rust, editable

pub trait Storage {
    ...
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> 
        Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
```
> 对于 get_iter() 方法，并不关心返回值是一个什么样的 Iterator，只要它能够允许我们不断调用 next() 方法，获得一个 Kvpair 的结构，就可以了。在实现里，使用了 trait object。
~~~

~~~admonish info title='上面的例子不能使用trait impl，因为rust还不允许' collapsible=true
```rust, editable
// 目前 trait 还不支持
fn get_iter(&self, table: &str) -> Result<impl Iterator<Item = Kvpair>, KvError>;
```

---

```rust, editable

pub trait ImplTrait {
    // 允许
    fn impl_in_args(s: impl Into<String>) -> String {
        s.into()
    }

    // 不允许
    fn impl_as_return(s: String) -> impl Into<String> {
        s
    }
}
```
~~~

~~~admonish info title='使用impl泛型参数做返回值，很难构造成功' collapsible=true
```rust, editable

// 目前 trait 还不支持
fn get_iter(&self, table: &str) -> Result<impl Iterator<Item = Kvpair>, KvError>;


// 可以正确编译
pub fn generics_as_return_working(i: u32) -> impl Iterator<Item = u32> {
    std::iter::once(i)
}

// 期待泛型类型，却返回一个具体类型
pub fn generics_as_return_not_working<T: Iterator<Item = u32>>(i: u32) -> T {
    std::iter::once(i)
}
```
~~~

~~~admonish info title='可以返回 trait object，它消除了类型的差异，把所有不同的实现 Iterator 的类型都统一到一个相同的 trait object 下' collapsible=true

```rust, editable

// 返回 trait object
pub fn trait_object_as_return_working(i: u32) -> Box<dyn Iterator<Item = u32>> {
    Box::new(std::iter::once(i))
}
```
~~~

> 使用 trait object 是有额外的代价的，首先这里有一次额外的堆分配，其次动态分派会带来一定的性能损失。

### 复杂泛型参数处理：一步步分解

~~~admonish info title='一个复杂泛型参数示例' collapsible=true
```rust, editable

pub fn comsume_iterator<F, Iter,  T>(mut f: F)
where
    F: FnMut(i32) -> Iter, 
    Iter: Iterator<Item = T>, 
    T: std::fmt::Debug, 
{
    // 根据 F 的类型，f(10) 返回 iterator，所以可以用 for 循环
    for item in f(10) {
        println!("{:?}", item); // item 实现了 Debug trait，所以可以用 {:?} 打印
    }
}
```
> 一步步分解，其实并不难理解其实质:
1. F: FnMut(i32) -> Iter: 参数 F 是一个闭包，接受 i32，返回 Iter 类型；
2. Iter: Iterator<Item = T>: 参数 Iter 是一个 Iterator，Item 是 T 类型；
3. T: std::fmt::Debug: 参数 T 是一个实现了 Debug trait 的类型。
~~~

