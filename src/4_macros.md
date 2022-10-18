# IV 宏编程

<!--ts-->

* [IV 宏编程](#iv-宏编程)
    * [资料](#资料)
    * [宏的分类](#宏的分类)
        * [使用泳道图](#使用泳道图)
        * [表格](#表格)
        * [声明宏的缺陷，而后有了过程宏](#声明宏的缺陷而后有了过程宏)
        * [声明宏(declarative macros): macro_rules!(bang)](#声明宏declarative-macros-macro_rulesbang)
        * [过程宏：深度定制与生成代码](#过程宏深度定制与生成代码)
            * [函数宏](#函数宏)
            * [属性宏](#属性宏)
            * [派生宏](#派生宏)
    * [声明宏](#声明宏)
        * [Rust常用声明宏](#rust常用声明宏)
            * [println!](#println)
            * [writeln!](#writeln)
            * [eprintln!](#eprintln)
        * [示例](#示例)
            * [声明宏示意图](#声明宏示意图)
            * [macro_rules!定义与使用](#macro_rules定义与使用)
        * [声明宏用到的参数类型](#声明宏用到的参数类型)
    * [过程宏手工定义图](#过程宏手工定义图)
        * [Cargo.toml添加proc-macro声明](#cargotoml添加proc-macro声明)
    * [过程函数宏](#过程函数宏)
    * [过程派生宏:](#过程派生宏)
        * [常用派生宏](#常用派生宏)
            * [#[derive(Debug)]](#derivedebug)
        * [原始实现builder模式](#原始实现builder模式)
        * [派生宏思路](#派生宏思路)
        * [使用syn/quote可以不用自己定义模版](#使用synquote可以不用自己定义模版)
    * [过程属性宏: proc_macro_derive(macro_name, attributes(attr_name))](#过程属性宏-proc_macro_derivemacro_name-attributesattr_name)
        * [使用syn/quote定义属性宏](#使用synquote定义属性宏)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 17 09:56:59 UTC 2022 -->

<!--te-->

## 资料

- [宏 - Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/ch19-06-macros.html)
- [Macros - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [Macros By Example - The Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)



