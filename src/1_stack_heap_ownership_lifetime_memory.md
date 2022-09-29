# I. 从栈堆、所有权、生命周期开始内存管理

<!--ts-->
* [I. 从栈堆、所有权、生命周期开始内存管理](#i-从栈堆所有权生命周期开始内存管理)
* [内存](#内存)
   * [字符串内存使用图](#字符串内存使用图)
   * [栈](#栈)
      * [栈帧示意图](#栈帧示意图)
      * [考虑栈溢出](#考虑栈溢出)
   * [堆](#堆)
      * [使用堆引用共享数据](#使用堆引用共享数据)
      * [考虑堆溢出](#考虑堆溢出)
* [编程四大类基本概念](#编程四大类基本概念)
   * [1. 数据](#1-数据)
      * [值和类型](#值和类型)
      * [指针和引用](#指针和引用)
   * [2. 代码](#2-代码)
      * [函数 -&gt; 方法 -&gt; 闭包](#函数---方法---闭包)
         * [闭包示意图](#闭包示意图)
      * [接口与虚表](#接口与虚表)
   * [3. 运行方式](#3-运行方式)
      * [并发与并行](#并发与并行)
      * [同步和异步](#同步和异步)
   * [4. 编程范式](#4-编程范式)
      * [泛型编程](#泛型编程)
      * [函数式编程](#函数式编程)
      * [面向对象编程](#面向对象编程)
* [Rust内存管理概览](#rust内存管理概览)
   * [一、所有权](#一所有权)
      * [单一所有权：掌控生杀大权](#单一所有权掌控生杀大权)
      * [所有权借用](#所有权借用)
      * [多个所有者](#多个所有者)
   * [二、生命周期](#二生命周期)
   * [三、融会贯通，从创建到消亡](#三融会贯通从创建到消亡)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Sep 29 09:51:58 UTC 2022 -->

<!--te-->

# 内存

## 字符串内存使用图

![字符串内存使用图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.jpg)

## 栈

### 栈帧示意图

![栈帧示意图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444135.jpg)

### 考虑栈溢出

## 堆

### 使用堆引用共享数据

![使用堆引用共享数据](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98-4444274.jpg)

### 考虑堆溢出

![堆问题](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/01%EF%BD%9C%E5%86%85%E5%AD%98%EF%BC%9A%E5%80%BC%E6%94%BE%E5%A0%86%E4%B8%8A%E8%BF%98%E6%98%AF%E6%94%BE%E6%A0%88%E4%B8%8A%EF%BC%8C%E8%BF%99%E6%98%AF%E4%B8%80%E4%B8%AA%E9%97%AE%E9%A2%98.png)

# 编程四大类基本概念

## 1. 数据

### 值和类型

### 指针和引用

## 2. 代码

### 函数 -> 方法 -> 闭包

#### 闭包示意图

![闭包与自由变量](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5.jpg)

### 接口与虚表

![虚表](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444557.jpg)

## 3. 运行方式

### 并发与并行

![并发与并行](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444672.jpg)

### 同步和异步

## 4. 编程范式

### 泛型编程

![泛型编程更抽象，更通用](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/02%EF%BD%9C%E4%B8%B2%E8%AE%B2%EF%BC%9A%E7%BC%96%E7%A8%8B%E5%BC%80%E5%8F%91%E4%B8%AD%EF%BC%8C%E9%82%A3%E4%BA%9B%E4%BD%A0%E9%9C%80%E8%A6%81%E6%8E%8C%E6%8F%A1%E7%9A%84%E5%9F%BA%E6%9C%AC%E6%A6%82%E5%BF%B5-4444741.jpg)

### 函数式编程

### 面向对象编程

# Rust内存管理概览

## 一、所有权

### 单一所有权：掌控生杀大权

### 所有权借用

### 多个所有者

## 二、生命周期

## 三、融会贯通，从创建到消亡