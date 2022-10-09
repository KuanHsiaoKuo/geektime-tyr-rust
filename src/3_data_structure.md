# III. 数据结构

<!--ts-->
* [III. 数据结构](#iii-数据结构)
   * [数据结构快速一览](#数据结构快速一览)
   * [分类图](#分类图)
* [一、智能指针](#一智能指针)
   * [指针还是引用](#指针还是引用)
   * [智能指针不仅是指针](#智能指针不仅是指针)
   * [Box: 在堆上分配内存](#box-在堆上分配内存)
      * [实现内存分配器](#实现内存分配器)
      * [内存如何释放](#内存如何释放)
   * [Cow&lt;'a, B&gt;： 写时拷贝](#cowa-b-写时拷贝)
      * [定义](#定义)
      * [两个trait：ToOwned、Borrowed](#两个traittoownedborrowed)
      * [ToOwned](#toowned)
      * [匹配分发](#匹配分发)
      * [Cow在需要时才进行内存分配拷贝](#cow在需要时才进行内存分配拷贝)
   * [MutexGuard： 数据加锁](#mutexguard-数据加锁)
      * [MutexGuard与String、Box、Cow&lt;'a, B&gt;的对比](#mutexguard与stringboxcowa-b的对比)
      * [使用Mutex::lock获取](#使用mutexlock获取)
      * [定义与Deref、Drop trait实现](#定义与derefdrop-trait实现)
      * [使用Mutex_MutexGuard的例子](#使用mutex_mutexguard的例子)
   * [自定义智能指针](#自定义智能指针)
* [二、集合容器](#二集合容器)
   * [对容器进行定义](#对容器进行定义)
   * [对集合容器进行定义](#对集合容器进行定义)
   * [切片](#切片)
      * [array vs vector](#array-vs-vector)
      * [Vec 和 &amp;[T]](#vec-和-t)
      * [解引用](#解引用)
      * [切片和迭代器 Iterator](#切片和迭代器-iterator)
      * [特殊的切片：&amp;str](#特殊的切片str)
      * [Box&lt;[T]&gt;](#boxt)
      * [常用切片对比图](#常用切片对比图)
   * [哈希表](#哈希表)
      * [哈希表还是列表](#哈希表还是列表)
      * [Rust 的哈希表](#rust-的哈希表)
   * [<a target="_blank" rel="noopener noreferrer nofollow" href="https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882967.jpg"><img src="https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/17%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9A%E8%BD%AF%E4%BB%B6%E7%B3%BB%E7%BB%9F%E6%A0%B8%E5%BF%83%E9%83%A8%E4%BB%B6%E5%93%88%E5%B8%8C%E8%A1%A8%EF%BC%8C%E5%86%85%E5%AD%98%E5%A6%82%E4%BD%95%E5%B8%83%E5%B1%80%EF%BC%9F-4882967.jpg" alt="17｜数据结构：软件系统核心部件哈希表，内存如何布局？" style="max-width: 100%;"></a>](#-1)
      * [HashMap 的数据结构](#hashmap-的数据结构)
      * [HashMap 的基本使用方法](#hashmap-的基本使用方法)
      * [HashMap 的内存布局](#hashmap-的内存布局)
      * [ctrl 表](#ctrl-表)
      * [哈希表重新分配与增长](#哈希表重新分配与增长)
      * [删除一个值](#删除一个值)
      * [让自定义的数据结构做 Hash key](#让自定义的数据结构做-hash-key)
      * [HashSet / BTreeMap / BTreeSet](#hashset--btreemap--btreeset)
      * [为什么 Rust 的 HashMap 要缺省采用加密安全的哈希算法？](#为什么-rust-的-hashmap-要缺省采用加密安全的哈希算法)
* [三、错误处理](#三错误处理)
   * [错误处理包含这么几部分](#错误处理包含这么几部分)
   * [错误处理的主流方法](#错误处理的主流方法)
   * [Rust 的错误处理](#rust-的错误处理)
      * [Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型。](#rust-偷师-haskell构建了对标-maybe-的-option-类型和-对标-either-的-result-类型)
      * [? 操作符](#-操作符)
      * [函数式错误处理](#函数式错误处理)
      * [panic! 和 catch_unwind](#panic-和-catch_unwind)
      * [Error trait 和错误类型的转换](#error-trait-和错误类型的转换)
* [四、闭包结构](#四闭包结构)
   * [闭包的定义](#闭包的定义)
   * [闭包本质上是什么？](#闭包本质上是什么)
   * [不同语言的闭包设计](#不同语言的闭包设计)
   * [Rust 的闭包类型](#rust-的闭包类型)
      * [FnOnce](#fnonce)
      * [怎么理解 FnOnce 的 Args 泛型参数呢？](#怎么理解-fnonce-的-args-泛型参数呢)
      * [FnMut](#fnmut)
      * [Fn](#fn)
      * [总结一下三种闭包使用的情况以及它们之间的关系](#总结一下三种闭包使用的情况以及它们之间的关系)
   * [闭包的使用场景](#闭包的使用场景)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct  9 02:50:12 UTC 2022 -->

<!--te-->

## 数据结构快速一览

~~~admonish tip title="数据结构快速一览"
> 用40分钟的时间，总结了Rust的主要数据结构的内 存布局。它能厘清"数据是如何在堆和栈上存储"的思路，在这里也推荐给你。
[Visualizing memory layout of Rust's data types - YouTube](https://www.youtube.com/watch?v=rDoqT-a6UFg)
~~~

## 分类图

> 数据结构可以看作对于类型系统的进一步整理，结构化。这其实是进一步抽象，从类型中提取出日常常用的工具并分类。

~~~admonish info title='从系统/容器/原生三个纬度分类' collapsible=false
![](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/16%EF%BD%9C%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84%EF%BC%9AVecT%E3%80%81%5BT%5D%E3%80%81Box%5BT%5D%20%EF%BC%8C%E4%BD%A0%E7%9C%9F%E7%9A%84%E4%BA%86%E8%A7%A3%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8%E4%B9%88%EF%BC%9F.jpg)
~~~




