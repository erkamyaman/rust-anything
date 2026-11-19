<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="The rust-anything crab">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">Tác giả: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · Tiếng Việt</p>

<p align="center"><sub>Bản dịch này do cộng đồng duy trì và có thể chậm hơn <a href="../README.md">README tiếng Anh</a>.</sub></p>

<p align="center"><em>Viết lại toàn bộ ứng dụng bằng Rust, từng phần một. Ứng dụng vẫn chạy được trong suốt quá trình.</em></p>

<p align="center"><em>CLI, máy chủ hay thư viện đều có thể chuyển từng phần, rồi đối chiếu từng phần với bản gốc.</em></p>

<p align="center">Node.js và TypeScript · Python · C · Go · Ruby · Java · PHP và C# đang ở giai đoạn preview</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="phiên bản crates.io"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="phiên bản npm"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 ngôn ngữ chính và 2 ngôn ngữ preview">
  <a href="#works-with"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="Hỗ trợ 4 agent"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Các bài kiểm tra"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="giấy phép MIT"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="Lượt sao trên GitHub"></a>
</p>

<p align="center">
  <a href="#quick-start">Bắt đầu nhanh</a> ·
  <a href="#languages">Ngôn ngữ</a> ·
  <a href="how-it-works.md#how-it-works">Cách hoạt động</a> ·
  <a href="how-it-works.md#the-loop">Quy trình</a> ·
  <a href="whole-apps.md#port-a-whole-app">Chuyển cả ứng dụng</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path</a> ·
  <a href="install.md#install">Cài đặt</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Giới hạn</a> ·
  <a href="faq.md#faq">FAQ</a>
</p>

<p align="center">nhanh hơn tới 20 lần · chạy lại 38.251 ca đã ghi · không sai khác · chỉ với một lệnh</p>

<p align="center">
  <img src="assets/race.gif" alt="Tìm user.profile trong 50.000 khóa của x18n: TypeScript mất 143 ms, còn batch của rust-anything mất 6,9 ms. Kết quả giống nhau sau mỗi lần gõ." width="820">
  <br>
  <sub>Tìm từ khóa trong 50.000 khóa của x18n bằng TypeScript gốc và batch của rust-anything. Kết quả giống nhau sau mỗi lần gõ.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Ghi lại (Record)</b><br>
      <code>record</code> chạy chương trình gốc với các ca biên do công cụ tạo ra, các semantic trap và input của bạn. Công cụ lưu lại giá trị trả về hoặc lỗi của từng ca.
    </td>
    <td width="25%" valign="top">
      <b>Cầu nối (Port)</b><br>
      <code>bridge</code> tạo crate Rust và lớp bọc để các phần khác tiếp tục gọi API cũ. Phần Rust do bạn hoặc coding agent viết.
    </td>
    <td width="25%" valign="top">
      <b>Kiểm tra (Check)</b><br>
      <code>diff</code> chạy lại toàn bộ ca đã ghi trên bản Rust. Chỉ phát hành khi mọi kết quả đều khớp và <code>gate</code> đạt.
    </td>
    <td width="25%" valign="top">
      <b>Nâng cấp (Lift)</b><br>
      Khi replay khớp hết mà không cần fallback, và mọi unit đều qua gate, <code>app lift</code> chuyển ứng dụng thành app Rust thuần. Code gốc vẫn nguyên vẹn.
    </td>
  </tr>
</table>

**Viết lại toàn bộ ứng dụng bằng Rust, từng phần một.** rust-anything ghi lại cách ứng dụng Node.js (TypeScript/JavaScript), Python, C, Go, Ruby hoặc Java hoạt động. Sau đó, công cụ đối chiếu từng phần Rust với bản ghi trước khi chuyển sang phần tiếp theo, cho đến khi ứng dụng chạy hoàn toàn bằng Rust. Dùng được cho CLI, HTTP server và thư viện. Rust do coding agent hoặc bạn viết. Công cụ chỉ kiểm tra được những gì có trong bản ghi; xem thêm [Những gì công cụ không kiểm tra được](how-it-works.md#what-it-cant-catch).

Nếu chỉ muốn tăng tốc đoạn chạy chậm, hãy chuyển riêng đoạn đó, từng hot path một:

| Tác vụ thực tế, kết quả vẫn như bản gốc | Bản gốc | Với rust-anything |
| --- | ---: | ---: |
| Tìm kiếm ở thanh bên của x18n, 50.000 keys (TypeScript, Node) | 10 ms/ query | **0,5 ms: nhanh hơn 20 lần** |
| Cùng tác vụ tìm kiếm trên trình duyệt (wasm, 1 luồng) | 10 ms | **0,95 ms: nhanh hơn 11,5 lần** |
| `slugify` với 20.000 strings (Python) | 98 ms | **nhanh hơn 17 lần** |
| `Shellwords.shellescape` trong thư viện chuẩn Ruby, 50.000 values | 88 ms | **nhanh hơn 11,8 lần** |
| `Slugify` với 50.000 strings (Go) | 49 ms | **nhanh hơn 4 lần** |
| `slugify` với 50.000 strings (Java) | 43 ms | **nhanh hơn 3 lần** |

Các phép đo chạy trên 4 nhân. Mỗi benchmark trong [`../demo/`](../demo) đều kiểm tra kết quả trước khi đo. Gọi Rust riêng cho từng hàm nhỏ thường còn *chậm hơn* JIT; rust-anything tìm vòng lặp đáng chuyển rồi đưa vòng lặp đó sang Rust.

> rust-anything là dự án độc lập, không liên kết với Rust Project hay Rust Foundation, cũng không được hai tổ chức này bảo trợ. "Rust" là nhãn hiệu của Rust Foundation.

> [!WARNING]
> `record` và `app record` chạy code gốc trên nhiều input tự tạo, gọi từng hàm hoặc cả ứng dụng. `app record` chạy mỗi ca hai lần. `diff`, `gate` và `app replay` chạy lại các ca đó trên bản port hoặc bản gốc. Nếu code ghi file, truy cập DB hay gọi mạng, những thao tác đó sẽ chạy lại theo từng ca. Hãy chạy trên bản sao của project, dùng test DB và đừng để thông tin xác thực thật trong biến môi trường.

<h2 id="quick-start">Bắt đầu nhanh</h2>

Dán đoạn này vào Claude Code, Codex, Cursor hoặc Gemini CLI để agent tự cài:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Hoặc tự tay cài:

```sh
cargo install rust-anything   # from crates.io
rust-anything setup           # registers it with Claude Code, Codex, Cursor và Gemini CLI
```

Chưa có bộ công cụ Rust? Tải binary dựng sẵn cho macOS, Linux hoặc Windows ở mục [GitHub release](https://github.com/erkamyaman/rust-anything/releases). Với project JavaScript, chạy `npm install -g rust-anything` để cài binary đó.

Trong Claude Code, bạn cũng có thể cài dưới dạng plugin. Plugin gồm MCP server và skill; cần thêm CLI ở trên vào `PATH` trước:

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` thêm MCP server cho từng agent tìm thấy, rồi cài workflow skill vào đúng thư mục agent đọc skill. Lệnh này giữ nguyên các cấu hình khác. Có thể chạy lại; dùng `--remove` để gỡ.

Sau đó, bạn có thể yêu cầu agent:

- *"Dùng rust-anything viết lại CLI này bằng Rust từ đầu đến cuối, rồi chứng minh nó chạy như bản cũ."*
- *"Tìm nguyên nhân chỗ search chạy chậm bằng rust-anything, rồi chuyển vòng lặp nóng sang Rust."*
- *"Lập kế hoạch chuyển `src/parser/` sang Rust bằng rust-anything. Bắt đầu với unit đầu tiên và kiểm tra kết quả."*
- *"Benchmark bản port Rust trong `rust/text-rs` với bản gốc rồi cho tôi biết có nên phát hành không."*

<h3 id="works-with">Tương thích</h3>

| Agent | Thành phần `rust-anything setup` cài |
| --- | --- |
| Claude Code | MCP server in `~/.claude.json`, skill in `~/.claude/skills/` |
| Codex | MCP server in `~/.codex/config.toml`, skill in `~/.codex/skills/` |
| Cursor | MCP server in `~/.cursor/mcp.json` |
| Gemini CLI | MCP server in `~/.gemini/settings.json` |
| các MCP client khác | tự thêm `rust-anything mcp` làm stdio server (xem [Install](install.md#install)) |

Trước khi sửa file có sẵn, `setup` lưu một bản sao cạnh file đó (`<file>.rust-anything.bak`).

Hoặc tự chạy từng bước:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile tìm thấy fuzzyScore trong vòng lặp, bridge --batch tạo crate Rust, gate đạt 2000/2000 ca đã ghi và benchmark cho thấy nhanh hơn TypeScript 20,7 lần." width="820">
  <br>
  <sub>Thử với <code>fuzzyScore</code> của x18n: <code>profile</code> tìm vòng lặp nóng, <code>bridge --batch</code> tạo crate, <code>gate</code> kiểm tra các ca đã ghi, rồi benchmark so sánh với TypeScript.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # what is hot, and who calls it
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# write src/port.rs (or let your agent), then:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # every recorded case must match
rust-anything bench rust/fuzzy-rs                      # faster? the verdict says
rust-anything gate rust/fuzzy-rs                       # done means this passes
```

rust-anything không tự dịch code sang Rust. Bạn hoặc coding agent (Claude Code, Codex, Cursor...) sẽ viết phần Rust. Công cụ giúp tìm đoạn code tốn thời gian, chọn phần nên chuyển trước và sắp xếp thứ tự thực hiện. `bridge` tạo cầu nối để phần còn lại tiếp tục gọi code Rust qua API cũ. Bản ghi làm chuẩn để kiểm tra hành vi, còn `gate` cho biết khi nào bản port đã sẵn sàng phát hành.

<h2 id="languages">Ngôn ngữ</h2>

| Ngôn ngữ | Đơn vị | Cầu nối | Ghi và chạy lại bằng |
| --- | --- | --- | --- |
| TypeScript/JavaScript | thư mục hoặc tệp | napi-rs hoặc wasm-bindgen, dùng wire nhị phân do công cụ tạo; hỗ trợ class và async | Node, chạy trực tiếp mã nguồn `.ts` |
| Python | package hoặc module | PyO3 (abi3), chuyển đổi trực tiếp, giữ nguyên kiểu exception | Python, import trực tiếp code gốc |
| C | tệp `.c` | hàm `extern "C"` giữ nguyên tên gốc, kèm header | code gốc được biên dịch thành shared library, gọi qua ctypes; mỗi ca chạy trong một fork |
| Go | package | lời gọi cgo có kiểu rõ ràng, dùng wire nhị phân do công cụ tạo; nằm trong package giữ nguyên API | Go driver do công cụ tạo, build thành module riêng |
| Ruby | tệp hoặc thư mục chứa các module có function | extension magnus nằm trong các module gốc | Ruby driver do công cụ tạo |
| Java | package chứa các hàm `public static` | JNI có kiểu rõ ràng, dùng wire nhị phân do công cụ tạo; wrapper class giữ nguyên method gốc | reflection driver do công cụ tạo |
| PHP (preview) | thư mục chứa function và method `public static` | sắp tới: PHP FFI dựa trên C ABI | có thể chạy với cả ứng dụng ngay hôm nay: `app record`, `app replay` và front door cho CLI lẫn server (`php -S`) |
| C# (preview) | thư mục chứa các method `public static` | sắp tới: P/Invoke dựa trên C ABI | có thể chạy với cả ứng dụng ngay hôm nay: `app record`, `app replay` và front door cho CLI lẫn server ASP.NET (`dotnet <app>.dll`) |

TypeScript/JavaScript, Python, Go, Ruby và Java hỗ trợ batch class (`--batch`); PHP và C# chưa được hỗ trợ. Dữ liệu chỉ qua cầu nối một lần rồi nằm trong Rust, mỗi lần gọi sẽ xử lý cả lô trên mọi core.

Công cụ nhận diện ngôn ngữ qua repo dựa trên các file như `go.mod`, `pom.xml`, `.csproj`, `.sln`, `pyproject.toml`, `Gemfile`, `composer.json`, `package.json` hoặc phần mở rộng xuất hiện nhiều nhất. Muốn chỉ định ngôn ngữ, dùng `--lang`. Kiểu dữ liệu được đọc từ source: type của TypeScript, type hint của Python, chữ ký hàm của C, Go, Java và C#, YARD tag của Ruby, type và phpdoc của PHP. Tham số không khai báo kiểu nhưng được dùng như số sẽ được suy luận là kiểu số.

PHP hiện ở trạng thái preview. `analyze` lập kế hoạch chuyển app PHP, gồm package Composer, PSR-4 autoloading và các rủi ro như so sánh lỏng lẻo, tràn số nguyên thành float hay ánh xạ package Composer sang crate. Hiện có thể ghi lại, replay và đặt Rust front door trước CLI hoặc server PHP. Bước tiếp theo là chuyển từng unit sang Rust qua PHP FFI ([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# cũng đang ở trạng thái preview. `analyze` đọc project `.csproj` và package NuGet, ánh xạ package sang crate, phân giải `using` thành thư mục rồi cảnh báo về các điểm riêng của C# như `decimal`, `dynamic`, deferred query của LINQ, `Task`, `lock` và reflection. Hiện có thể ghi lại, replay và đặt Rust front door trước CLI C# hoặc server ASP.NET. Tiếp theo là chuyển các unit qua P/Invoke.

<h2 id="learn-more">Đọc thêm</h2>

- [Cách hoạt động](how-it-works.md): quy trình, cách `bridge` và `analyze` đưa ra quyết định, hot path và những gì công cụ không kiểm tra được
- [Chuyển cả ứng dụng](whole-apps.md): chuyển server, CLI và thư viện từ front door đến `app lift`
- [Ví dụ và tình huống thực tế](examples.md): x18n và code thực tế bằng sáu ngôn ngữ
- [Các bản port thực tế](real-world.md): json-server, Fuse.js, vercel/ms và benchmark
- [Cài đặt, cập nhật và phát triển](install.md): các cách cài, hướng dẫn cho Windows và cách build từ source
- [Câu hỏi thường gặp](faq.md) · [Trạng thái dự án](STATUS.md) · [Công cụ MCP](tools.md)

## Nhà tài trợ

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> là dự án mã nguồn mở miễn phí. Nếu dự án hữu ích với bạn, hãy cân nhắc tài trợ.<br>
  Khoản tài trợ giúp dự án tiếp tục được phát triển.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Nhà tài trợ doanh nghiệp: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Nhà tài trợ: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Trở thành nhà tài trợ"></a>
</p>

## Đóng góp

Hoan nghênh issue và pull request. [CONTRIBUTING.md](../CONTRIBUTING.md) hướng dẫn cài đặt môi trường, vị trí đặt test và cách thêm ngôn ngữ. Các lựa chọn thiết kế cùng lý do được ghi trong [DECISIONS.md](../DECISIONS.md). Bạn cũng có thể chia sẻ những lần port ứng dụng thực tế. Nếu phát hiện vấn đề bảo mật, hãy báo riêng theo hướng dẫn trong [SECURITY.md](../SECURITY.md), đồng thời đọc [quy tắc ứng xử](../CODE_OF_CONDUCT.md).

## Giấy phép

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Các bản port và code được tạo trong repo thuộc về bạn.