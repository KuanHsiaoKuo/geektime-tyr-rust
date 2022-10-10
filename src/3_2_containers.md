# 二、集合容器

<!--ts-->
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

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 10 10:34:48 UTC 2022 -->

<!--te-->

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
