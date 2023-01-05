# school-cats
校园猫管理平台。

[需求分析](docs/requirements.md) [开发进展](https://www.chiro.work/6a177a7af0574bd998e02290e35ad808) [数据库设计](docs/database.md)

总之就是作死尝试了使用同一种语言构建一个前后端系统。

### 运行

前端将 Rust 编译到 Web Assembly 平台，程序逻辑在 wasm 上运行，通过 [yew](https://github.com/yewstack/yew) 操作 Dom 以及通过 [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) 操作各种 Web API bindings 来进行交互。

1. 前端
   1. `cd cats-frontend`
   2. 在线调试：`trunk serve`
   3. 生成静态文件：`trunk build --release`
2. 后端
   1. 运行：`cargo run`
   2. 编译为可执行文件：`cargo build --release`

### 想法

1. 使用同一种语言能在数据模型方面上简化数据流动吗
   1. 理论上可行，不过本项目中只能看到一堆坑
   2. `crate cats-api` 是一些统一的结构体数据模型，能使用相同的代码进行序列化/反序列化
2. 序列化/反序列化
   1. Rust 的 `struct` 很适合序列化/反序列化
   2. 结构体和 JSON 之间的序列化/反序列化非常顺滑
   3. 但是到数据库的序列化并没有找到好的方案，一个查询写一种解析函数少不了
3. 前后端数据交换
   1. 实现的过程中使用了 Restful 风格的 API
   2. 其实让人非常头疼，对单个数据模型的完整性有一定破坏
   3. 一个模型在数据库中一个样，在前后端 API 中又往往是另一个样
   4. Rust 又没有继承，写了非常多的同种数据结构的类型转换代码
   5. 其实可以直接用 RPC 实现远程调用，例如使用 gRPC 实现，固定 API，还能省点流量
4. 数据库交互
   1. 首先试用了[sqlx](https://github.com/launchbadge/sqlx)，它对多种 SQL 数据库都有实现
   2. 但是 sqlx 往往会尝试直接在编译期静态解析 SQL 字符串…
   3. 例如 `query_file("xxx.sql", ...)`，它甚至会在编译开始的时候读取 `xxx.sql` 然后做语法解析
   4. 但是这种解析有很大的局限性，它对一些 MySQL 的语法会报错从而无法通过编译
   5. 同时一个查询还只能有一条语句，对使用 SQL 文件十分不方便
   6. 最后使用了比较简单的 [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple)，只对 MySQL 实现了 Client
5. Rust 写前端怎么样
   1. 使用 `yew` 框架的 Rust 前端实际写起来和 React 很像
   2. 例如将函数或者一个 Component 作为一个组件，通过 `use_state` 管理状态等
   3. 不过要比 React 要麻烦一些，例如每次将一个 `state` 移动到闭包内都需要手动 `.clone()` 一下
   4. 而且文档较少，API 变更频繁，经常遇到 Example 在不同版本运行不起来的问题
   5. 对 Web API 的 bindings 也没有文档，需要查 MDN 才知道大致用法，而且这部分经常报错，一点不 rusty
   6. 性能上…其实差别不大，Web Assembly 性能大概能有 Native 一半，但是一多半用于加载 `.wasm` 文件本身了（
   7. 体积上，rust on wasm 编译出的 release 挺大的，本项目 `trunk build --release` 后生成的 `.wasm` 单个文件有约 1.6 MiB，调试模式下有大约 5MiB，不太适合多线程加载，对网速有要求
   8. Rust 的异步写起来很舒服，除了要建立新线程的时候传递的那些拷贝
6. 总之
   1. 本项目代码是踩坑踩出来的💩⛰️，这么多提交全是坑
   2. 用同一种语言构建一个前后端系统……挺好玩的