# 基于网络协议

<!--ts-->
* [基于网络协议](#基于网络协议)
   * [回顾网络协议](#回顾网络协议)
   * [Rust生态对网络协议的支持](#rust生态对网络协议的支持)
   * [std::net](#stdnet)
      * [服务端：TcpListener](#服务端tcplistener)
      * [客户端：TcpStream](#客户端tcpstream)
   * [处理网络连接的一般方法](#处理网络连接的一般方法)
      * [问题一：如何处理大量连接？](#问题一如何处理大量连接)
      * [问题二：如何处理共享信息？](#问题二如何处理共享信息)
   * [处理网络数据的一般方法](#处理网络数据的一般方法)
      * [JSON序列化](#json序列化)
      * [使用 protobuf 自定义协议](#使用-protobuf-自定义协议)
         * [如何界定一个消息帧（frame）](#如何界定一个消息帧frame)
         * [tokio/tokio_util](#tokiotokio_util)
   * [Tokio](#tokio)
   * [尝试改写](#尝试改写)
   * [总结：如何用 Rust 做基于 TCP 的网络开发](#总结如何用-rust-做基于-tcp-的网络开发)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Mar 29 06:23:10 UTC 2023 -->

<!--te-->

在互联网时代，谈到网络开发，我们想到的首先是 Web 开发以及涉及的**部分** HTTP 协议和 WebSocket 协议。

~~~admonish question title=" 为什么说部分？ " collapsible=true
> 之所以说部分，是因为很多协议考虑到的部分，比如[**更新时的并发控制**](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/If-Match)，大多数 Web 开发者并不知道。

- 当谈论到 [**gRPC**](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md) 时，很多人就会认为这是比较神秘的“底层”协议了，其实只不过是 HTTP/2 下的一种对二进制消息格式的封装。
~~~

> 所以对于网络开发，这个非常宏大的议题，当然是不可能、也没有必要覆盖全部内容的，这里按照如下顺序进行：

1. 先简单聊聊网络开发的大全景图
2. 然后重点学习如何使用 Rust 标准库以及生态系统中的库来做网络处理，包括网络连接、网络数据处理的一些方法
3. 最后也会介绍几种典型的网络通讯模型的使用。

## 回顾网络协议

~~~admonish info title=" 先来简单回顾一下 ISO/OSI 七层模型以及对应的协议 " collapsible=true
物理层主要跟 **[PHY 芯片](https://en.wikipedia.org/wiki/Physical_layer)**有关，就不多提了：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/909ec0f611352fyy5b99f27bb2f557ef.jpg)

~~~

七层模型中：

1. **链路层和网络层**一般构建在操作系统之中，我们并不需要直接触及
2. 而**表现层和应用层**关系紧密
3. 所以在实现过程中，大部分应用程序**只关心网络层、传输层和应用层**：

~~~admonish tip title="**只关心网络层、传输层和应用层**  " collapsible=true
- 网络层目前 IPv4 和 IPv6 分庭抗礼，IPv6 还未完全对 IPv4 取而代之；
- 传输层除了对延迟非常敏感的应用（比如游戏），绝大多数应用都使用 TCP；
- 而在应用层，对用户友好，且对防火墙友好的 HTTP 协议家族：HTTP、WebSocket、HTTP/2，以及尚处在草案之中的 HTTP/3，在漫长的进化中，脱颖而出，成为应用程序主流的选择。
~~~

## Rust生态对网络协议的支持

~~~admonish note title="笔记：从std::net到tokio  " collapsible=true
![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/8ff212b28a88d697303a5fcd35174d78.jpg)

- Rust 标准库提供了 std::net，为整个 TCP/IP 协议栈的使用提供了封装。
- 然而 std::net 是同步的，所以，如果你要构建一个高性能的异步网络，可以使用 tokio。
~~~

> tokio::net 提供了和 std::net 几乎一致的封装，一旦你熟悉了 std::net，tokio::net 里的功能对你来说都并不陌生。所以先从 std::net 开始了解。

## std::net

> std::net 下提供了处理 TCP / UDP 的数据结构，以及一些辅助结构：

- TCP：TcpListener / TcpStream，处理服务器的监听以及客户端的连接

- UDP：UdpSocket，处理 UDP socket

- 其它：IpAddr 是 IPv4 和 IPv6 地址的封装；SocketAddr，表示 IP 地址 + 端口的数据结构

> 这里就主要介绍一下 TCP 的处理（TcpListener/TcpStream），顺带会使用到 IpAddr / SocketAddr。

### 服务端：TcpListener

如果要创建一个 TCP server，我们可以:

1. 绑定监听：使用 TcpListener 绑定某个端口
2. 循环处理♻️：用 loop 循环处理接收到的客户端请求
3. 接收到请求后，会得到一个 TcpStream
4. 它实现了 Read / Write trait，可以像读写文件一样，进行 socket 的读写：

~~~admonish example title="例子: 使用std::net 创建一个 TCP server ([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/listener.rs))" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/listener.rs}}
```
~~~

### 客户端：TcpStream

对于客户端，我们可以用 TcpStream::connect() 得到一个 TcpStream。

> 一旦客户端的请求被服务器接受，就可以发送或者接收数据：

~~~admonish example title="例子:  客户端使用TcpStream::connect() ([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/client.rs)) " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/client.rs}}
```
~~~

在这个例子中:

1. 客户端在连接成功后，会发送 12 个字节的 "hello world!"给服务器
2. 服务器读取并回复后，客户端会尝试接收完整的、来自服务器的 17 个字节的 “glad to meet you!”。

> 但是，目前客户端和服务器都需要硬编码要接收数据的大小，这样不够灵活，后续我们会看到如何通过使用消息帧（frame）更好地处理。

~~~admonish question title=" 为什么上方的客户端代码无需显式关闭TcpStream？ " collapsible=true
从客户端的代码中可以看到，我们无需显式地关闭 TcpStream:

- 因为 TcpStream 的内部实现也处理了 **Drop trait**，使得其离开作用域时会被关闭。

- 但如果你去看 **[TcpStream 的文档](https://doc.rust-lang.org/std/net/struct.TcpStream.html)**，会发现它并没有实现 Drop。

这是因为:

1. TcpStream 内部包装了 **sys_common::net::TcpStream** 
2. 然后它又包装了 Socket
3. 而 Socket 是一个平台相关的结构
4. **比如，在 Unix 下的实现是 FileDesc**
5. 然后它内部是一个 OwnedFd
6. 最终会调用 libc::close(self.fd) 来关闭 fd，也就关闭了 TcpStream。
~~~

## 处理网络连接的一般方法

如果你使用某个 Web Framework 处理 Web 流量，那么无需关心网络连接，框架会帮你打点好一切，你只需要关心某个路由或者某个 RPC 的处理逻辑就可以了。

> 但如果你要在 TCP 之上构建自己的协议，那么你需要认真考虑如何妥善处理网络连接。

~~~admonish question title="  之前的 listener 代码有什么问题？ " collapsible=true
我们在之前的 listener 代码中也看到了，在网络处理的主循环中，会不断 accept() 一个新的连接：

```rust
fn main() {
    ...
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        thread::spawn(move || {
            ...
        });
    }
}
```

> 但是，处理连接的过程，需要放在另一个线程或者另一个异步任务中进行，而不要在主循环中直接处理。
> 因为这样会阻塞主循环，使其在处理完当前的连接前，无法 accept() 新的连接。
~~~

~~~admonish question title=" 换成loop+spawn可以吗？ " collapsible=true
所以，**loop + spawn 是处理网络连接的基本方式**：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/1f6c58c3944ec6f0f5a01a3235740693.jpg)

但是使用线程处理频繁连接和退出的网络连接会有如下问题:

1. 一来会有效率上的问题

2. 二来线程间如何共享公共的数据也让人头疼
~~~

> 我们来详细看看。

### 问题一：如何处理大量连接？

~~~admonish question title=" 为什么使用线程处理大量连接的网络服务不是一个好的方式？ " collapsible=true
1. 从资源的角度，如果不断创建线程，那么当连接数一高，就容易把系统中可用的线程资源吃光。

过多的线程占用过多的内存:
- Rust 缺省的栈大小是 2M
- 10k 连接就会占用 20G 内存（当然缺省栈大小也可以**[根据需要修改](https://doc.rust-lang.org/std/thread/struct.Builder.html#method.stack_size)**）

2. 从算力的角度，太多线程在连接数据到达时，会来来回回切换线程，导致 CPU 过分忙碌，无法处理更多的连接请求。

> 所以如果使用线程，那么，在遭遇到 C10K 的瓶颈，也就是连接数到万这个级别，系统就会遭遇到资源和算力的双重瓶颈。

3. 从算力的角度：因为线程的调度是操作系统完成的，每次调度都要经历一个**[复杂的、不那么高效的 save and load 的上下文切换过程](https://en.wikipedia.org/wiki/Context_switch)**

> 所以，对于潜在的有大量连接的网络服务，使用线程不是一个好的方式。
~~~

~~~admonish question title=" 如果要突破 C10K 的瓶颈，达到 C10M，该如何处理？  " collapsible=true
如果要突破 C10K 的瓶颈，达到 C10M，我们就只能使用在用户态的协程来处理:

- 要么是类似 Erlang/Golang 那样的有栈协程（stackful coroutine）
- 要么是类似 Rust 异步处理这样的无栈协程（stackless coroutine）。

> 所以，在 Rust 下大部分处理网络相关的代码中，你会看到，很少直接有用 std::net 进行处理的，大部分都是用某个异步网络运行时，比如 tokio。
~~~

### 问题二：如何处理共享信息？

~~~admonish question title=" 服务器的一些共享的状态，比如数据库连接该如何共享？ " collapsible=true
在构建服务器时，我们总会有一些共享的状态供所有的连接使用，比如数据库连接。

> 对于这样的场景，如果共享数据不需要修改，我们可以考虑使用 Arc\<T>，如果需要修改，可以使用 Arc<RwLock\<T>>。

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/9dae3be8e76512611d54eb8875fb962d.jpg)
~~~

> 但使用锁，就意味着一旦在关键路径上需要访问被锁住的资源，整个系统的吞吐量都会受到很大的影响。

~~~admonish note title="笔记： 降低粒度 " collapsible=true
> 一种思路是，我们把锁的粒度降低，这样冲突就会减少。

比如在 kv server 中，我们把 key 哈希一下模 N，将不同的 key 分摊到 N 个 memory store 中。这样，锁的粒度就降低到之前的 1/N 了：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/fb40a748abfdbb731ea2f15a4aea590d.jpg)
~~~

~~~admonish note title="笔记：特定访问 " collapsible=true
> 另一种思路是我们改变共享资源的访问方式，使其只被一个特定的线程访问；

> 其它线程或者协程只能通过给其发消息的方式与之交互。

如果你用 Erlang / Golang，这种方式你应该不陌生，在 Rust 下，可以使用 channel 数据结构。

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/b4171bafa947d925c11087c83yyb7b44.jpg)

> Rust 下 channel，无论是标准库，还是第三方库，都有非常棒的的实现:

- 同步 channel 的有**[标准库的 mpsc:channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)** 和[第三方的 crossbeam_channel](https://docs.rs/crossbeam-channel/latest/crossbeam_channel/)
- 异步 channel 有 [**tokio 下的 mpsc:channel**](https://docs.rs/tokio/1.12.0/tokio/sync/mpsc/fn.channel.html)，以及 **[flume](https://docs.rs/flume/latest/flume/)**
~~~

## 处理网络数据的一般方法

我们再来看看如何处理网络数据。

> 大部分时候，我们可以使用已有的应用层协议来处理网络数据，比如 HTTP。

### JSON序列化

在 HTTP 协议下，基本上使用 JSON 构建 REST API / JSON API 是业界的共识，客户端和服务器也有足够好的生态系统来支持这样的处理。

你只需要使用 serde 让你定义的 Rust 数据结构具备 Serialize/Deserialize 的能力，然后用 serde_json 生成序列化后的 JSON 数据。

> Rocket 是 Rust 的一个全功能的 Web 框架，类似于 Python 的 Django。

下面是一个使用 rocket 来处理 JSON 数据的例子。

~~~admonish info title=" 首先在 Cargo.toml 中引入： " collapsible=true
```toml
rocket = { version = "0.5.0-rc.1", features = ["json"] }
```
~~~

~~~admonish info title=" 然后在 main.rs 里添加代码：([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/rocket_server.rs)) " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/rocket_server.rs}}
```
~~~

可以看到，使用 rocket，10 多行代码，我们就可以运行起一个 Web Server。

> 如果你出于性能或者其他原因，可能需要定义自己的客户端 / 服务器间的协议。那么，可以使用传统的 TLV（Type-Length-Value）来描述协议数据，或者使用更加高效简洁的 protobuf。

### 使用 protobuf 自定义协议

protobuf 是一种非常方便的定义**向后兼容协议**的工具，它不仅能使用在构建 gRPC 服务的场景，还能用在其它网络服务中。

在之前的小项目中，无论是 thumbor 的实现，还是 kv server 的实现，都用到了 protobuf。

kv server 的实现在 TCP 之上构建了基于 protobuf 的协议，支持一系列 HXXX 命令。

不过，使用 protobuf 构建协议消息的时候需要注意，因为 protobuf 生成的是**不定长消息**，所以需要在客户端和服务器之间约定好：

#### 如何界定一个消息帧（frame）

~~~admonish question title=" 常用的界定消息帧的方法有哪些？ " collapsible=true
> 常用的界定消息帧的方法有：

1. **在消息尾添加** “\r\n”以及

消息尾添加 “\r\n” 一般用于基于文本的协议，比如 HTTP 头 / POP3 / Redis 的 RESP 协议等。但对于二进制协议，更好的方式是在消息前面添加固定的长度，比如对于 protobuf 这样的二进制而言，消息中的数据可能正好出现连续的"\r\n"，如果使用 “\r\n” 作为消息的边界，就会发生紊乱，所以不可取。

2. 在**消息头添加**长度

> 不过两种方式也可以混用

比如 HTTP 协议，本身使用 “\r\n” 界定头部，但它的 body 会使用长度界定，只不过这个长度在 HTTP 头中的 Content-Length 来声明。
~~~

~~~admonish question title=" 前面说到 gRPC 使用 protobuf，那么 gRPC 是怎么界定消息帧呢？  " collapsible=true
- gRPC 使用了**[五个字节的 Length-Prefixed-Message](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md)**，其中包含一个字节的压缩标志和四个字节的消息长度。

- 这样，在处理 gRPC 消息时，我们先读取 5 个字节，取出其中的长度 N，再读取 N 个字节就得到一个完整的消息了。

因为这种处理方式很常见，所以 **[tokio 提供了 length_delimited codec](https://docs.rs/tokio-util/0.6.8/tokio_util/codec/length_delimited/index.html)**，来处理用长度隔离的消息帧，它可以和 **[tokio的Framed 结构](https://docs.rs/tokio-util/0.6.8/tokio_util/codec/struct.Framed.html)**配合使用。如果你看它的文档，会发现它除了简单支持在消息前加长度外，还支持各种各样复杂的场景。
~~~

~~~admonish question title=" 如何采用这样的方法来处理使用 protobuf 自定义的协议？  " collapsible=true
比如消息有一个固定的消息头，其中包含 3 字节长度，5 字节其它内容，LengthDelimitedCodec 处理完后，会把完整的数据给你。你也可以通过 num_skip(3) 把长度丢弃，总之非常灵活：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/9186d8aef9aea6fb760d0c2537d3e6ed.png)
~~~

#### tokio/tokio_util

下面是使用 tokio / tokio_util 撰写的服务器和客户端，你可以看到，服务器和客户端都使用了 LengthDelimitedCodec 来处理消息帧。

~~~admonish example title="例子:  服务器的代码 ([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/async_framed_server.rs))" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/async_framed_server.rs}}
```
~~~

~~~admonish example title="例子:  客户端代码 ([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/async_framed_client.rs)) " collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/async_framed_client.rs}}
```
~~~

和前面[**std::net的 TcpListener / TcpStream 代码**](#stdnet)相比:

-

双方都不需要知道对方发送的数据的长度，就可以通过 **[StreamExt trait 的 next() 接口](https://docs.rs/futures/0.3.17/futures/stream/trait.StreamExt.html)**
得到下一个消息；

- 在发送时，只需要调用 **[SinkExt trait 的 send() 接口](https://docs.rs/futures/0.3.17/futures/sink/trait.SinkExt.html)**
  发送，相应的长度就会被自动计算并添加到要发送的消息帧的开头。

当然啦，如果你想自己运行这两段代码,

~~~admonish info title="  记得在 Cargo.toml 里添加：" collapsible=true
```rust
[dependencies]
anyhow = "1"
bytes = "1"
futures = "0.3"
tokio = { version = "1", features = ["full"] }
tokio-util = { version = "0.6", features = ["codec"] }
```
~~~

这里为了代码的简便，并没有直接使用 protobuf。你可以把发送和接收到的 Bytes 里的内容视作 protobuf 序列化成的二进制（如果你想看 protobuf 的处理，可以看thumbor 和 kv server 的源代码）。

> 我们可以看到，使用 LengthDelimitedCodec，构建一个自定义协议，变得非常简单。短短二十行代码就完成了非常繁杂的工作。

## Tokio

绝大多数情况下，我们应该使用支持异步的网络开发，所以你会在各种网络相关的代码中，看到 tokio 的身影。作为 Rust 下主要的异步网络运行时，你可以多花点时间了解它的功能。

## 尝试改写

在kv server 的 examples 里，使用过[**async_prost**](https://github.com/tyrchen/async-prost)，

~~~admonish info title=" 可以尝试使用使用 [**tokio_util 下的 LengthDelimitedCodec**](https://docs.rs/tokio-util/0.6.8/tokio_util/codec/length_delimited/index.html) 来改写下方的example： " collapsible=true
```rust
use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv1::{CommandRequest, CommandResponse, Service, ServiceInner, SledDb};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service: Service<SledDb> = ServiceInner::new(SledDb::new("/tmp/kvserver"))
        .fn_before_send(|res| match res.message.as_ref() {
            "" => res.message = "altered. Original message is empty.".into(),
            s => res.message = format!("altered: {}", s),
        })
        .into();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                info!("Got a new command: {:?}", cmd);
                let res = svc.execute(cmd);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
```
~~~

## 总结：如何用 Rust 做基于 TCP 的网络开发

1. 通过 TcpListener 监听，使用 TcpStream 连接。
2. 在 *nix 操作系统层面，一个 TcpStream 背后就是一个文件描述符。
3. 值得注意的是，当我们在处理网络应用的时候，

~~~admonish question title=" 有些问题一定要正视  " collapsible=true
- 网络是不可靠的

可以使用 TCP 以及构建在 TCP 之上的协议应对网络的不可靠；

- 网络的延迟可能会非常大

我们使用队列和超时来应对网络的延时；

- 带宽是有限的

使用精简的二进制结构、压缩算法以及某些技巧（比如 HTTP 的 304）来减少带宽的使用，以及不必要的网络传输；

- 网络是非常不安全的

需要使用 TLS 或者 noise protocol 这样的安全协议来保护传输中的数据。
~~~
