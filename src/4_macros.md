# IV 宏编程

<!--ts-->
<!--te-->

## 宏的分类

### 声明宏: macro_rules!

> 对代码模版做简单替换
> 声明宏可以用 macro_rules! 来描述, 如果重复性的代码无法用函数来封装，那么声明宏就是一个好的选择

### 过程宏：深度定制与生成代码

- [Rust 过程宏(1)](https://www.bilibili.com/video/BV1Za411q7LQ)
- [Rust 过程宏(2)](https://www.bilibili.com/video/BV1Fu411m7W7)
- [Rust 过程宏(3)](https://www.bilibili.com/video/BV1dr4y1v74n)

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