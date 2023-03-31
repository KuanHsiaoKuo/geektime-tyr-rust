# 自定义智能指针

<!--ts-->
* [自定义智能指针](#自定义智能指针)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Mar 30 03:25:11 UTC 2023 -->

<!--te-->

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
