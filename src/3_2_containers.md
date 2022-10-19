# 二、集合容器

<!--ts-->

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

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Oct 19 03:20:17 UTC 2022 -->

<!--te-->

## 对容器进行定义

~~~admonish tip title="容器数据结构如何理解" collapsible=true
提到容器，很可能你首先会想到的就是数组、列表这些可以遍历的容器，但其实只要把某 种特定的数据封装在某个数据结构中，这个数据结构就是一个容器。比如 Option<T>，它 是一个包裹了 T 存在或不存在的容器，而 Cow 是一个封装了内部数据 B 或被借用或拥有 所有权的容器。
~~~

## 对集合容器进行定义

~~~admonish tip title="把拥有相同类型对数据放在一起，统一处理" collapsible=true
集合容器，顾名思义，就是把一系列拥有相同类型的数据放在一起，统一处理，比如：

1. 我们熟悉的字符串 String、数组 [T; n]、列表 Vec<T> 和哈希表 HashMap<K, V> 等；

2. 虽然到处在使用，但还并不熟悉的切片 slice；

3. 在其他语言中使用过，但在 Rust 中还没有用过的循环缓冲区 VecDeque<T>、双向列 表 LinkedList<T> 等。

> 这些集合容器有很多共性，比如可以被遍历、可以进行 map-reduce 操作、可以从一种类 型转换成另一种类型等等。
~~~


