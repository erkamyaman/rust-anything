<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="The Rust Anything crab">
  </a>
</p>

<h1 align="center">Rust Anything</h1>

<p align="center">
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io" alt="crates.io version"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm" alt="npm version"></a>
<a href="https://github.com/erkamyaman/rust-anything/stargazers"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=stars" alt="GitHub stars"></a>
<a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/discord-join-111111?style=flat&colorA=111111&colorB=b34a1a" alt="Discord"></a>
<a href="https://github.com/erkamyaman/rust-anything#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-111111?style=flat&colorA=111111&colorB=b34a1a" alt="MIT or Apache-2.0 license"></a><br>
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/d/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io%20downloads" alt="crates.io downloads"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/d18m/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm%20downloads" alt="npm downloads"></a>
</p>

<p align="center">
Viết lại ứng dụng bằng Rust, và chứng minh nó vẫn hoạt động y như cũ.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Tài liệu:</b> <a href="https://rustanything.ai/vi">https://rustanything.ai/vi</a></p>

<p align="center"><a href="translations.md">🌐 Ngôn ngữ khác</a></p>

<p align="center"><sub>Bản dịch này do cộng đồng duy trì và có thể chậm hơn <a href="../README.md">README tiếng Anh</a>.</sub></p>

## Nhà tài trợ

<p align="center">Sản xuất tại Türkiye 🇹🇷 bởi <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything miễn phí và mã nguồn mở. Nếu nó giúp ích cho công việc của bạn, tài trợ sẽ giúp dự án tiếp tục.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Nhà tài trợ doanh nghiệp: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://www.atakdomain.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/domain-dark.svg">
      <img src="assets/sponsors/domain-light.svg" width="800" alt="Domain sponsor: Atak Domain">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Nhà tài trợ: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Trở thành nhà tài trợ"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Tài trợ với tư cách công ty, hay việc gì khác? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Đang chuyển một codebase sang Rust? Tôi cung cấp dịch vụ tư vấn chuyển đổi sang Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Bắt đầu nhanh

Dán đoạn này vào agent lập trình của bạn:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Hoặc tự cài đặt:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **Agent:** đọc [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) để xem các bước cài đặt, mọi lệnh và quy trình chuyển đổi.

## Cách hoạt động

**Thế nào là xong.** Hành vi, không phải test của bạn. `record` chạy bản gốc trên các input được sinh ra và được cung cấp, rồi lưu lại những gì mỗi lời gọi trả về, ném ra và in ra. Một bản port xong khi `gate` qua: mọi case đã ghi đều khớp, và fmt, clippy, `cargo test`, kiểm tra `unsafe`, kiểm tra stub và kiểm tra câu trả lời đã ghi bị dán vào bản port đều qua. Test của chính bạn cũng được chạy.

**Báo cáo hay chuyển đổi, bạn chọn.** `analyze` và `profile` chỉ báo cáo: một kế hoạch xếp hạng những gì nên port, và phần nào nóng dưới tải thực. Đây là các lệnh thường, nên báo cáo không tốn token nào. `go` chuyển đổi một unit: tạo cầu nối cho hàm nóng nhất, ghi lại nó, giao bản port cho agent của bạn, rồi build, diff và chạy `gate`, và chỉ bật nó lên khi mọi thứ khớp (TS/JS, Python và PowerShell; `disable` tắt nó đi).

**Một lần chạy mất bao lâu.** Một lần chạy `go` port một unit, với tối đa `--rounds` lượt agent (mặc định 3): một lượt cho bản port, rồi một lượt cho mỗi diff hoặc gate thất bại. Cả ứng dụng là một lần chạy cho mỗi unit, nên bạn có thể dừng sau bất kỳ lần nào.

**Model rẻ cho kế hoạch, model mạnh cho bản port.** Kế hoạch không cần model. Cho bản port, truyền bất kỳ lệnh agent nào: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agent.** `rust-anything setup` thêm nó vào Claude Code, Codex, Cursor, Gemini CLI và OpenCode. Bất kỳ client MCP nào khác cũng có thể chạy `rust-anything mcp`, và skill cũng có thể cài riêng bằng `npx skills add erkamyaman/rust-anything` (xem [cài đặt](install.md)).

## Giấy phép

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Các bản port và code được tạo trong repo thuộc về bạn.
