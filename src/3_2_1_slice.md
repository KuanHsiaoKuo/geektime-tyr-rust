# 切片

<!--ts-->
* [切片](#切片)
   * [切片到底是什么](#切片到底是什么)
      * [切片的三个特点](#切片的三个特点)
      * [切片的三种访问方式](#切片的三种访问方式)
      * [切片与数据的关系](#切片与数据的关系)
      * [常用切片对比图](#常用切片对比图)
   * [array vs vector](#array-vs-vector)
   * [Vec&lt;T&gt; 和 &amp;[T]](#vect-和-t)
   * [解引用：转为切片类型](#解引用转为切片类型)
   * [切片和迭代器 Iterator](#切片和迭代器-iterator)
   * [特殊的切片：&amp;str](#特殊的切片str)
   * [Box&lt;[T]&gt;](#boxt)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Oct 20 02:05:33 UTC 2022 -->

<!--te-->

## 切片到底是什么

### 切片的三个特点

~~~admonish tip title="1. 同一类型、长度不确定的、在内存中连续存放的数据结构" collapsible=true
在 Rust 里，切片是描述一组属于同一类型、长度不确定的、在内存中连续存放的数据结 构，用 [T] 来表述。因为长度不确定，所以切片是个 DST（Dynamically Sized Type）。
~~~

### 切片的三种访问方式

~~~admonish tip title="2. 切片不能直接访问，需要通过引用访问。有三种访问方式：" collapsible=true
切片一般只出现在数据结构的定义中，不能直接访问，在使用中主要用以下形式：

- &[T]：表示一个只读的切片引用。

- &mut [T]：表示一个可写的切片引用。

- Box<[T]>：一个在堆上分配的切片。

~~~

### 切片与数据的关系

> 怎么理解切片呢？我打个比方，切片之于具体的数据结构，就像数据库中的视图之于表。
> 你可以把它看成一种工具，让我们可以统一访问行为相同、结构类似但有些许差异的类 型。

~~~admonish tip title="3. 举例切片与数据的关系" collapsible=true

```rust, editable
{{#include ../geektime_rust_codes/16_data_structure/src/slice1.rs}}
```
1. 对于 array 和 vector，虽然是不同的数据结构，一个放在栈上，一个放在堆上，但它们的 切片是类似的；
2. 而且对于相同内容数据的相同切片，比如 &arr[1…3] 和 &vec[1…3]，这 两者是等价的。
3. 除此之外，切片和对应的数据结构也可以直接比较，这是因为它们之间实 现了 PartialEq trait
~~~

~~~admonish tip title="3.1 切片与数据的关系图" collapsible=true
![切片与具体数据的关系](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4785008.jpg)
~~~

### 常用切片对比图

下图描述了切片和数组 [T;n]、列表 Vec<T>、切片引用 &[T] /&mut [T]，以及在堆上分配的切片 Box<[T]> 之间的关系。
> 建议花些时间理解这张图，也可以用相同的方式去总结学到的其他有关联的数据结构。

~~~admonish info title="&str、[T;n]、Vec<T>、&[T]、&mut[T]的区别与联系图 " collapsible=true
![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867546.jpg)
~~~

## array vs vector

~~~admonish info title="接前面的例子，再次说明array和vector的区别与联系" collapsible=true
对于 array 和 vector，虽然是不同的数据结构：
- 一个放在栈上
- 一个放在堆上

> 但它们的切片是类似的, 而且对于相同内容数据的相同切片
- 比如 &arr[1…3] 和 &vec[1…3]，这两者是等价的。
- 除此之外，切片和对应的数据结构也可以直接比较，这是因为它们之间实现了 PartialEq trait（源码参考资料）。
> 下图比较清晰地呈现了切片和数据之间的关系：
![切片和数据之间的关系](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4866674.jpg)
~~~

## Vec\<T\> 和 \&\[T\]

~~~admonish tip title="&[T]与&Vec[T]关系" collapsible=true
![二者关系图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4785147.jpg)
~~~

## 解引用：转为切片类型

> 在使用的时候，支持切片的具体数据类型，你可以根据需要，解引用转换成切片类型。

- 比如 Vec<T> 和 [T; n] 会转化成为 &[T]，这是因为 Vec<T> 实现了 Deref trait，而 array 内建了到 &[T] 的解引用。

~~~admonish info title="Deref、&、AsRef：我们可以写一段代码验证这一行为（代码）" collapsible=true
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
~~~

> 这也就意味着，通过解引用，这几个和切片有关的数据结构都会获得切片的所有能力，包括下列丰富的功能：

- binary_search
- chunks
- concat
- contains
- start_with
- end_with
- group_by
- iter
- join
- sort
- split
- swap

## 切片和迭代器 Iterator

~~~admonish info title="迭代器可以说是切片的孪生兄弟" collapsible=true
迭代器可以说是切片的孪生兄弟:
- 切片是集合数据的视图
- 而迭代器定义了对集合数据的各种各样的访问操作。
~~~

~~~admonish info title="Iterator trait 有大量的方法，但绝大多数情况下，只需要定义它的关联类型 Item 和 next() 方法。" collapsible=true
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

~~~admonish info title="一个例子：对切片类型Vec<T> 使用 iter() 方法，并进行各种 map / filter / take 操作" collapsible=true

> 在函数式编程语言中，这样的写法很常见，代码的可读性很强。Rust 也支持这种写法（代码）：

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

~~~admonish question title="Rust的迭代器是个懒接口，这是如何实现的？" collapsible=true
需要注意的是 Rust 下的迭代器是个懒接口（lazy interface）:
1. 也就是说这段代码直到运行到 collect 时才真正开始执行
2. 之前的部分不过是在不断地生成新的结构，来累积处理逻辑而已。

你可能好奇，这是怎么做到的呢？

原来，Iterator 大部分方法都返回一个实现了 Iterator 的数据结构，所以可以这样一路链式下去.

在 Rust 标准库中，这些数据结构被称为 [Iterator Adapter](https://doc.rust-lang.org/src/core/iter/adapters/mod.rs.html)。

比如上面的 map 方法，它返回 Map 结构，而 Map 结构实现了 [Iterator（源码）](https://doc.rust-lang.org/src/core/iter/adapters/map.rs.html#93-133)。

整个过程是这样的（链接均为源码资料）：


1. 在 collect() 执行的时候，它[实际试图使用 FromIterator 从迭代器中构建一个集合类型](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#1744-1749)，这会不断调用 next() 获取下一个数据；
2. 此时的 Iterator 是 Take，Take 调自己的 next()，也就是它会[调用 Filter 的 next()](https://doc.rust-lang.org/src/core/iter/adapters/take.rs.html#34-41)；
3. Filter 的 next() 实际上[调用自己内部的 iter 的 find()](https://time.geekbang.org/column/article/422975)，此时内部的 iter 是 Map，find() 会使用 [try_fold()](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2312-2325)，它会[继续调用 next()](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2382-2406)，也就是 Map 的 next()；
4. Map 的 next() 会[调用其内部的 iter 取 next() 然后执行 map 函数](https://time.geekbang.org/column/article/422975)。而此时内部的 iter 来自 Vec<i32>。

所以，只有在 collect() 时，才触发代码一层层调用下去，并且调用会根据需要随时结束。这段代码中我们使用了 take(1)，整个调用链循环一次，就能满足 take(1) 以及所有中间过程的要求，所以它只会循环一次。
~~~

~~~admonish question title="Rust的函数式编程写法性能如何？" collapsible=true
你可能会有疑惑：这种函数式编程的写法，代码是漂亮了，然而这么多无谓的函数调用，性能肯定很差吧？毕竟，函数式编程语言的一大恶名就是性能差。

这个你完全不用担心， Rust 大量使用了 inline 等优化技巧，这样非常清晰友好的表达方式，性能和 C 语言的 for 循环差别不大。
~~~

~~~admonish info title="与Python类似，Rust的iterator除了标准库，还有itertools提供更多功能" collapsible=true
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

在实际开发中，我们可能从一组 Future 中汇聚出一组结果，里面有成功执行的结果，也有失败的错误信息。

如果想对成功的结果进一步做 filter/map，那么标准库就无法帮忙了，就需要用 itertools 里的 filter_map_ok()。
~~~

## 特殊的切片：&str

~~~admonish info title="String、&String和&str的区别与联系" collapsible=true
我们来看一种特殊的切片：&str。之前讲过，String 是一个特殊的 Vec<u8>，所以在 String 上做切片，也是一个特殊的结构 &str。

对于 String、&String、&str，很多人也经常分不清它们的区别.

在&str。对于 &String 和 &str，如果你理解了上文中 &Vec<T> 和 &[T] 的区别，那么它们也是一样的：

![&String和&str](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867212.jpg)
~~~

~~~admonish info title="String在解引用时会转换成&str" collapsible=true
可以用下面的代码验证（代码）：
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

~~~admonish question title="字符的列表array和字符串String有什么关系和区别" collapsible=true
我们直接写一段代码来看看：

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

> 可以看到，字符列表可以通过迭代器转换成 String，String 也可以通过 chars() 函数转换成字符列表，如果不转换，二者不能比较。
~~~

~~~admonish info title="下图把数组、列表、字符串以及它们的切片放在一起比较，可以更好地理解它们的区别：" collapsible=true
![数组、列表、字符串和各自的切片](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F-4867332.jpg)
~~~

## Box\<\[T\]\>

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

~~~admonish question title="那么如何产生 Box<[T]> 呢？" collapsible=true

> 目前可用的接口就只有一个：从已有的 Vec<T> 中转换。

我们看代码：

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
> 运行代码可以看到:
1. Vec<T> 可以通过 into_boxed_slice() 转换成 Box<[T]>
2. Box<[T]> 也可以通过 into_vec() 转换回 Vec<T>。

> 这两个转换都是很轻量的转换，只是变换一下结构，不涉及数据的拷贝。

区别是:
1. 当 Vec<T> 转换成 Box<[T]> 时，没有使用到的容量就会被丢弃，所以整体占用的内存可能会降低。
2. 而且 Box<[T]> 有一个很好的特性是，不像 Box<[T;n]> 那样在编译时就要确定大小，它可以在运行期生成，以后大小不会再改变。

所以，当我们需要在堆上创建固定大小的集合数据，且不希望自动增长，那么，可以先创建 Vec<T>，再转换成 Box<[T]>。
> tokio 在提供 broadcast channel 时，就使用了 Box<[T]> 这个特性，你感兴趣的话，可以自己看看[源码](https://github.com/tokio-rs/tokio/blob/master/tokio/src/sync/broadcast.rs#L447)。
~~~


