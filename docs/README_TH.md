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
เขียนแอปของคุณใหม่ด้วย Rust และพิสูจน์ว่าพฤติกรรมการทำงานยังคงเหมือนเดิมทุกประการ
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>เอกสารคู่มือ:</b> <a href="https://rustanything.ai/">https://rustanything.ai/</a></p>

<p align="center"><a href="translations.md">🌐 ภาษาอื่น</a></p>

<p align="center"><sub>คำแปลนี้ดูแลโดยชุมชน และอาจอัปเดตช้ากว่า <a href="../README.md">README ภาษาอังกฤษ</a></sub></p>

## ผู้สนับสนุน

<p align="center">สร้างใน Türkiye 🇹🇷 โดย <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything ฟรีและเป็นโอเพนซอร์ส ถ้ามันช่วยงานของคุณ การสนับสนุนของคุณช่วยให้โครงการเดินหน้าต่อ</p>

<p align="center"><sub>ความปลอดภัยของคุณมาเป็นอันดับแรก: ผู้สนับสนุนจะได้รับโลโก้และลิงก์ในส่วนนี้เท่านั้น ไม่มีการรับเงินเพื่อเชียร์สินค้าและไม่มีคู่มือแนะนำผู้ให้บริการ AI API ภายนอก ดังนั้นจะไม่มีสิ่งใดในนี้ชี้นำโค้ด คีย์ หรือข้อมูลของคุณไปยังที่ที่คุณไม่ได้เลือก</sub></p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Company sponsor: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsors: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Become a sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

## เริ่มต้นอย่างรวดเร็ว

วางข้อความนี้ลงในเอเจนต์เขียนโค้ดของคุณ:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

หรือติดตั้งเอง:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **สำหรับเอเจนต์:** อ่าน [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) เพื่อดูขั้นตอนติดตั้ง คำสั่งทั้งหมด และขั้นตอนการพอร์ต

## สัญญาอนุญาต

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman) โค้ดที่พอร์ตและโค้ดที่สร้างขึ้นในคลังของคุณเป็นกรรมสิทธิ์ของคุณทั้งหมด
