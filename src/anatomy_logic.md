# 源码解析逻辑

<!--ts-->
* [源码解析逻辑](#源码解析逻辑)
   * [项目文档](#项目文档)
      * [来源](#来源)
      * [cargo doc命令使用](#cargo-doc命令使用)
         * [--open](#--open)
         * [--no-deps](#--no-deps)
      * [使用区别](#使用区别)
      * [使用细节](#使用细节)
         * [Crates](#crates)
         * [crate包含成员](#crate包含成员)
         * [Re-exports](#re-exports)
         * [Modules](#modules)
         * [Macros](#macros)
         * [Derive Macros](#derive-macros)
         * [Attribute Macros](#attribute-macros)
         * [Structs](#structs)
            * [Definition](#definition)
            * [Associated Types](#associated-types)
            * [Implementations](#implementations)
            * [Trait Implementations](#trait-implementations)
            * [Auto Trait Implementations](#auto-trait-implementations)
            * [Blanket Implementations](#blanket-implementations)
         * [Enums](#enums)
            * [Definition](#definition-1)
            * [Variants](#variants)
            * [Trait Implementations](#trait-implementations-1)
            * [Auto Trait Implementations](#auto-trait-implementations-1)
            * [Blanket Implementations](#blanket-implementations-1)
         * [Constants](#constants)
         * [Traits](#traits)
            * [Definition](#definition-2)
            * [Required methods](#required-methods)
            * [Implementations on Foreign Types](#implementations-on-foreign-types)
            * [Implementors](#implementors)
         * [Functions](#functions)
            * [Definition](#definition-3)
         * [Type Definitions](#type-definitions)
            * [Definition](#definition-4)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Sep 27 06:12:21 UTC 2022 -->

<!--te-->

## 项目文档

### 来源

1. 第三方crate可以在[官方文档](https://docs.rs/)上面搜索
2. 本地crate可以使用命令`cargo doc --open`.

### cargo doc命令使用

> [更多内容](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

#### --open

自动生成文档并在浏览器打开

#### --no-deps

默认情况下会把项目依赖的包文档也生成，这也是文档左侧Crates的来源之一。
这个参数可以屏蔽掉依赖的crates

### 使用区别

1. 二者内容没有区别，官方文档也是执行命令生成文档。
2. 官方文档还可以提供很多细节，比如git分支地址

### 使用细节

> 这里按照文档的层级进行递进说明

#### Crates

所有文档的首页都会有这一项，列出项目包含的crate

#### crate包含成员

> 点击Crates下的某个crate，右侧页面就会显示当前crate包含的元素， 主要有下列内容

- Re-exports
- Modules
- Macros
- Derive Macros
- Attribute Macros
- Structs
- Enums
- Constants
- Traits
- Functions
- Type Definitions

#### Re-exports

> futures_util

```rust
pub use reader::DecompressorCustomIo;
```

#### Modules

其实就是mod，点击之后将会列出某个mod里面的成员

#### Macros

#### Derive Macros

> darling_macro、clap_derive

#### Attribute Macros

> tokio_macros、futures_macro

#### Structs

##### Definition

##### Associated Types

##### Implementations

##### Trait Implementations

##### Auto Trait Implementations

##### Blanket Implementations

#### Enums

##### Definition

##### Variants

##### Trait Implementations

##### Auto Trait Implementations

##### Blanket Implementations

#### Constants

#### Traits

##### Definition

##### Required methods

##### Implementations on Foreign Types

##### Implementors

#### Functions

##### Definition

#### Type Definitions

##### Definition

> html2md

```rust
pub type BoxedError = Box<dyn Error + Send + Sync>;
```