# V 并发与异步

<!--ts-->
* [V 并发与异步](#v-并发与异步)
   * [区分并发与并行](#区分并发与并行)
   * [并发的难点、工作模式和核心](#并发的难点工作模式和核心)
   * [并发原语](#并发原语)
      * [Atomic](#atomic)
         * [从锁开始](#从锁开始)
         * [Atomic+CAS](#atomiccas)
         * [ordering](#ordering)
      * [Mutex](#mutex)
      * [Atomic和Mutex的联系](#atomic和mutex的联系)
      * [Condvar](#condvar)
         * [Atomic和Mutex不能解决DAG模式](#atomic和mutex不能解决dag模式)
         * [condvar介绍与使用](#condvar介绍与使用)
      * [Channel](#channel)
      * [Actor](#actor)
      * [小结一下各种并发原语的使用场景](#小结一下各种并发原语的使用场景)
   * [自己实现一个基本的MPSC Channel](#自己实现一个基本的mpsc-channel)
      * [测试驱动的设计](#测试驱动的设计)
      * [实现 MPSC channel](#实现-mpsc-channel)
      * [回顾测试驱动开发](#回顾测试驱动开发)
   * [并发原语与异步的关系](#并发原语与异步的关系)
   * [Future](#future)
      * [actor是有栈协程，Future是无栈协程](#actor是有栈协程future是无栈协程)
      * [Rust的Future](#rust的future)
      * [Future和async/await](#future和asyncawait)
      * [为什么需要 Future？](#为什么需要-future)
      * [深入思路](#深入思路)
      * [深入了解](#深入了解)
      * [executor](#executor)
      * [reactor pattern](#reactor-pattern)
      * [怎么用 Future 做异步处理？](#怎么用-future-做异步处理)
      * [使用 Future 的注意事项](#使用-future-的注意事项)
      * [对比线程学习Future](#对比线程学习future)
      * [为什么标准库的 Mutex 不能跨越 await？](#为什么标准库的-mutex-不能跨越-await)
   * [async/await内部是怎么实现的？](#asyncawait内部是怎么实现的)
      * [Context、Pin](#contextpin)
      * [Context::waker: Waker 的调用机制](#contextwaker-waker-的调用机制)
      * [async 究竟生成了什么？](#async-究竟生成了什么)
      * [为什么需要 Pin？](#为什么需要-pin)
      * [自引用数据结构](#自引用数据结构)
      * [Unpin](#unpin)
      * [### Box的Unpin思考](#-box的unpin思考)
      * [async 产生的 Future 究竟是什么类型？](#async-产生的-future-究竟是什么类型)
      * [回顾整理Future的Context、Pin/Unpin，以及async/await](#回顾整理future的contextpinunpin以及asyncawait)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Oct  9 02:33:19 UTC 2022 -->

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

## 并发原语

~~~admonish info title="在这些并发模式背后，都有哪些并发原语可以为我们所用呢" collapsible=true
在这些并发模式背后，都有哪些并发原语可以为我们所用呢：
1. Atomic
2. Mutex
3. Condvar
4. Channel 
5. Actor model
~~~

### Atomic

Atomic 是所有并发原语的基础，它为并发任务的同步奠定了坚实的基础。

#### 从锁开始

~~~admonish info title="互斥锁会导致乱序" collapsible=true
谈到同步，相信你首先会想到锁，所以在具体介绍 atomic 之前，我们从最基本的锁该如何实现讲起。自由竞争模式下，我们需要用互斥锁来保护某个临界区，使进入临界区的任务拥有独占访问的权限。

> 为了简便起见，在获取这把锁的时候，如果获取不到，就一直死循环，直到拿到锁为止（代码）：

```rust, editable

use std::{cell::RefCell, fmt, sync::Arc, thread};

struct Lock<T> {
    locked: RefCell<bool>,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            locked: RefCell::new(false),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果没拿到锁，就一直 spin
        while *self.locked.borrow() != false {} // **1

        // 拿到，赶紧加锁
        *self.locked.borrow_mut() = true; // **2

        // 开始干活
        op(&mut self.data.borrow_mut()); // **3

        // 解锁
        *self.locked.borrow_mut() = false; // **4
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
```
> 这段代码模拟了 Mutex 的实现，它的核心部分是 lock() 方法。

我们之前说过，Mutex 在调用 lock() 后，会得到一个 MutexGuard 的 RAII 结构，这里为了简便起见，要求调用者传入一个闭包，来处理加锁后的事务。在 lock() 方法里，拿不到锁的并发任务会一直 spin，拿到锁的任务可以干活，干完活后会解锁，这样之前 spin 的任务会竞争到锁，进入临界区。

> 这样的实现看上去似乎问题不大，但是你细想，它有好几个问题：

1. 在多核情况下，**1 和 **2 之间，有可能其它线程也碰巧 spin 结束，把 locked 修改为 true。这样，存在多个线程拿到这把锁，破坏了任何线程都有独占访问的保证。

2. 即便在单核情况下，**1 和 **2 之间，也可能因为操作系统的可抢占式调度，导致问题 1 发生。

3. 如今的编译器会最大程度优化生成的指令，如果操作之间没有依赖关系，可能会生成乱序的机器码，比如**3 被优化放在 **1 之前，从而破坏了这个 lock 的保证。

4. 即便编译器不做乱序处理，CPU 也会最大程度做指令的乱序执行，让流水线的效率最高。同样会发生 3 的问题。

> 所以，我们实现这个锁的行为是未定义的。可能大部分时间如我们所愿，但会随机出现奇奇怪怪的行为。一旦这样的事情发生，bug 可能会以各种不同的面貌出现在系统的各个角落。而且，这样的 bug 几乎是无解的，因为它很难稳定复现，表现行为很不一致，甚至，只在某个 CPU 下出现。

> 这里再强调一下 unsafe 代码需要足够严谨，需要非常有经验的工程师去审查，这段代码之所以破快了并发安全性，是因为我们错误地认为：为 Lock<T> 实现 Sync，是安全的。

为了解决上面这段代码的问题，我们必须在 CPU 层面做一些保证，让某些操作成为原子操作。
~~~

#### Atomic+CAS

~~~admonish info title="使用Atomic+CAS确保原子操作顺序l" collapsible=true
最基础的保证是：CAS
1. 可以通过一条指令读取某个内存地址，判断其值是否等于某个前置值，如果相等，将其修改为新的值。这就是 Compare-and-swap 操作，简称CAS。它是操作系统的几乎所有并发原语的基石，使得我们能实现一个可以正常工作的锁。

所以，刚才的代码，我们可以把一开始的循环改成：

```rust, editable

while self
  .locked
  .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
  .is_err() {}
```
这句的意思是：
1. 如果 locked 当前的值是 false，就将其改成 true。
2. 这整个操作在一条指令里完成，不会被其它线程打断或者修改；
3. 如果 locked 的当前值不是 false，那么就会返回错误，我们会在此不停 spin，直到前置条件得到满足。
4. 这里，compare_exchange 是 Rust 提供的 CAS 操作，它会被编译成 CPU 的对应 CAS 指令。

> 当这句执行成功后，locked 必然会被改变为 true，我们成功拿到了锁，而任何其他线程都会在这句话上 spin。

同样在释放锁的时候，相应地需要使用 atomic 的版本，而非直接赋值成 false：

```rust, editable

self.locked.store(false, Ordering::Release);
```

当然，为了配合这样的改动，我们还需要把 locked 从 bool 改成 AtomicBool。

在 Rust 里，std::sync::atomic 有大量的 atomic 数据结构，对应各种基础结构。

我们看使用了 AtomicBool 的新实现（代码）：
```rust, editable

use std::{
    cell::RefCell,
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

struct Lock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果没拿到锁，就一直 spin
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {} // **1

        // 已经拿到并加锁，开始干活
        op(&mut self.data.borrow_mut()); // **3

        // 解锁
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
```

可以看到:
1. 通过使用 compare_exchange ，规避了 1 和 2 面临的问题
2. 但对于和编译器 /CPU 自动优化相关的 3 和 4，我们还需要一些额外处理。
~~~

#### ordering

~~~admonish info title="Ordering枚举体说明" collapsible=true
这就是这个函数里额外的两个和 [Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html) 有关的奇怪参数。

```rust, editable

pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

> 文档里解释了几种 Ordering 的用途，稍稍扩展一下。

1. 第一个 Relaxed，这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序执行。
2. Release，当我们写入数据（比如上面代码里的 store）的时候，如果用了 Release order，那么：
- 对于当前线程，任何读取或写入操作都不能被乱序排在这个 store 之后。
- 也就是说，在上面的例子里，CPU 或者编译器不能把 **3 挪到 **4 之后执行。

3. 对于其它线程，如果使用了 Acquire 来读取这个 atomic 的数据， 那么它们看到的是修改后的结果。
- 上面代码我们在 compare_exchange 里使用了 Acquire 来读取，所以能保证读到最新的值。
- 而 Acquire 是当我们读取数据的时候，如果用了 Acquire order，那么：
- 对于当前线程，任何读取或者写入操作都不能被乱序排在这个读取之前。在上面的例子里，CPU 或者编译器不能把 **3 挪到 **1 之前执行。
- 对于其它线程，如果使用了 Release 来修改数据，那么，修改的值对当前线程可见。

4. 第四个 AcqRel 是 Acquire 和 Release 的结合，同时拥有 Acquire 和 Release 的保证。
- 这个一般用在 fetch_xxx 上，比如你要对一个 atomic 自增 1，你希望这个操作之前和之后的读取或写入操作不会被乱序，并且操作的结果对其它线程可见。

5. 最后的 SeqCst 是最严格的 ordering，除了 AcqRel 的保证外，它还保证所有线程看到的所有 SeqCst 操作的顺序是一致的。

~~~

因为 CAS 和 ordering 都是系统级的操作，所以这里描述的 Ordering 的用途在各种语言中都大同小异。对于 Rust 来说，它的 atomic
原语[继承于 C++](https://en.cppreference.com/w/cpp/atomic/memory_order)。如果读 Rust 的文档你感觉云里雾里，那么 C++ 关于 ordering 的文档要清晰得多。

~~~admonish info title="对Atomic背后的CAS进行获取锁的优化" collapsible=true
其实上面获取锁的 spin 过程性能不够好，更好的方式是这样处理一下：

```rust, editable

while self
    .locked
    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
    .is_err()
{
    // 性能优化：compare_exchange 需要独占访问，当拿不到锁时，我们
    // 先不停检测 locked 的状态，直到其 unlocked 后，再尝试拿锁
    while self.locked.load(Ordering::Relaxed) == true {}
}
```
注意，我们在 while loop 里，又嵌入了一个 loop。
1. 这是因为 CAS 是个代价比较高的操作，它需要获得对应内存的独占访问（exclusive access）
2. 我们希望失败的时候只是简单读取 atomic 的状态，只有符合条件的时候再去做独占访问，进行 CAS。
3. 所以，看上去多做了一层循环，实际代码的效率更高。


> 以下是两个线程同步的过程，一开始 t1 拿到锁、t2 spin，之后 t1 释放锁、t2 进入到临界区执行：

![33｜并发处理（上）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/33%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8A%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F-4950274.jpg)
~~~

讲到这里，相信你对 atomic 以及其背后的 CAS 有初步的了解了。

~~~admonish info title="那么，atomic 除了做其它并发原语，还有什么作用？" collapsible=true
那么，atomic 除了做其它并发原语，还有什么作用？

我个人用的最多的是做各种 lock-free 的数据结构。比如，需要一个全局的 ID 生成器。当然可以使用 UUID 这样的模块来生成唯一的 ID，但如果我们同时需要这个 ID 是有序的，那么 AtomicUsize 就是最好的选择。

你可以用 fetch_add 来增加这个 ID，而 fetch_add 返回的结果就可以用于当前的 ID。这样，不需要加锁，就得到了一个可以在多线程中安全使用的 ID 生成器。

另外，atomic 还可以用于记录系统的各种 metrics。比如一个简单的 in-memory Metrics 模块：

```rust, editable

use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

// server statistics
pub struct Metrics(HashMap<&'static str, AtomicUsize>);

impl Metrics {
    pub fn new(names: &[&'static str]) -> Self {
        let mut metrics: HashMap<&'static str, AtomicUsize> = HashMap::new();
        for name in names.iter() {
            metrics.insert(name, AtomicUsize::new(0));
        }
        Self(metrics)
    }

    pub fn inc(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn add(&self, name: &'static str, val: usize) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(val, Ordering::Relaxed);
        }
    }

    pub fn dec(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn snapshot(&self) -> Vec<(&'static str, usize)> {
        self.0
            .iter()
            .map(|(k, v)| (*k, v.load(Ordering::Relaxed)))
            .collect()
    }
}
```

它允许你初始化一个全局的 metrics 表，然后在程序的任何地方，无锁地操作相应的 metrics：

```rust, editable

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

lazy_static! {
    pub(crate) static ref METRICS: Metrics = Metrics::new(&[
        "topics",
        "clients",
        "peers",
        "broadcasts",
        "servers",
        "states",
        "subscribers"
    ]);
}

// server statistics
pub struct Metrics(HashMap<&'static str, AtomicUsize>);

impl Metrics {
    pub fn new(names: &[&'static str]) -> Self {
        let mut metrics: HashMap<&'static str, AtomicUsize> = HashMap::new();
        for name in names.iter() {
            metrics.insert(name, AtomicUsize::new(0));
        }
        Self(metrics)
    }

    pub fn inc(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn add(&self, name: &'static str, val: usize) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(val, Ordering::Relaxed);
        }
    }

    pub fn dec(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn snapshot(&self) -> Vec<(&'static str, usize)> {
        self.0
            .iter()
            .map(|(k, v)| (*k, v.load(Ordering::Relaxed)))
            .collect()
    }
}

fn main() {
    METRICS.inc("topics");
    METRICS.inc("subscribers");

    println!("{:?}", METRICS.snapshot());
}

```
~~~

### Mutex

~~~admonish info title="Atomic有什么限制，Mutex又如何解决？" collapsible=true
Atomic 虽然可以处理自由竞争模式下加锁的需求，但毕竟用起来不那么方便，我们需要更高层的并发原语，来保证软件系统控制多个线程对同一个共享资源的访问，使得每个线程在访问共享资源的时候，可以独占或者说互斥访问（mutual exclusive access）。

我们知道:
1. 对于一个共享资源，如果所有线程只做读操作，那么无需互斥，大家随时可以访问，很多 immutable language（如 Erlang / Elixir）做了语言层面的只读保证，确保了并发环境下的无锁操作。
- 这牺牲了一些效率（常见的 list/hashmap 需要使用 [persistent data structure](https://en.wikipedia.org/wiki/Persistent_data_structure)），额外做了不少内存拷贝，换来了并发控制下的简单轻灵。
2. 然而，一旦有任何一个或多个线程要修改共享资源，不但写者之间要互斥，读写之间也需要互斥。毕竟如果读写之间不互斥的话，读者轻则读到脏数据，重则会读到已经被破坏的数据，导致 crash。
- 比如读者读到链表里的一个节点，而写者恰巧把这个节点的内存释放掉了，如果不做互斥访问，系统一定会崩溃。

> 所以操作系统提供了用来解决这种读写互斥问题的基本工具：Mutex（RwLock 我们放下不表）。

其实上文中，为了展示如何使用 atomic，我们制作了一个非常粗糙简单的 SpinLock，就可以看做是一个广义的 Mutex。SpinLock，顾名思义，就是线程通过 CPU 空转（spin，就像前面的 while loop）忙等（busy wait），来等待某个临界区可用的一种锁。

> 然而，这种通过 SpinLock 做互斥的实现方式有使用场景的限制：如果受保护的临界区太大，那么整体的性能会急剧下降， CPU 忙等，浪费资源还不干实事，不适合作为一种通用的处理方法。

更通用的解决方案是：
1. 当多个线程竞争同一个 Mutex 时，获得锁的线程得到临界区的访问，其它线程被挂起，放入该 Mutex 上的一个等待队列里
2. 当获得锁的线程完成工作，退出临界区时，Mutex 会给等待队列发一个信号，把队列中第一个线程唤醒，于是这个线程可以进行后续的访问。整个过程如下：

![33｜并发处理（上）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/33%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8A%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F.jpg)
~~~

### Atomic和Mutex的联系

~~~admonish info title="Atomic、Mutex、semaphore的联系" collapsible=true
我们前面也讲过，线程的上下文切换代价很大，所以频繁将线程挂起再唤醒，会降低整个系统的效率。所以很多 Mutex 具体的实现会将 SpinLock（确切地说是 spin wait）和线程挂起结合使用：
线程的 lock 请求如果拿不到会先尝试 spin 一会，然后再挂起添加到等待队列。

Rust 下的 [parking_lot ](https://github.com/Amanieu/parking_lot)就是这样实现的。

当然，这样实现会带来公平性的问题：
1. 如果新来的线程恰巧在 spin 过程中拿到了锁
2. 而当前等待队列中还有其它线程在等待锁
3. 那么等待的线程只能继续等待下去
4. 这不符合 FIFO，不适合那些需要严格按先来后到排队的使用场景。
5. 为此，parking_lot 提供了 fair mutex。

Mutex 的实现依赖于 CPU 提供的 atomic。你可以把 Mutex 想象成一个粒度更大的 atomic，只不过这个 atomic 无法由 CPU 保证，而是通过软件算法来实现。

两个基本的并发原语 Atomic 和 Mutex。Atomic 是一切并发同步的基础，通过 CPU 提供特殊的 CAS 指令，操作系统和应用软件可以构建更加高层的并发原语，比如 SpinLock 和 Mutex。

SpinLock 和 Mutex 最大的不同是，使用 SpinLock，线程在忙等（busy wait），而使用 Mutex lock，线程在等待锁的时候会被调度出去，等锁可用时再被调度回来。

听上去 SpinLock 似乎效率很低，其实不是，这要具体看锁的临界区大小。如果临界区要执行的代码很少，那么和 Mutex lock 带来的上下文切换（context switch）相比，SpinLock 是值得的。在 Linux Kernel 中，很多时候我们只能使用 SpinLock。

至于操作系统里另一个重要的概念信号量（semaphore），你可以认为是 Mutex 更通用的表现形式。比如在新冠疫情下，图书馆要控制同时在馆内的人数，如果满了，其他人就必须排队，出来一个才能再进一个。这里，如果总人数限制为 1，就是 Mutex，如果 > 1，就是 semaphore。
~~~

### Condvar

#### Atomic和Mutex不能解决DAG模式

~~~admonish info title="Atomic和Mutex主要用于哪种工作模式？基于什么需求提出Condvar原语？" collapsible=true
对于并发状态下这三种常见的工作模式：自由竞争模式、map/reduce 模式、DAG 模式，我们的难点是如何在这些并发的任务中进行同步。atomic / Mutex 解决了自由竞争模式下并发任务的同步问题，也能够很好地解决 map/reduce 模式下的同步问题，因为此时同步只发生在 map 和 reduce 两个阶段。

![34｜并发处理（下）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/34%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8B%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F-4951814.jpg)

然而，它们没有解决一个更高层次的问题，也就是 DAG 模式：如果这种访问需要按照一定顺序进行或者前后有依赖关系，该怎么做？

这个问题的典型场景是生产者 - 消费者模式：生产者生产出来内容后，需要有机制通知消费者可以消费。比如 socket 上有数据了，通知处理线程来处理数据，处理完成之后，再通知 socket 收发的线程发送数据。

所以，操作系统还提供了 Condvar。
~~~

#### condvar介绍与使用

~~~admonish info title="Condvar介绍与使用" collapsible=true
Condvar 有两种状态：
1. 等待（wait）：线程在队列中等待，直到满足某个条件。
2. 通知（notify）：当 condvar 的条件满足时，当前线程通知其他等待的线程可以被唤醒。通知可以是单个通知，也可以是多个通知，甚至广播（通知所有人）。

在实践中，Condvar 往往和 Mutex 一起使用：
- Mutex 用于保证条件在读写时互斥
- Condvar 用于控制线程的等待和唤醒。

我们来看一个例子：

```rust, editable

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        eprintln!("I'm a happy worker!");
        // 通知主线程
        cvar.notify_one();
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("working...");
        }
    });

    // 等待工作线程的通知
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    eprintln!("Worker started!");
}
```

1. 这段代码通过 condvar，我们实现了 worker 线程在执行到一定阶段后通知主线程，然后主线程再做一些事情。

2. 这里，我们使用了一个 Mutex 作为互斥条件，然后在 [cvar.wait() ](https://doc.rust-lang.org/src/std/sync/condvar.rs.html#184-191)中传入这个 Mutex。这个接口需要一个 MutexGuard，以便于知道需要唤醒哪个 Mutex 下等待的线程：

```rust, editable

pub fn wait<'a, T>(
    &self,
    guard: MutexGuard<'a, T>
) -> LockResult<MutexGuard<'a, T>>
```
~~~

### Channel

~~~admonish info title="Mutex和Condvar的局限性在哪？Channel如何解决的？" collapsible=true
但是用 Mutex 和 Condvar 来处理复杂的 DAG 并发模式会比较吃力。所以，Rust 还提供了各种各样的 Channel 用于处理并发任务之间的通讯。

由于 Golang 不遗余力地推广，Channel 可能是最广为人知的并发手段。相对于 Mutex，Channel 的抽象程度最高，接口最为直观，使用起来的心理负担也没那么大。使用 Mutex 时，你需要很小心地避免死锁，控制临界区的大小，防止一切可能发生的意外。

> 虽然在 Rust 里，我们可以“无畏并发”（Fearless concurrency）—— 当代码编译通过，绝大多数并发问题都可以规避，但性能上的问题、逻辑上的死锁还需要开发者照料。

Channel 把锁封装在了队列写入和读取的小块区域内，然后把读者和写者完全分离，使得读者读取数据和写者写入数据，对开发者而言，除了潜在的上下文切换外，完全和锁无关，就像访问一个本地队列一样。所以，对于大部分并发问题，我们都可以用 Channel 或者类似的思想来处理（比如 actor model）。

~~~

~~~admonish info title="Channel根据使用场景和读写者数量分别如何分类？" collapsible=true
Channel 在具体实现的时候，根据不同的使用场景，会选择不同的工具。Rust 提供了以下四种 Channel：

1. oneshot：这可能是最简单的 Channel，写者就只发一次数据，而读者也只读一次。

这种一次性的、多个线程间的同步可以用 oneshot channel 完成。由于 oneshot 特殊的用途，实现的时候可以直接用 atomic swap 来完成。

2. rendezvous：很多时候，我们只需要通过 Channel 来控制线程间的同步，并不需要发送数据。

rendezvous channel 是 channel size 为 0 的一种特殊情况。

这种情况下，我们用 Mutex + Condvar 实现就足够了，在具体实现中，rendezvous channel 其实也就是 Mutex + Condvar 的一个包装。

3. bounded：bounded channel 有一个队列，但队列有上限。

一旦队列被写满了，写者也需要被挂起等待。当阻塞发生后，读者一旦读取数据，channel 内部就会使用 Condvar 的 notify_one 通知写者，唤醒某个写者使其能够继续写入。

因此，实现中，一般会用到 Mutex + Condvar + VecDeque 来实现；如果不用 Condvar，可以直接使用 thread::park + thread::notify 来完成（flume 的做法）；如果不用 VecDeque，也可以使用双向链表或者其它的 ring buffer 的实现。

4. unbounded：queue 没有上限，如果写满了，就自动扩容。
我们知道，Rust 的很多数据结构如 Vec 、VecDeque 都是自动扩容的。unbounded 和 bounded 相比，除了不阻塞写者，其它实现都很类似。

> 所有这些 channel 类型，同步和异步的实现思路大同小异，主要的区别在于挂起 / 唤醒的对象：
- 在同步的世界里，挂起 / 唤醒的对象是线程；
- 而异步的世界里，是粒度很小的 task。

![34｜并发处理（下）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/34%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8B%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F-4951803.jpg)
--- 
根据 Channel 读者和写者的数量，Channel 又可以分为：

- SPSC：Single-Producer Single-Consumer，单生产者，单消费者。最简单，可以不依赖于 Mutex，只用 atomics 就可以实现。

- SPMC：Single-Producer Multi-Consumer，单生产者，多消费者。需要在消费者这侧读取时加锁。

- MPSC：Multi-Producer Single-Consumer，多生产者，单消费者。需要在生产者这侧写入时加锁。

- MPMC：Multi-Producer Multi-Consumer。多生产者，多消费者。需要在生产者写入或者消费者读取时加锁。

> 在众多 Channel 类型中，使用最广的是 MPSC channel，多生产者，单消费者，因为往往我们希望通过单消费者来保证，用于处理消息的数据结构有独占的写访问。

![34｜并发处理（下）：从atomics到Channel，Rust都提供了什么工具？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/34%EF%BD%9C%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86%EF%BC%88%E4%B8%8B%EF%BC%89%EF%BC%9A%E4%BB%8Eatomics%E5%88%B0Channel%EF%BC%8CRust%E9%83%BD%E6%8F%90%E4%BE%9B%E4%BA%86%E4%BB%80%E4%B9%88%E5%B7%A5%E5%85%B7%EF%BC%9F.jpg)

比如，[在 xunmi 的实现中](https://github.com/tyrchen/xunmi/blob/master/src/indexer.rs#L50): 
1. index writer 内部是一个多线程的实现
2. 但在使用时，我们需要用到它的可写引用。

如果要能够在各种上下文中使用 index writer，我们就不得不将其用 Arc<Mutex<T>> 包裹起来，但这样在索引大量数据时效率太低，所以我们可以用 MPSC channel，让各种上下文都把数据发送给单一的线程，使用 index writer 索引，这样就避免了锁：

```rust, editable

pub struct IndexInner {
    index: Index,
    reader: IndexReader,
    config: IndexConfig,
    updater: Sender<Input>,
}

pub struct IndexUpdater {
    sender: Sender<Input>,
    t2s: bool,
    schema: Schema,
}

impl Indexer {
    // 打开或者创建一个 index
    pub fn open_or_create(config: IndexConfig) -> Result<Self> {
        let schema = config.schema.clone();
        let index = if let Some(dir) = &config.path {
            fs::create_dir_all(dir)?;
            let dir = MmapDirectory::open(dir)?;
            Index::open_or_create(dir, schema.clone())?
        } else {
            Index::create_in_ram(schema.clone())
        };

        Self::set_tokenizer(&index, &config);

        let mut writer = index.writer(config.writer_memory)?;

        // 创建一个 unbounded MPSC channel
        let (s, r) = unbounded::<Input>();

        // 启动一个线程，从 channel 的 reader 中读取数据
        thread::spawn(move || {
            for input in r {
                // 然后用 index writer 处理这个 input
                if let Err(e) = input.process(&mut writer, &schema) {
                    warn!("Failed to process input. Error: {:?}", e);
                }
            }
        });

        // 把 channel 的 sender 部分存入 IndexInner 结构
        Self::new(index, config, s)
    }

    pub fn get_updater(&self) -> IndexUpdater {
        let t2s = TextLanguage::Chinese(true) == self.config.text_lang;
        // IndexUpdater 内部包含 channel 的 sender 部分
        // 由于是 MPSC channel，所以这里可以简单 clone 一下 sender
        // 这也意味着，我们可以创建任意多个 IndexUpdater 在不同上下文发送数据
        // 而数据最终都会通过 channel 给到上面创建的线程，由 index writer 处理
        IndexUpdater::new(self.updater.clone(), self.index.schema(), t2s)
    }
}
```

~~~

### Actor

~~~admonish info title="简单介绍Actor，并举例" collapsible=true
最后我们简单介绍一下 [actor model](https://en.wikipedia.org/wiki/Actor_model)，它在业界主要的使用者是 Erlang VM 以及 [akka](https://akka.io/)。actor 是一种有栈协程:
1. 每个 actor，有自己的一个独立的、轻量级的调用栈
2. 以及一个用来接受消息的消息队列（mailbox 或者 message queue）
3.  外界跟 actor 打交道的唯一手段就是，给它发送消息。

Rust 标准库没有 actor 的实现，但是社区里有比较成熟的 [actix](https://github.com/actix/actix)（大名鼎鼎的 actix-web 就是基于 actix 实现的），以及 [bastion](https://github.com/bastion-rs/bastion)。

下面的代码用 actix 实现了一个简单的 DummyActor，它可以接收一个 InMsg，返回一个 OutMsg：

```rust, editable

use actix::prelude::*;
use anyhow::Result;

// actor 可以处理的消息
#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "OutMsg")]
enum InMsg {
    Add((usize, usize)),
    Concat((String, String)),
}

#[derive(MessageResponse, Debug, Clone, PartialEq)]
enum OutMsg {
    Num(usize),
    Str(String),
}

// Actor
struct DummyActor;

impl Actor for DummyActor {
    type Context = Context<Self>;
}

// 实现处理 InMsg 的 Handler trait
impl Handler<InMsg> for DummyActor {
    type Result = OutMsg; // <-  返回的消息

    fn handle(&mut self, msg: InMsg, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InMsg::Add((a, b)) => OutMsg::Num(a + b),
            InMsg::Concat((mut s1, s2)) => {
                s1.push_str(&s2);
                OutMsg::Str(s1)
            }
        }
    }
}

#[actix::main]
async fn main() -> Result<()> {
    let addr = DummyActor.start();
    let res = addr.send(InMsg::Add((21, 21))).await?;
    let res1 = addr
        .send(InMsg::Concat(("hello, ".into(), "world".into())))
        .await?;

    println!("res: {:?}, res1: {:?}", res, res1);

    Ok(())
}
```

> 可以看到，对 DummyActor，我们只需要实现 Actor trait 和 Handler<InMsg> trait 。

~~~

### 小结一下各种并发原语的使用场景

~~~admonish info title="如何根据使用场景选择使用Atomic、Mutex、RwLock、Semaphore、Condvar、Channel、Actor" collapsible=true
>  Atomic、Mutex、RwLock、Semaphore、Condvar、Channel、Actor。

1. Atomic 在处理简单的原生类型时非常有用，如果你可以通过 AtomicXXX 结构进行同步，那么它们是最好的选择。
2. 当你的数据结构无法简单通过 AtomicXXX 进行同步，但你又的确需要在多个线程中共享数据，那么 Mutex / RwLock 可以是一种选择。不过，你需要考虑锁的粒度，粒度太大的 Mutex / RwLock 效率很低。
3. 如果你有 N 份资源可以供多个并发任务竞争使用，那么，Semaphore 是一个很好的选择。比如你要做一个 DB 连接池。
4. 当你需要在并发任务中通知、协作时，Condvar 提供了最基本的通知机制，而 Channel 把这个通知机制进一步广泛扩展开，于是你可以用 Condvar 进行点对点的同步，用 Channel 做一对多、多对一、多对多的同步。

> 所以，当我们做大部分复杂的系统设计时，Channel 往往是最有力的武器，除了可以让数据穿梭于各个线程、各个异步任务间，它的接口还可以很优雅地跟 stream 适配。

如果说在做整个后端的系统架构时，我们着眼的是：有哪些服务、服务和服务之间如何通讯、数据如何流动、服务和服务间如何同步；
那么在做某一个服务的架构时，着眼的是有哪些功能性的线程（异步任务）、它们之间的接口是什么样子、数据如何流动、如何同步。

在这里，Channel 兼具接口、同步和数据流三种功能，所以我说是最有力的武器。

然而它不该是唯一的武器。我们面临的真实世界的并发问题是多样的，解决方案也应该是多样的，计算机科学家们在过去的几十年里不断探索，构建了一系列的并发原语，也说明了很难有一种银弹解决所有问题。

就连 Mutex 本身，在实现中，还会根据不同的场景做不同的妥协（比如做 faireness 的妥协），因为这个世界就是这样，鱼与熊掌不可兼得，没有完美的解决方案，只有妥协出来的解决方案。所以 Channel 不是银弹，actor model 不是银弹，lock 不是银弹。

一门好的编程语言，可以提供大部分场景下的最佳实践（如 Erlang/Golang），但不该营造一种气氛，只有某个最佳实践才是唯一方案。我很喜欢 Erlang 的 actor model 和 Golang 的 Channel，但很可惜，它们过分依赖特定的、唯一的并发方案，使得开发者拿着榔头，看什么都是钉子。

相反，Rust 提供几乎你需要的所有解决方案，并且并不鼓吹它们的优劣，完全交由你按需选择。我在用 Rust 撰写多线程应用时，Channel 仍然是第一选择，但我还是会在合适的时候使用 Mutex、RwLock、Semaphore、Condvar、Atomic 等工具，而不是试图笨拙地用 Channel 叠加 Channel 来应对所有的场景。

~~~

## 自己实现一个基本的MPSC Channel

之前我们谈论了如何在搜索引擎的 Index writer 上使用 MPSC channel：

1. 要更新 index 的上下文有很多（可以是线程也可以是异步任务）
2. 而 IndexWriter 只能是唯一的。
3. 为了避免在访问 IndexWriter 时加锁，我们可以使用 MPSC channel
4. 在多个上下文中给 channel 发消息，然后在唯一拥有 IndexWriter 的线程中读取这些消息，非常高效。

好，来看看今天要实现的 MPSC channel 的基本功能。为了简便起见，我们只关心 unbounded MPSC
channel。也就是说，当队列容量不够时，会自动扩容，所以，任何时候生产者写入数据都不会被阻塞，但是当队列中没有数据时，消费者会被阻塞：

![35｜实操项目：如何实现一个基本的MPSC channel？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/35%EF%BD%9C%E5%AE%9E%E6%93%8D%E9%A1%B9%E7%9B%AE%EF%BC%9A%E5%A6%82%E4%BD%95%E5%AE%9E%E7%8E%B0%E4%B8%80%E4%B8%AA%E5%9F%BA%E6%9C%AC%E7%9A%84MPSC%20channel%EF%BC%9F.jpg)

### 测试驱动的设计

之前我们会从需求的角度来设计接口和数据结构，今天我们就换种方式，完全站在使用者的角度，用使用实例（测试）来驱动接口和数据结构的设计。

~~~admonish info title="需求1: 基本的 send/recv" collapsible=true
要实现刚才说的 MPSC channel，都有什么需求呢？首先，生产者可以产生数据，消费者能够消费产生出来的数据，也就是基本的 send/recv，我们以下面这个单元测试 1 来描述这个需求：

```rust, editable

#[test]
fn channel_should_work() {
    let (mut s, mut r) = unbounded();
    s.send("hello world!".to_string()).unwrap();
    let msg = r.recv().unwrap();
    assert_eq!(msg, "hello world!");
}
```

1. 这里，通过 unbounded() 方法， 可以创建一个 sender 和一个 receiver
2. sender 有 send() 方法，可以发送数据
3. receiver 有 recv() 方法，可以接受数据。
4. 整体的接口，我们设计和 std::sync::mpsc 保持一致，避免使用者使用上的心智负担。

为了实现这样一个接口，需要什么样的数据结构呢？
1. 首先，生产者和消费者之间会共享一个队列，可以用 VecDeque。
2. 显然，这个队列在插入和取出数据时需要互斥，所以需要用 Mutex 来保护它。

```rust, editable

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}
```

这样的数据结构应该可以满足单元测试 1。
~~~

~~~admonish info title="需求2: 允许多个 sender 往 channel 里发送数据 " collapsible=true
由于需要的是 MPSC，所以，我们允许多个 sender 往 channel 里发送数据，用单元测试 2 来描述这个需求：

```rust, editable

#[test]
fn multiple_senders_should_work() {
    let (mut s, mut r) = unbounded();
    let mut s1 = s.clone();
    let mut s2 = s.clone();
    let t = thread::spawn(move || {
        s.send(1).unwrap();
    });
    let t1 = thread::spawn(move || {
        s1.send(2).unwrap();
    });
    let t2 = thread::spawn(move || {
        s2.send(3).unwrap();
    });
    for handle in [t, t1, t2] {
        handle.join().unwrap();
    }

    let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
    // 在这个测试里，数据到达的顺序是不确定的，所以我们排个序再 assert
    result.sort();

    assert_eq!(result, [1, 2, 3]);
}
```

这个需求，刚才的数据结构就可以满足，只是 Sender 需要实现 Clone trait。不过我们在写这个测试的时候稍微有些别扭，因为这一行有不断重复的代码：

```rust, editable

let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
```

注意，测试代码的 DRY 也很重要，我们之前强调过。所以，当写下这个测试的时候，也许会想，我们可否提供 Iterator 的实现？恩这个想法先暂存下来。
~~~

~~~admonish info title="需求3: 当队列空的时候，receiver 所在的线程会被阻塞" collapsible=true
接下来考虑当队列空的时候，receiver 所在的线程会被阻塞这个需求。那么，如何对这个需求进行测试呢？这并不简单，我们没有比较直观的方式来检测线程的状态。

不过，我们可以通过检测“线程是否退出”来间接判断线程是否被阻塞。

理由很简单:
1. 如果线程没有继续工作，又没有退出，那么一定被阻塞住了。
2. 阻塞住之后，我们继续发送数据，消费者所在的线程会被唤醒，继续工作
3. 所以最终队列长度应该为 0。我们看单元测试 3：

```rust, editable

#[test]
fn receiver_should_be_blocked_when_nothing_to_read() {
    let (mut s, r) = unbounded();
    let mut s1 = s.clone();
    thread::spawn(move || {
        for (idx, i) in r.into_iter().enumerate() {
            // 如果读到数据，确保它和发送的数据一致
            assert_eq!(idx, i);
        }
        // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
        assert!(false);
    });

    thread::spawn(move || {
        for i in 0..100usize {
            s.send(i).unwrap();
        }
    });

    // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
    thread::sleep(Duration::from_millis(1));

    // 再次发送数据，唤醒消费者
    for i in 100..200usize {
        s1.send(i).unwrap();
    }

    // 留点时间让 receiver 处理
    thread::sleep(Duration::from_millis(1));

    // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
    assert_eq!(s1.total_queued_items(), 0);
}
```

这个测试代码中:
1. 我们假定 receiver 实现了 Iterator
2. 还假定 sender 提供了一个方法 total_queued_items()。这些可以在实现的时候再处理。

你可以花些时间仔细看看这段代码，想想其中的处理逻辑。虽然代码很简单，不难理解，但是把一个完整的需求转化成合适的测试代码，还是要颇费些心思的。

好，如果要能支持队列为空时阻塞，我们需要使用 Condvar。

所以 Shared<T> 需要修改一下：

```rust, editable

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}
```

这样当实现 Receiver 的 recv() 方法后，我们可以在读不到数据时阻塞线程：

```rust, editable

// 拿到锁
let mut inner = self.shared.queue.lock().unwrap();
// ... 假设读不到数据
// 使用 condvar 和 MutexGuard 阻塞当前线程
self.shared.available.wait(inner)
```

~~~

~~~admonish info title="需求4: Receiver没有数据可读" collapsible=true
顺着刚才的多个 sender 想，如果现在所有 Sender 都退出作用域，Receiver 继续接收，到没有数据可读了，该怎么处理？是不是应该产生一个错误，让调用者知道，现在 channel 的另一侧已经没有生产者了，再读也读不出数据了？

我们来写单元测试 4：

```rust, editable

#[test]
fn last_sender_drop_should_error_when_receive() {
    let (s, mut r) = unbounded();
    let s1 = s.clone();
    let senders = [s, s1];
    let total = senders.len();

    // sender 即用即抛
    for mut sender in senders {
        thread::spawn(move || {
            sender.send("hello").unwrap();
            // sender 在此被丢弃
        })
        .join()
        .unwrap();
    }

    // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
    for _ in 0..total {
        r.recv().unwrap();
    }

    // 然而，读取更多数据时会出错
    assert!(r.recv().is_err());
}
```

这个测试依旧很简单。你可以想象一下，使用什么样的数据结构可以达到这样的目的:
1. 首先，每次 Clone 时，要增加 Sender 的计数；
2. 在 Sender Drop 时，减少这个计数；
3. 然后，我们为 Receiver 提供一个方法 total_senders()，来读取 Sender 的计数
4. 当计数为 0，且队列中没有数据可读时，recv() 方法就报错。

> 有了这个思路，你想一想，这个计数器用什么数据结构呢？用锁保护么？

哈，你一定想到了可以使用 atomics。对，我们可以用 AtomicUsize。所以，Shared 数据结构需要更新一下：

```rust, editable

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
}
```

~~~

~~~admonish info title="需求5: 没有Receiver处理数据" collapsible=true
既然没有 Sender 了要报错，那么如果没有 Receiver 了，Sender 发送时是不是也应该错误返回？这个需求和上面类似，就不赘述了。看构造的单元测试 5：

```rust, editable

#[test]
fn receiver_drop_should_error_when_send() {
    let (mut s1, mut s2) = {
        let (s, _) = unbounded();
        let s1 = s.clone();
        let s2 = s.clone();
        (s1, s2)
    };

    assert!(s1.send(1).is_err());
    assert!(s2.send(1).is_err());
}
```

这里，我们创建一个 channel，产生两个 Sender 后便立即丢弃 Receiver。两个 Sender 在发送时都会出错。

同样的，Shared 数据结构要更新一下：

```rust, editable

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}
```

~~~

### 实现 MPSC channel

现在写了五个单元测试，我们已经把需求摸透了，并且有了基本的接口和数据结构的设计。接下来，我们来写实现的代码。

~~~admonish info title="创建一个新的项目" collapsible=true
```shell
cargo new con_utils --lib
```

1. 在 cargo.toml 中添加 anyhow 作为依赖。
2. 在 lib.rs 里，我们就写入一句：pub mod channel , 然后创建 src/channel.rs
3. 把刚才设计时使用的 test case、设计的数据结构，以及 test case 里使用到的接口，用代码全部放进来：

```rust, editable

use anyhow::Result;
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc, Condvar, Mutex},
};

/// 发送者
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// 接收者
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

/// 发送者和接收者之间共享一个 VecDeque，用 Mutex 互斥，用 Condvar 通知
/// 同时，我们记录有多少个 senders 和 receivers

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Sender<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        todo!()
    }

    pub fn total_receivers(&self) -> usize {
        todo!()
    }

    pub fn total_queued_items(&self) -> usize {
        todo!()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        todo!()
    }

    pub fn total_senders(&self) -> usize {
        todo!()
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// 克隆 sender
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

/// Drop sender
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        todo!()
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    // 此处省略所有 test case
}
```

~~~

~~~admonish info title="实现单元测试相关功能" collapsible=true
目前这个代码虽然能够编译通过，但因为没有任何实现，所以 cargo test 全部出错。接下来，我们就来一点点实现功能。

1. 创建 unbounded channel

```rust, editable

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver { shared },
    )
}

const INITIAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}
```

因为这里使用 default() 创建了 Shared<T> 结构，所以我们需要为其实现 Default。创建时，我们有 1 个生产者和 1 个消费者。

2. 实现消费者

对于消费者，我们主要需要实现 recv 方法。

在 recv 中:
- 如果队列中有数据，那么直接返回；
- 如果没数据，且所有生产者都离开了，我们就返回错误；
- 如果没数据，但还有生产者，我们就阻塞消费者的线程：

```rust, editable

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        // 拿到队列的锁
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                // 读到数据返回，锁被释放
                Some(t) => {
                    return Ok(t);
                }
                // 读不到数据，并且生产者都退出了，释放锁并返回错误
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                // 读不到数据，把锁提交给 available Condvar，它会释放锁并挂起线程，等待 notify
                None => {
                    // 当 Condvar 被唤醒后会返回 MutexGuard，我们可以 loop 回去拿数据
                    // 这是为什么 Condvar 要在 loop 里使用
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}
```

注意看这里 Condvar 的使用。
- 在 wait() 方法里，它接收一个 MutexGuard，然后释放这个 Mutex，挂起线程。
- 等得到通知后，它会再获取锁，得到一个 MutexGuard，返回。所以这里是：

```rust, editable

inner = self.shared.available.wait(inner).map_err(|_| anyhow!("lock poisoned"))?;
```

因为 recv() 会返回一个值，所以阻塞回来之后，我们应该循环回去拿数据。这是为什么这段逻辑要被 loop {} 包裹。我们前面在设计时考虑过：当发送者发送数据时，应该通知被阻塞的消费者。所以，在实现 Sender 的 send() 时，需要做相应的 notify 处理。

记得还要处理消费者的 drop：

```rust, editable

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}
```

很简单，消费者离开时，将 receivers 减一。
~~~

~~~admonish info title="实现生产者功能" collapsible=true
接下来我们看生产者的功能怎么实现。

1. 首先，在没有消费者的情况下，应该报错。
2. 正常应该使用 thiserror 定义自己的错误，不过这里为了简化代码，就使用 anyhow! 宏产生一个 adhoc 的错误。
3. 如果消费者还在，那么我们获取 VecDeque 的锁，把数据压入：

```rust, editable

impl<T> Sender<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        // 如果没有消费者了，写入时出错
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }

        // 加锁，访问 VecDeque，压入数据，然后立刻释放锁
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        // 通知任意一个被挂起等待的消费者有数据
        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}
```

这里，获取 total_receivers 时，我们使用了 Ordering::SeqCst，保证所有线程看到同样顺序的对 receivers 的操作。这个值是最新的值。

在压入数据时，需要判断一下之前是队列是否为空，因为队列为空的时候，我们需要用 notify_one() 来唤醒消费者。这个非常重要，如果没处理的话，会导致消费者阻塞后无法复原接收数据。

> 由于我们可以有多个生产者，所以要允许它 clone：

```rust, editable

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}
```

实现 Clone trait 的方法很简单，但记得要把 shared.senders 加 1，使其保持和当前的 senders 的数量一致。

> 当然，在 drop 的时候我们也要维护 shared.senders 使其减 1：

```rust, editable

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        
    }
}
```

~~~

~~~admonish info title="其他功能实现" collapsible=true
目前还缺乏 Receiver 的 Iterator 的实现，这个很简单，就是在 next() 里调用 recv() 方法，Rust 提供了支持在 Option / Result 之间很方便转换的函数，所以这里我们可以直接通过 ok() 来将 Result 转换成 Option：

```rust, editable

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}
```

好，目前所有需要实现的代码都实现完毕， cargo test 测试一下。wow！测试一次性通过！这也太顺利了吧！

最后来仔细审视一下代码。很快，我们发现 Sender 的 Drop 实现似乎有点问题。如果 Receiver 被阻塞，而此刻所有 Sender 都走了，那么 Receiver 就没有人唤醒，会带来资源的泄露。这是一个很边边角角的问题，所以之前的测试没有覆盖到。

我们来设计一个场景让这个问题暴露：

```rust, editable

#[test]
fn receiver_shall_be_notified_when_all_senders_exit() {
    let (s, mut r) = unbounded::<usize>();
    // 用于两个线程同步
    let (mut sender, mut receiver) = unbounded::<usize>();
    let t1 = thread::spawn(move || {
        // 保证 r.recv() 先于 t2 的 drop 执行
        sender.send(0).unwrap();
        assert!(r.recv().is_err());
    });

    thread::spawn(move || {
        receiver.recv().unwrap();
        drop(s);
    });

    t1.join().unwrap();
}
```

在我进一步解释之前，你可以停下来想想:
1. 为什么这个测试可以保证暴露这个问题？
2. 它是怎么暴露的？
3. 如果想不到，再 cargo test 看看会出现什么问题。

来一起分析分析，这里，我们创建了两个线程 t1 和 t2，分别让它们处理消费者和生产者。t1 读取数据，此时没有数据，所以会阻塞，而 t2 直接把生产者 drop 掉。所以，此刻如果没有人唤醒 t1，那么 t1.join() 就会一直等待，因为 t1 一直没有退出。

> 所以，为了保证一定是 t1 r.recv()先执行导致阻塞、t2 再 drop(s)，我们（eat your own dog food）用另一个 channel 来控制两个线程的执行顺序。这是一种很通用的做法，你可以好好琢磨一下。

运行 cargo test 后，测试被阻塞。这是因为，t1 没有机会得到唤醒，所以这个测试就停在那里不动了。

要修复这个问题，我们需要妥善处理 Sender 的 Drop：

```rust, editable

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        // sender 走光了，唤醒 receiver 读取数据（如果队列中还有的话），读不到就出错
        if old <= 1 {
            // 因为我们实现的是 MPSC，receiver 只有一个，所以 notify_all 实际等价 notify_one
            self.shared.available.notify_all();
        }
    }
}
```
这里，如果减一之前，旧的 senders 的数量小于等于 1，意味着现在是最后一个 Sender 要离开了，不管怎样我们都要唤醒 Receiver ，所以这里使用了 notify_all()。如果 Receiver 之前已经被阻塞，此刻就能被唤醒。修改完成，cargo test 一切正常。
~~~

~~~admonish info title="对锁进行性能优化" collapsible=true
从功能上来说，目前我们的 MPSC unbounded channel 没有太多的问题，可以应用在任何需要 MPSC channel 的场景。然而，每次读写都需要获取锁，虽然锁的粒度很小，但还是让整体的性能打了个折扣。有没有可能优化锁呢？

之前我们讲到，优化锁的手段无非是减小临界区的大小，让每次加锁的时间很短，这样冲突的几率就变小。另外，就是降低加锁的频率，对于消费者来说，如果我们能够一次性把队列中的所有数据都读完缓存起来，以后在需要的时候从缓存中读取，这样就可以大大减少消费者加锁的频次。

顺着这个思路，我们可以在 Receiver 的结构中放一个 cache：

```rust, editable

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}
```

如果你之前有 C 语言开发的经验，也许会想，到了这一步，何必把 queue 中的数据全部读出来，存入 Receiver 的 cache 呢？这样效率太低，如果能够直接 swap 两个结构内部的指针，这样，即便队列中有再多的数据，也是一个 O(1) 的操作。

Rust 有类似的 [std::mem::swap](https://doc.rust-lang.org/std/mem/fn.swap.html) 方法。比如

```rust, editable

use std::mem;

fn main() {
    let mut x = "hello world".to_string();
    let mut y = "goodbye world".to_string();
    
    mem::swap(&mut x, &mut y);
    
    assert_eq!("goodbye world", x);
    assert_eq!("hello world", y);
}
```

> 好，了解了 swap 方法，我们看看如何修改 Receiver 的 recv() 方法来提升性能：

```rust, editable

pub fn recv(&mut self) -> Result<T> {
    // 无锁 fast path
    if let Some(v) = self.cache.pop_front() {
        return Ok(v);
    }

    // 拿到队列的锁
    let mut inner = self.shared.queue.lock().unwrap();
    loop {
        match inner.pop_front() {
            // 读到数据返回，锁被释放
            Some(t) => {
                // 如果当前队列中还有数据，那么就把消费者自身缓存的队列（空）和共享队列 swap 一下
                // 这样之后再读取，就可以从 self.queue 中无锁读取
                if !inner.is_empty() {
                    std::mem::swap(&mut self.cache, &mut inner);
                }
                return Ok(t);
            }
            // 读不到数据，并且生产者都退出了，释放锁并返回错误
            None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
            // 读不到数据，把锁提交给 available Condvar，它会释放锁并挂起线程，等待 notify
            None => {
                // 当 Condvar 被唤醒后会返回 MutexGuard，我们可以 loop 回去拿数据
                // 这是为什么 Condvar 要在 loop 里使用
                inner = self
                    .shared
                    .available
                    .wait(inner)
                    .map_err(|_| anyhow!("lock poisoned"))?;
            }
        }
    }
}
```

1. 当 cache 中有数据时，总是从 cache 中读取；
2. 当 cache 中没有，我们拿到队列的锁，读取一个数据
3. 然后看看队列是否还有数据，有的话，就 swap cache 和 queue，然后返回之前读取的数据。

好，做完这个重构和优化，我们可以运行 cargo test，看看已有的测试是否正常。如果你遇到报错，应该是 cache 没有初始化，你可以自行解决，也可以参考：

```rust, editable

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE),
        },
    )
}
```

虽然现有的测试全数通过，但我们并没有为这个优化写测试，这里补个测试：

```rust, editable

#[test]
    fn channel_fast_path_should_work() {
    let (mut s, mut r) = unbounded();
    for i in 0..10usize {
        s.send(i).unwrap();
    }

    assert!(r.cache.is_empty());
    // 读取一个数据，此时应该会导致 swap，cache 中有数据
    assert_eq!(0, r.recv().unwrap());
    // 还有 9 个数据在 cache 中
    assert_eq!(r.cache.len(), 9);
    // 在 queue 里没有数据了
    assert_eq!(s.total_queued_items(), 0);

    // 从 cache 里读取剩下的数据
    for (idx, i) in r.into_iter().take(9).enumerate() {
        assert_eq!(idx + 1, i);
    }
}
```

这个测试很简单，详细注释也都写上了。
~~~

~~~admonish info title="完整代码" collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/35_con_utils/src/channel.rs}}
```
~~~

### 回顾测试驱动开发

~~~admonish info title="回顾测试驱动开发" collapsible=true
这里完全顺着需求写测试，然后在写测试的过程中进行数据结构和接口的设计。和普通的 TDD 不同的是，先一口气把主要需求涉及的行为用测试来表述，然后通过这个表述，构建合适的接口，以及能够运行这个接口的数据结构。

在开发产品的时候，这也是一种非常有效的手段，可以让我们通过测试完善设计，最终得到一个能够让测试编译通过的、完全没有实现代码、只有接口的版本。之后，我们再一个接口一个接口实现，全部实现完成之后，运行测试，看看是否出问题。

在这里你可以多多关注构建测试用例的技巧。之前的课程中，我反复强调过单元测试的重要性，也以身作则在几个重要的实操中都有详尽地测试。不过相比之前写的测试，这一讲中的测试要更难写一些，尤其是在并发场景下那些边边角角的功能测试。

不要小看测试代码，有时候构造测试代码比撰写功能代码还要烧脑。但是，当你有了扎实的单元测试覆盖后，再做重构，比如最后我们做和性能相关的重构，就变得轻松很多，因为只要cargo test通过，起码这个重构没有引起任何回归问题（regression bug）。

当然，重构没有引入回归问题，并不意味着重构完全没有问题，我们还需要考虑撰写新的测试，覆盖重构带来的改动。
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

## Future

### actor是有栈协程，Future是无栈协程

~~~admonish info title="actor是有栈协程，Future是无栈协程" collapsible=true
待补充
~~~

### Rust的Future

~~~admonish info title="Rust 的 Future 跟 JavaScript 的 Promise、Python的Future非常相似" collapsible=true
其实 Rust 的 Future 跟 JavaScript 的 Promise 非常类似。

如果你熟悉 JavaScript，应该熟悉 Promise 的概念，它代表了在未来的某个时刻才能得到的结果的值，Promise 一般存在三个状态；
1. 等待（pending）状态
2. Promise 已运行，但还未结束；
3. 结束状态，Promise 成功解析出一个值，或者执行失败。

只不过 JavaScript 的 Promise 和线程类似，一旦创建就开始执行，对 Promise await 只是为了“等待”并获取解析出来的值；而 Rust 的 Future，只有在主动 await 后才开始执行。
~~~

### Future和async/await

~~~admonish info title="一般而言，async/await和Future是什么关系" collapsible=true
讲到这里估计你也看出来了，谈 Future 的时候，我们总会谈到 async/await。

一般而言：
1. async 定义了一个可以并发执行的任务
2. 而 await 则触发这个任务并发执行。
3. 大多数语言，包括 Rust，async/await 都是一个语法糖（syntactic sugar）
4. 它们使用状态机将 Promise/Future 这样的结构包装起来进行处理。
~~~

### 为什么需要 Future？

~~~admonish info title="为什么需要Future，那不用async/await有什么问题？" collapsible=true
首先，谈一谈为什么需要 Future 这样的并发结构。

在 Future 出现之前，我们的 Rust 代码都是同步的。也就是说：
1. 当你执行一个函数，CPU 处理完函数中的每一个指令才会返回。
2. 如果这个函数里有 IO 的操作，实际上，操作系统会把函数对应的线程挂起，放在一个等待队列中
3. 直到 IO 操作完成，才恢复这个线程，并从挂起的位置继续执行下去。

> 这个模型非常简单直观，代码是一行一行执行的，开发者并不需要考虑哪些操作会阻塞，哪些不会，只关心他的业务逻辑就好。

> 然而，随着 CPU 技术的不断发展，新世纪应用软件的主要矛盾不再是 CPU 算力不足，而是过于充沛的 CPU 算力和提升缓慢的 IO 速度之间的矛盾。如果有大量的 IO 操作，你的程序大部分时间并没有在运算，而是在不断地等待 IO。

```rust, editable

use anyhow::Result;
use serde_yaml::Value;
use std::fs;

fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let content1 = fs::read_to_string("./Cargo.toml")?;
    // 读取 Cargo.lock，IO 操作 2
    let content2 = fs::read_to_string("./Cargo.lock")?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    fs::write("/tmp/Cargo.yml", &yaml1)?;
    // 写入 /tmp/Cargo.lock，IO 操作 4
    fs::write("/tmp/Cargo.lock", &yaml2)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
```

> 这段代码读取 Cargo.toml 和 Cargo.lock 将其转换成 yaml，再分别写入到 /tmp 下。

虽然说这段代码的逻辑并没有问题，但性能有很大的问题:
1. 在读 Cargo.toml 时，整个主线程被阻塞，直到 Cargo.toml 读完，才能继续读下一个待处理的文件。
2. 整个主线程，只有在运行 toml2yaml 的时间片内，才真正在执行计算任务，之前的读取文件以及之后的写入文件，CPU 都在闲置。

![38｜异步处理：Future是什么？它和asyncawait是什么关系？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/38%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9AFuture%E6%98%AF%E4%BB%80%E4%B9%88%EF%BC%9F%E5%AE%83%E5%92%8Casyncawait%E6%98%AF%E4%BB%80%E4%B9%88%E5%85%B3%E7%B3%BB%EF%BC%9F-4959308.jpg)

当然，你会辩解，在读文件的过程中，我们不得不等待，因为 toml2yaml 函数的执行有赖于读取文件的结果。

嗯没错，但是，这里还有很大的 CPU 浪费：我们读完第一个文件才开始读第二个文件，有没有可能两个文件同时读取呢？这样总共等待的时间是 max(time_for_file1, time_for_file2)，而非 time_for_file1 + time_for_file2 。

这并不难，我们可以把文件读取和写入的操作放入单独的线程中执行，比如（代码）：

```rust, editable

use anyhow::{anyhow, Result};
use serde_yaml::Value;
use std::{
    fs,
    thread::{self, JoinHandle},
};

/// 包装一下 JoinHandle，这样可以提供额外的方法
struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    /// 等待 thread 执行完（类似 await）
    pub fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("failed"))?
    }
}

fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let t1 = thread_read("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let t2 = thread_read("./Cargo.lock");

    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let t3 = thread_write("/tmp/Cargo.yml", yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let t4 = thread_write("/tmp/Cargo.lock", yaml2);

    let yaml1 = t3.thread_await()?;
    let yaml2 = t4.thread_await()?;

    fs::write("/tmp/Cargo.yml", &yaml1)?;
    fs::write("/tmp/Cargo.lock", &yaml2)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)?;
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        Ok::<_, anyhow::Error>(content)
    });
    MyJoinHandle(handle)
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
```
> 这样，读取或者写入多个文件的过程并发执行，使等待的时间大大缩短。

1. 但是，如果要同时读取 100 个文件呢？
- 显然，创建 100 个线程来做这样的事情不是一个好主意。
- 在操作系统中，线程的数量是有限的，创建 / 阻塞 / 唤醒 / 销毁线程，都涉及不少的动作
- 每个线程也都会被分配一个不小的调用栈
- 所以从 CPU 和内存的角度来看，创建过多的线程会大大增加系统的开销。

2. 其实，绝大多数操作系统对 I/O 操作提供了非阻塞接口，也就是说: 
- 你可以发起一个读取的指令
- 自己处理类似 EWOULDBLOCK这样的错误码
- 来更好地在同一个线程中处理多个文件的 IO
- 而不是依赖操作系统通过调度帮你完成这件事。

3. 不过这样就意味着，你需要:
- 定义合适的数据结构来追踪每个文件的读取
- 在用户态进行相应的调度
- 阻塞等待 IO 的数据结构的运行
- 让没有等待 IO 的数据结构得到机会使用 CPU
- 以及当 IO 操作结束后，恢复等待 IO 的数据结构的运行等等。
> 这样的操作粒度更小，可以最大程度利用 CPU 资源。
> 这就是类似 Future 这样的并发结构的主要用途。

4. 然而，如果这么处理，我们需要在用户态做很多事情,包括:
- 处理 IO 任务的事件通知
- 创建 Future
- 合理地调度 Future

> 这些事情，统统交给开发者做显然是不合理的。所以，Rust 提供了相应处理手段 async/await ：
- async 来方便地生成 Future
- await 来触发 Future 的调度和执行。
---
我们看看，同样的任务，如何用 async/await 更高效地处理（代码）：

```rust, editable

use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let f1 = fs::read_to_string("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let f2 = fs::read_to_string("./Cargo.lock");
    let (content1, content2) = try_join!(f1, f2)?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let f3 = fs::write("/tmp/Cargo.yml", &yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2);
    try_join!(f3, f4)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
```
在这段代码里:
1. 我们使用了 tokio::fs，而不是 std::fs
2. tokio::fs 的文件操作都会返回一个 Future，然后可以 join 这些 Future，得到它们运行后的结果。
3. join / try_join 是用来轮询多个 Future 的宏:
- 它会依次处理每个 Future
- 遇到阻塞就处理下一个
- 直到所有 Future 产生结果。

> 整个等待文件读取的时间是 max(time_for_file1, time_for_file2)，性能和使用线程的版本几乎一致，但是消耗的资源（主要是线程）要少很多。

建议你好好对比这三个版本的代码，写一写，运行一下，感受它们的处理逻辑。

注意在最后的 async/await 的版本中，我们不能把代码写成这样：

```rust, editable

// 读取 Cargo.toml，IO 操作 1
let content1 = fs::read_to_string("./Cargo.toml").await?;
// 读取 Cargo.lock，IO 操作 2
let content1 = fs::read_to_string("./Cargo.lock").await?;
```
这样写的话，和第一版同步的版本没有区别，因为 await 会运行 Future 直到 Future 执行结束，所以依旧是先读取 Cargo.toml，再读取 Cargo.lock，并没有达到并发的效果。
~~~

### 深入思路

~~~admonish info title="从async fn了解到future的思路" collapsible=true
1. 拆解 async fn 有点奇怪的返回值结构
2. 我们学习了 Reactor pattern
3. 大致了解了 tokio 如何通过 executor 和 reactor 共同作用，完成 Future 的调度、执行、阻塞，以及唤醒。这是一个完整的循环，直到 Future 返回 Poll::Ready(T)。
~~~

### 深入了解

好，了解了 Future 在软件开发中的必要性，来深入研究一下 Future/async/await。

~~~admonish info title="异步函数（async fn）其实是语法糖，它有等价函数写法。" collapsible=true
在前面代码撰写过程中，不知道你有没有发现，异步函数（async fn）的返回值是一个奇怪的 impl Future<Output> 的结构：

![38｜异步处理：Future是什么？它和asyncawait是什么关系？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/38%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9AFuture%E6%98%AF%E4%BB%80%E4%B9%88%EF%BC%9F%E5%AE%83%E5%92%8Casyncawait%E6%98%AF%E4%BB%80%E4%B9%88%E5%85%B3%E7%B3%BB%EF%BC%9F.png)

> 我们知道，一般会用 impl 关键字为数据结构实现 trait，也就是说接在 impl 关键字后面的东西是一个 trait，所以，显然 Future 是一个 trait，并且还有一个关联类型 Output。

来看 [Future 的定义](https://doc.rust-lang.org/std/future/trait.Future.html)：

```rust, editable

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

1. 除了 Output 外，它还有一个 poll() 方法，这个方法返回 [PollSelf::Output](https://doc.rust-lang.org/std/future/trait.Future.html)。
2. 而 Poll<T> 是个 enum，包含 Ready 和 Pending 两个状态。
3. 显然，当 Future 返回 Pending 状态时，活还没干完，但干不下去了，需要阻塞一阵子，等某个事件将其唤醒；
4. 当 Future 返回 Ready 状态时，Future 对应的值已经得到，此时可以返回了。

> 你看，这样一个简单的数据结构，就托起了庞大的 Rust 异步 async/await 处理的生态。

回到 async fn 的返回值我们接着说，显然它是一个 impl Future.

> 那么如果我们给一个普通的函数返回 impl Future<Output>，它的行为和 async fn 是不是一致呢？

来写个简单的实验（代码）：

```rust, editable

use futures::executor::block_on;
use std::future::Future;

#[tokio::main]
async fn main() {
    let name1 = "Tyr".to_string();
    let name2 = "Lindsey".to_string();

    say_hello1(&name1).await;
    say_hello2(&name2).await;

    // Future 除了可以用 await 来执行外，还可以直接用 executor 执行
    block_on(say_hello1(&name1));
    block_on(say_hello2(&name2));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello {}", name);
    42
}

// async fn 关键字相当于一个返回 impl Future<Output> 的语法糖
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output = usize> + 'fut {
    async move {
        println!("Hello {}", name);
        42
    }
}
```
> 运行这段代码你会发现，say_hello1 和 say_hello2 是等价的，二者都可以使用 await 来执行，也可以将其提供给一个 executor 来执行。
~~~

### executor

~~~admonish info title="executor是什么-> Rust如何支持->Rust常用的executor有哪些？" collapsible=true
这里我们见到了一个新的名词：executor。

什么是 executor？

1. 你可以把 executor 大致想象成一个 Future 的调度器。
2. 对于线程来说，操作系统负责调度；
3. 但操作系统不会去调度用户态的协程（比如 Future），所以任何使用了协程来处理并发的程序，都需要有一个 executor 来负责协程的调度。

很多在语言层面支持协程的编程语言，比如 Golang / Erlang，都自带一个用户态的调度器。

Rust 虽然也提供 Future 这样的协程，但它在语言层面并不提供 executor，把要不要使用 executor 和使用什么样的 executor 的自主权交给了开发者。
- 好处是，当我的代码中不需要使用协程时，不需要引入任何运行时；
- 而需要使用协程时，可以在生态系统中选择最合适我应用的 executor。

---

常见的 executor 有：

1. futures 库自带的很简单的 executor，上面的代码就使用了它的 block_on 函数；
2. tokio 提供的 executor，当使用 #[tokio::main] 时，就隐含引入了 tokio 的 executor；
3. [async-std](https://github.com/async-rs/async-std) 提供的 executor，和 tokio 类似；
4. [smol 提供的 async-executor](https://github.com/smol-rs/smol)，主要提供了 block_on。

注意，上面的代码我们混用了 #[tokio::main] 和 futures:executor::block_on，这只是为了展示 Future 使用的不同方式，在正式代码里，不建议混用不同的 executor，会降低程序的性能，还可能引发奇怪的问题。
~~~

### reactor pattern

~~~admonish info title="Reactor Pattern如何组成？" collapsible=true
当我们谈到 executor 时，就不得不提 reactor，它俩都是 Reactor Pattern 的组成部分，作为构建高性能事件驱动系统的一个很典型模式，Reactor pattern 它包含三部分：

1. task，待处理的任务

任务可以被打断，并且把控制权交给 executor，等待之后的调度；

2. executor，一个调度器。

维护等待运行的任务（ready queue），以及被阻塞的任务（wait queue）；

3. reactor，维护事件队列。

当事件来临时，通知 executor 唤醒某个任务等待运行。

- executor 会调度执行待处理的任务，当任务无法继续进行却又没有完成时，它会挂起任务，并设置好合适的唤醒条件。
- 之后，如果 reactor 得到了满足条件的事件，它会唤醒之前挂起的任务，然后 executor 就有机会继续执行这个任务。
- 这样一直循环下去，直到任务执行完毕。
~~~

### 怎么用 Future 做异步处理？

~~~admonish info title="Rust如何基于Reactor pattern使用Future做异步处理" collapsible=true
理解了 Reactor pattern 后，Rust 使用 Future 做异步处理的整个结构就清晰了，我们以 tokio 为例：

1. async/await 提供语法层面的支持
2. Future 是异步任务的数据结构
3. 当 fut.await 时，executor 就会调度并执行它。
---
- tokio 的调度器（executor）会运行在多个线程上，运行线程自己的 ready queue 上的任务（Future）
- 如果没有，就去别的线程的调度器上“偷”一些过来运行。
- 当某个任务无法再继续取得进展，此时 Future 运行的结果是 Poll::Pending，那么调度器会挂起任务，并设置好合适的唤醒条件（Waker），等待被 reactor 唤醒。
- 而 reactor 会利用操作系统提供的异步 I/O，比如 epoll / kqueue / IOCP，来监听操作系统提供的 IO 事件，当遇到满足条件的事件时，就会调用 Waker.wake() 唤醒被挂起的 Future。这个 Future 会回到 ready queue 等待执行。
---
整个流程如下：

![38｜异步处理：Future是什么？它和asyncawait是什么关系？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/38%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9AFuture%E6%98%AF%E4%BB%80%E4%B9%88%EF%BC%9F%E5%AE%83%E5%92%8Casyncawait%E6%98%AF%E4%BB%80%E4%B9%88%E5%85%B3%E7%B3%BB%EF%BC%9F.jpg)
---
我们以一个具体的代码示例来进一步理解这个过程（代码）：

```rust, editable

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            for line in r.next().await {
                // 每读到一行就加个前缀发回
                w.send(format!("I got: {}", line?)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
```

这是一个简单的 TCP 服务器:
1. 服务器每收到一个客户端的请求，就会用[ tokio::spawn ](https://docs.rs/tokio/1.13.0/tokio/fn.spawn.html)创建一个异步任务，放入 executor 中执行。
2. 这个异步任务接受客户端发来的按行分隔（分隔符是 “\r\n”）的数据帧，服务器每收到一行，就加个前缀把内容也按行发回给客户端。
3. 你可以用 telnet 和这个服务器交互：

```shell
❯ telnet localhost 8080
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
I got: hello
Connection closed by foreign host.
```

- 假设我们在客户端输入了很大的一行数据，服务器在做 r.next().await 在执行的时候，收不完一行的数据，因而这个 Future 返回 Poll::Pending，此时它被挂起。
- 当后续客户端的数据到达时，reactor 会知道这个 socket 上又有数据了，于是找到 socket 对应的 Future，将其唤醒，继续接收数据。
- 这样反复下去，最终 r.next().await 得到 Poll::Ready(Ok(line))，于是它返回 Ok(line)，程序继续往下走，进入到 w.send() 的阶段。

> 从这段代码中你可以看到，在 Rust 下使用异步处理是一件非常简单的事情

1. 除了几个你可能不太熟悉的概念:
- 比如用于创建 Future 的 async 关键字
- 用于执行和等待 Future 执行完毕的 await 关键字
- 以及用于调度 Future 执行的运行时 #[tokio:main] 

2. 整体的代码和使用线程处理的代码完全一致。所以，它的上手难度非常低，很容易使用。
~~~

### 使用 Future 的注意事项

~~~admonish info title="使用Future处理异步任务的三个注意事项" collapsible=true
目前我们已经基本明白 Future 运行的基本原理了，也可以在程序的不同部分自如地使用 Future/async/await 来进行异步处理。

但是要注意，不是所有的应用场景都适合用 async/await，在使用的时候，有一些不容易注意到的坑需要我们妥善考虑。

1. 处理计算密集型任务时

当你要处理的任务是 CPU 密集型，而非 IO 密集型，更适合使用线程，而非 Future。

这是因为 Future 的调度是协作式多任务（Cooperative Multitasking），也就是说，除非 Future 主动放弃 CPU，不然它就会一直被执行，直到运行结束。我们看一个例子（代码）：

```rust, editable

use anyhow::Result;
use std::time::Duration;

// 强制 tokio 只使用一个工作线程，这样 task 2 不会跑到其它线程执行
#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
    // 先开始执行 task 1 的话会阻塞，让 task 2 没有机会运行
    tokio::spawn(async move {
        eprintln!("task 1");
        // 试试把这句注释掉看看会产生什么结果
        // tokio::time::sleep(Duration::from_millis(1)).await;
        loop {}
    });

    tokio::spawn(async move {
        eprintln!("task 2");
    });

    tokio::time::sleep(Duration::from_millis(1)).await;
    Ok(())
}
```

task 1 里有一个死循环，你可以把它想象成是执行时间很长又不包括 IO 处理的代码。运行这段代码，你会发现，task 2 没有机会得到执行。这是因为 task 1 不执行结束，或者不让出 CPU，task 2 没有机会被调度。

2. 异步代码中使用 Mutex 时

大部分时候，标准库的 Mutex 可以用在异步代码中，而且，这是推荐的用法。

然而，标准库的 MutexGuard 不能安全地跨越 await，所以，当我们需要获得锁之后执行异步操作，必须使用 tokio 自带的 Mutex，看下面的例子（代码）：

```rust, editable

use anyhow::Result;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

struct DB;

impl DB {
    // 假装在 commit 数据
    async fn commit(&mut self) -> Result<usize> {
        Ok(42)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = Arc::clone(&db1);

    tokio::spawn(async move {
        let mut db = db1.lock().await;
        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
        let affected = db.commit().await?;
        println!("db1: Total affected rows: {}", affected);
        Ok::<_, anyhow::Error>(())
    });

    tokio::spawn(async move {
        let mut db = db2.lock().await;
        let affected = db.commit().await?;
        println!("db2: Total affected rows: {}", affected);

        Ok::<_, anyhow::Error>(())
    });

    // 让两个 task 有机会执行完
    tokio::time::sleep(Duration::from_millis(1)).await;

    Ok(())
}
```

> 这个例子模拟了一个数据库的异步 commit() 操作
- 如果我们需要在多个 tokio task 中使用这个 DB，需要使用 Arc<Mutext<DB>>。
- 然而，db1.lock() 拿到锁后，我们需要运行 db.commit().await，这是一个异步操作。
- 前面讲过，因为 tokio 实现了 work-stealing 调度，Future 有可能在不同的线程中执行，普通的 MutexGuard 编译直接就会出错，所以需要使用 tokio 的 Mutex。[更多信息可以看文档](https://docs.rs/tokio/1.13.0/tokio/sync/struct.Mutex.html)。

> 在这个例子里，我们又见识到了 Rust 编译器的伟大之处：如果一件事，它觉得你不能做，会通过编译器错误阻止你，而不是任由编译通过，然后让程序在运行过程中听天由命，让你无休止地和捉摸不定的并发 bug 斗争。

3. 在线程和异步任务间做同步时

在一个复杂的应用程序中，会兼有计算密集和 IO 密集的任务。

前面说了，要避免在 tokio 这样的异步运行时中运行大量计算密集型的任务，一来效率不高，二来还容易饿死其它任务。

所以，一般的做法是我们使用 channel 来在线程和 future 两者之间做同步。看一个例子：

```rust, editable

use std::thread;

use anyhow::Result;
use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::prelude::*;
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);

    // 创建 tokio task 和 thread 之间的 channel
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用 thread 处理计算密集型任务
    thread::spawn(move || {
        // 读取从 tokio task 过来的 msg，注意这里用的是 blocking_recv，而非 await
        while let Some((line, reply)) = receiver.blocking_recv() {
            // 计算 pow
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, once: {}", hash, nonce),
                None => "Not found".to_string(),
            };
            // 把计算结果从 oneshot channel 里发回
            if let Err(e) = reply.send(result) {
                println!("Failed to send: {}", e);
            }
        }
    });

    // 使用 tokio task 处理 IO 密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        let sender1 = sender.clone();
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            for line in r.next().await {
                // 为每个消息创建一个 oneshot channel，用于发送回复
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line?, reply))?;

                // 接收 pow 计算完成后的 hash 和 nonce
                if let Ok(v) = reply_receiver.await {
                    w.send(format!("Pow calculated: {}", v)).await?;
                }
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}

// 使用 rayon 并发计算 u32 空间下所有 nonce，直到找到有头 N 个 0 的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

// 计算携带 nonce 后的哈希
fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

// 计算数据的哈希
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
```

在这个例子里:
1. 我们使用了之前撰写的 TCP server
2. 只不过这次，客户端输入过来的一行文字，会被计算出一个 POW（Proof of Work）的哈希
3. 调整 nonce，不断计算哈希，直到哈希的头三个字节全是零为止。
4. 服务器要返回计算好的哈希和获得该哈希的 nonce。
5. 这是一个典型的计算密集型任务，所以我们需要使用线程来处理它。
6. 而在 tokio task 和 thread 间使用 channel 进行同步。我们使用了一个 ubounded MPSC channel 从 tokio task 侧往 thread 侧发送消息，每条消息都附带一个 oneshot channel 用于 thread 侧往 tokio task 侧发送数据。

> 建议你仔细读读这段代码，最好自己写一遍，感受一下使用 channel 在计算密集型和 IO 密集型任务同步的方式。

如果你用 telnet 连接，发送 “hello world!”，会得到不同的哈希和 nonce，它们都是正确的结果：

```shell

❯ telnet localhost 8080
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello world!
Pow calculated: hash: 0000006e6e9370d0f60f06bdc288efafa203fd99b9af0480d040b2cc89c44df0, once: 403407307
Connection closed by foreign host.

❯ telnet localhost 8080
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello world!
Pow calculated: hash: 000000e23f0e9b7aeba9060a17ac676f3341284800a2db843e2f0e85f77f52dd, once: 36169623
Connection closed by foreign host.

```
~~~

### 对比线程学习Future

~~~admonish info title="对比线程来学习future" collapsible=true
在学习 Future 的使用时，估计你也发现了，我们可以对比线程来学习，可以看到，下列代码的结构多么相似：

```rust, editable

fn thread_async() -> JoinHandle<usize> {
    thread::spawn(move || {
        println!("hello thread!");
        42
    })
}

fn task_async() -> impl Future<Output = usize> {
    async move {
        println!("hello async!");
        42
    }
}
```

~~~

### 为什么标准库的 Mutex 不能跨越 await？

想想看，为什么标准库的 Mutex 不能跨越 await？

你可以把文中使用 tokio::sync::Mutex 的代码改成使用 std::sync::Mutex，并对使用的接口做相应的改动（把 lock().await 改成 lock().unwrap()），看看编译器会报什么错。

对着错误提示，你明白为什么了么？

## async/await内部是怎么实现的？

~~~admonish info title="对 Future 和 async/await 的基本概念有一个比较扎实的理解" collapsible=true
对 Future 和 async/await 的基本概念有一个比较扎实的理解:
1. 知道在什么情况下该使用 Future
2. 什么情况下该使用 Thread
3. executor 和 reactor 是怎么联动最终让 Future 得到了一个结果。
~~~

然而，我们并不清楚为什么 async fn 或者 async block 就能够产生 Future，也并不明白 Future 是怎么被 executor 处理的。继续深入下去，看看 async/await
这两个关键词究竟施了什么样的魔法，能够让一切如此简单又如此自然地运转起来。

### Context、Pin

~~~admonish info title="从Future定义中的Pin和Context开始" collapsible=true
提前说明一下，我们会继续围绕着 Future 这个简约却又并不简单的接口，来探讨一些原理性的东西，主要是 Context 和 Pin 这两个结构：

```rust, editable

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
~~~

### Context::waker: Waker 的调用机制

~~~admonish info title="Context 就是 Waker 的一个封装" collapsible=true
先来看这个接口的 Context 是个什么东西。

1. executor 通过调用 poll 方法来让 Future 继续往下执行
2. 如果 poll 方法返回 Poll::Pending，就阻塞 Future
3. 直到 reactor 收到了某个事件，然后调用 Waker.wake() 把 Future 唤醒。
4. 这个 Waker 是哪来的呢？

其实，它隐含在 Context 中：

```rust, editable

pub struct Context<'a> {
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

所以，Context 就是 Waker 的一个封装。

如果你去看 Waker 的定义和相关的代码，会发现它非常抽象，内部使用了一个 vtable 来允许各种各样的 waker 的行为：

```rust, editable

pub struct RawWakerVTable {
    clone: unsafe fn(*const ()) -> RawWaker,
    wake: unsafe fn(*const ()),
    wake_by_ref: unsafe fn(*const ()),
    drop: unsafe fn(*const ()),
}
```

1. 这种手工生成 vtable 的做法，可以最大程度兼顾效率和灵活性。
2. Rust 自身并不提供异步运行时，它只在标准库里规定了一些基本的接口，至于怎么实现，可以由各个运行时（如 tokio）自行决定。
3. 所以在标准库中，你只会看到这些接口的定义，以及“高层”接口的实现，比如 Waker 下的 wake 方法，只是调用了 vtable 里的 wake() 而已：

```rust, editable

impl Waker {
    /// Wake up the task associated with this `Waker`.
    #[inline]
    pub fn wake(self) {
        // The actual wakeup call is delegated through a virtual function call
        // to the implementation which is defined by the executor.
        let wake = self.waker.vtable.wake;
        let data = self.waker.data;

        // Don't call `drop` -- the waker will be consumed by `wake`.
        crate::mem::forget(self);

        // SAFETY: This is safe because `Waker::from_raw` is the only way
        // to initialize `wake` and `data` requiring the user to acknowledge
        // that the contract of `RawWaker` is upheld.
        unsafe { (wake)(data) };
    }
    ...
}
```

> 如果你想顺藤摸瓜找到 vtable 是怎么设置的，却发现一切线索都悄无声息地中断了，那是因为，具体的实现并不在标准库中，而是在第三方的异步运行时里，比如 tokio。

~~~

不过，虽然我们开发时会使用 tokio，但阅读、理解代码时，我建议看 futures 库，比如 waker vtable 的定义。futures 库还有一个简单的 executor，也非常适合进一步通过代码理解 executor 的原理。

### async 究竟生成了什么？

我们接下来看 Pin。这是一个奇怪的数据结构，正常数据结构的方法都是直接使用 self / &self / &mut self，可是 poll() 却使用了 Pin<&mut self>，为什么？

~~~admonish info title="Rust 在编译 async fn 或者 async block 时，就会生成类似的状态机的实现" collapsible=true
为了讲明白 Pin，我们得往前追踪一步，看看产生 Future 的一个 async block/fn 内部究竟生成了什么样的代码？来看下面这个简单的 async 函数：

```rust, editable

async fn write_hello_file_async(name: &str) -> anyhow::Result<()> {
    let mut file = fs::File::create(name).await?;
    file.write_all(b"hello world!").await?;

    Ok(())
}
```

1. 首先它创建一个文件，然后往这个文件里写入 “hello world!”。
2. 这个函数有两个 await，创建文件的时候会异步创建，写入文件的时候会异步写入。
3. 最终，整个函数对外返回一个 Future。

> 其它人可以这样调用：

```rust, editable

write_hello_file_async("/tmp/hello").await?;
```

我们知道，executor 处理 Future 时，会不断地调用它的 poll() 方法，于是，上面那句实际上相当于：

```rust, editable

match write_hello_file_async.poll(cx) {
    Poll::Ready(result) => return result,
    Poll::Pending => return Poll::Pending
}
```

这是单个 await 的处理方法，那更加复杂的，一个函数中有若干个 await，该怎么处理呢？

以前面write_hello_file_async 函数的内部实现为例，显然，我们只有在处理完 create()，才能处理 write_all()，所以，应该是类似这样的代码：

```rust, editable

let fut = fs::File::create(name);
match fut.poll(cx) {
    Poll::Ready(Ok(file)) => {
        let fut = file.write_all(b"hello world!");
        match fut.poll(cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => return Poll::Pending,
        }
    }
    Poll::Pending => return Poll::Pending,
}
```

> 但是，前面说过，async 函数返回的是一个 Future，所以，还需要把这样的代码封装在一个 Future 的实现里，对外提供出去。

因此，我们需要实现一个数据结构，把内部的状态保存起来，并为这个数据结构实现 Future。比如：

```rust, editable

enum WriteHelloFile {
    // 初始阶段，用户提供文件名
    Init(String),
    // 等待文件创建，此时需要保存 Future 以便多次调用
    // 这是伪代码，impl Future 不能用在这里
    AwaitingCreate(impl Future<Output = Result<fs::File, std::io::Error>>),
    // 等待文件写入，此时需要保存 Future 以便多次调用
    AwaitingWrite(impl Future<Output = Result<(), std::io::Error>>),
    // Future 处理完毕
    Done,
}

impl WriteHelloFile {
    pub fn new(name: impl Into<String>) -> Self {
        Self::Init(name.into())
    }
}

impl Future for WriteHelloFile {
    type Output = Result<(), std::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

fn write_hello_file_async(name: &str) -> WriteHelloFile {
    WriteHelloFile::new(name)
}
```

这样，我们就把刚才的 write_hello_file_async 异步函数，转化成了一个返回 WriteHelloFile Future 的函数。

来看这个 Future 如何实现（详细注释了）：

```rust, editable

impl Future for WriteHelloFile {
    type Output = Result<(), std::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        loop {
            match this {
                // 如果状态是 Init，那么就生成 create Future，把状态切换到 AwaitingCreate
                WriteHelloFile::Init(name) => {
                    let fut = fs::File::create(name);
                    *self = WriteHelloFile::AwaitingCreate(fut);
                }
                // 如果状态是 AwaitingCreate，那么 poll create Future
                // 如果返回 Poll::Ready(Ok(_))，那么创建 write Future
                // 并把状态切换到 Awaiting
                WriteHelloFile::AwaitingCreate(fut) => match fut.poll(cx) {
                    Poll::Ready(Ok(file)) => {
                        let fut = file.write_all(b"hello world!");
                        *self = WriteHelloFile::AwaitingWrite(fut);
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Pending => return Poll::Pending,
                },
                // 如果状态是 AwaitingWrite，那么 poll write Future
                // 如果返回 Poll::Ready(_)，那么状态切换到 Done，整个 Future 执行成功
                WriteHelloFile::AwaitingWrite(fut) => match fut.poll(cx) {
                    Poll::Ready(result) => {
                        *self = WriteHelloFile::Done;
                        return Poll::Ready(result);
                    }
                    Poll::Pending => return Poll::Pending,
                },
                // 整个 Future 已经执行完毕
                WriteHelloFile::Done => return Poll::Ready(Ok(())),
            }
        }
    }
}
```

> 这个 Future 完整实现的内部结构 ，其实就是一个状态机的迁移。

这段（伪）代码和之前异步函数是等价的：

```rust, editable

async fn write_hello_file_async(name: &str) -> anyhow::Result<()> {
    let mut file = fs::File::create(name).await?;
    file.write_all(b"hello world!").await?;

    Ok(())
}
```

> Rust 在编译 async fn 或者 async block 时，就会生成类似的状态机的实现。你可以看到，看似简单的异步处理，内部隐藏了一套并不难理解、但是写起来很生硬很啰嗦的状态机管理代码。
~~~

好搞明白这个问题，回到 pin 。刚才我们手写状态机代码的过程，能帮你理解为什么会需要 Pin 这个问题。

### 为什么需要 Pin？

~~~admonish info title="什么场景下会出现自引用数据结构？有什么问题需要pin解决？" collapsible=true
在上面实现 Future 的状态机中，我们引用了 file 这样一个局部变量：

```rust, editable

WriteHelloFile::AwaitingCreate(fut) => match fut.poll(cx) {
    Poll::Ready(Ok(file)) => {
        let fut = file.write_all(b"hello world!");
        *self = WriteHelloFile::AwaitingWrite(fut);
    }
    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
    Poll::Pending => return Poll::Pending,
}
```

这个代码是有问题的，file 被 fut 引用，但 file 会在这个作用域被丢弃。

所以，我们需要把它保存在数据结构中：

```rust, editable

enum WriteHelloFile {
    // 初始阶段，用户提供文件名
    Init(String),
    // 等待文件创建，此时需要保存 Future 以便多次调用
    AwaitingCreate(impl Future<Output = Result<fs::File, std::io::Error>>),
    // 等待文件写入，此时需要保存 Future 以便多次调用
    AwaitingWrite(AwaitingWriteData),
    // Future 处理完毕
    Done,
}

struct AwaitingWriteData {
    fut: impl Future<Output = Result<(), std::io::Error>>,
    file: fs::File,
}
```

1. 可以生成一个 AwaitingWriteData 数据结构，把 file 和 fut 都放进去
2. 然后在 WriteHelloFile 中引用它。
3. 此时，在同一个数据结构内部，fut 指向了对 file 的引用，这样的数据结构，叫自引用结构（Self-Referential Structure）。

> 自引用结构有一个很大的问题是：一旦它被移动，原本的指针就会指向旧的地址。

![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F-4961729.jpg)

所以需要有某种机制来保证这种情况不会发生。

Pin 就是为这个目的而设计的一个数据结构: 我们可以 Pin 住指向一个 Future 的指针

看文稿中 Pin 的声明：

```rust, editable

pub struct Pin<P> {
    pointer: P,
}

impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        Pin::get_ref(Pin::as_ref(self))
    }
}

impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
}
```

1. Pin 拿住的是一个可以解引用成 T 的指针类型 P，而不是直接拿原本的类型 T。
2. 所以，对于 Pin 而言，你看到的都是 Pin<Box<T>>、Pin<&mut T>，但不会是 Pin<T>。
3. 因为 Pin 的目的是，把 T 的内存位置锁住，从而避免移动后自引用类型带来的引用失效问题。

![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F-4961721.jpg)

这样数据结构可以正常访问，但是你无法直接拿到原来的数据结构进而移动它。
~~~

### 自引用数据结构

~~~admonish bug title="举例说明自引用类型存在什么潜在危害" collapsible=true
当然，自引用数据结构并非只在异步代码里出现，只不过异步代码在内部生成用状态机表述的 Future 时，很容易产生自引用结构。我们看一个和 Future 无关的例子（代码）：
```rust, editable

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后指向 name
    name_ptr: *const String,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String;
    }

    pub fn print_name(&self) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            // 在使用 ptr 是需要 unsafe
            // SAFETY: 这里 name_ptr 潜在不安全，会指向旧的位置
            unsafe { &*self.name_ptr },
        );
    }
}

fn main() {
    let data = move_creates_issue();
    println!("data: {:?}", data);
    // 如果把下面这句注释掉，程序运行会直接 segment error
    // data.print_name();
    print!("\\n");
    mem_swap_creates_issue();
}

fn move_creates_issue() -> SelfReference {
    let mut data = SelfReference::new("Tyr");
    data.init();

    // 不 move，一切正常
    data.print_name();

    let data = move_it(data);

    // move 之后，name_ref 指向的位置是已经失效的地址
    // 只不过现在 move 前的地址还没被回收挪作它用
    data.print_name();
    data
}

fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("Tyr");
    data1.init();

    let mut data2 = SelfReference::new("Lindsey");
    data2.init();

    data1.print_name();
    data2.print_name();

    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn move_it(data: SelfReference) -> SelfReference {
    data
}
```

1. 我们创建了一个自引用结构 SelfReference，它里面的 name_ref 指向了 name。
2. 正常使用它时，没有任何问题
3. 但一旦对这个结构做 move 操作，name_ref 指向的位置还会是 move 前 name 的地址，这就引发了问题。看下图：

![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F-4961711.jpg)

同样的，如果我们使用 std::mem:swap，也会出现类似的问题，一旦 swap，两个数据的内容交换，然而，由于 name_ref 指向的地址还是旧的，所以整个指针体系都混乱了：

![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F-4961697.jpg)

看代码的输出，辅助你理解：

```shell

struct 0x7ffeea91d6e8: (name: 0x7ffeea91d6e8 name_ptr: 0x7ffeea91d6e8), name: Tyr, name_ref: Tyr
struct 0x7ffeea91d760: (name: 0x7ffeea91d760 name_ptr: 0x7ffeea91d6e8), name: Tyr, name_ref: Tyr
data: SelfReference { name: "Tyr", name_ptr: 0x7ffeea91d6e8 }

struct 0x7ffeea91d6f0: (name: 0x7ffeea91d6f0 name_ptr: 0x7ffeea91d6f0), name: Tyr, name_ref: Tyr
struct 0x7ffeea91d710: (name: 0x7ffeea91d710 name_ptr: 0x7ffeea91d710), name: Lindsey, name_ref: Lindsey
struct 0x7ffeea91d6f0: (name: 0x7ffeea91d6f0 name_ptr: 0x7ffeea91d710), name: Lindsey, name_ref: Tyr
struct 0x7ffeea91d710: (name: 0x7ffeea91d710 name_ptr: 0x7ffeea91d6f0), name: Tyr, name_ref: Lindsey
```

> 可以看到，swap 之后，name_ref 指向的内容确实和 name 不一样了。这就是自引用结构带来的问题。

你也许会奇怪，不是说 move 也会出问题么？为什么第二行打印 name_ref 还是指向了 “Tyr”？

这是因为 move 后，之前的内存失效，但是内存地址还没有被挪作它用，所以还能正常显示 “Tyr”。但这样的内存访问是不安全的，如果你把 main 中这句代码注释掉，程序就会 crash：

```rust, editable

fn main() {
    let data = move_creates_issue();
    println!("data: {:?}", data);
    // 如果把下面这句注释掉，程序运行会直接 segment error
    // data.print_name();
    print!("\\n");
    mem_swap_creates_issue();
}
```

> 现在你应该了解到在 Rust 下，自引用类型带来的潜在危害了吧。
~~~

~~~admonish info title="使用pin之后如何解决问题" collapsible=true
所以，Pin 的出现，对解决这类问题很关键，如果你试图移动被 Pin 住的数据结构:
- 要么，编译器会通过编译错误阻止你；
- 要么，你强行使用 unsafe Rust，自己负责其安全性。

我们来看使用 Pin 后如何避免移动带来的问题：

```rust, editable

use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后指向 name
    name_ptr: *const String,
    // PhantomPinned 占位符
    _marker: PhantomPinned,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const String;
        // SAFETY: 这里并不会把任何数据从 &mut SelfReference 中移走
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
    }

    pub fn print_name(self: Pin<&Self>) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            // 在使用 ptr 是需要 unsafe
            // SAFETY: 因为数据不会移动，所以这里 name_ptr 是安全的
            unsafe { &*self.name_ptr },
        );
    }
}

fn main() {
    move_creates_issue();
}

fn move_creates_issue() {
    let mut data = SelfReference::new("Tyr");
    let mut data = unsafe { Pin::new_unchecked(&mut data) };
    SelfReference::init(data.as_mut());

    // 不 move，一切正常
    data.as_ref().print_name();

    // 现在只能拿到 pinned 后的数据，所以 move 不了之前
    move_pinned(data.as_mut());
    println!("{:?} ({:p})", data, &data);

    // 你无法拿回 Pin 之前的 SelfReference 结构，所以调用不了 move_it
    // move_it(data);
}

fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:p})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) {
    println!("{:?} ({:p})", data, &data);
}
```

由于数据结构被包裹在 Pin 内部，所以在函数间传递时，变化的只是指向 data 的 Pin：

![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F-4961106.jpg)
~~~

### Unpin

~~~admonish info title="了解pin的作用后，Unpin有什么用？" collapsible=true
学习了 Pin，不知道你有没有想起 Unpin 。

那么，Unpin 是做什么的？

Pin 是为了让某个数据结构无法合法地移动，而 Unpin 则相当于声明数据结构是可以移动的，它的作用类似于 Send / Sync，通过类型约束来告诉编译器哪些行为是合法的、哪些不是。

在 Rust 中，绝大多数数据结构都是可以移动的，所以它们都自动实现了 [Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)。

即便这些结构被 Pin 包裹，它们依旧可以进行移动，比如：

```rust, editable

use std::mem;
use std::pin::Pin;

let mut string = "this".to_string();
let mut pinned_string = Pin::new(&mut string);

// We need a mutable reference to call `mem::replace`.
// We can obtain such a reference by (implicitly) invoking `Pin::deref_mut`,
// but that is only possible because `String` implements `Unpin`.
mem::replace(&mut *pinned_string, "other".to_string());
```

当我们不希望一个数据结构被移动，可以使用 !Unpin。在 Rust 里，实现了 !Unpin 的，除了内部结构（比如 Future），主要就是 PhantomPinned：

```rust, editable

pub struct PhantomPinned;
impl !Unpin for PhantomPinned {}
```

所以，如果你希望你的数据结构不能被移动，可以为其添加 PhantomPinned 字段来隐式声明 !Unpin。

当数据结构满足 Unpin 时，创建 Pin 以及使用 Pin（主要是 DerefMut）都可以使用安全接口，否则，需要使用 unsafe 接口：

```rust, editable

// 如果实现了 Unpin，可以通过安全接口创建和进行 DerefMut
impl<P: Deref<Target: Unpin>> Pin<P> {
    pub const fn new(pointer: P) -> Pin<P> {
        // SAFETY: the value pointed to is `Unpin`, and so has no requirements
        // around pinning.
        unsafe { Pin::new_unchecked(pointer) }
    }
    pub const fn into_inner(pin: Pin<P>) -> P {
        pin.pointer
    }
}

impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
}

// 如果没有实现 Unpin，只能通过 unsafe 接口创建，不能使用 DerefMut
impl<P: Deref> Pin<P> {
    pub const unsafe fn new_unchecked(pointer: P) -> Pin<P> {
        Pin { pointer }
    }

    pub const unsafe fn into_inner_unchecked(pin: Pin<P>) -> P {
        pin.pointer
    }
}
```
~~~

### ### Box<T>的Unpin思考

~~~admonish info title="如果一个数据结构 T: !Unpin，我们为其生成 Box<T>，那么 Box<T> 是 Unpin 还是 !Unpin 的？" collapsible=true
Box<T>是Unpin，因为Box<T>实现了Unpin trait
~~~

### async 产生的 Future 究竟是什么类型？

~~~admonish info title="Rust中Future是一个trait，async返回的是一个实现了Future的GenFuture类型。这里颇似python的yield如何变成协程的过程。" collapsible=true
现在，我们对 Future 的接口有了一个完整的认识，也知道 async 关键字的背后都发生了什么事情：

```rust, editable

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

那么，当你写一个 async fn 或者使用了一个 async block 时，究竟得到了一个什么类型的数据呢？比如：

```rust, editable

let fut = async { 42 };
```

你肯定能拍着胸脯说，这个我知道，不就是 impl Future<Output = i32> 么？

对，但是 impl Future 不是一个具体的类型啊，我们讲过，它相当于 T: Future，那么这个 T 究竟是什么呢？

我们来写段代码探索一下（代码）：

```rust, editable

fn main() {
    let fut = async { 42 };

    println!("type of fut is: {}", get_type_name(&fut));
}

fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
```

它的输出如下：

```shell

type of fut is: core::future::from_generator::GenFuture<xxx::main::{{closure}}>
```

实现 Future trait 的是一个叫 GenFuture 的结构，它内部有一个闭包。猜测这个闭包是 async { 42 } 产生的？

我们看 GenFuture 的定义（感兴趣可以在 Rust 源码中搜 from_generator），可以看到它是一个泛型结构，内部数据 T 要满足 Generator trait：

```rust, editable

struct GenFuture<T: Generator<ResumeTy, Yield = ()>>(T);

pub trait Generator<R = ()> {
    type Yield;
    type Return;
    fn resume(
        self: Pin<&mut Self>, 
        arg: R
    ) -> GeneratorState<Self::Yield, Self::Return>;
}
```

[Generator](https://doc.rust-lang.org/std/ops/trait.Generator.html) 是 Rust nightly 的一个 trait，还没有进入到标准库。大致看看官网展示的例子，它是怎么用的：

```rust, editable

#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }
}
```

1. 可以看到，如果你创建一个闭包，里面有 yield 关键字，就会得到一个 Generator。
2. 如果你在 Python 中使用过 yield，二者其实非常类似。
3. 因为 Generator 是一个还没进入到稳定版的功能，大致了解一下就行，以后等它的 API 稳定后再仔细研究。
~~~

### 回顾整理Future的Context、Pin/Unpin，以及async/await

~~~admonish info title="回顾整理Future的Context、Pin/Unpin，以及async/await" collapsible=true
![39｜异步处理：asyncawait内部是怎么实现的？](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/39%EF%BD%9C%E5%BC%82%E6%AD%A5%E5%A4%84%E7%90%86%EF%BC%9Aasyncawait%E5%86%85%E9%83%A8%E6%98%AF%E6%80%8E%E4%B9%88%E5%AE%9E%E7%8E%B0%E7%9A%84%EF%BC%9F.jpg)


> 并发任务运行在 Future 这样的协程上时，async/await 是产生和运行并发任务的手段，async 定义一个可以并发执行的 Future 任务，await 触发这个任务并发执行。具体来说：

1. 当我们使用 async 关键字时，它会产生一个 impl Future 的结果。
2. 对于一个 async block 或者 async fn 来说，内部的每个 await 都会被编译器捕捉，并成为返回的 Future 的 poll() 方法的内部状态机的一个状态。
3. Rust 的 Future 需要异步运行时来运行 Future
- 以 tokio 为例，它的 executor 会从 run queue 中取出 Future 进行 poll()
- 当 poll() 返回 Pending 时，这个 Future 会被挂起
- 直到 reactor 得到了某个事件，唤醒这个 Future，将其添加回 run queue 等待下次执行。
- tokio 一般会在每个物理线程（或者 CPU core）下运行一个线程
- 每个线程有自己的 run queue 来处理 Future。
- 为了提供最大的吞吐量，tokio 实现了 work stealing scheduler，这样，当某个线程下没有可执行的 Future，它会从其它线程的 run queue 中“偷”一个执行。
~~~



