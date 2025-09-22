# MCP 服务器 - 膳食准备 (Rust)

一个高性能的 MCP (模型上下文协议) 服务器，使用 Rust 构建，提供基于菜系的食谱和智能膳食规划功能。

## 🚀 功能特性

- **多菜系食谱数据库**: 包含法国、泰国、意大利、墨西哥、中国和越南的传统食谱
- **智能膳食规划**: 生成周度膳食计划，包含优化的购物清单
- **成分重叠分析**: 通过识别共同成分来减少食物浪费
- **资源模板**: 通过 `file://recipes/{cuisine}` URI 动态访问食谱
- **高性能**: 基于 Rust 构建，具有出色的性能和内存安全
- **类型安全**: 编译时保证，零成本抽象
- **全面安全**: 预提交钩子、漏洞扫描、SBOM 生成
- **跨平台**: 可在任何地方运行的可移植二进制文件，无需依赖

## 🏗️ 架构

### Rust 实现优势

- **内存安全**: 无空指针异常或内存泄漏
- **性能**: 接近 C 语言级别的性能，具有出色的并发性
- **类型系统**: 强大的编译时类型检查
- **宏系统**: 用于 MCP 协议合规的代码生成
- **异步运行时**: 基于 Tokio 的 async/await，高吞吐量

### 项目结构

```
src/
├── main.rs           # MCP 服务器实现，包含工具和提示
├── recipes.rs        # 食谱数据结构和数据库
├── meal_planner.rs   # 膳食规划逻辑和优化
├── lib.rs            # 库入口点
tests/
├── integration_tests.rs # 综合测试套件
scripts/
├── security-check.sh    # 安全验证脚本
├── generate-sbom.sh     # SBOM 生成脚本
benches/
├── benchmark.rs         # 性能基准测试
Cargo.toml              # 依赖项和项目配置
deny.toml               # 许可证和安全策略
Makefile                # 开发命令
```

## 🛠️ 安装设置

### 先决条件

- Rust 1.70+ (从 [rustup.rs](https://rustup.rs/) 安装)
- Cargo (随 Rust 一起安装)
- Git (用于版本控制)

### 安装

```sh
# 克隆或导航到项目目录
cd mcp-server-meal-prep

# 构建项目
cargo build --release

# 运行服务器
cargo run --release
```

### 开发

```sh
# 开发模式运行
cargo run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 安全检查
make security-check

# 生成 SBOM
make sbom

# 运行基准测试
cargo bench
```

## 🎯 可用功能

### 工具

1. **`get_cuisines`**: 列出所有可用的菜系
2. **`get_recipes`**: 获取特定菜系的食谱
3. **`generate_meal_plan`**: 创建包含购物清单的膳食计划
4. **`analyze_ingredient_overlap`**: 查找食谱间的共同成分
5. **`weekly_meal_planner`**: 综合周度膳食规划，支持菜系选择

### 提示

- **`weekly_meal_planner`**: 综合周度膳食规划，支持菜系选择

### 资源

- **`file://recipes/{cuisine}`**: 按菜系访问食谱 (法国、泰国、意大利、墨西哥、中国、越南)

## 📋 使用示例

### MCP 客户端配置

添加到您的 MCP 客户端配置中：

```json
{
  "mcpServers": {
    "meal-prep": {
      "command": "/path/to/mcp-server-meal-prep/target/release/mcp-server-meal-prep",
      "args": [],
      "env": {}
    }
  }
}
```

### 工具调用示例

```json
// 获取可用菜系
{
  "method": "tools/call",
  "params": {
    "name": "get_cuisines",
    "arguments": {}
  }
}

// 获取法国食谱
{
  "method": "tools/call",
  "params": {
    "name": "get_recipes",
    "arguments": {
      "cuisine": "French"
    }
  }
}

// 为意大利菜系生成 7 天膳食计划
{
  "method": "tools/call",
  "params": {
    "name": "generate_meal_plan",
    "arguments": {
      "cuisine": "Italian",
      "days": 7,
      "servings": 4
    }
  }
}

// 分析墨西哥菜系的成分重叠
{
  "method": "tools/call",
  "params": {
    "name": "analyze_ingredient_overlap",
    "arguments": {
      "cuisine": "Mexican"
    }
  }
}
```

### 提示使用示例

```json
// 越南菜系周度膳食规划
{
  "method": "prompts/get",
  "params": {
    "name": "weekly_meal_planner",
    "arguments": {
      "cuisine": "Vietnamese"
    }
  }
}
```

## 🍳 支持的菜系

### 法国菜
- **红酒鸡**: 经典法式红酒炖鸡
- **法式洋葱汤**: 传统洋葱汤配格鲁耶尔奶酪
- **焦糖布丁**: 香草蛋奶配焦糖

### 泰国菜
- **泰式炒河粉**: 炒河粉配虾和蔬菜
- **冬阴功汤**: 酸辣虾汤
- **芒果糯米饭**: 椰奶甜点配芒果

### 意大利菜
- **卡波纳拉意面**: 鸡蛋、奶酪和培根意面
- **玛格丽特披萨**: 经典番茄、马苏里拉和罗勒披萨
- **提拉米苏**: 咖啡味马斯卡彭甜点

### 墨西哥菜
- **牧师塔可**: 香料猪肉塔可配菠萝
- **鳄梨酱**: 鳄梨蘸酱配青柠和香菜
- **油条**: 肉桂糖炸面团

### 中国菜
- **宫保鸡丁**: 辣炒鸡丁配花生和蔬菜
- **叉烧**: 甜咸烤猪肉
- **蛋挞**: 奶油蛋奶挞配酥皮

### 越南菜
- **牛肉河粉**: 传统越南牛肉汤面
- **越南三明治**: 越南三明治配腌菜
- **三色甜汤**: 分层甜点配豆类和椰子

## 🧪 测试

```sh
# 运行所有测试
cargo test

# 运行带输出的测试
cargo test -- --nocapture

# 运行特定测试
cargo test test_meal_plan_generation
```

## 📊 性能特征

- **启动时间**: 0.020s (比 TypeScript 快 5.5 倍)
- **内存使用**: ~10-20MB (原生二进制效率)
- **二进制大小**: 1.6M (自包含可执行文件)
- **构建时间**: 1.97s (发布构建)
- **吞吐量**: 优秀的 CPU 密集型膳食规划
- **延迟**: 食谱访问的最小开销

## 🔧 开发

### 添加新菜系

1. 编辑 `src/recipes.rs`
2. 在 `get_recipe_database()` 函数中添加新食谱
3. 重新构建: `cargo build --release`
4. 重启 MCP 客户端

### 添加新工具

1. 在 `MealPrepServer` 实现中添加工具函数
2. 使用 `#[tool(description = "...")]` 属性
3. 使用 `McpError` 实现错误处理
4. 为新功能添加测试

### 添加新提示

1. 在 `MealPrepServer` 实现中添加提示函数
2. 使用 `#[prompt(name = "...")]` 属性
3. 返回 `GetPromptResult` 和消息
4. 添加参数验证

## 🚀 部署

### 生产构建

```sh
# 构建优化发布二进制文件
cargo build --release

# 二进制文件将位于:
# target/release/mcp-server-meal-prep
```

### 跨平台编译

```sh
# 安装跨编译目标
rustup target add x86_64-unknown-linux-gnu

# 从 macOS 构建 Linux 版本
cargo build --release --target x86_64-unknown-linux-gnu
```

### Docker 部署

```dockerfile
FROM scratch
COPY target/release/mcp-server-meal-prep /app/
ENTRYPOINT ["/app/mcp-server-meal-prep"]
```

## 🔍 故障排除

### 常见问题

1. **构建错误**: 确保安装了 Rust 1.70+
2. **运行时错误**: 检查所有依赖项是否可用
3. **MCP 连接**: 验证客户端配置中的二进制路径
4. **权限问题**: 确保二进制文件可执行

### 调试模式

```sh
# 使用调试日志运行
RUST_LOG=debug cargo run
```

## 📈 性能对比 (Rust vs TypeScript)

| 指标 | Rust | TypeScript | 胜者 |
|------|------|------------|------|
| **启动时间** | 0.020s | 0.110s | 🏆 Rust |
| **内存使用** | ~10-20MB | ~50-100MB | 🏆 Rust |
| **二进制大小** | 1.6M | 4.0K | 🏆 TypeScript |
| **构建时间** | 1.97s | 1.15s | 🏆 TypeScript |
| **依赖项** | 3.9G (target) | 78M (node_modules) | 🏆 TypeScript |
| **类型安全** | 编译 + 运行时 | 编译时 | 🏆 Rust |
| **并发性** | 优秀 (Tokio) | 良好 (Node.js) | 🏆 Rust |
| **可移植性** | 通用二进制 | 需要 Node.js | 🏆 Rust |

## 🔒 安全功能

此 MCP 服务器实现了全面的安全最佳实践：

### 安全工具
- **cargo audit**: 依赖项漏洞扫描
- **cargo deny**: 许可证合规和咨询检查
- **预提交钩子**: 自动化安全验证
- **SBOM 生成**: 供应链安全的软件物料清单

### 安全加固
- **内存安全**: 零不安全代码块
- **剥离二进制**: 从发布构建中移除调试符号
- **安全配置文件**: 加固的构建配置
- **依赖项审计**: 自动化漏洞扫描

### 安全文档
- **SECURITY.md**: 漏洞报告流程
- **RUST_DEVELOPMENT_GUIDE.md**: 安全感知的开发实践
- **SETUP_TROUBLESHOOTING.md**: 安全故障排除指南

## 🤝 贡献

1. Fork 仓库
2. 创建功能分支: `git checkout -b feature/new-cuisine`
3. 添加您的更改和测试
4. 运行 `cargo test` 和 `cargo clippy`
5. 运行安全检查: `make security-check`
6. 提交拉取请求

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 使用 [RMCP](https://github.com/modelcontextprotocol/rust-sdk) Rust SDK 构建
- 受 TypeScript MCP 服务器示例启发
- 食谱数据来自传统烹饪资源
- 安全实践基于 OWASP 指南
- 作者: Jin Wen <jin.wen@hisgarden.org>

---

**准备用 Rust 烹饪！🦀🍳** 您的高性能 MCP 服务器已准备好进行膳食规划和食谱访问。

## 🌍 可移植性

### 二进制文件特性
- **大小**: 1.6M (紧凑高效)
- **类型**: 自包含可执行文件
- **依赖项**: 无 (静态链接)
- **运行时**: 原生二进制 (无需解释器)

### 平台支持

| 平台 | 状态 | 说明 |
|------|------|------|
| **macOS ARM64** | ✅ **工作** | 原生 Apple Silicon |
| **macOS x86_64** | ✅ **支持** | Intel Mac |
| **Linux x86_64** | ✅ **支持** | 大多数 Linux 发行版 |
| **Windows x86_64** | ✅ **支持** | Windows 10/11 |

### 部署场景

二进制文件可以部署在：
- ✅ **本地开发** 环境
- ✅ **生产服务器** (任何操作系统)
- ✅ **Docker 容器** (scratch 基础镜像)
- ✅ **云平台** (AWS、GCP、Azure)
- ✅ **CI/CD 管道** (GitHub Actions 等)
- ✅ **Kubernetes** 集群
- ✅ **无服务器函数**

### 使用

只需将二进制文件复制到任何地方并运行：
```bash
./mcp-server-meal-prep
```

**Rust 二进制文件真正可移植，可在任何地方运行，无需任何设置或依赖！** 🎉
