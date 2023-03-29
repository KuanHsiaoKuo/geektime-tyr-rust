# 声明宏

<!--ts-->
* [声明宏](#声明宏)
   * [Rust常用声明宏](#rust常用声明宏)
      * [println!](#println)
      * [writeln!](#writeln)
      * [eprintln!](#eprintln)
   * [示例](#示例)
      * [声明宏示意图](#声明宏示意图)
      * [macro_rules!定义与使用](#macro_rules定义与使用)
   * [声明宏用到的参数类型](#声明宏用到的参数类型)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Mar 29 06:23:09 UTC 2023 -->

<!--te-->

## Rust常用声明宏

### println!

- [println in std - Rust](https://doc.rust-lang.org/std/macro.println.html)
- [Rust声明宏println剖析_一线coder的博客-CSDN博客_rust 声明宏](https://blog.csdn.net/jiangjkd/article/details/120994956)

~~~admonish info title="println!使用示例" collapsible=true
```rust
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    })
}
```
~~~

### writeln!

> 可以将内容输入到指定文件

- [writeln in std - Rust](https://doc.rust-lang.org/std/macro.writeln.html)
- [rust入门笔记---翻译rust write！宏 - 知乎](https://zhuanlan.zhihu.com/p/148945862)

```shell
cargo run --example raw_command > examples/raw_command_output.txt
```

### eprintln!

- [eprintln in std - Rust](https://doc.rust-lang.org/std/macro.eprintln.html)

## 示例

### 声明宏示意图

~~~admonish info title="声明宏定义使用流程图" collapsible=true
```plantuml
{{#include ../materials/plantumls/declarative_macros.puml}}
```
~~~

### macro_rules!定义与使用

~~~admonish info title="macro_rules!定义示例" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/rule.rs:1:20}}
```
~~~

~~~admonish info title="$($el:expr), *)" collapsible=true
1. 在声明宏中，条件捕获的参数使用 $ 开头的标识符来声明。
2. 每个参数都需要提供类型，这里`expr`代表表达式，所以 $el:expr 是说把匹配到的表达式命名为 $el。
3. $(...),* 告诉编译器可以匹配任意多个以逗号分隔的表达式，然后捕获到的每一个表达式可以用 $el 来访问。
4. 由于匹配的时候匹配到一个 $(...)* （我们可以不管分隔符），在执行的代码块中，我们也要相应地使用 $(...)* 展开。
5. 所以这句 $(v.push($el);)* 相当于匹配出多少个 $el就展开多少句 push 语句。
~~~

~~~admonish info title="macro_rules!使用示例" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/47_48_macros/examples/rule.rs:22:}}
```
~~~

## 声明宏用到的参数类型

~~~admonish info title='类型列表' collapsible=true
1. item，比如一个函数、结构体、模块等。 
2. block，代码块。比如一系列由花括号包裹的表达式和语句。 
3. stmt，语句。比如一个赋值语句。 
4. pat，模式。 
5. expr，表达式。刚才的例子使用过了。 
6. ty，类型。比如 Vec。 
7. ident，标识符。比如一个变量名。 
8. path，路径。比如：foo、::std::mem::replace、transmute::<_, int>。 
9. meta，元数据。一般是在 #[...] 和 #![...] 属性内部的数据。 
10. tt，单个的 token 树。 
11. vis，可能为空的一个 Visibility 修饰符。比如 pub、pub(crate)
~~~
