# 内存

<!--ts-->
* [内存](#内存)
   * [字符串内存使用图](#字符串内存使用图)
   * [栈](#栈)
      * [栈帧示意图](#栈帧示意图)
      * [考虑栈溢出](#考虑栈溢出)
   * [堆](#堆)
      * [使用堆引用共享数据](#使用堆引用共享数据)
      * [考虑堆溢出](#考虑堆溢出)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 17 09:56:57 UTC 2022 -->

<!--te-->

## 字符串内存使用图

~~~admonish info title='字符串内存使用图' collapsible=true
![字符串内存使用图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.jpg)
~~~

## 栈

### 栈帧示意图

~~~admonish info title='栈帧示意图' collapsible=true
![栈帧示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444135.jpg)
~~~

### 考虑栈溢出

## 堆

### 使用堆引用共享数据

~~~admonish info title='使用堆引用共享内存数据' collapsible=true
![使用堆引用共享数据](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444274.jpg)
~~~

### 考虑堆溢出

~~~admonish info title='堆问题示意图' collapsible=true
![堆问题](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.png)
~~~
