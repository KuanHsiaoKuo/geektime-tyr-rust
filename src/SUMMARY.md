# 目录

- [源码阅读逻辑](anatomy_logic.md)
- [gdb/lldb调试或查看内存结构](intro_gdb_lldb.md)
- [get hands dirty](get_hands_dirty.md)
    - [httpie](httpie.md)
    - [rgrep](rgrep.md)
    - [thumbor](thumbor.md)
    - [queryer](queryer.md)
- [Rust核心深入](rust_cores.md)
    - [I. 从栈堆、所有权、生命周期开始内存管理](1_stack_heap_ownership_lifetime_memory.md)
        - [从堆栈开始](1_1_stack_heap.md)
        - [四大编程基础概念](1_2_programming_basic_concepts.md)
        - [所有权](1_3_ownership.md)
        - [生命周期](1_4_lifetime.md)
        - [从创建到消亡](1_5_go_through.md)
    - [II. 类型系统](2_type_system.md)
        - [类型系统特点](2_1_type_details.md)
        - [泛型概览：就像函数](2_2_generic_overview.md)
        - [trait概览](2_3_trait_overview.md)
        - [三种多态形式](2_4_0_three_polymorphics.md)
            - [Generics: <>](2_4_1_generic_usage.md)
            - [Trait impl/bound: self](2_4_2_trait_impl.md)
            - [Trait object: &self](2_4_3_trait_object.md)
            - [复杂多态综合使用实例](2_4_4_comprehensive_polymorphics.md)
        - [常用trait](2_5_trait_frequently.md)
        - [Trait设计](2_6_trait_design.md)
    - [III. 数据结构](3_data_structure.md)
        - [智能指针](3_1_smart_pointer.md)
            - [Box<T>](3_1_1_box.md)
            - [Cow<'a, B>](3_1_2_cow.md)
            - [MutexGuard<'_, T>](3_1_3_mutex_guard.md)
        - [集合容器](3_2_containers.md)
            - [切片](3_2_1_slice.md)
            - [哈希表](3_2_2_hashmap.md)
        - [错误处理](3_3_error_handling.md)
            - [错误处理内容和主流方法](3_3_1_error_content.md)
            - [Rust如何处理错误: ?/unwrap/expect](3_3_2_rust_error_handling.md)
        - [闭包](3_4_closure.md)
    - [IV. 宏编程](4_macros.md)
        - [宏分类](4_1_macros_classify.md)
        - [声明宏](4_2_declarative_macros.md)
        - [过程宏](4_3_procedural_macros.md)
    - [V. 并发与异步](5_concurrency_async.md)
        - [并发原语(Concurrency Primitives)](5_1_concurrency_primitives.md)
        - [异步：Future/Async/Await](5_2_future_async_await.md)
        - [深入async/await](5_3_deepin_async_await.md)
    - [VI. 混合编程](6_unsafe_ffi.md)
    - [VII. 网络开发](7_network.md)
        - [网络协议](7_1_protocols.md)
        - [网络模型](7_2_network_models.md)
- [kv server设计与实现](kv_server_design.md)
    - [基本流程](kv1_basic.md)
    - [实现并验证协议层](kv2_protocols.md)
    - [高级trait改造](kv3_advanced_traits.md)
    - [网络处理](kv4_network.md)
    - [网络安全](kv5_network_security.md)
    - [异步改造](kv6_async_refactor.md)
    - [重大重构](kv7_big_refactor.md)
    - [配置、测试、监控、CI/CD](kv8_config_ci_cd.md)
- [构建自己的类axum异步Web框架](custom_axum_async_web_framework.md)
