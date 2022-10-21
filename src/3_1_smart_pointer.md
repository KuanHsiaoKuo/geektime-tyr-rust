# 一、智能指针

<!--ts-->
* [一、智能指针](#一智能指针)
   * [指针还是引用](#指针还是引用)
   * [智能指针不仅是指针](#智能指针不仅是指针)
* [自定义智能指针](#自定义智能指针)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 21 11:58:31 UTC 2022 -->

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




# 自定义智能指针

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
