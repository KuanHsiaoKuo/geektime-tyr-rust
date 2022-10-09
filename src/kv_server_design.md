# KV Server设计与实现

<!--ts-->
* [KV Server设计与实现](#kv-server设计与实现)
* [整体回顾](#整体回顾)
   * [考虑提供日志配置](#考虑提供日志配置)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct  9 07:40:22 UTC 2022 -->

<!--te-->

# 整体回顾

我们的 KV server 之旅就到此为止了。在整整 7 堂课里，我们一点点从零构造了一个完整的 KV server，包括注释在内，撰写了近三千行代码：

~~~admonish note title="使用tokei检查代码行数 " collapsible=true
```shell

❯ tokei .
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Makefile                1           24           16            1            7
 Markdown                1            7            7            0            0
 Protocol Buffers        1          119           79           23           17
 Rust                   25         3366         2730          145          491
 TOML                    2          268          107          142           19
-------------------------------------------------------------------------------
 Total                  30         3784         2939          311          534
-------------------------------------------------------------------------------
```
~~~

~~~admonish info title=" 在这个系列里: " collapsible=true
1. 我们大量使用 trait 和泛型，构建了很多复杂的数据结构；
2. 还为自己的类型实现了 AsyncRead / AsyncWrite / Stream / Sink 这些比较高阶的 trait
3. 通过良好的设计，我们把网络层和业务层划分地非常清晰，网络层的变化不会影响到业务层，反之亦然：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/53f5e5cf68b4300c3231885b10c784f3.jpeg)

4. 我们还模拟了比较真实的开发场景，通过大的需求变更，引发了一次不小的代码重构。

5. 最终，通过性能测试，发现了一个客户端实现的小 bug。在处理这个 bug 的时候，我们欣喜地看到，Rust 有着非常强大的测试工具链，除了我们使用的单元测试、集成测试、性能测试，Rust 还支持模糊测试（fuzzy testing）和基于特性的测试（property testing）。

6. 对于测试过程中发现的问题，Rust 有着非常完善的 tracing 工具链，可以和整个 opentelemetry 生态系统（包括 jaeger、prometheus 等工具）打通。我们就是通过使用 jaeger 找到并解决了问题。除此之外，Rust tracing 工具链还支持生成 [flamegraph](https://github.com/tokio-rs/tracing/tree/master/tracing-flame)，篇幅关系，没有演示，你感兴趣的话可以试试。
~~~

希望通过这个系列，你对如何使用 Rust 的特性来构造应用程序有了深度的认识。

> 我相信，如果你能够跟得上这个系列的节奏，另外如果遇到新的库，用阅读代码的方式快速掌握，那么，大部分 Rust 开发中的挑战，对你而言都不是难事。

## 考虑提供日志配置

我们目前并未对日志做任何配置。一般来说，怎么做日志，会有相应的开关以及日志级别，如果希望能通过如下的配置记录日志，该怎么做？试试看：

```toml
[log]
enable_log_file = true
enable_jaeger = false
log_level = 'info'
path = '/tmp/kv-log'
rotation = 'Daily'
```