# 基于通讯模型

<!--ts-->
* [基于通讯模型](#基于通讯模型)
   * [通讯模型](#通讯模型)
      * [双向通讯](#双向通讯)
      * [请求响应](#请求响应)
      * [控制平面 / 数据平面分离](#控制平面--数据平面分离)
      * [P2P 网络](#p2p-网络)
   * [如何构建 P2P 网络](#如何构建-p2p-网络)
      * [解决网络连通](#解决网络连通)
      * [承载多个协议](#承载多个协议)
      * [网络安全解决](#网络安全解决)
   * [Rust 如何处理 P2P 网络](#rust-如何处理-p2p-网络)
      * [P2P聊天应用](#p2p聊天应用)
      * [演示](#演示)
      * [通讯模型练习题](#通讯模型练习题)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Oct 26 01:15:50 UTC 2022 -->

<!--te-->

## 通讯模型

### 双向通讯

~~~admonish note title="笔记：前面[**TCP 服务器的例子**](7_1_protocols.html#stdnet)里，所做的都是双向通讯。这是最典型的一种通讯方式：  " collapsible=true
![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/fbe99846847d7d495685eb7bd62889c0.jpg)

1. 服务端监听9527端口
2. 客户端连接9527端口
3. 一旦连接建立，服务器和客户端都可以根据需要主动向对方发起传输。
4. 服务器收到消息后就spawn一个线程处理
5. 整个网络运行在全双工模式下（full duplex）

我们熟悉的 TCP / WebSocket 就运行在这种模型下。

双向通讯的好处是：**数据的流向是没有限制的，一端不必等待另一端才能发送数据，网络可以进行比较实时地处理。**
~~~

### 请求响应

~~~admonish note title="笔记：在 Web 开发的世界里，请求 - 响应模型是我们最熟悉的模型。  " collapsible=true
客户端发送请求，服务器根据请求返回响应。

整个网络处在半双工模式下（half duplex）。HTTP/1.x 就运行在这种模式下。

- 一般而言，请求响应模式下，在客户端没有发起请求时，服务器不会也无法主动向客户端发送数据。
- 除此之外，请求发送的顺序和响应返回的顺序是一一对应的，不会也不能乱序，这种处理方式会导致**[应用层的队头阻塞（Head-Of-Line blocking）](https://en.wikipedia.org/wiki/Head-of-line_blocking)**。

请求响应模型处理起来很简单，由于 HTTP 协议的流行，尽管有很多限制，请求响应模型还是得到了非常广泛的应用。

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/3ab96bc728d939b57695ba732ab187ba.jpg)
~~~

### 控制平面 / 数据平面分离

但有时候，服务器和客户端之间会进行复杂的通讯，这些通讯包含**控制信令和数据流**。

因为 TCP 有**天然的网络层的队头阻塞**，所以当控制信令和数据交杂在同一个连接中时，过大的数据流会阻塞控制信令，使其延迟加大，无法及时响应一些重要的命令。

以 FTP 为例：

- 如果用户在传输一个 1G 的文件后，再进行 ls 命令
- 如果文件传输和 ls 命令都在同一个连接中进行，那么，只有文件传输结束，用户才会看到 ls 命令的结果，这样显然对用户非常不友好。

~~~admonish note title="> 所以，我们会采用控制平面和数据平面分离的方式，进行网络处理:" collapsible=true
1. connect()/ctrl_send/ctrl_recv:

   客户端会首先连接服务器，建立控制连接。控制连接是一个长连接，会一直存在，直到交互终止。

2. connect/data_send/data_recv:

   然后，二者会根据需要额外创建新的临时的数据连接，用于传输大容量的数据，数据连接在完成相应的工作后，会自动关闭。

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/9617389f71bbf9ef08c4955754107eb8.jpg)
~~~

~~~admonish note title="除 FTP 外，还有很多协议都是类似的处理方式:" collapsible=true
1. 比如**[多媒体通讯协议SIP 协议](https://en.wikipedia.org/wiki/Session_Initiation_Protocol)**。

2. HTTP/2 和借鉴了 HTTP/2 的用于多路复用的 Yamux 协议，虽然运行在同一个 TCP 连接之上，它们在应用层也构建了类似的控制平面和数据平面。

> 以 HTTP/2 为例

- 控制平面（ctrl stream）可以创建很多新的 stream，用于并行处理多个应用层的请求

- 比如使用 HTTP/2 的 gRPC，各个请求可以并行处理

- 不同 stream 之间的数据可以乱序返回，而不必受请求响应模型的限制

> 虽然 HTTP/2 依旧受困于 TCP 层的队头阻塞，但它解决了应用层的队头阻塞。
~~~

### P2P 网络

前面我们谈论的网络通讯模型，都是传统的客户端 / 服务器交互模型（C/S 或 B/S）:

客户端和服务器在网络中的作用是不对等的，客户端永远是连接的发起方，而服务器是连接的处理方。

> 不对等的网络模型有很多好处

比如客户端不需要公网地址，可以隐藏在网络地址转换（NAT）设备（比如 NAT 网关、防火墙）之后，只要服务器拥有公网地址，这个网络就可以连通。

所以，客户端 / 服务器模型是天然中心化的，所有连接都需要经过服务器这个中间人，即便是两个客户端的数据交互也不例外。

这种模型随着互联网的大规模使用成为了网络世界的主流。

> 然而，很多应用场景需要通讯的两端可以直接交互，而无需一个中间人代为中转。

比如 A 和 B 分享一个 1G 的文件，如果通过服务器中转，数据相当于传输了两次，效率很低。

> P2P 模型打破了这种不对等的关系，使得任意两个节点在理论上可以直接连接，每个节点既是客户端，又是服务器。

## 如何构建 P2P 网络

可是由于历史上 IPv4 地址的缺乏，以及对隐私和网络安全的担忧，互联网的运营商在接入端，大量使用了 NAT 设备，使得普通的网络用户，缺乏直接可以访问的公网 IP。

因而，构建一个 P2P 网络首先需要解决网络的连通性。

### 解决网络连通

~~~admonish note title="笔记：主流的解决方法  " collapsible=true
1. 探索

   P2P 网络的每个节点，都会首先会通过 STUN 服务器探索自己的公网 IP/port

2. 注册

   然后在 bootstrap/signaling server 上注册自己的公网 IP/port，让别人能发现自己，从而和潜在的“邻居”建立连接。

在一个大型的 P2P 网络中，一个节点常常会拥有几十个邻居，通过这些邻居以及邻居掌握的网络信息，每个节点都能构建一张如何找到某个节点（某个数据）的路由表。

在此之上，节点还可以加入某个或者某些 topic，然后通过某些协议（比如 gossip）在整个 topic 下扩散消息：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/ef8f35f961d4771729a18f69becd4274.jpg)
~~~

### 承载多个协议

P2P 网络的构建，一般要比客户端 / 服务器网络复杂，因为节点间的连接要承载很多协议：

- 节点发现（mDNS、bootstrap、Kad DHT）

- 节点路由（Kad DHT）

- 内容发现（pubsub、Kad DHT）

- 应用层协议

> 同时，连接的安全性受到的挑战也和之前不同。

~~~admonish note title="笔记：所以我们会看到，P2P 协议的连接，往往在一个 TCP 连接中，**使用类似 yamux 的多路复用协议来承载很多其他协议**  " collapsible=true
![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/765b2b7f05986c87dfa524ff9f5980f3.jpg)
~~~

### 网络安全解决

在网络安全方面，TLS 虽然能很好地保护客户端 / 服务器模型，**然而证书的创建、发放以及信任对 P2P 网络是个问题**

> 所以 P2P 网络倾向于使用自己的安全协议，或者使用 noise protocol，来构建安全等级可以媲美 TLS 1.3 的安全协议。

## Rust 如何处理 P2P 网络

> 在 Rust 下，有 **[libp2p 这个比较成熟的库](https://docs.rs/libp2p/latest/libp2p/)**来处理 P2P 网络。

### P2P聊天应用

~~~admonish example title="例子:  下面是一个简单的 P2P 聊天应用：在本地网络中通过 MDNS 做节点发现，使用 floodpub 做消息传播。 ([github](https://github.com/KuanHsiaoKuo/geektime-tyr-rust/blob/main/geektime_rust_codes/29_network/examples/p2p_chat.rs)) " collapsible=true
在关键位置都写了注释：
```rust, editable
{{#include ../geektime_rust_codes/29_network/examples/p2p_chat.rs}}
```
~~~

~~~admonish example title="要运行这段代码，你需要在 Cargo.toml 中使用 futures 和 libp2p： " collapsible=true
```toml
futures = "0.3"
libp2p = { version = "0.39",  features = ["tcp-tokio"] }
```
~~~

### 演示

~~~admonish example title="例子: 开一个窗口 A 运行：  " collapsible=true
```shell
❯ cargo run --example p2p_chat --quiet
Local peer id: PeerId("12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg")
Listening on "/ip4/127.0.0.1/tcp/51654"
// 下面的内容在新节点加入时逐渐出现
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/192.168.86.23/tcp/51656
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/127.0.0.1/tcp/51656
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/192.168.86.23/tcp/51661
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/127.0.0.1/tcp/51661
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/192.168.86.23/tcp/51670
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/127.0.0.1/tcp/51670
```
~~~

~~~admonish example title="例子:  窗口 B  " collapsible=true
```rust
❯ cargo run --example p2p_chat --quiet
Local peer id: PeerId("12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA")
Listening on "/ip4/127.0.0.1/tcp/51656"
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/192.168.86.23/tcp/51654
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/127.0.0.1/tcp/51654
// 下面的内容在新节点加入时逐渐出现
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/192.168.86.23/tcp/51661
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/127.0.0.1/tcp/51661
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/192.168.86.23/tcp/51670
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/127.0.0.1/tcp/51670
```
~~~

~~~admonish example title="例子:  窗口C " collapsible=true
```shell
❯ cargo run --example p2p_chat --quiet
Local peer id: PeerId("12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT")
Listening on "/ip4/127.0.0.1/tcp/51661"
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/192.168.86.23/tcp/51656
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/127.0.0.1/tcp/51656
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/192.168.86.23/tcp/51654
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/127.0.0.1/tcp/51654
// 下面的内容在新节点加入时逐渐出现
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/192.168.86.23/tcp/51670
Got peer: 12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh with addr /ip4/127.0.0.1/tcp/51670
```
~~~

~~~admonish example title="例子:  窗口 D 使用 topic 参数，让它和其它的 topic 不同 " collapsible=true
```rust
❯ cargo run --example p2p_chat --quiet -- hello
Local peer id: PeerId("12D3KooWRy9r8j7UQMxavqTcNmoz1JmnLcTU5UZvzvE5jz4Zw3eh")
Listening on "/ip4/127.0.0.1/tcp/51670"
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/192.168.86.23/tcp/51661
Got peer: 12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT with addr /ip4/127.0.0.1/tcp/51661
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/192.168.86.23/tcp/51656
Got peer: 12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA with addr /ip4/127.0.0.1/tcp/51656
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/192.168.86.23/tcp/51654
Got peer: 12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg with addr /ip4/127.0.0.1/tcp/51654
```
~~~

你会看到，每个节点运行时，都会通过 MDNS 广播，来发现本地已有的 P2P 节点。

> 现在 A/B/C/D 组成了一个 P2P 网络，其中 A/B/C 都订阅了 lobby，而 D 订阅了 hello。

~~~admonish example title="例子:  我们在 A/B/C/D 四个窗口中分别输入 “Hello from X”，可以看到 " collapsible=true
- 窗口 A：

```shell
hello from A

PeerId("12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA"): "hello from B"

PeerId("12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT"): "hello from C"
```



- 窗口 B：

```shell
PeerId("12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg"): "hello from A"

hello from B

PeerId("12D3KooWMRQvxJcjcexCrNfgSVd2iChpiDWzbgRRS6c5mn9bBzdT"): "hello from C"
```

- 窗口 C：

```rust
PeerId("12D3KooWDJtZVKBCa7B9C8ZQmRpP7cB7CgeG7PWLXYCnN3aXkaVg"): "hello from A"

PeerId("12D3KooWAw1gTLCesw1bvTiKNYFyacwbAcjvKwfDsJiH8AuBFgFA"): "hello from B"

hello from C
```

- 窗口 D：

```rust
hello from D
```
~~~

> 可以看到，在 lobby 下的 A/B/C 都收到了各自的消息。

这个使用 libp2p 的聊天代码，如果你读不懂，没关系。

> P2P 有大量的新的概念和协议需要预先掌握，所以如果你对这些概念和协议感兴趣，可以自行阅读 **[libp2p 的文档](https://docs.rs/libp2p/latest/libp2p/)**
> ，以及它的**[示例代码](https://github.com/libp2p/rust-libp2p/tree/master/examples)**。

### 通讯模型练习题

1. 看一看 libp2p 的文档和示例代码，把 libp2p clone 到本地，运行每个示例代码。
2. 阅读 [**libp2p 的 NetworkBehaviour
   trait**](https://docs.rs/libp2p-swarm/0.30.0/src/libp2p_swarm/behaviour.rs.html#56-185)
   ，以及 **[floodsub 对应的实现](https://docs.rs/libp2p-floodsub/0.30.0/src/libp2p_floodsub/layer.rs.html#244-399)**。
3. 尝试把[P2P聊天应用](#P2P聊天应用)例子中的 floodsub
   替换成**[更高效更节省带宽的 gossipsub](https://docs.rs/libp2p/0.39.1/libp2p/gossipsub/struct.Gossipsub.html)**。