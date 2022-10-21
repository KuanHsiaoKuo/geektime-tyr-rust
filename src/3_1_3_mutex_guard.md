# MutexGuard<T>： 数据加锁

<!--ts-->
* [MutexGuard： 数据加锁](#mutexguard-数据加锁)
   * [MutexGuard与String、Box、Cow&lt;'a, B&gt;的对比](#mutexguard与stringboxcowa-b的对比)
   * [使用Mutex::lock获取](#使用mutexlock获取)
   * [定义与Deref、Drop trait实现](#定义与derefdrop-trait实现)
   * [使用Mutex_MutexGuard的例子](#使用mutex_mutexguard的例子)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Oct 21 11:58:31 UTC 2022 -->

<!--te-->

## MutexGuard与String、Box<T>、Cow<'a, B>的对比

~~~admonish info title='Deref+Drop' collapsible=true
String、Box<T>、Cow<'a, B> 等智能指针，都是通过 Deref 来提 供良好的用户体验， 
MutexGuard<T> 是另外一类很有意思的智能指针：
1. 它不但通过 Deref 提供良好的用户体验
2. 还通过 Drop trait 来确保，使用到的内存以外的资源在退出 时进行释放。
~~~

## 使用Mutex::lock获取

~~~admonish info title='MutexGuard这个结构是在调用 Mutex::lock 时生成的' collapsible=true

```rust, editable

pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
    unsafe {
        self.inner.raw_lock();
        MutexGuard::new(self)
    }
}
```
---
[rust文档](https://doc.rust-lang.org/src/std/sync/mutex.rs.html#279-284)
1. 首先，它会取得锁资源，如果拿不到，会在这里等待；
2. 如果拿到了，会把 Mutex 结构的引 用传递给 MutexGuard。
~~~

## 定义与Deref、Drop trait实现

~~~admonish info title='MutexGuard 的定义以及它的 Deref 和 Drop 的实现' collapsible=true
```rust, editable

// 这里用 must_use，当你得到了却不使用 MutexGuard 时会报警
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
    poison: poison::Guard,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.lock.poison.done(&self.poison);
            self.lock.inner.raw_unlock();
        }
    }
}
```
---
从代码中可以看到:
1. 当 MutexGuard 结束时，Mutex 会做 unlock
2. 这样用户在使用 Mutex 时，可以不必关心何时释放这个互斥锁。
3. 因为无论你在调用栈上怎样传递 MutexGuard ，哪怕在错误处理流程上提前退出，Rust 有所有权机制，可以确保只要 MutexGuard 离开作用域，锁就会被释放
~~~

## 使用Mutex_MutexGuard的例子

~~~admonish info title='Mutex & MutexGuard example' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/guard.rs}}
```
---
> 在解析 URL 的时候，我们经常需要将 querystring 中的参数，提取成 KV pair 来进一步使 用。
> 绝大多数语言中，提取出来的 KV 都是新的字符串，在每秒钟处理几十 k 甚至上百 k 请求的系统中，你可以想象这会带来多少次堆内存的分配。 
> 但在 Rust 中，我们可以用 Cow 类型轻松高效处理它，在读取 URL 的过程中：
1. 每解析出一个 key 或者 value，我们可以用一个 &str 指向 URL 中相应的位置，然后用 Cow 封装它 
2. 而当解析出来的内容不能直接使用，需要 decode 时，比如 “hello%20world”，我们 可以生成一个解析后的 String，同样用 Cow 封装它。
~~~

~~~admonish info title='你可以把 MutexGuard 的引用传给另一个线程使用，但你无法把 MutexGuard 整个移动到另一个线程' collapsible=true
```rust, editable
{{#include ../geektime_rust_codes/15_smart_pointers/src/guard1.rs}}
```
~~~

~~~admonish info title='MutexGuard 的智能指针有很多用途' collapsible=true
- r2d2类似实现一个数据库连接池：[源码](https://github.com/sfackler/r2d2/blob/master/src/lib.rs#L611-L638)
```rust
impl<M> Drop for PooledConnection<M>
where
    M: ManageConnection,
{
    fn drop(&mut self) {
        self.pool.put_back(self.checkout, self.conn.take().unwrap());
    }
}

impl<M> Deref for PooledConnection<M>
where
    M: ManageConnection,
{
    type Target = M::Connection;

    fn deref(&self) -> &M::Connection {
        &self.conn.as_ref().unwrap().conn
    }
}

impl<M> DerefMut for PooledConnection<M>
where
    M: ManageConnection,
{
    fn deref_mut(&mut self) -> &mut M::Connection {
        &mut self.conn.as_mut().unwrap().conn
    }
}
```
- 类似 MutexGuard 的智能指针有很多用途。比如要创建一个连接池，你可以在 Drop trait 中，回收 checkout 出来的连接，将其再放回连接池。
~~~
