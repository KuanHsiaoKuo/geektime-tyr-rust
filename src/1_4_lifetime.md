# 二、生命周期

<!--ts-->
* [二、生命周期](#二生命周期)
   * [动态还是静态？](#动态还是静态)
   * [如何识别生命周期](#如何识别生命周期)
      * [只有传址的参数，且多于一个，才可能需要生命周期标注](#只有传址的参数且多于一个才可能需要生命周期标注)
      * [两个小例子](#两个小例子)
      * [编译器其实会自动进行生命周期标注](#编译器其实会自动进行生命周期标注)
      * [需要生命周期标注的情况](#需要生命周期标注的情况)
      * [生命周期标注练习](#生命周期标注练习)
      * [生命周期标注的目的](#生命周期标注的目的)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 24 07:25:36 UTC 2022 -->

<!--te-->

## 动态还是静态？

~~~admonish info title='动态/静态生命周期定义与表示方式' collapsible=true
1. 静态生命周期: 'static str
- 如果一个值的生命周期贯穿整个进程的生命周期，那么我们就称这种生命周期为静态生命周期。
- 当值拥有静态生命周期，其引用也具有静态生命周期。
- 我们在表述这种引用的时候，可以用 'static 来表示。比如： &'static str 代表这是一个具有静态生命周期的字符串引用。
- 一般来说，全局变量、静态变量、字符串字面量（string literal (字面) ）等，都拥有静态生命周期。
- 堆内存，如果使用了 Box::leak 后，也具有静态生命周期。

2. 动态生命周期: 'a 、'b 或者 'hello 这样的小写字符或者字符串来表述
- 如果一个值是在某个作用域中定义的，也就是说它被创建在栈上或者堆上，那么其生命周期是动态的。
- 当这个值的作用域结束时，值的生命周期也随之结束。
- 对于动态生命周期，我们约定用 'a 、'b 或者 'hello 这样的小写字符或者字符串来表述。 
- ' 后面具体是什么名字不重要，它代表某一段动态的生命周期
- 其中， &'a str 和 &'b str 表示这两个字符串引用的生命周期可能不一致。
~~~

~~~admonish info title='动静态生命周期示意图' collapsible=true
![动静态生命周期示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F.jpg)

1. 分配在堆和栈上的内存有其各自的作用域，它们的生命周期是动态的。
2. 全局变量、静态变量、字符串字面量、代码等内容，在编译时，会被编译到可执行文件中的 BSS/Data/RoData/Text 段，然后在加载时，装入内存。
3. 因而，它们的生命周期和进程的生命周期一致，所以是静态的。
4. 所以，函数指针的生命周期也是静态的，因为函数在 Text 段中，只要进程活着，其内存一直存在。
~~~

## 如何识别生命周期

### 只有传址的参数，且多于一个，才可能需要生命周期标注

### 两个小例子

~~~admonish info title='两个小例子' collapsible=true
![识别生命周期的两个小例子](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F-4607802.jpg)
1. x 引用了在内层作用域中创建出来的变量 y。由于，变量从开始定义到其作用域结束的这段时间，是它的生命周期，所以 x 的生命周期 'a 大于 y 的生命周期 'b，当 x 引用 y 时，编译器报错。
----
2. y 和 x 处在同一个作用域下， x 引用了 y，我们可以看到 x 的生命周期 'a 和 y 的生命周期 'b 几乎同时结束，或者说 'a 小于等于 'b，所以，x 引用 y 是可行的。
~~~

### 编译器其实会自动进行生命周期标注

> 编译器希望尽可能减轻开发者的负担，其实所有使用了引用的函数，都需要生命周期的标注，只不过编译器会自动做这件事，省却了开发者的麻烦

~~~admonish info title='编译器自动进行生命周期标注' collapsible=true
1. 无标注版本
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime4.rs}}
```
----
2. 自动标注

```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime3.rs}}
```
> 返回值如何标注？是 'a 还是'b 呢？这里的冲突，编译器无能为力。
~~~

~~~admonish info title='自动标注规则' collapsible=true
1. 所有引用类型的参数都有独立的生命周期 'a 、'b 等。
2. 如果只有一个引用型输入，它的生命周期会赋给所有输出。
3. 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。
~~~

### 需要生命周期标注的情况

~~~admonish info title='missing lifetime specifier' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime.rs}}
```
1. 编译器在编译 max() 函数时，无法判断 s1、s2 和返回值的生命周期。
2. 函数本身携带的信息，就是编译器在编译时使用的全部信息。
3. 这里函数本身提供的信息就告诉编译期，生命周期不一致
~~~

~~~admonish info title='添加生命周期标注即可编译通过' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/lifetime1.rs}}
```
~~~

### 生命周期标注练习

~~~admonish info title='标注练习题' collapsible=true
```rust, editable

pub fn strtok(s: &mut &str, delimiter: char) -> &str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else { // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}
```
1. 按照编译器的规则， &mut &str 添加生命周期后变成 &'b mut &'a str
2. 这将导致返回的 '&str 无法选择一个合适的生命周期。
~~~

~~~admonish info title='标注练习题参考' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/10_lifetime/src/strtok.rs}}
```
----
![标注练习示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/10%EF%BD%9C%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%EF%BC%9A%E4%BD%A0%E5%88%9B%E5%BB%BA%E7%9A%84%E5%80%BC%E7%A9%B6%E7%AB%9F%E8%83%BD%E6%B4%BB%E5%A4%9A%E4%B9%85%EF%BC%9F-4609161.jpg)
~~~

### 生命周期标注的目的

~~~admonish info title='生命周期标注的目的是，在参数和返回值之间建立联系或者约束' collapsible=true
生命周期标注的目的是，在参数和返回值之间建立联系或者约束:
1. 调用函数时，传入的参数的生命周期需要大于等于标注的生命周期。
2. 当每个函数都添加好生命周期标注后，编译器，就可以从函数调用的上下文中分析出，在传参时，引用的生命周期，是否和函数签名中要求的生命周期匹配。
3. 如果不匹配，就违背了“引用的生命周期不能超出值的生命周期”，编译器就会报错。
~~~
