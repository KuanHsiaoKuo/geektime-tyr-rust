# thumbor图片服务

<!--ts-->
<!--te-->

## abi.proto

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/abi.proto}}
```

## build.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/build.rs}}
```

- [option_env in std - Rust](https://doc.rust-lang.org/std/macro.option_env.html)

> 在编译时可选择检查环境变量。

## 关于rust的模块

> 可以参考这篇：[Rust 模块系统理解 - 知乎](https://zhuanlan.zhihu.com/p/443926839)

~~~admonish tip title='mod全认识'
1. mod(mod.rs或mod关键字)将代码分为多个逻辑模块，并管理这些模块的可见性（public / private）。
2. 模块是项（item）的集合，项可以是：函数，结构体，trait，impl块，甚至其它模块。
3. 一个目录下的所有代码，可以通过 mod.rs 声明
4. Rust模块有三种形式:
    - mod.rs: 一个目录下的所有代码，可以通过 mod.rs 声明
    - 文件/目录即模块：编译器的机制决定，除了mod.rs外，每一个文件和目录都是一个模块。不允许只分拆文件，但是不声明mod，我们通常使用pub use，在父空间直接调用子空间的函数。
    - mod关键字: 在文件内部分拆模块
5. Rust编译器只接受一个源文件，输出一个crate
6. 每一个crate都有一个匿名的根命名空间，命名空间可以无限嵌套
7. “mod mod-name { ... }“ 将大括号中的代码置于命名空间mod-name之下
8. “use mod-name1::mod-name2;" 可以打开命名空间，减少无休止的::操作符
9. “mod mod-name;“ 可以指导编译器将多个文件组装成一个文件
10. “pub use mod-nam1::mod-name2::item-name;“
    语句可以将mod-name2下的item-name提升到这条语句所在的空间，item-name通常是函数或者结构体。Rust社区通常用这个方法来缩短库API的命名空间深度
    编译器规定use语句一定要在mod语句之前
~~~

## mod文件定义与实现分离

在rust中，一般会在模块的mod.rs文件中对供外部使用的项进行实现, 项可以是：函数，结构体，trait，impl块，甚至其它模块.
这样有个好处，高内聚，可以在代码增长时，将变动局限在服务提供者内部，对外提供的api不变，不会造成破坏性更新。

## pb模块: 处理protobuf

### pb/mod.rs声明模块

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:5:6}}
```

### pb/abi.rs里面还有子模块

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:94:113}}
```

### pb/abi.rs另外定义了spec::Data里面的各个元素结构体/嵌套模块mod

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:1:87}}
```

### pb/abi.rs有个特殊结构体

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:88:93}}
```

### ImageSpec

#### 定义：有序数组

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:1:6}}
```

#### 实现：new方法、From&TryFrom实现类型转化

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:8:30}}
```

### Filter

#### 定义：枚举体mod

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:68:79}}
```

#### 实现：双引号的使用、模式匹配

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:32:42}}
```

### SampleFilter

#### 定义：枚举体mod

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:19:37}}
```

### 实现：mod使用双引号、From转为不同结果

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:45:56}}
```

### Spec

#### 定义：结构体

- pb/abi.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/abi.rs:88:93}}
```

#### 实现：类似面向对象中添加类方法Self

> 注意区别Self和self的使用：
> [rust - What's the difference between self and Self? - Stack Overflow](https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self)

- pb/mod.rs

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:58:95}}
```

### 单元测试

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/pb/mod.rs:97:110}}
```

## engine模块: 处理图片

### mod.rs: 定义统一的引擎trait

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/mod.rs:7:19}}
```

### photon.rs > 静态变量加载

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:11:18}}
```

### photon.rs > 具体引擎Photon的定义与转化TryFrom

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:21:30}}
```

### photon.rs > 具体引擎Photon的trait实现

#### Engine Trait

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:32:47}}
```

#### SpecTransform Trait

##### 格式语义化

```rust
impl SpecTransform(&OpreationName) for SpecificEngine {
    fn transform(&mut self, _op: &OperationName) {
        transform::OperationMethod(&mut self.0)
    }
}
```

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:54:108}}
```

### photon.rs > 在内存中对图片转换格式的方法

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/engine/photon.rs:111:122}}
```

## main.rs

### 先引入mod，再use

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:34:39}}
```

### 图片资源用到Lru策略缓存type定义

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:41:41}}
```

### 主流程main函数

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:43:71}}
```

#### 建造者模式

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:53:60}}
```

#### 类型转换

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:63:64}}
```

##### 数字与字符串

|         | i32                | u32                | f64                | String*       |
|---------|--------------------|--------------------|--------------------|---------------|
| i32     | \                  | x as u32           | x as f64           | x.to_string() |
| u32     | x as i32           | \                  | x as f64           | x.to_string() |
| f64     | x as i32           | x as u32           | \                  | x.to_string() |
| String* | x.parse().unwrap() | x.parse().unwrap() | x.parse().unwrap() | \             |

##### String 与 & str

| \      | String        | &str |
|--------|---------------|------|
| String | \             | &*x  |
| &str   | x.to_string() | \    |

##### 智能指针

| \        | Vec\<T\>   | &[T]    | Box<[T]>             |
|----------|------------|---------|----------------------|
| Vec\<T\> | \          | &x[...] | x.into_boxed_slice() |
| &[T]     | x.to_vec() | \       | Box::new(\*x)        |
| Box<[T]> | x.to_vec() | &\*x    | \                    |

### 路由绑定的处理函数handler

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:73:101}}
```

### 处理函数用到的图片获取方法

> 对于图片的网络请求，我们先把 URL 做个哈希，在 LRU 缓存中查找，找不到才用 reqwest 发送请求。

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:103:125}}
```

### 一个用于调试的辅助函数

```rust, editable
{{#include ../geektime_rust_codes/05_thumbor/src/main.rs:127:137}}
```

## 运行与日志

> 将RUST_LOG级别设置为info

```shell
cargo build --release
RUST_LOG=info target/release/thumbor
```