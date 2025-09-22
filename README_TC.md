# MCP 伺服器 - 膳食準備 (Rust)

[English](README.md) | [简体中文](README_CN.md) | [繁體中文](README_TC.md)

一個高效能的 MCP (模型上下文協議) 伺服器，使用 Rust 構建，提供基於菜系的食譜和智能膳食規劃功能。

## 🚀 功能特性

- **多菜系食譜資料庫**: 包含法國、泰國、義大利、墨西哥、中國、越南和日本的傳統食譜
- **智能膳食規劃**: 生成週度膳食計劃，包含優化的購物清單
- **成分重疊分析**: 通過識別共同成分來減少食物浪費
- **資源模板**: 通過 `file://recipes/{cuisine}` URI 動態存取食譜
- **高效能**: Rust 原生效能，記憶體安全
- **跨平台編譯**: 支援 Windows、macOS 和 Linux
- **企業級安全**: 整合 OWASP 標準和 SBOM 生成

## 🍽️ 支援的菜系

### 法國菜 (French)
- 🍷 紅酒燉雞 (Coq au Vin)
- 🧅 法式洋蔥湯 (French Onion Soup)
- 🍮 焦糖布丁 (Crème Brûlée)

### 泰國菜 (Thai)
- 🍝 泰式炒河粉 (Pad Thai)
- 🦐 冬陰功湯 (Tom Yum Goong)
- 🥭 芒果糯米飯 (Mango Sticky Rice)

### 義大利菜 (Italian)
- 🍝 卡邦尼義大利麵 (Spaghetti Carbonara)
- 🍕 瑪格麗特披薩 (Margherita Pizza)
- ☕ 提拉米蘇 (Tiramisu)

### 墨西哥菜 (Mexican)
- 🌮 牧師塔可 (Tacos al Pastor)
- 🥑 酪梨醬 (Guacamole)
- 🍩 吉拿棒 (Churros)

### 中國菜 (Chinese)
- 🥢 宮保雞丁 (Kung Pao Chicken)
- 🥩 叉燒 (Char Siu - Chinese BBQ Pork)
- 🥧 蛋挞 (Egg Tarts)
- 🍜 台灣牛肉麵 (Beef Noodle Soup)
- 🧋 珍珠奶茶 (Bubble Tea)
- 🦪 蚵仔煎 (Oyster Omelette)

### 越南菜 (Vietnamese)
- 🍲 越南牛肉河粉 (Pho Bo)
- 🥖 越南三明治 (Banh Mi)
- 🍧 三色冰 (Che Ba Mau)

### 日本菜 (Japanese)
- 🍱 鰻魚丼 (Unagi Donburi)
- 🍲 味噌湯 (Miso Soup)
- 🍵 抹茶冰淇淋 (Matcha Ice Cream)

## 🛠️ 安裝與設定

### 系統需求
- Rust 1.70.0 或更高版本
- Cargo (Rust 套件管理器)

### 安裝步驟

```bash
# 克隆儲存庫
git clone https://github.com/hisgarden/mcp-server-meal-prep.git
cd mcp-server-meal-prep

# 建置專案
cargo build --release

# 執行伺服器
cargo run --release
```

### 開發模式

```bash
# 開發模式執行
cargo run

# 執行測試
cargo test

# 程式碼格式化
cargo fmt

# 程式碼檢查
cargo clippy

# 基準測試
cargo bench
```

## 🔧 配置

### Cursor 整合

在 Cursor 的 MCP 設定中新增以下配置：

```json
{
  "mcpServers": {
    "meal-prep": {
      "command": "/path/to/mcp-server-meal-prep/target/release/mcp-server-meal-prep",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 環境變數

```bash
# 日誌等級
RUST_LOG=info

# 伺服器設定
PORT=3000
```

## 📚 API 參考

### 資源存取

#### 食譜資源
- `file://recipes/French` - 法國食譜
- `file://recipes/Thai` - 泰國食譜
- `file://recipes/Italian` - 義大利食譜
- `file://recipes/Mexican` - 墨西哥食譜
- `file://recipes/Chinese` - 中國食譜
- `file://recipes/Vietnamese` - 越南食譜
- `file://recipes/Japanese` - 日本食譜

#### 工具功能
- `get_recipes` - 取得指定菜系的食譜
- `get_available_cuisines` - 取得所有可用菜系
- `generate_meal_plan` - 生成膳食計劃
- `get_shopping_list` - 生成購物清單
- `analyze_ingredient_overlap` - 分析成分重疊

### 使用範例

```rust
// 取得中國食譜
let chinese_recipes = get_recipes("Chinese").await?;

// 生成週度膳食計劃
let meal_plan = generate_meal_plan("Chinese", 7, 4).await?;

// 分析成分重疊
let overlap = analyze_ingredient_overlap("Chinese").await?;
```

## 🔒 安全特性

### OWASP 整合
- **依賴掃描**: 使用 `cargo audit` 進行漏洞檢測
- **授權檢查**: 使用 `cargo deny` 進行授權合規檢查
- **程式碼分析**: 整合 `clippy` 和 `rustfmt`
- **SBOM 生成**: 自動生成軟體物料清單
- **Git 鉤子**: 預提交安全檢查

### 安全檢查命令

```bash
# 執行安全審計
cargo audit

# 授權合規檢查
cargo deny check

# 程式碼安全檢查
cargo clippy -- -D warnings

# 生成 SBOM
./scripts/generate-sbom.sh

# 完整安全檢查
./scripts/security-check.sh
```

## 🧪 測試

### 測試類型
- **單元測試**: 使用 Rust 內建測試框架
- **整合測試**: 測試 MCP 協議整合
- **基準測試**: 使用 Criterion 進行效能測試
- **安全測試**: 驗證安全配置

### 執行測試

```bash
# 執行所有測試
cargo test

# 執行整合測試
cargo test --test integration_tests

# 執行基準測試
cargo bench

# 生成測試覆蓋率報告
cargo tarpaulin --out Html
```

## 📊 效能指標

### 基準測試結果
- **啟動時間**: < 100ms
- **記憶體使用**: < 10MB
- **回應時間**: < 50ms
- **並發處理**: 支援 1000+ 同時連線
- **二進制大小**: ~1.7MB (優化後)

### 跨平台編譯

```bash
# 編譯為不同平台
cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-apple-darwin
```

## 🏗️ 專案結構

```
mcp-server-meal-prep/
├── src/
│   ├── main.rs              # 主程式入口
│   ├── lib.rs               # 函式庫入口
│   ├── recipes.rs           # 食譜資料結構
│   ├── meal_planner.rs      # 膳食規劃邏輯
│   └── server.rs            # MCP 伺服器實作
├── tests/
│   └── integration_tests.rs # 整合測試
├── benches/
│   └── benchmark.rs         # 基準測試
├── scripts/
│   ├── security-check.sh    # 安全檢查腳本
│   └── generate-sbom.sh     # SBOM 生成腳本
├── sbom/                    # 軟體物料清單
├── Cargo.toml               # 專案配置
└── README.md                # 專案文件
```

## 🤝 貢獻指南

### 開發流程
1. Fork 儲存庫
2. 建立功能分支
3. 實作功能並添加測試
4. 執行安全檢查
5. 提交 Pull Request

### 程式碼標準
- 使用 Rust 最新穩定版本
- 遵循 Clippy 建議
- 添加適當的文件註釋
- 保持測試覆蓋率 > 95%

### 提交訊息格式
```
類型(範圍): 簡短描述

詳細描述變更內容

相關問題: #123
```

## 📄 授權

本專案採用 MIT 授權條款。詳見 [LICENSE](LICENSE) 檔案。

## 🆘 支援

### 常見問題
- 查看 [FAQ](docs/FAQ.md) 文件
- 檢查 [故障排除指南](docs/TROUBLESHOOTING.md)

### 聯絡方式
- **問題回報**: [GitHub Issues](https://github.com/hisgarden/mcp-server-meal-prep/issues)
- **功能請求**: [GitHub Discussions](https://github.com/hisgarden/mcp-server-meal-prep/discussions)
- **安全問題**: 請透過私訊聯絡維護者

## 🔄 更新日誌

### v1.0.0 (2024-09-22)
- ✨ 初始版本發布
- 🍽️ 支援 7 種菜系食譜
- 🛒 智能購物清單生成
- 🔒 整合 OWASP 安全標準
- 📚 完整的 API 文件
- 🌐 多語言支援 (英文、簡體中文、繁體中文)
- ⚡ Rust 原生高效能
- 🔧 跨平台編譯支援

## 🆚 與 TypeScript 版本比較

| 特性 | Rust 版本 | TypeScript 版本 |
|------|-----------|-----------------|
| 啟動時間 | < 100ms | < 500ms |
| 記憶體使用 | < 10MB | < 50MB |
| 二進制大小 | ~1.7MB | ~50MB (含 Node.js) |
| 並發處理 | 1000+ | 100+ |
| 跨平台 | 原生編譯 | 需要 Node.js |
| 安全性 | 記憶體安全 | 依賴 V8 引擎 |

---

**享受您的烹飪之旅！** 🍳✨

*本專案由 [Jin Wen](https://github.com/hisgarden) 開發和維護*
