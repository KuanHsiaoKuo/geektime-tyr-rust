# Rust 的错误处理: 使用类型系统来构建

<!--ts-->
* [Rust 的错误处理: 使用类型系统来构建](#rust-的错误处理-使用类型系统来构建)
   * [1. 可恢复错误：Option/Result错误类型处理](#1-可恢复错误optionresult错误类型处理)
      * [Option](#option)
      * [Result](#result)
   * [2. 抛出异常：panic! 和 catch_unwind](#2-抛出异常panic-和-catch_unwind)
      * [panic!抛出异常: 不可/不想恢复](#panic抛出异常-不可不想恢复)
      * [unwrap/expect语法糖](#unwrapexpect语法糖)
      * [catch_uwind捕获异常：崩溃后恢复上下文](#catch_uwind捕获异常崩溃后恢复上下文)
   * [3. 自定义异常类型](#3-自定义异常类型)
      * [3.1 使用Error trait](#31-使用error-trait)
      * [3.2 使用thiserror简化](#32-使用thiserror简化)
      * [3.3 使用anyhow扩展 ？操作符](#33-使用anyhow扩展-操作符)
      * [3.4 让错误无所遁形](#34-让错误无所遁形)
   * [4. 捕获异常](#4-捕获异常)
      * [4.1 ? 操作符](#41--操作符)
         * [由来](#由来)
         * [用于传播<g-emoji class="g-emoji" alias="mega" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f4e3.png">📣</g-emoji>](#用于传播)
         * [？操作符展开](#操作符展开)
      * [4.2 函数式错误处理: map/map_err/and_then](#42-函数式错误处理-mapmap_errand_then)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Mar 30 03:25:13 UTC 2023 -->

<!--te-->

## 1. 可恢复错误：Option/Result错误类型处理

> 由于诞生的年代比较晚，Rust 有机会从已有的语言中学习到各种错误处理的优劣。对于 Rust 来说，目前的几种方式相比而言，最佳的方法是，使用类型系统来构建主要的错误处理流程。

~~~admonish info title="Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型" collapsible=true
![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895260.jpg)
~~~

### Option

~~~admonish info title="Option 是一个 enum，其定义如下：" collapsible=true
```rust, editable

pub enum Option<T> {
    None,
    Some(T),
}
```

> 它可以承载有值 / 无值这种最简单的错误类型。
~~~

### Result

~~~admonish info title="Result 是一个更加复杂的 enum，其定义如下：" collapsible=true
```rust, editable

#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> 当函数出错时，可以返回 Err(E)，否则 Ok(T)。
~~~

~~~admonish info title="Result 类型声明时还有个 must_use 的标注" collapsible=true
我们看到，Result 类型声明时还有个 must_use 的标注

> 编译器会对有 must_use 标注的所有类型做特殊处理：

如果该类型对应的值没有被显式使用，则会告警。这样，保证错误被妥善处理。

如下图所示：

![img](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/e2100e3f17a9587c4d4bf50523c10653.png)

这里，如果我们调用 read_file 函数时，直接丢弃返回值，由于 #[must_use] 的标注，Rust 编译器报警，要求我们使用其返回值。
~~~

## 2. 抛出异常：panic! 和 catch_unwind

### panic!抛出异常: 不可/不想恢复

- [To panic! or Not to panic! - The Rust Programming Language](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

> 使用 Option 和 Result 是 Rust 中处理错误的首选，绝大多数时候我们也应该使用，但 Rust 也提供了特殊的异常处理能力。

> 在 Rust 看来，一旦你需要抛出异常，那抛出的一定是严重的错误。

所以，Rust 跟 Golang 一样，使用了诸如 panic! 这样的字眼警示开发者：想清楚了再使用我。

一般而言，panic! 是不可恢复或者不想恢复的错误，我们希望在此刻，程序终止运行并得到崩溃信息。

### unwrap/expect语法糖

~~~admonish info title="unwrap() 或者 expect()就是准备panic!" collapsible=true
在使用 Option 和 Result 类型时，开发者也可以对其 unwrap() 或者 expect()，强制把 Option<T> 和 Result<T, E> 转换成 T，如果无法完成这种转换，也会 panic! 出来。

比如下面的代码，它解析[ noise protocol](https://noiseprotocol.org/noise.html#protocol-names-and-modifiers)的协议变量：

```rust, editable
let params: NoiseParams = "Noise_XX_25519_AESGCM_SHA256".parse().unwrap();
```
~~~

> 如果开发者不小心把协议变量写错了，最佳的方式是立刻 panic! 出来，让错误立刻暴露，以便解决这个问题。

### catch_uwind捕获异常：崩溃后恢复上下文

~~~admonish info title="catch_unwind(): 有些场景下，我们也希望能够像异常处理那样能够栈回溯，把环境恢复到捕获异常的上下文。" collapsible=true
Rust 标准库下提供了 catch_unwind() ，把调用栈回溯到 catch_unwind 这一刻，作用和其它语言的 try {…} catch {…} 一样。见如下代码：
```rust, editable

use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:#?}", result);
}
```
~~~

> 当然，和异常处理一样，并不意味着你可以滥用这一特性.
> 我想这也是 Rust 把抛出异常称作 panic! ，而捕获异常称作 catch_unwind 的原因:让初学者望而生畏，不敢轻易使用。这也是一个不错的用户体验。

~~~admonish question title="catch_unwind 在哪些场景下非常有用:" collapsible=true
catch_unwind 在某些场景下非常有用:
1. 比如你在使用 Rust 为 erlang VM 撰写 NIF，你不希望 Rust 代码中的任何 panic! 导致 erlang VM 崩溃。
因为崩溃是一个非常不好的体验，它违背了 erlang 的设计原则：process 可以 let it crash，但错误代码不该导致 VM 崩溃。

2. 此刻，你就可以把 Rust 代码整个封装在 catch_unwind() 函数所需要传入的闭包中。
这样，一旦任何代码中，包括第三方 crates 的代码，含有能够导致 panic! 的代码，都会被捕获，并被转换为一个 Result。
~~~

## 3. 自定义异常类型

### 3.1 使用Error trait

> 我们可以定义我们自己的数据类型，然后为其实现 Error trait。

~~~admonish info title="使用Error trait自定义错误类型" collapsible=true
上文中，我们讲到 Result<T, E> 里 E 是一个代表错误的数据类型。为了规范这个代表错误的数据类型的行为，Rust 定义了 Error trait：

```rust, editable

pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn backtrace(&self) -> Option<&Backtrace> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
}
```
~~~

### 3.2 使用thiserror简化

> 不过，这样的工作已经有人替我们简化了：我们可以使用 thiserror和 anyhow来简化这个步骤。

~~~admonish info title="thiserror 提供了一个派生宏（derive macro）来简化错误类型的定义" collapsible=true
```rust, editable

use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```
~~~

如果你在撰写一个 Rust 库，那么 thiserror 可以很好地协助你对这个库里所有可能发生的错误进行建模。

### 3.3 使用anyhow扩展 ？操作符

1. anyhow 实现了 anyhow::Error 和任意符合 Error trait 的错误类型之间的转换，让你可以使用 ? 操作符，不必再手工转换错误类型。
2. anyhow 还可以让你很容易地抛出一些临时的错误，而不必费力定义错误类型，当然，我们不提倡滥用这个能力。

### 3.4 让错误无所遁形

> 作为一名严肃的开发者，我非常建议你在开发前:

- 先用类似 thiserror 的库定义好你项目中主要的错误类型
- 并随着项目的深入，不断增加新的错误类型，让系统中所有的潜在错误都无所遁形。

## 4. 捕获异常

### 4.1 ? 操作符

#### 由来

~~~admonish info title="? 操作符的由来" collapsible=true
这虽然可以极大避免遗忘错误的显示处理，但如果我们并不关心错误，只需要传递错误，还是会写出像 C 或者 Golang 一样比较冗余的代码。怎么办？

好在 Rust 除了有强大的类型系统外，还具备元编程的能力：
- 早期 Rust 提供了 try! 宏来简化错误的显式处理
- 后来为了进一步提升用户体验，try! 被进化成 ? 操作符。
~~~

#### 用于传播📣

~~~admonish info title="如果你只想传播错误，不想就地处理，可以用 ? 操作符" collapsible=true
所以在 Rust 代码中，如果你只想传播错误，不想就地处理，可以用 ? 操作符，比如（代码）:

```rust, editable

use std::fs::File;
use std::io::Read;

fn read_file(name: &str) -> Result<String, std::io::Error> {
  let mut f = File::open(name)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  Ok(contents)
}
```
~~~

> 通过 ? 操作符，Rust 让错误传播的代价和异常处理不相上下，同时又避免了异常处理的诸多问题。

#### ？操作符展开

~~~admonish info title=" ? 操作符内部被展开成类似这样的代码：" collapsible=true
```rust, editable

match result {
  Ok(v) => v,
  Err(e) => return Err(e.into())
}
```

> 所以，我们可以方便地写出类似这样的代码，简洁易懂，可读性很强：

```rust, editable

fut
  .await?
  .process()?
  .next()
  .await?;
```

整个代码的执行流程如下：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F-4895239.jpg)
~~~

虽然 ? 操作符使用起来非常方便，但你要注意在不同的错误类型之间是无法直接使用的，需要实现 From trait 在二者之间建立起转换的桥梁，这会带来额外的麻烦。

### 4.2 函数式错误处理: map/map_err/and_then

> Rust 还为 Option 和 Result 提供了大量的辅助函数，如 map / map_err / and_then，你可以很方便地处理数据结构中部分情况。

~~~admonish info title="map / map_err / and_then: 使用函数式错误处理" collapsible=true
如下图所示：

![18｜错误处理：为什么Rust的错误处理与众不同？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/18%EF%BD%9C%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%EF%BC%9A%E4%B8%BA%E4%BB%80%E4%B9%88Rust%E7%9A%84%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86%E4%B8%8E%E4%BC%97%E4%B8%8D%E5%90%8C%EF%BC%9F.jpg)

通过这些函数，你可以很方便地对错误处理引入 [Railroad oriented programming 范式](https://www.slideshare.net/ScottWlaschin/railway-oriented-programming)。

> 比如用户注册的流程，你需要校验用户输入，对数据进行处理，转换，然后存入数据库中。你可以这么撰写这个流程：

```rust, editable
Ok(data)
  .and_then(validate)
  .and_then(process)
  .map(transform)
  .and_then(store)
  .map_error(...)
```

> 执行流程如下图所示：

![image-20221004225336162](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20221004225336162.png)
~~~

此外，Option 和 Result 的互相转换也很方换，这也得益于 Rust 构建的强大的函数式编程的能力。

> 我们可以看到，无论是通过 ? 操作符，还是函数式编程进行错误处理，Rust 都力求让错误处理灵活高效，让开发者使用起来简单直观。

