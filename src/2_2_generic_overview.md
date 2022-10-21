# 泛型概览

<!--ts-->
* [泛型概览](#泛型概览)
   * [泛型实现方式](#泛型实现方式)
   * [泛型就像定义函数](#泛型就像定义函数)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 21 14:41:26 UTC 2022 -->

<!--te-->

## 泛型实现方式

> 阅读这张图的内容，可以理出不少设计。

比如trait object和trait bound其实从一开始就是不同路线的解决方案

~~~admonish info title='不同语言实现泛型的方式' collapsible=true
![不同语言实现泛型的方式](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/12%EF%BD%9C%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%EF%BC%9ARust%E7%9A%84%E7%B1%BB%E5%9E%8B%E7%B3%BB%E7%BB%9F%E6%9C%89%E4%BB%80%E4%B9%88%E7%89%B9%E7%82%B9%EF%BC%9F.png)
~~~

## 泛型就像定义函数

```extended-markdown-table
|          | Define                                                                    | Usage                                                                             |
|----------|---------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
| Function | Extract args from **duplicate code**, to make it more universal           | Get **different results** via different args when calling functions               |
|----------|---------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
| Generics | Extract args from **duplicate data structure**, to make it more universal | Get **different specific data structures** via different args when using generics |
```