# 三种多态形式

<!--ts-->
* [三种多态形式](#三种多态形式)
   * [参数多态：](#参数多态)
   * [特设多态：](#特设多态)
   * [子类型多态](#子类型多态)
   * [三种多态两两对比](#三种多态两两对比)
      * [参数多态 vs 特设多态](#参数多态-vs-特设多态)
      * [参数多态 vs 子类型多态](#参数多态-vs-子类型多态)
      * [特设多态 vs 子类型多态](#特设多态-vs-子类型多态)
      * [具体再对比一下rust的两种多态实现方式：](#具体再对比一下rust的两种多态实现方式)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr  1 15:59:50 UTC 2023 -->

<!--te-->

> 通俗来讲，只要是为不同类型的数据或操作提供了相同的名字就可以叫多态。

## 参数多态：

- 操作与类型无关

> 又叫编译时多态，Rust主要使用泛型来实现

~~~admonish tip title="对不同类型，调用相同方法，执行相同逻辑" collapsible=true
参数化多态(Parametric polymorphism)是指一段代码适用于不同的类型，把类型作为参数，在实际需要的时候根据类型进行实例化，所有的实例有相同的行为
~~~

## 特设多态：

- 重写继承过来的指定方法, 就是实现接口方法

> 又叫ad-hoc，Rust主要使用Trait Impl实现。

~~~admonish tip title="ad-hoc是一个拉丁词，意思为特定，临时" collapsible=true
Ad hoc是一个拉丁文常用短语。

> 这个短语的意思是“特设的、特定目的的（地）、即席的、临时的、将就的、专案的”。

这个短语通常用来形容一些特殊的、不能用于其它方面的，为一个特定的问题、任务而专门设定的解决方案。
~~~

> 称其为特设是因为这种多态并不像全称量化一样适用于所有类型，而是**手动为某些特定类型提供不同的实现**(这也就是trait impl过程)。

## 子类型多态

> 又叫运行时多态，Rust主要使用Trait Object实现

## 三种多态两两对比

### 参数多态 vs 特设多态

> 可以与参数多态对比一下：

1. 相同点：对不同类型，调用相同方法
2. 不同点：

- ad-hoc: 但是执行不同逻辑
- 参数多态：执行相同逻辑

### 参数多态 vs 子类型多态

1. 在编程语言中，参数多态的主要实现方式有很多，Rust选择了泛型作为实现方式。
2. [泛型的实现方式主要有擦除类型和单态化](https://blog.yzsun.me/polymorphic-polymorphism/#%E6%B3%9B%E5%9E%8B)，前者是运行时操作，后者是编译时操作；
3. Rust的泛型使用单态化实现，因此对应的参数多态也是发生在编译时，所以又被称为编译时多态；
4. 而[Trait Object的实现方式是运行时擦除类型](https://blog.yzsun.me/polymorphic-polymorphism/#%E5%8A%A8%E6%80%81%E5%AE%9E%E7%8E%B0)
   ，所以它其实是类似于泛型的另一种实现方式。

- java使用运行时擦除类型，其实就是在编译时替换为Object，进而保证类型安全。

> [Type Erasure in Java Explained | Baeldung](https://www.baeldung.com/java-type-erasure)

- Rust使用运行时擦除类型，其实就是采用C++的方式，编译时替换为一个胖指针。
- 与C++不同的是，C++将虚函数表放在实例里，而Rust的胖指针指向实例和虚函数表

### 特设多态 vs 子类型多态

1. 特设多态的实现很明确：特定实现，发生在编译期
2. 子类型多态：胖指针实现，发生在运行期

### 具体再对比一下rust的两种多态实现方式：

~~~admonish tip title="trait impl（特设多态）vs trait object（子类型多态）" collapsible=true
> trait object（动态分发）：

1. 前提：trait里面的每个方法都要有&slef参数。
2. 优势：使用指针，对参数的要求更灵活，二进制文件小。
3. 劣势：运行时动态分发，造成性能不如静态分发。
4. 特点：传指针或者引用。

> trait bound（静态分发）:

1. 前提: 只要程序代码中并没初始化类型，那么针对这个类型的trait中的方法也没有生成。
2. 执行：编译期分析代码，为每个实现了trait的类型执行单态化（必须满足前提）。
3. 优势：编译期决定为类型分发trait实现，不影响运行时，性能高。
4. 劣势：编译器为每个实现过trait的类型编译一份方法实现，二进制文件大。
5. 特点：传值
~~~

~~~admonish tip title="具体到self和&self" collapsible=true
```rust, editable
#[warn(dead_code)]
trait Animal {
    // trait object的方法都要带&self参数
    fn hello(&self){println!("hello");}
}

struct Good{}
struct Hello{}

impl Animal for Good{}//默认实现
impl Animal for Hello{}//默认实现

 //动态分发体现在传递引用&dyn
fn test_dyn(x: &dyn Animal) 
{
    x.hello();
}

//静态分发体现在传递值impl Animal
fn test(x: impl Animal){
    x.hello();
}

fn main() {
    {
        let good = Good{};
        test_dyn(&good);
        // let hello = Hello{};
        // test_dyn(&hello);
    }
}
```

1. trait object的方法都要带&self参数
2. 动态分发体现在参数传递引用&dyn
3. 静态分发体现在参数传递值impl Animal
~~~
