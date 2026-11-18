<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything 螃蟹">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">作者：<a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · 繁體中文 · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a></p>

<p align="center"><sub>本譯文由社群維護，可能落後於<a href="../README.md">英文 README</a>。</sub></p>

<p align="center"><em>用 Rust 重寫你的整個應用程式，並在每一步都保持正常運作。</em></p>

<p align="center"><em>無論是命令列工具（CLI）、伺服器還是函式庫，都能逐塊重寫，每一步都與原版實作進行比對驗證。</em></p>

<p align="center">Node.js 與 TypeScript · Python · C · Go · Ruby · Java · PHP 與 C# 搶先預覽中</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io 版本"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm 版本"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="支援 6 種語言，另有 2 種預覽中">
  <a href="#支援的代理程式"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="支援 4 種代理程式"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="測試狀態"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT 授權條款"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub Stars"></a>
</p>

<p align="center">
  <a href="#快速開始">快速開始</a> ·
  <a href="#支援語言">支援語言</a> ·
  <a href="how-it-works.md#how-it-works">運作原理</a> ·
  <a href="how-it-works.md#the-loop">核心流程</a> ·
  <a href="whole-apps.md#port-a-whole-app">遷移整個應用程式</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">熱點路徑</a> ·
  <a href="install.md#install">安裝</a> ·
  <a href="how-it-works.md#what-it-cant-catch">它偵測不到什麼</a> ·
  <a href="faq.md#faq">常見問題</a>
</p>

<p align="center">最高提速 20 倍 · 重播 38,251 筆錄製案例 · 0 筆差異 · 一條指令搞定</p>

<p align="center">
  <img src="assets/race.gif" alt="在 x18n 的 50,000 個鍵中搜尋 user.profile：TypeScript 耗時 143 ms，rust-anything 批次處理僅耗時 6.9 ms，且每次按鍵的結果完全相同。" width="820">
  <br>
  <sub>在 x18n 的 50,000 個鍵中輸入查詢：TypeScript 原版對上 rust-anything 批次處理，每次按鍵的結果都完全相同。</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>錄製</b><br>
      <code>record</code> 會以自動產生的邊界案例、語意陷阱以及你自己的輸入執行原版程式，並儲存它回傳的值或擲出的例外。
    </td>
    <td width="25%" valign="top">
      <b>移植</b><br>
      <code>bridge</code> 會產生 Rust crate 與包裝層，讓呼叫端繼續使用原本的 API。Rust 程式碼由你或你的代理程式撰寫。
    </td>
    <td width="25%" valign="top">
      <b>檢查</b><br>
      <code>diff</code> 會對 Rust 重播每一筆錄製案例，在全部吻合且 <code>gate</code> 通過之前，什麼都不會上線。
    </td>
    <td width="25%" valign="top">
      <b>提升</b><br>
      當重播在沒有回退的情況下通過，且每個單元都通過 gate，<code>app lift</code> 就會把它變成純 Rust 應用程式。原版程式完全不會被更動。
    </td>
  </tr>
</table>

**用 Rust 逐塊重寫你的整個應用程式。** rust-anything 會先錄製你現有的
Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 或 Java 應用程式的行為，然後針對每一塊
Rust 重寫程式碼對照該錄製紀錄進行驗證，確認無誤後再繼續下一步，直到整個應用程式成為純 Rust。CLI、HTTP
伺服器與函式庫皆適用；由你的程式開發代理程式（或你本人）來撰寫 Rust 程式碼。它只能偵測到錄製內容所涵蓋的行為：請參閱[它偵測不到什麼](how-it-works.md#what-it-cant-catch)。如果只需要加速慢的部分，
也可以只針對該熱點路徑逐一移植：

| 真實工作負載（結果與原版完全一致） | 原版 | 使用 rust-anything |
| --- | ---: | ---: |
| x18n 的側邊欄搜尋，50,000 個鍵（TypeScript，Node） | 每次查詢 10 ms | **0.5 ms：提速 20 倍** |
| 瀏覽器端執行相同搜尋（wasm，單執行緒） | 10 ms | **0.95 ms：提速 11.5 倍** |
| 對 20,000 個字串執行 `slugify`（Python） | 98 ms | **提速 17 倍** |
| Ruby 標準函式庫 `Shellwords.shellescape`，50,000 個值 | 88 ms | **提速 11.8 倍** |
| 對 50,000 個字串執行 `Slugify`（Go） | 49 ms | **提速 4 倍** |
| 對 50,000 個字串執行 `slugify`（Java） | 43 ms | **提速 3 倍** |

以上數據均在 4 核心環境下測得；每組數字在 [`demo/`](../demo) 中都有對應的基準測試，且都會先驗證結果完全一致。每個小函式呼叫一次 Rust 通常比 JIT 更*慢*；
rust-anything 會找出真正值得搬移的迴圈，並將整個迴圈移過去。

> rust-anything 是一個獨立專案，與 Rust Project 或 Rust Foundation 無關，亦未獲其背書。「Rust」為 Rust Foundation 之商標。

> [!WARNING]
> `record` 與 `app record` 會執行你的原版程式碼，以大量自動產生的輸入呼叫每個函式（或整個應用程式），
> 而 `app record` 會將每筆案例執行兩次。`diff`、`gate` 與 `app replay`
> 會在移植版或原版上再次執行相同的案例。會寫入檔案、存取資料庫或呼叫網路的程式碼，
> 每筆案例都會做一次這些事。請在專案的副本上執行它們，使用
> 測試資料庫，且環境中不要有正式環境的憑證。

## 快速開始

將這段話貼給 Claude Code、Codex、Cursor 或 Gemini CLI，讓你的代理程式自動安裝：

```text
安裝 rust-anything：執行 `cargo install rust-anything`，接著執行 `rust-anything setup`。
告訴我什麼時候需要重新啟動你。
```

或是自行安裝：

```sh
cargo install rust-anything   # 從 crates.io 安裝
rust-anything setup           # 向 Claude Code、Codex、Cursor 與 Gemini CLI 註冊
```

還沒有安裝 Rust 工具鏈？每個 [GitHub release](https://github.com/erkamyaman/rust-anything/releases) 都附帶了 macOS、Linux 與 Windows 的預先編譯執行檔；針對 JavaScript
專案，執行 `npm install -g rust-anything` 也能安裝相同的執行檔。

在 Claude Code 中你也可以將它安裝為外掛（包含 MCP 伺服器與工作流程技能；上述 CLI
必須位於你的 `PATH` 中）：

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` 會將 MCP 伺服器加入它所偵測到的每個代理程式中，並將工作流程技能安裝至各代理程式讀取技能的目錄。它會保留你的其他設定、可安全重複執行，且可用 `--remove` 復原。
接著你就可以直接請你的代理程式協助，例如：

- *「用 rust-anything 將這個 CLI 端到端重寫為 Rust，並證明其行為完全相同。」*
- *「這個搜尋感覺很慢。用 rust-anything 做效能分析，並將熱點迴圈搬移到 Rust。」*
- *「用 rust-anything 規劃將 `src/parser/` 遷移至 Rust；先移植第一個單元並進行驗證。」*
- *「將 `rust/text-rs` 中的 Rust 移植版與原版進行基準測試比對，告訴我是否適合上線。」*

### 支援的代理程式

| 代理程式 | `rust-anything setup` 會加入的設定 |
| --- | --- |
| Claude Code | `~/.claude.json` 中的 MCP 伺服器、`~/.claude/skills/` 中的技能 |
| Codex | `~/.codex/config.toml` 中的 MCP 伺服器、`~/.codex/skills/` 中的技能 |
| Cursor | `~/.cursor/mcp.json` 中的 MCP 伺服器 |
| Gemini CLI | `~/.gemini/settings.json` 中的 MCP 伺服器 |
| 其他任意 MCP 用戶端 | 手動將 `rust-anything mcp` 新增為 stdio 伺服器（見[安裝](install.md#install)） |

修改任何已存在的設定檔前，`setup` 都會在同目錄旁留存一份備份（`<file>.rust-anything.bak`）。

你也可以自行執行這些步驟：

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile 找到迴圈中的 fuzzyScore，bridge --batch 產生 Rust crate，gate 通過 2000/2000 筆錄製案例，且基準測試顯示比 TS 快 20.7 倍。" width="820">
  <br>
  <sub>以 x18n 的 <code>fuzzyScore</code> 為例：<code>profile</code> 找出熱點迴圈，<code>bridge --batch</code> 產生 crate，<code>gate</code> 通過，基準測試再將它與 TypeScript 版本比較。</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # 找出耗時熱點以及誰呼叫了它
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# 撰寫 src/port.rs（或交由代理程式撰寫），接著執行：
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # 每筆錄製案例都必須吻合
rust-anything bench rust/fuzzy-rs                      # 真的更快嗎？讓評估結論說話
rust-anything gate rust/fuzzy-rs                       # 通過此門禁才算完成
```

rust-anything 並不直接翻譯程式碼：Rust 程式碼是由你的程式開發代理程式（Claude Code、Codex、Cursor 等）或你本人所撰寫。rust-anything 提供的是代理程式無法可靠猜測的資訊：時間花在哪裡、該先移植什麼
以及依何種順序移植、讓其餘程式碼無需改變 API 即可呼叫 Rust 的橋接層、
移植版必須完全吻合的精確行為、安全上線的方式，以及判定移植是否達標的門禁。

## 支援語言

| 語言 | 遷移單元 | 橋接層 | 錄製與重播方式 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 資料夾或單檔 | napi-rs 或 wasm-bindgen 搭配自動產生的二進位傳輸協定；支援類別與非同步 | Node，直接執行 `.ts` 原始碼 |
| Python | 套件或模組 | PyO3（abi3），直接型別轉換，支援型別化例外 | Python，匯入原版模組 |
| C | `.c` 檔案 | 與原函式同名的 `extern "C"` 函式與標頭檔 | 將原版編譯為共享函式庫，透過 ctypes 呼叫，每筆案例獨立 fork |
| Go | 套件（package） | 型別化 cgo 呼叫與自動產生的二進位傳輸協定，封裝於維持原 API 的套件內 | 自動產生的 Go 驅動程式，作為獨立模組建置執行 |
| Ruby | 包含模組函式的檔案或資料夾 | 掛載於原模組名稱下的 magnus 擴充套件 | 自動產生的 Ruby 驅動程式 |
| Java | 包含 `public static` 方法的套件 | 型別化 JNI 與自動產生的二進位傳輸協定，置於維持原方法的包裝類別中 | 自動產生的反射（reflection）驅動程式 |
| PHP（預覽） | 包含函式與 `public static` 方法的資料夾 | 下一步推出：透過 C ABI 運作的 PHP FFI | 目前支援整套應用程式：`app record`、`app replay` 與前門代理，適用於 CLI 與伺服器（`php -S`） |
| C#（預覽） | 包含 `public static` 方法的資料夾 | 下一步推出：透過 C ABI 運作的 P/Invoke | 目前支援整套應用程式：`app record`、`app replay` 與前門代理，適用於 CLI 與 ASP.NET 伺服器（`dotnet <app>.dll`） |

除了 C 語言外，每種語言皆可產生批次處理類別（`--batch`）：所有資料只需跨邊界傳入一次並留在
Rust 中，每次呼叫便會在所有 CPU 核心上平行運算全部項目。

系統會根據儲存庫內容自動偵測語言（依序檢查 `go.mod`、`pom.xml`、`.csproj` 或 `.sln`、
`pyproject.toml`、`Gemfile`、`composer.json`、`package.json`，否則依最常見的原始碼檔案判斷）；
也可以透過 `--lang` 手動覆寫。型別資訊直接來自原始碼：TS 型別、Python 型別提示、C、Go、Java 與 C#
函式簽章、Ruby YARD 標籤、PHP 型別與 phpdoc；未標註型別但當作數值使用的參數，會被自動推斷
為數值型別。

PHP 目前為搶先預覽階段：`analyze` 可為 PHP 應用程式規劃遷移（支援 Composer 套件與 PSR-4 自動載入，偵測
寬鬆比較與整數溢位轉浮點數等風險，並將 Composer 套件對應至 crate），
且支援對 PHP CLI 或伺服器進行錄製、重播與套用 Rust 前門代理。透過 PHP FFI 將各個
單元移植到 Rust 是下一步規劃（[issue #2](https://github.com/erkamyaman/rust-anything/issues/2)）。

C# 同樣處於搶先預覽階段：`analyze` 會讀取 `.csproj` 專案與其 NuGet 套件
（對應至 crate）、將 `using` 指示詞解析至對應資料夾，並標記 C# 的特殊差異
（`decimal`、`dynamic`、LINQ 的延遲查詢、`Task`、`lock`、反射）。目前可對 C# CLI 或 ASP.NET
伺服器進行錄製、重播與套用 Rust 前門代理；透過 P/Invoke 移植個別單元則為下一步支援項目。

## 深入了解

- [運作原理](how-it-works.md)：核心流程、`bridge` 與 `analyze` 如何決策、熱點路徑，以及它偵測不到什麼
- [遷移整個應用程式](whole-apps.md)：伺服器、CLI 與函式庫，從前門代理到 `app lift`
- [範例與案例研究](examples.md)：x18n，以及六種語言的真實程式碼
- [真實移植案例](real-world.md)：json-server、Fuse.js、vercel/ms，附基準測試
- [安裝、更新與開發](install.md)：所有安裝方式、Windows，以及從原始碼建置
- [常見問題](faq.md) · [專案狀態](STATUS.md) · [MCP 工具](tools.md)

## 贊助商

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> 是一款免費且開源的專案。如果它對你的工作有所幫助，歡迎考慮贊助支持。<br>
  你的支持能讓開發持續推進。
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企業贊助商：CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="個人贊助商：Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E6%88%90%E7%82%BA%E8%B4%8A%E5%8A%A9%E5%95%86-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="成為贊助商"></a>
</p>

## 參與貢獻

歡迎提交 Issue 與 Pull Request；[CONTRIBUTING.md](../CONTRIBUTING.md) 說明了開發環境設定、測試存放位置以及如何新增語言支援。設計決策與理由載於
[DECISIONS.md](../DECISIONS.md)。尤其歡迎真實移植專案的案例研究。若發現安全性問題請私下通報（[SECURITY.md](../SECURITY.md)），並請參閱
[行為準則](../CODE_OF_CONDUCT.md)。

## 授權條款

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman)。你在自己儲存庫中產生的移植程式碼與產物均歸你所有。
