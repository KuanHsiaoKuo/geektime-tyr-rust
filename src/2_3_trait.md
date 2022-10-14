# Trait

<!--ts-->
* [Trait](#trait)
   * [接口抽象 or 特设多态](#接口抽象-or-特设多态)
   * [孤儿规则](#孤儿规则)
   * [基本练习](#基本练习)
      * [支持泛型](#支持泛型)
      * [支持继承](#支持继承)
      * [Self和self](#self和self)
   * [递进练习trait使用场景](#递进练习trait使用场景)
      * [基础使用：具体类型实现](#基础使用具体类型实现)
      * [进阶使用](#进阶使用)
         * [泛型实现+trait约束](#泛型实现trait约束)
         * [trait带有泛型参数+trait约束](#trait带有泛型参数trait约束)
      * [补充使用：使用关联类型+添加Result&lt;T, E&gt;](#补充使用使用关联类型添加resultt-e)
   * [Trait Object](#trait-object)
      * [子类型多态: 动态分派](#子类型多态-动态分派)
      * [实现机理：ptr+vtable](#实现机理ptrvtable)
      * [对象安全](#对象安全)
      * [使用场景](#使用场景)
         * [在函数中使用](#在函数中使用)
         * [在函数返回值中使用](#在函数返回值中使用)
         * [在数据结构中使用](#在数据结构中使用)
   * [常用trait](#常用trait)
      * [内存相关](#内存相关)
         * [Copy](#copy)
         * [Drop](#drop)
      * [标签trait](#标签trait)
         * [Sized](#sized)
         * [Send/Sync](#sendsync)
      * [类型转换](#类型转换)
         * [From/Into: 值到值](#frominto-值到值)
         * [TryFrom/TryInto: 值到值，可能出现错误](#tryfromtryinto-值到值可能出现错误)
         * [AsRef/AsMut: 引用到引用](#asrefasmut-引用到引用)
      * [操作符相关: Deref/DerefMut](#操作符相关-derefderefmut)
      * [其他：Debug/Display/Default](#其他debugdisplaydefault)
   * [设计架构](#设计架构)
      * [顺手自然](#顺手自然)
      * [桥接](#桥接)
      * [控制反转](#控制反转)
      * [SOLID原则](#solid原则)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 14 08:06:11 UTC 2022 -->

<!--te-->






