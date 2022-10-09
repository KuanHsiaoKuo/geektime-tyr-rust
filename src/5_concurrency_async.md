# V 并发与异步

<!--ts-->
* [V 并发与异步](#v-并发与异步)
   * [区分并发与并行](#区分并发与并行)
   * [并发的难点、工作模式和核心](#并发的难点工作模式和核心)
   * [并发原语与异步的关系](#并发原语与异步的关系)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct  9 06:00:29 UTC 2022 -->

<!--te-->

## 区分并发与并行

~~~admonish info title="再次区分并发与并行" collapsible=true
很多人分不清并发和并行的概念，Rob Pike，Golang 的创始人之一，对此有很精辟很直观的解释：

Concurrency is about dealing with lots of things at once. Parallelism is about doing lots of things at once.

- 并发是一种同时处理很多事情的能力
- 并行是一种同时执行很多事情的手段。

我们把要做的事情放在多个线程中，或者多个异步任务中处理，这是并发的能力。在多核多 CPU 的机器上同时运行这些线程或者异步任务，是并行的手段。可以说，并发是为并行赋能。当我们具备了并发的能力，并行就是水到渠成的事情。
~~~

## 并发的难点、工作模式和核心

~~~admonish info title="处理并发的难点在哪里？衍生出哪些工作模式？核心是什么" collapsible=true
其实有很多和并发相关的内容。比如用 std::thread 来创建线程、用 std::sync 下的并发原语（Mutex）来处理并发过程中的同步问题、用 Send/Sync trait 来保证并发的安全等等。

在处理并发的过程中，难点并不在于如何创建多个线程来分配工作，在于如何在这些并发的任务中进行同步。

我们来看并发状态下几种常见的工作模式：自由竞争模式、map/reduce 模式、DAG 模式：

![33｜并发处理（上）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/33%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8A%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F-4950283.jpg)

---
1. 在自由竞争模式下，多个并发任务会竞争同一个临界区的访问权。任务之间在何时、以何种方式去访问临界区，是不确定的，或者说是最为灵活的，只要在进入临界区时获得独占访问即可。
- Atomic & Mutex

2. 在自由竞争的基础上，我们可以限制并发的同步模式，典型的有 map/reduce 模式和 DAG 模式：
- map/reduce 模式，把工作打散，按照相同的处理完成后，再按照一定的顺序将结果组织起来；
- DAG 模式，把工作切成不相交的、有依赖关系的子任务，然后按依赖关系并发执行。

> 这三种基本模式组合起来，可以处理非常复杂的并发场景。所以，当我们处理复杂问题的时候，应该先厘清其脉络，用分治的思想把问题拆解成正交的子问题，然后组合合适的并发模式来处理这些子问题。
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


