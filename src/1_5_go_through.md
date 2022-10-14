# 三、融会贯通，从创建到消亡

<!--ts-->
* [三、融会贯通，从创建到消亡](#三融会贯通从创建到消亡)
   * [创建](#创建)
      * [堆内存生命周期管理发展史](#堆内存生命周期管理发展史)
      * [struct/enum/vec/String创建时的内存布局](#structenumvecstring创建时的内存布局)
         * [内存布局优化什么意思？](#内存布局优化什么意思)
         * [struct](#struct)
         * [enum](#enum)
         * [vec和String](#vec和string)
         * [引用类型的内存布局](#引用类型的内存布局)
      * [更多可见cheats.rs](#更多可见cheatsrs)
   * [使用](#使用)
      * [copy和move](#copy和move)
   * [销毁](#销毁)
      * [drop释放堆内存](#drop释放堆内存)
      * [RAII释放其他资源](#raii释放其他资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 14 07:46:58 UTC 2022 -->

<!--te-->

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

#### 内存布局优化什么意思？

~~~admonish info title='内存布局优化示意图' collapsible=true
![内存布局优化示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4639867.jpg)
~~~

#### struct

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

~~~admonish info title='Rust在编译时、运行时检查调用drop' collapsible=true
![Rust在编译时、运行时检查调用drop](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/11%EF%BD%9C%E5%86%85%E5%AD%98%E7%AE%A1%E7%90%86%EF%BC%9A%E4%BB%8E%E5%88%9B%E5%BB%BA%E5%88%B0%E6%B6%88%E4%BA%A1%EF%BC%8C%E5%80%BC%E9%83%BD%E7%BB%8F%E5%8E%86%E4%BA%86%E4%BB%80%E4%B9%88%EF%BC%9F-4641604.jpg)
~~~

### RAII释放其他资源

~~~admonish info title='Rust基于RAII释放文件资源' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/11_memory/examples/raii.rs}}
```
~~~

