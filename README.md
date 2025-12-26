# Wealth Hunter

## 项目介绍

Wealth Hunter 是一个基于 Rust 语言开发的高性能量化交易系统，采用工作区结构组织代码，支持实时市场数据获取、分析、策略优化和自动交易执行。

## 功能特点

- **高性能**：基于 Rust 语言开发，充分利用现代硬件资源，支持高并发和低延迟操作
- **完整的 OODA 循环**：实现了观察（Observe）、定位（Orient）、决策（Decide）、行动（Act）的完整交易循环
- **多种优化算法**：
  - 蚁群优化（Ant Colony Optimization）
  - 贝叶斯优化（Bayesian Optimization）
  - KNN 体制分类（KNN Regime Classification）
- **实时市场数据**：支持从多个交易所获取实时市场数据
- **模拟交易**：支持模拟交易模式，安全测试策略
- **风险控制**：内置风险控制机制，保护资金安全
- **系统状态管理**：支持系统状态的保存和恢复
- **跨平台支持**：支持 Web、桌面和移动平台

## 技术栈

- **后端**：Rust、Tokio、Actix-web、SQLx、ndarray
- **前端**：Rust Web 技术栈
- **桌面应用**：Tauri
- **数据库**：SQLite
- **测试**：Tokio-test、Proptest、Mockall
- **CI/CD**：GitHub Actions

## 安装方法

### 前置要求

- Rust 1.70+ (推荐使用 rustup 安装)
- Cargo (Rust 包管理器)
- Git

### 克隆项目

```bash
git clone https://github.com/A-PCO/Wealth-Hunter.git
cd Wealth-Hunter
```

### 安装依赖

```bash
cargo build
```

### 运行测试

```bash
cargo test --all
```

### 启动应用

```bash
cargo run --bin wealth_hunter_launcher
```

## 使用指南

### 配置文件

在运行应用之前，需要创建配置文件 `config.toml`，示例配置如下：

```toml
[exchange]
api_key = "your_api_key"
api_secret = "your_api_secret"

[strategy]
type = "box_theory"
parameters = [10, 20, 30]

[risk]
max_drawdown = 0.05
position_limit = 0.1
```

### 运行模式

Wealth Hunter 支持两种运行模式：

1. **模拟交易模式**：安全测试策略，不实际执行交易
2. **实盘交易模式**：连接真实交易所，执行实际交易

### 命令行参数

```bash
cargo run --bin wealth_hunter_launcher -- --help
```

## 项目结构

```
Wealth-Hunter/
├── backend/                # 后端核心功能
│   ├── src/                # 源代码
│   │   ├── api/            # API 模块
│   │   ├── core/           # 核心功能
│   │   ├── decision/       # 决策引擎
│   │   ├── execution/      # 执行引擎
│   │   ├── main_orchestrator/ # 主协调器
│   │   ├── optimization/   # 优化算法
│   │   ├── perception/     # 感知模块
│   │   ├── risk/           # 风险控制
│   │   └── strategy/       # 策略模块
│   └── tests/              # 测试代码
├── frontend/               # 前端应用
├── src-tauri/              # Tauri 桌面应用包装
├── .github/workflows/      # CI/CD 配置
├── .kiro/specs/            # 质量审计和 TDD 覆盖冲刺的设计文档
└── Cargo.toml              # Rust 工作区配置
```

## 贡献指南

1. 克隆项目到本地
2. 创建功能分支：`git checkout -b feature/your-feature`
3. 编写代码和测试
4. 运行测试：`cargo test --all`
5. 提交代码：`git commit -m "Add your feature"`
6. 推送分支：`git push origin feature/your-feature`
7. 创建 Pull Request

## 开发流程

Wealth Hunter 严格遵循 **测试驱动开发（TDD）** 和 **规格驱动设计（SDD）** 原则：

1. **构思方案**：明确功能需求和设计方案
2. **编写测试**：先编写测试用例，定义预期行为
3. **实现功能**：编写代码实现功能，确保测试通过
4. **重构优化**：优化代码结构和性能，保持测试通过
5. **质量检查**：执行静态分析、类型检查和代码格式化

## 许可证

Wealth Hunter 采用 MIT 许可证，详情请查看 [LICENSE](LICENSE) 文件。

## 联系方式

- 项目地址：https://github.com/A-PCO/Wealth-Hunter
- 问题反馈：https://github.com/A-PCO/Wealth-Hunter/issues
- 贡献指南：https://github.com/A-PCO/Wealth-Hunter/blob/main/CONTRIBUTING.md

## 致谢

感谢所有为 Wealth Hunter 项目做出贡献的开发者和社区成员！
