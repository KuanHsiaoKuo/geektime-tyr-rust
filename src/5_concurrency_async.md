# V 并发与异步

<!--ts-->
* [V 并发与异步](#v-并发与异步)
   * [区分并发与并行](#区分并发与并行)
   * [并发原语与异步的关系](#并发原语与异步的关系)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Oct 24 03:22:20 UTC 2022 -->

<!--te-->

## 区分并发与并行

~~~admonish info title="再次区分并发与并行" collapsible=true
很多人分不清并发和并行的概念，Rob Pike，Golang 的创始人之一，对此有很精辟很直观的解释：

Concurrency is about dealing with lots of things at once. Parallelism is about doing lots of things at once.

- 并发是一种同时处理很多事情的能力
- 并行是一种同时执行很多事情的手段。

我们把要做的事情放在多个线程中，或者多个异步任务中处理，这是并发的能力。在多核多 CPU 的机器上同时运行这些线程或者异步任务，是并行的手段。可以说，并发是为并行赋能。当我们具备了并发的能力，并行就是水到渠成的事情。
~~~

## 并发原语与异步的关系

~~~admonish info title="区别并发原语与Future" collapsible=true
- 并发原语是并发任务之间同步的手段
-  Future 以及在更高层次上处理 Future 的 async/await，是产生和运行并发任务的手段。

不过产生和运行并发任务的手段有很多，async/await 只是其中之一。

1. 在一个分布式系统中，并发任务可以运行在系统的某个节点上；
2. 在某个节点上，并发任务又可以运行在多个进程中；
3. 而在某个进程中，并发任务可以运行在多个线程中；
4. 在某个（些）线程上，并发任务可以运行在多个 Promise / Future / Goroutine / Erlang process 这样的协程上。

它们的粒度从大到小如图所示：

![38｜异步处理：Future是什么？它和asyncawait是什么关系？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/38%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9AFuture%E6%98%AF%E4%BB%80%E4%B9%88%EF%BC%9F%E5%AE%83%E5%92%8Casyncawait%E6%98%AF%E4%BB%80%E4%B9%88%E5%85%B3%E7%B3%BB%EF%BC%9F-4959318.jpg)

~~~


