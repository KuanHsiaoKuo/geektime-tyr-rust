# Trait概览

<!--ts-->
* [Trait概览](#trait概览)
   * [接口抽象 or 特设多态](#接口抽象-or-特设多态)
   * [孤儿规则](#孤儿规则)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Oct 15 03:44:48 UTC 2022 -->

<!--te-->

~~~admonish info title='trait概览图' collapsible=false
![trait概览图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/13%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9A%E5%A6%82%E4%BD%95%E4%BD%BF%E7%94%A8trait%E6%9D%A5%E5%AE%9A%E4%B9%89%E6%8E%A5%E5%8F%A3%EF%BC%9F.jpg)
~~~

## 接口抽象 or 特设多态

~~~admonish info title='接口抽象：对不同类型统一实现 + trait将各种接口定义场景考虑进去' collapsible=true
trait 作为对不同数据结构中相同行为的一种抽象。
1. 延迟实现:
除了基本 trait 之外，当行为和具体的数据关联时，比如字符串解析时定义的 Parse trait，我们引入了带有关联类型的 trait，把和行为有关的数据类型的定义，进一步延迟到 trait 实现的时候。
2. 对于同一个类型的同一个 trait 行为，可以有不同的实现，比如我们之前大量使用的 From，此时可以用泛型 trait。可以说 Rust 的 trait 就像一把瑞士军刀，把需要定义接口的各种场景都考虑进去了。
~~~

~~~admonish info title='对不同类型的不同实现: 特设多态' collapsible=true
特设多态是同一种行为的不同实现。所以其实，通过定义 trait 以及为不同的类型实现这个 trait，我们就已经实现了特设多态。
1. Add trait 就是一个典型的特设多态，同样是加法操作，根据操作数据的不同进行不同的处理。
2. Service trait 是一个不那么明显的特设多态，同样是 Web 请求，对于不同的 URL，我们使用不同的代码去处理。
~~~

## 孤儿规则

~~~admonish info title='定义或实现，至少有一个' collapsible=true
trait 和实现 trait 的数据类型，至少有一个是在当前 crate 中定义的，也就是说，你不能为第三方的类型实现第三方的 trait，当你尝试这么做时，Rust 编译器会报错。
~~~