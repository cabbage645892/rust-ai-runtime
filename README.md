# Rust AI Runtime Toolkit

## 1. 项目简介

`Rust AI Runtime Toolkit` 是一个基于 Rust 实现的轻量级本地 AI 推理工具链。

本项目不依赖在线大模型 API，而是在本地实现了一个简化版 AI Runtime 原型，支持文本 Tokenizer、规则推理模型、Markov 文本生成模型、统一推理引擎、异步任务调度、命令行推理、批量推理、Benchmark 测试和推理历史记录等功能。

项目目标是使用 Rust 构建一个结构完整、可运行、可测试、可扩展的小型本地 AI 推理系统，体现 Rust 在系统工具、命令行工具、模块化设计、错误处理和异步并发方面的优势。

## 2. 功能特性

本项目目前实现了以下功能：

（1）本地文本 Tokenizer
  - 支持文本编码
  - 支持 token 解码
  - 支持词表维护

（2）统一推理接口
  - 使用 `InferenceModel` trait 抽象不同模型
  - 支持模型注册
  - 支持模型切换
  - 支持统一的 `infer` 推理调用

（3）本地推理模型
  - `RuleBasedModel`：基于本地 JSON 规则文件进行关键词匹配推理
  - `MarkovModel`：基于本地语料构建 Markov 状态转移关系并生成文本

（4）异步任务调度
  - 使用 Tokio 异步运行时
  - 使用 mpsc channel 实现任务队列
  - 使用 WorkerPool 处理多个推理任务
  - 使用 `Arc` 在线程任务之间共享推理引擎

（5）命令行工具
  - 支持单次推理
  - 支持批量推理
  - 支持 Benchmark 性能测试
  - 支持查看推理历史
  - 支持 Demo 演示模式

（6）推理历史记录
  - 将推理输入、模型名称、输出结果和耗时保存到本地 JSON 文件

（7）测试与工程规范
  - 包含单元测试和关键功能测试
  - 支持 `cargo fmt`
  - 支持 `cargo clippy`
  - 支持 `cargo test`


## 3. 项目目录结构

```text
rust-ai-runtime/
├── assets/
│   └── models/
│       ├── rules.json
│       └── markov_corpus.txt
│
├── config/
│   └── settings.toml
│
├── examples/
│   └── prompts.txt
│
├── history/
│   └── history.json
│
├── src/
│   ├── cli.rs
│   ├── config.rs
│   ├── demo.rs
│   ├── history.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── model.rs
│   ├── runtime.rs
│   ├── tokenizer.rs
│   ├── tui.rs
│   ├── utils.rs
│   │
│   ├── inference/
│   │   ├── context.rs
│   │   ├── engine.rs
│   │   ├── traits.rs
│   │   └── models/
│   │       ├── markov.rs
│   │       ├── mod.rs
│   │       └── rule_based.rs
│   │
│   └── scheduler/
│       ├── pool.rs
│       ├── queue.rs
│       ├── task.rs
│       ├── mod.rs
│       └── worker.rs
│
├── tests/
│   ├── tokenizer_test.rs
│   ├── model_test.rs
│   └── engine_test.rs
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## 4. 依赖说明

项目主要依赖如下：

| 依赖          | 用途                              |
|--------------|----------------------------------|          
| `anyhow`     | 统一错误处理，简化 `Result` 返回   |
| `serde`      | 数据序列化与反序列化               |
| `serde_json` | 读取规则模型 JSON 文件、保存历史记录|
| `tokio`      | 异步运行时、异步任务、mpsc channel |
| `rand`       | Markov 模型文本生成时进行随机选择  |

对应的 `Cargo.toml` 依赖示例：

```toml
[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
rand = "0.8"
```

如果项目实际 `Cargo.toml` 中已经存在这些依赖，无需重复添加。

## 5. 编译方法

在项目根目录下执行：

```bash
cargo build
```

如果需要生成 release 版本：

```bash
cargo build --release
```

编译成功后，debug 版本可执行文件位于：

```text
target/debug/
```

release 版本可执行文件位于：

```text
target/release/
```

## 6. 运行方法

### 6.1 查看帮助信息

```bash
cargo run 
```

该命令会显示当前支持的所有命令，包括：

- `demo`
- `infer`
- `batch`
- `benchmark`
- `history`


### 6.2 运行 Demo

```bash
cargo run -- demo
```

Demo 模式会启动 Runtime，加载本地模型，并通过 WorkerPool 提交几个示例推理任务，用于展示异步任务调度和本地推理流程。

### 6.3 使用规则模型进行单次推理

```bash
cargo run -- infer --model rule --prompt "hello rust"
```

该命令会使用 `RuleBasedModel` 进行一次本地推理。

示例流程：

1. 读取命令行输入
2. 使用 Tokenizer 对 prompt 编码
3. 构造 `InferenceContext`
4. 调用当前模型的 `infer` 方法
5. 输出推理结果
6. 将推理记录保存到本地 history 文件

### 6.4 使用 Markov 模型进行单次推理

```bash
cargo run -- infer --model markov --prompt "Rust is"
```

该命令会使用 `MarkovModel` 进行文本生成。

`MarkovModel` 会从本地语料文件 `assets/models/markov_corpus.txt` 中构建状态转移表，并根据输入 prompt 生成简单文本。

### 6.5 批量推理

```bash
cargo run -- batch --model rule --input examples/prompts.txt
```

该命令会读取 `examples/prompts.txt` 文件中的多行 prompt，并依次执行推理。

输入文件示例：

```text
hello rust
what is ownership
ai inference with rust
cargo build project
```

每一行会作为一次独立推理请求。

### 6.6 Benchmark 性能测试

```bash
cargo run -- benchmark --model rule --requests 20 --workers 4
```

参数说明：

| 参数         | 说明                             |
|--------------|----------------------------------|
| `--model`    | 指定模型，支持 `rule` 和 `markov` |
| `--requests` | 模拟推理请求数量                  |
| `--workers`  | 并发 worker 数量                 |

Benchmark 会输出：

- 请求总数
- worker 数量
- 总耗时
- 平均吞吐量
- 生成结果大小等信息

该功能用于展示项目的异步任务调度和并发处理能力。

### 6.7 查看推理历史

```bash
cargo run -- history --limit 5
```

该命令会读取本地历史文件，并显示最近的推理记录。

历史记录中包含：

- 记录 ID
- 时间戳
- 使用的模型
- 输入 prompt
- 输出 response
- 推理耗时

历史文件默认保存位置：

```text
history/history.json
```

## 7. 配置文件说明

项目配置文件位于：

```text
config/settings.toml
```

示例内容：

```toml
rules_path = "assets/models/rules.json"
markov_corpus_path = "assets/models/markov_corpus.txt"
history_path = "history/history.json"
worker_count = 4
```

字段说明：

| 字段                  | 说明                     |
|----------------------|--------------------------|
| `rules_path`         | 规则模型 JSON 文件路径    |
| `markov_corpus_path` | Markov 模型语料文件路径   |
| `history_path`       | 推理历史记录保存路径      |
| `worker_count`       | Demo模式下默认worker数量 |

如果配置文件不存在，程序会使用默认配置。

## 8. 模型文件说明

### 8.1 规则模型文件

路径：

```text
assets/models/rules.json
```

该文件用于存储规则模型的匹配规则。

规则模型会根据输入 prompt 中的关键词返回对应响应。如果没有匹配到规则，则返回默认回复。

### 8.2 Markov 语料文件

路径：

```text
assets/models/markov_corpus.txt
```

该文件用于训练 Markov 文本生成模型。

程序启动时会读取该文本文件，并构建简单的词语状态转移关系。

## 9. 测试方法

运行全部测试：

```bash
cargo test
```

项目测试主要覆盖：

- Tokenizer 编码与解码
- 重复词 token 分配
- RuleBasedModel 规则匹配
- RuleBasedModel 默认回复
- MarkovModel 语料加载
- InferenceEngine 模型切换
- InferenceEngine 推理调用

## 10. 代码格式化与静态检查

格式化代码：

```bash
cargo fmt
```

运行 Clippy 静态检查：

```bash
cargo clippy
```

## 11. Rust 特性体现

本项目主要体现了以下 Rust 语言特性：

### 11.1 所有权与借用

项目中大量使用所有权和借用机制来管理字符串、推理上下文、模型对象和任务数据。例如：

- Tokenizer 对输入文本进行借用读取
- 推理上下文持有 prompt 和 tokens
- 推理引擎通过引用调用模型推理方法
- WorkerPool 中使用 `Arc` 安全共享推理引擎

### 11.2 结构体与枚举

项目中使用多个结构体组织数据，例如：

- `InferenceContext`
- `InferenceResult`
- `InferenceEngine`
- `InferenceTask`
- `WorkerPool`
- `HistoryRecord`
- `AppConfig`

命令行模块使用枚举表示不同命令，例如：

```rust
enum Command {
    Demo,
    Infer,
    Batch,
    Benchmark,
    History,
    Help,
}
```

### 11.3 Trait 抽象

项目通过 `InferenceModel` trait 抽象不同模型：

```rust
pub trait InferenceModel {
    fn name(&self) -> &str;
    fn infer(&self, context: &InferenceContext) -> Result<InferenceResult>;
}
```

这样 `RuleBasedModel` 和 `MarkovModel` 可以通过统一接口被推理引擎调用，提高了项目的可扩展性。

### 11.4 Result 错误处理

项目使用 `anyhow::Result` 统一处理错误，避免大量使用 `unwrap` 或 `expect`。

例如：

- 模型文件读取失败
- JSON 解析失败
- 命令行参数错误
- 历史文件写入失败
- 模型不存在

这些错误都会通过 `Result` 返回并向上传播。

### 11.5 异步与并发

项目使用 Tokio 实现异步任务调度：

- 使用 `tokio::spawn` 启动 worker
- 使用 `tokio::sync::mpsc` 实现任务队列
- 使用 `Arc` 共享推理引擎
- 使用 WorkerPool 并发处理多个推理任务

这部分体现了 Rust 在安全并发方面的优势。

### 11.6 模块化设计

项目按功能拆分为多个模块：

- `tokenizer`
- `inference`
- `scheduler`
- `runtime`
- `cli`
- `config`
- `history`

这种结构便于维护、测试和扩展。

## 12. 示例运行流程

```bash
cargo run -- help
cargo run -- demo
cargo run -- infer --model rule --prompt "hello rust"
cargo run -- infer --model markov --prompt "Rust is"
cargo run -- batch --model rule --input examples/prompts.txt
cargo run -- benchmark --model rule --requests 20 --workers 4
cargo run -- history --limit 5
cargo test
```

## 13. 项目总结

本项目实现了一个基于 Rust 的轻量级本地 AI 推理工具链。虽然它不是完整的大模型推理框架，但已经包含了本地模型加载、统一推理接口、异步任务调度、命令行交互、批量处理、性能测试和历史记录等完整流程。

通过该项目，可以较好地体现 Rust 在以下方面的能力：

- 内存安全
- 类型系统
- Trait 抽象
- Result 错误处理
- 模块化工程组织
- 异步并发
- 命令行工具开发
- 本地系统工具构建

后续可以继续扩展更多模型类型，例如 n-gram 模型、简单神经网络推理、ONNX Runtime 接入、TUI 界面、模型配置热加载等功能。
