# 源码解析逻辑

<!--ts-->
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