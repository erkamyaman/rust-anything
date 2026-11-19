<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything 螃蟹">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">作者：<a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · 简体中文 · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a></p>

<p align="center"><sub>本译文由社区维护，可能落后于<a href="../README.md">英文 README</a>。</sub></p>

<p align="center"><em>用 Rust 重写你的整个应用，并在每一步都保持正常运行。</em></p>

<p align="center"><em>无论是命令行工具（CLI）、服务端还是库，都能逐块迁移，每一步都与原实现严格比对校验。</em></p>

<p align="center">Node.js 与 TypeScript · Python · C · Go · Ruby · Java · PHP 与 C# 处于预览阶段</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io 版本"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm 版本"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 种语言，另有 2 种处于预览阶段">
  <a href="#适配工具"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="适配 4 种智能体"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="测试"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT 许可证"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub stars"></a>
</p>

<p align="center">
  <a href="#快速开始">快速开始</a> ·
  <a href="#支持语言">支持语言</a> ·
  <a href="how-it-works.md#how-it-works">工作原理</a> ·
  <a href="how-it-works.md#the-loop">核心闭环</a> ·
  <a href="whole-apps.md#port-a-whole-app">迁移整个应用</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">热点路径</a> ·
  <a href="install.md#install">安装</a> ·
  <a href="how-it-works.md#what-it-cant-catch">它捕获不到什么</a> ·
  <a href="faq.md#faq">常见问题</a>
</p>

<p align="center">最高提速 20 倍 · 回放 38,251 条录制用例 · 0 差异 · 一条命令搞定</p>

<p align="center">
  <img src="assets/race.gif" alt="在 x18n 的 50,000 个键中搜索 user.profile：TypeScript 耗时 143 ms，rust-anything 批处理仅耗时 6.9 ms，且每次击键的结果完全一致。" width="820">
  <br>
  <sub>在 x18n 的 50,000 个键中输入查询：TypeScript 原实现对比 rust-anything 批处理，每次击键的结果完全一致。</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>录制</b><br>
      <code>record</code> 在生成的边界用例、语义陷阱和你自己的输入上运行原实现，并保存它的返回值或抛出的异常。
    </td>
    <td width="25%" valign="top">
      <b>移植</b><br>
      <code>bridge</code> 生成 Rust crate 和一层包装，让调用方继续使用原有 API。Rust 代码由你或你的智能体编写。
    </td>
    <td width="25%" valign="top">
      <b>校验</b><br>
      <code>diff</code> 在 Rust 实现上回放每一条录制用例，在全部一致且 <code>gate</code> 通过之前，什么都不会上线。
    </td>
    <td width="25%" valign="top">
      <b>收口</b><br>
      当回放在零回退的情况下通过、且每个单元都通过门禁时，<code>app lift</code> 将其变为纯 Rust 应用。原实现始终不会被改动。
    </td>
  </tr>
</table>

**逐块将你的整个应用重写为 Rust。** rust-anything 会先录制你现有的 Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby 或 Java 应用的行为，然后每重写一块 Rust 代码就对照录制结果进行校验，合格后再继续推进，直到整个应用变成纯 Rust。无论是 CLI、HTTP 服务端还是程序库都能适用，由你的编程智能体（或你本人）来编写 Rust 代码。它只能捕获录制用例覆盖到的行为，详见[它捕获不到什么](how-it-works.md#what-it-cant-catch)。如果只想加速慢的部分，也可以只针对热点路径逐个迁移：

| 真实工作负载（结果与原实现完全一致） | 原实现 | 使用 rust-anything |
| --- | ---: | ---: |
| x18n 侧边栏搜索，50,000 个键（TypeScript，Node） | 每次查询 10 ms | **0.5 ms：提速 20 倍** |
| 浏览器端同一搜索（wasm，单线程） | 10 ms | **0.95 ms：提速 11.5 倍** |
| 20,000 个字符串的 `slugify`（Python） | 98 ms | **提速 17 倍** |
| Ruby 标准库 `Shellwords.shellescape`，50,000 个值 | 88 ms | **提速 11.8 倍** |
| 50,000 个字符串的 `Slugify`（Go） | 49 ms | **提速 4 倍** |
| 50,000 个字符串的 `slugify`（Java） | 43 ms | **提速 3 倍** |

以上数据均在 4 核环境下测得；每一项数字在 [`demo/`](../demo) 中都有对应的基准测试，并且都会先校验结果完全一致。逐个小函数调用 Rust 通常比 JIT 更*慢*；rust-anything 会找出真正值得迁移的循环，再把整个循环搬过去。

> rust-anything 是一个独立项目，与 Rust Project 或 Rust Foundation 没有任何关联或背书关系。“Rust” 是 Rust Foundation 的商标。

> [!WARNING]
> `record` 和 `app record` 会运行你的原始代码，用大量生成的输入调用每个函数（或整个应用），并且 `app record` 会把每条用例运行两次。`diff`、`gate` 和 `app replay` 会在移植版或原实现上再次运行相同的用例。会写文件、访问数据库或调用网络的代码，每条用例都会执行一次这些操作。请在项目的副本上运行它们，使用测试数据库，并确保环境中没有生产环境凭据。

## 快速开始

将下面这段话粘贴给 Claude Code、Codex、Cursor 或 Gemini CLI，让你的智能体自动完成安装：

```text
安装 rust-anything：先运行 `cargo install rust-anything`，再运行 `rust-anything setup`。
完成后告诉我什么时候需要重启你。
```

或者自行执行安装：

```sh
cargo install rust-anything   # 从 crates.io 安装
rust-anything setup           # 自动注册到 Claude Code、Codex、Cursor 和 Gemini CLI
```

还没有安装 Rust 工具链？每个 [GitHub release](https://github.com/erkamyaman/rust-anything/releases) 都附带了 macOS、Linux 和 Windows 的预编译二进制文件；对于 JavaScript 项目，也可以直接运行 `npm install -g rust-anything` 安装相同的二进制文件。

在 Claude Code 中还可以将其作为插件安装（包含 MCP 服务器和工作流技能，需确保上述 CLI 已加入 `PATH`）：

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` 会把 MCP 服务器添加到它检测到的每一个智能体中，并把工作流技能安装到对应智能体读取技能的目录。它会保留你的其他配置，可以安全地重复运行，`--remove` 则会撤销这些改动。之后你就可以直接向智能体提需求，例如：

- *"使用 rust-anything 将这个 CLI 端到端重写为 Rust，并证明其行为完全一致。"*
- *"这个搜索感觉很慢。用 rust-anything 做性能分析，把热点循环迁移到 Rust。"*
- *"用 rust-anything 规划 `src/parser/` 向 Rust 的迁移；先迁移第一个单元并验证。"*
- *"将 `rust/text-rs` 中的 Rust 移植版与原实现做基准对比，告诉我是否值得上线。"*

### 适配工具

| 智能体 | `rust-anything setup` 会写入的配置 |
| --- | --- |
| Claude Code | `~/.claude.json` 中的 MCP 服务器、`~/.claude/skills/` 中的技能 |
| Codex | `~/.codex/config.toml` 中的 MCP 服务器、`~/.codex/skills/` 中的技能 |
| Cursor | `~/.cursor/mcp.json` 中的 MCP 服务器 |
| Gemini CLI | `~/.gemini/settings.json` 中的 MCP 服务器 |
| 其他任意 MCP 客户端 | 手动将 `rust-anything mcp` 添加为 stdio 服务器（见[安装](install.md#install)） |

修改任何已有配置文件前，`setup` 都会在同目录下保存一份备份（`<file>.rust-anything.bak`）。

你也可以手动执行每一步：

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile 定位到循环中的 fuzzyScore，bridge --batch 生成 Rust crate，gate 通过全部 2000/2000 条录制用例，基准测试显示比 TS 快 20.7 倍。" width="820">
  <br>
  <sub>以 x18n 的 <code>fuzzyScore</code> 为例：<code>profile</code> 找到热点循环，<code>bridge --batch</code> 生成 crate，<code>gate</code> 通过，基准测试再将其与 TypeScript 版本做对比。</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # 找出热点函数及其调用方
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# 编写 src/port.rs（或交给智能体编写），然后运行：
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # 每条录制用例都必须一致
rust-anything bench rust/fuzzy-rs                      # 是否更快？由基准测试结论说了算
rust-anything gate rust/fuzzy-rs                       # 通过该门禁才算完成
```

rust-anything 并不负责自动翻译代码：具体 Rust 代码由你的编程智能体（Claude Code、Codex、Cursor 等）或你本人编写。rust-anything 提供的是智能体难以靠猜测稳妥获得的东西：时间花在哪里、先迁移什么以及按什么顺序迁移、让其余代码无需改动 API 就能调用 Rust 的桥接层、移植代码必须匹配的精确行为、安全的上线方式，以及判断迁移是否达标的门禁。

## 支持语言

| 语言 | 迁移单元 | 桥接层 | 录制与回放方式 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | 文件夹或单文件 | napi-rs 或 wasm-bindgen，配合生成的二进制传输协议；支持类和异步 | Node 直接运行 `.ts` 源码 |
| Python | 包或模块 | PyO3（abi3），直接类型转换，保留类型化异常 | Python 直接导入原模块 |
| C | `.c` 文件 | 与原函数同名的 `extern "C"` 函数及头文件 | 将原代码编译为共享库，通过 ctypes 调用，每条用例独立 fork |
| Go | 包（package） | 类型化 cgo 调用与生成的二进制传输协议，封装为保持原 API 的包 | 生成独立的 Go 驱动模块进行构建运行 |
| Ruby | 包含模块函数的文件或文件夹 | 挂载在原模块名下的 magnus 扩展 | 生成的 Ruby 驱动 |
| Java | 包含 `public static` 方法的包 | 类型化 JNI 与生成的二进制传输协议，封装为保持原方法的包装类 | 生成的反射驱动 |
| PHP（预览） | 包含函数和 `public static` 方法的文件夹 | 下一步规划：基于 C ABI 的 PHP FFI | 现已支持整应用级工作流：`app record`、`app replay` 和前门代理，适用于 CLI 与服务端（`php -S`） |
| C#（预览） | 包含 `public static` 方法的文件夹 | 下一步规划：基于 C ABI 的 P/Invoke | 现已支持整应用级工作流：`app record`、`app replay` 和前门代理，适用于 CLI 与 ASP.NET 服务端（`dotnet <app>.dll`） |

除 C 语言外，每种语言都支持生成批处理类（`--batch`）：数据只需跨边界传入一次并常驻 Rust 侧，每次调用即可在全部 CPU 核心上并行处理所有条目。

语言会根据代码库自动检测（依次检查 `go.mod`、`pom.xml`、`.csproj` 或 `.sln`、`pyproject.toml`、`Gemfile`、`composer.json`、`package.json`，否则统计最常见的源码文件）；也可以通过 `--lang` 手动指定。类型信息直接从源码提取：TS 类型、Python 类型注解、C/Go/Java/C# 函数签名、Ruby YARD 标签、PHP 类型与 phpdoc；对于未标注类型但按数值使用的参数，会自动推断为数值类型。

PHP 目前处于预览阶段：`analyze` 可为 PHP 应用生成迁移规划（支持 Composer 包与 PSR-4 自动加载、识别松散比较和整数溢出转浮点等风险、将 Composer 包映射为 Rust crate），并且支持对 PHP CLI 或服务端进行录制、回放与 Rust 前门接管。通过 PHP FFI 将单元逐个迁入 Rust 是下一步计划（[issue #2](https://github.com/erkamyaman/rust-anything/issues/2)）。

C# 同样处于预览阶段：`analyze` 会读取 `.csproj` 工程及其 NuGet 包（映射为 crate），将 `using` 指令解析到对应目录，并标记 C# 特有的语义差异（`decimal`、`dynamic`、LINQ 延迟查询、`Task`、`lock`、反射）。现已支持对 C# CLI 或 ASP.NET 服务端进行录制、回放与 Rust 前门接管；通过 P/Invoke 迁移单个单元将在后续支持。

## 了解更多

- [工作原理](how-it-works.md)：核心闭环、`bridge` 和 `analyze` 如何做决定、热点路径，以及它捕获不到什么
- [迁移整个应用](whole-apps.md)：服务端、CLI 和程序库，从前门到 `app lift`
- [示例与案例研究](examples.md)：x18n，以及六种语言的真实代码
- [真实项目迁移](real-world.md)：json-server、Fuse.js、vercel/ms，附基准测试
- [安装、更新与开发](install.md)：所有安装方式、Windows，以及从源码构建
- [常见问题](faq.md) · [项目状态](STATUS.md) · [MCP 工具](tools.md)

## 赞助商

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> 是一款免费开源项目。如果它对你的工作有所帮助，欢迎考虑赞助支持。<br>
  你的支持将助力项目持续开发。
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企业赞助商：CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="赞助商：Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E6%88%90%E4%B8%BA%E8%B5%9E%E5%8A%A9%E5%95%86-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="成为赞助商"></a>
</p>

## 参与贡献

欢迎提交 Issue 和 Pull Request；[CONTRIBUTING.md](../CONTRIBUTING.md) 说明了开发环境搭建、测试存放位置以及如何新增语言支持。设计决策与原因见 [DECISIONS.md](../DECISIONS.md)。特别欢迎提交真实项目的迁移案例研究。安全漏洞请私下报告（[SECURITY.md](../SECURITY.md)），并请遵守[行为准则](../CODE_OF_CONDUCT.md)。

## 许可证

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman)。你在自己仓库中生成的移植代码与产物均归你所有。
