# IV 宏编程

<!--ts-->

* [IV 宏编程](#iv-宏编程)
    * [宏的分类](#宏的分类)
        * [声明宏: macro_rules!](#声明宏-macro_rules)
        * [过程宏：深度定制与生成代码](#过程宏深度定制与生成代码)
            * [函数宏](#函数宏)
            * [属性宏](#属性宏)
            * [派生宏](#派生宏)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Sep 22 13:11:36 UTC 2022 -->

<!--te-->

## 宏的分类

### 声明宏: macro_rules!

> 对代码模版做简单替换
> 声明宏可以用 macro_rules! 来描述, 如果重复性的代码无法用函数来封装，那么声明宏就是一个好的选择

### 过程宏：深度定制与生成代码

- [Rust 过程宏(1): 如何硬生生解析和手写过程宏](https://www.bilibili.com/video/BV1Za411q7LQ)

> 主要以如何使用 function-like macro 在不依赖于 syn / quote 的情况下，把 Json Schema 在编译期转换成 Rust struct。主要目的是让大家熟悉基本的处理 TokenStream 的思路

- [Rust 过程宏(2): 使用 syn/quote 撰写过程宏](https://www.bilibili.com/video/BV1Fu411m7W7)

> 主要通过一个 derive Builder 宏，来展示使用 syn/quote 如何开发过程宏。

- [Rust 过程宏(3): 使用 darling 处理 attributes](https://www.bilibili.com/video/BV1dr4y1v74n)

> 做个收尾，对上一讲的 derive macro 支持 attributes。 我们可以直接解析 attributes 相关的 TokenStream，也可以使用 darling 这个很方便的库，直接把 attributes 像
> Clap/Structopts 那样收集到一个数据结构中，然后再进一步处理。

~~~admonish info title='总结'
这三讲的内容虽然简单，但足以应付大家绝大多数宏编程的需求。
其实我们现在对 syn 库的使用还只是一个皮毛，我们还没有深入
去撰写自己的数据结构去实现 Parse trait，像 DeriveInput 
那样可以直接把 TokenStream 转换成我们想要的东西。

大家感兴趣的话，可以自行去看 syn 库的文档。
~~~

#### 函数宏

看起来像函数的宏，但在编译期进行处理.
> sqlx 用函数宏来处理SQL query、tokio使用属性宏 #[tokio::main] 来引入 runtime。
> 它们可以帮助目标代码的实现逻辑变得更加简单， 但一般除非特别必要，否则并不推荐写。
> 并没有特定的使用场景

#### 属性宏

可以在其他代码块上添加属性，为代码块提供更多功能。

#### 派生宏

为 derive属性添加新的功能。这是我们平时使用最多的宏，比如 #[derive(Debug)].
> 如果你定义的 trait 别人实现起来有固定的模式可循，那么可以考虑为其构建派生宏