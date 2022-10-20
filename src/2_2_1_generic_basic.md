# 泛型基础认识

<!--ts-->
<!--te-->

## 泛型就像定义函数

```extended-markdown-table
|          | Define                                                                    | Usage                                                                             |
|----------|---------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
| Function | Extract args from **duplicate code**, to make it more universal           | Get **different results** via different args when calling functions               |
|----------|---------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
| Generics | Extract args from **duplicate data structure**, to make it more universal | Get **different specific data structures** via different args when using generics |
```

## 泛型基本使用示例

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
---
Vec有两个参数:
1. 一个是 T，是列表里的每个数据的类型
2. 另一个是 A，它有进一步的限制 A: Allocator 
> 也就是说 A 需要满足 Allocator trait。
> A 这个参数有默认值 Global，它是 Rust 默认的全局分配器
3. 这也是为什么 Vec 虽然有两个参数，使用时都只需要用 T。
~~~

### Generic Cow<T>

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

这里对 B 的三个约束分别是：

1. 生命周期 'a

~~~admonish info title='Cow就像Option' collapsible=true
Cow（Clone-on-Write）是 Rust 中一个很有意思且很重要的数据结构。
它就像 Option 一样，在返回数据的时候，提供了一种可能：
- 要么返回一个借用的数据（只读）
- 要么返回一个拥有所有权的数据（可写）。
~~~

2. 长度可变 ?Sized

~~~admonish info title='?Sized代表可变大小' collapsible=true
?Sized 是一种特殊的约束写法:
- ? 代表可以放松问号之后的约束
- 由于 Rust 默认的泛型参数都需要是 Sized，也就是固定大小的类型，所以这里 ?Sized 代表用可变大小的类型。
~~~

3. 符合 ToOwned trait

~~~admonish info title='ToOwned: 可以把借用的数据克隆出一个拥有所有权的数据' collapsible=true
[ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) 是一个 trait，它可以把借用的数据克隆出一个拥有所有权的数据。
~~~

~~~admonish info title='\<B as ToOwned\>::Owned' collapsible=true
1. 它对 B 做了一个强制类型转换，转成 ToOwned trait
2. 然后访问 ToOwned trait 内部的 Owned 类型
> 因为在 Rust 里，子类型可以强制转换成父类型，B 可以用 ToOwned 约束，所以它是 ToOwned trait 的子类型，因而 B 可以安全地强制转换成 ToOwned。这里 B as ToOwned 是成立的。
~~~


