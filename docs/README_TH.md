<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="The rust-anything crab">
</p>

<h1 align="center">rust-anything</h1>

<p align="center"><sub><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a> · <a href="README_ES.md">Español</a> · <a href="README_IT.md">Italiano</a> · <a href="README_ID.md">Bahasa Indonesia</a> · <a href="README_AR.md">العربية</a> · <a href="README_FA.md">فارسی</a> · <a href="README_HI.md">हिन्दी</a> · ไทย · <a href="https://github.com/erkamyaman/rust-anything/issues/425">+ add yours</a></sub></p>
<p align="center"><sub>คำแปลนี้ดูแลโดยชุมชน และอาจอัปเดตช้ากว่า <a href="../README.md">README ภาษาอังกฤษ</a></sub></p>

<p align="center"><b>เขียนแอปของคุณใหม่ด้วย Rust และพิสูจน์ว่าพฤติกรรมการทำงานยังคงเหมือนเดิมทุกประการ</b><br>
เพียงชี้เครื่องมือไปที่แอปของคุณ: ระบบจะบันทึกสิ่งที่แอปเดิมทำ จากนั้น coding agent ของคุณจะเขียนโค้ด Rust ทีละส่วน<br>
และทุกส่วนจะถูกตรวจสอบเทียบกับสิ่งที่บันทึกไว้ก่อนจะข้ามไปทำส่วนถัดไป มันไม่ได้แค่แปลโค้ด แต่มันพิสูจน์การเขียนใหม่</p>

<p align="center"><b>เอกสารคู่มือ:</b> <a href="https://rust-anything.com/">https://rust-anything.com/</a></p>

<p align="center">
  <img src="https://img.shields.io/github/languages/top/erkamyaman/rust-anything?logo=rust&label=Rust&color=CE422B" alt="Rust share of the code">
  <img src="https://img.shields.io/badge/macOS%20·%20Linux%20·%20Windows-native-2563EB" alt="Native binaries for macOS, Linux and Windows">
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-555" alt="MIT license"></a>
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io version"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm version"></a>
  <br>
  <a href="https://coderabbit.link/er-kam"><img src="https://img.shields.io/coderabbit/prs/github/erkamyaman/rust-anything?labelColor=171717&color=FF570A&label=Reviewed+by+CodeRabbit" alt="Pull requests reviewed by CodeRabbit"></a>
  <a href="https://github.com/rust-unofficial/awesome-rust#development-tools"><img src="https://awesome.re/mentioned-badge.svg" alt="Mentioned in Awesome Rust"></a>
  <a href="https://github.com/AIshihzong/awesome-reverse-engineer-anything"><img src="https://img.shields.io/badge/Featured%20in-Awesome%20REA-f0b429" alt="Featured in Awesome REA"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub stars"></a>
</p>

<p align="center"><a href="https://github.com/erkamyaman/rust-anything/discussions/127"><img src="https://img.shields.io/badge/Join%20the%20discussion-6E40C9?style=for-the-badge&logo=github&logoColor=white" alt="Join the discussion"></a> <a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/Join%20the%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Join the Discord"></a></p>

<p align="center"><a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily" alt="erkamyaman/rust-anything: #2 repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly" alt="erkamyaman/rust-anything: #6 repository of the week on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a></p>

<p align="center"><b><a href="#quick-start">ติดตั้ง</a></b> · <a href="https://github.com/erkamyaman/rust-anything/releases">Releases</a> · <a href="#quick-start">ปลั๊กอิน Claude Code</a> · <a href="https://registry.modelcontextprotocol.io/v0.1/servers?search=rust-anything">MCP Registry</a> · <a href="real-world.md">การพอร์ตในโลกจริง</a> · <a href="faq.md#faq">คำถามที่พบบ่อย (FAQ)</a><br>
<sub><a href="#languages">ภาษาที่รองรับ</a> · <a href="how-it-works.md#how-it-works">การทำงาน</a> · <a href="whole-apps.md#port-a-whole-app">พอร์ตทั้งแอป</a> · <a href="#sponsors">ผู้สนับสนุน</a> · <a href="../LICENSE">MIT</a></sub><br>
Made in Türkiye 🇹🇷 by <a href="https://github.com/erkamyaman">erKam Yaman</a> · <a href="https://github.com/sponsors/erkamyaman">❤️ สนับสนุนฉัน</a> · สนับสนุนโดย <a href="https://coderabbit.link/er-kam"><img src="assets/brands/coderabbit.png" width="22" height="22" align="absmiddle" alt=""> <b>CodeRabbit</b></a></p>

<p align="center">ผลงานอื่นของผู้พัฒนา: <a href="https://github.com/erkamyaman/safe-anything"><b>safe-anything</b></a> เครื่องมือสแกนความปลอดภัยด้วย Rust 100% สำหรับโค้ด คีย์ความลับ การตั้งค่า และระบบ AI agent ตรวจหาช่องโหว่ก่อนที่จะมีคนอื่นพบ</p>

<p align="center">ใหม่ใน 2.0: <a href="real-world.md#prettier">Prettier ถูกพอร์ตสู่ Rust</a>. เร็วขึ้น 19 เท่า · 15,861 จาก 17,224 การรันที่บันทึกไว้ทำงานบน Rust ทั้งหมด · ผลลัพธ์ตรงกันทุกไบต์</p>

<p align="center">
  <img src="assets/prettier.gif" alt="Prettier และเวอร์ชันพอร์ต Rust จัดรูปแบบ 59 ไฟล์จริงเหมือนกัน: Prettier ใช้เวลา 3.63 วินาที ส่วนพอร์ต Rust ใช้ 0.20 วินาที ได้ผลลัพธ์ตรงกันทุกไบต์" width="820">
  <br>
  <sub>Prettier เทียบกับพอร์ต Rust บนไฟล์จริง 59 ไฟล์จาก ms, Fuse.js และ json-server (Apple M4) ผลลัพธ์ตรงกันทุกไบต์ <a href="real-world.md#prettier">วิธีการวัดผล</a></sub>
</p>

<p align="center"><sub>PHP, C# และ PowerShell: พอร์ตระดับยูนิต ผ่าน FFI และ P/Invoke</sub></p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>บันทึก (Record)</b><br>
      <code>record</code> รันโค้ดต้นฉบับบน edge cases ที่สร้างขึ้น กับดักเชิงความหมาย (semantic traps) และอินพุตของคุณเอง พร้อมบันทึกค่าที่ส่งกลับหรือ error ที่เกิดขึ้น
    </td>
    <td width="25%" valign="top">
      <b>พอร์ต (Port)</b><br>
      <code>bridge</code> สร้าง Rust crate และ wrapper ขึ้นมา เพื่อให้โค้ดส่วนอื่นยังเรียกใช้ API เดิมได้ โดยคุณหรือ agent ของคุณเป็นผู้เขียนโค้ด Rust
    </td>
    <td width="25%" valign="top">
      <b>ตรวจสอบ (Check)</b><br>
      <code>diff</code> รันทุกกรณีที่บันทึกไว้ซ้ำอีกครั้งเทียบกับโค้ด Rust และจะยังไม่มีการปล่อยใช้งานจนกว่าทุกผลลัพธ์จะตรงกันและผ่าน <code>gate</code>
    </td>
    <td width="25%" valign="top">
      <b>ยกระดับ (Lift)</b><br>
      เมื่อการรันซ้ำผ่านทั้งหมดโดยไม่ต้องย้อนกลับ (fallback) และทุกยูนิตผ่านเกณฑ์ <code>app lift</code> จะเปลี่ยนให้กลายเป็นแอป Pure Rust โดยไม่แตะต้องโค้ดเดิม
    </td>
  </tr>
</table>

**เขียนแอปทั้งหมดของคุณใหม่ด้วย Rust ทีละส่วน** rust-anything บันทึกสิ่งที่แอป Node.js (TypeScript/JavaScript), Python, C, Go, Ruby, Java หรือ Kotlin ของคุณทำในปัจจุบัน จากนั้นตรวจสอบโค้ด Rust แต่ละส่วนที่เขียนใหม่เทียบกับบันทึกนั้นก่อนจะข้ามไปทำส่วนถัดไป จนกระทั่งแอปกลายเป็น Pure Rust เหมาะสำหรับ CLI, HTTP server และไลบรารี โดย coding agent ของคุณ (หรือตัวคุณเอง) เป็นผู้เขียนโค้ด Rust เครื่องมือนี้สร้างขึ้นบน [napi-rs](https://github.com/napi-rs/napi-rs), [PyO3](https://github.com/PyO3/pyo3) และ [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) ซึ่งทำหน้าที่เชื่อมต่อการเรียกใช้ระหว่างภาษาของคุณกับ Rust; rust-anything จะตัดสินใจว่าควรย้ายส่วนใดและพิสูจน์ว่าผลลัพธ์ตรงกันหรือไม่ ทั้งนี้ระบบจะตรวจจับได้เฉพาะสิ่งที่ครอบคลุมในการบันทึกเท่านั้น: ดู [สิ่งที่ระบบไม่สามารถตรวจจับได้](how-it-works.md#what-it-cant-catch) คุณสามารถพอร์ตทั้งเครื่องมือและเซิร์ฟเวอร์ หรือเลือกเฉพาะส่วนที่ทำงานช้า ทีละ hot path ได้:

| โปรเจกต์จริง, ผลลัพธ์เหมือนต้นฉบับ | ต้นฉบับเดิม | เมื่อใช้ rust-anything |
| --- | ---: | ---: |
| [Prettier](real-world.md#prettier) 3.6.2 บน 2,360 ไฟล์ JS/TS จากคอมมานด์ไลน์ | 128.6 วินาที | **6.9 วินาที: เร็วขึ้น 19 เท่า** |
| Prettier บน 6,305 ไฟล์ Markdown จากคอมมานด์ไลน์ | 352 วินาที | **22.5 วินาที: เร็วขึ้น 15.6 เท่า** |
| [json-server](real-world.md#json-server) ทั้ง Node server: หน่วยความจำขณะ idle | 74 MB | **3.5 MB**, ไบนารีเดียวขนาด 5.3 MB |
| json-server ตั้งแต่เริ่มต้นจนตอบกลับ request แรก | 79 ถึง 85 ms | **8 ถึง 10 ms** |
| [Fuse.js](real-world.md#fusejs) ค้นหาบนสตริง 50,000 รายการ ต่อหนึ่งคำค้นหา | 91.2 ms | **2.8 ms: เร็วขึ้น 32 เท่า** |

Prettier: 15,861 จาก 17,224 กรณีรันที่บันทึกไว้จัดรูปแบบด้วย Rust อย่างสมบูรณ์ใน 10 ภาษา (JavaScript, TypeScript, JSX, JSON, GraphQL, YAML, Markdown, CSS, SCSS, Less) และทั้งหมด 17,224 รายการให้ผลลัพธ์ตรงกับ Prettier ทุกไบต์ เนื่องจากส่วนใดที่ยังไม่ได้พอร์ตจะ fallback กลับไปใช้ Prettier เดิม ส่วน json-server: 514/514 กรณีทดสอบ HTTP ที่บันทึกไว้ทำงานตรงกันบนไบนารี Pure Rust โดยวัดผลบน Apple M4 รันอย่างละสองรอบและเปรียบเทียบผลลัพธ์ทุกครั้งก่อนจับเวลา ([รายละเอียด](real-world.md)) ทั้งนี้การเรียกใช้ Rust บนฟังก์ชันขนาดเล็กๆ ทีละครั้งมักจะ *ช้ากว่า* JIT; rust-anything จึงค้นหาลูปที่คุ้มค่าแก่การย้ายและจัดการย้ายลูปนั้น สำหรับภาษา C สิ่งที่ได้คือความปลอดภัยไม่ใช่ความเร็ว: [libavutil ของ ffmpeg](real-world.md#ffmpeg) ที่พอร์ตไปยัง Rust ให้ผลลัพธ์ตรงกับ 226,447 การเรียกใช้งานที่บันทึกไว้ทั้งหมด และทำงานด้วยความเร็วใกล้เคียงกับ C (18 จาก 42 เบนช์มาร์กเร็วเท่าเดิม, 8 เร็วขึ้น และ 16 ช้าลง 3 ถึง 40%)

## เพียงเฉพาะส่วนที่ช้า

คุณไม่จำเป็นต้องพอร์ตทั้งแอป เพียงชี้คำสั่ง `go` ไปยังคำสั่งที่ทำงานช้าเกินไป ระบบจะพอร์ตเฉพาะฟังก์ชันที่ใช้เวลานานที่สุดให้:

```sh
rust-anything go --agent claude -- python3 app.py
```

ระบบจะทำโพรไฟล์การรัน เลือกฟังก์ชันชั้นนอกสุดที่คุ้มค่าแก่การย้าย (เช่น ลูปการทำงาน ไม่ใช่ฟังก์ชันย่อยขนาดเล็กที่อยู่ข้างใน พร้อมจัดกลุ่มแบบ batch ตามที่ผลโพรไฟล์แนะนำ) บันทึกฟังก์ชันนั้นบนอินพุตที่สร้างขึ้นพร้อมการเรียกใช้จริงที่เกิดขึ้น แล้วส่งมอบไฟล์ `src/port.rs` ให้กับ agent ของคุณ (`claude`, `codex` หรือคำสั่งใดๆ ที่รับ prompt ทาง stdin; หากไม่ระบุ `--agent` ระบบจะสร้างไฟล์ `TASK.md` ให้คุณแล้วหยุดการทำงาน) ระบบจะเปิดใช้งานฟังก์ชัน Rust เฉพาะเมื่อทุกกรณีทดสอบที่บันทึกไว้ตรงกันและผ่าน `gate` เท่านั้น จากนั้น:

```sh
rust-anything report rust/dedupe-rs     # สร้าง REPORT.md และ card.svg สำหรับ pull request
rust-anything disable rust/dedupe-rs    # สลับกลับไปใช้โค้ดเดิมแบบตรงกันทุกไบต์
```

แถว [Fuse.js](real-world.md#fusejs) ในตารางข้างต้นก็ใช้รูปแบบนี้: การคำนวณคะแนนของ Fuse สำหรับสตริงหนึ่งตัวถูกรวมเป็น batch บนลิสต์ทั้งหมดแล้วย้ายไปยัง Rust ในขณะที่คีย์ของอ็อบเจกต์ การค้นหาแบบขยาย และส่วนที่เหลือยังคงอยู่ใน JavaScript ทำให้เวลาลดลงจาก 91.2 ms เหลือเพียง 2.8 ms ต่อการค้นหาบนสตริง 50,000 ตัว สามารถลองใช้ `go` ได้ที่ [`tests/fixtures/go-fuzzy`](../tests/fixtures/go-fuzzy) ซึ่งเป็นโค้ด Python ตัดคำซ้ำขนาด 20 บรรทัด ดูวิธีการเปิดและปิดการใช้งานพอร์ตได้ที่: [การเปิดและปิดพอร์ต](how-it-works.md#switch-a-port-on-and-off)

<p align="center">
  <img src="assets/json-server.gif" alt="json-server ในฐานะ Node server และในฐานะไบนารี Rust เดี่ยว: หน่วยความจำขณะ idle 74 MB และ 3.5 MB, เวลาเริ่มจนถึง response แรก 79-85 ms และ 8-10 ms และผลทดสอบ HTTP 514/514 กรณีตรงกัน" width="820">
  <br>
  <sub>json-server เซิร์ฟเวอร์ Node ทั้งตัวถูกยกระดับเป็นไบนารี Pure Rust (Apple M4) <a href="real-world.md#json-server">วิธีการวัดผล</a></sub>
</p>

> rust-anything เป็นโครงการอิสระ ไม่ได้มีความเกี่ยวข้องหรือได้รับการรับรองจาก Rust Project หรือ Rust Foundation เครื่องหมายการค้า "Rust" เป็นกรรมสิทธิ์ของ Rust Foundation

> [!WARNING]
> คำสั่ง `record` และ `app record` จะรันโค้ดต้นฉบับของคุณ โดยเรียกแต่ละฟังก์ชัน (หรือทั้งแอป) ด้วยอินพุตที่สร้างขึ้นจำนวนมาก และ `app record` จะรันแต่ละกรณีสองครั้ง ส่วน `diff`, `gate` และ `app replay` จะรันกรณีเหล่านั้นซ้ำอีกครั้งบนโค้ดที่พอร์ตหรือโค้ดต้นฉบับ โค้ดที่มีการเขียนไฟล์ ติดต่อฐานข้อมูล หรือเชื่อมต่อเครือข่ายจะทำงานจริงในแต่ละกรณีทดสอบ ดังนั้นควรสั่งรันบนสำเนาของโปรเจกต์ ใช้ฐานข้อมูลทดสอบ และไม่มีคีย์ข้อมูลรับรองสำหรับใช้งานจริง (production credentials) อยู่ใน Environment Variables

## ติดตั้งพอร์ตสำเร็จรูป

พอร์ต ESLint มีให้บริการในรูปแบบแพ็กเกจ npm โดยจะรันพอร์ต Rust ในส่วนที่ครอบคลุม และส่งต่อไปยัง ESLint 10.15.0 (ที่ระบุเป็น dependency) ในส่วนที่เหลือ ดังนั้นผลลัพธ์จึงเป็นของ ESLint ทั้งสองกรณี:

```sh
npm install -g eslint-rs     # ESLint 10.15.0
```

| คำสั่ง | ประมวลผลใน Rust | ส่งต่อไปยัง ESLint 10.15.0 |
| --- | --- | --- |
| `eslint-rs` | `--no-config-lookup --rule ... --stdin` พร้อม 90 rules ที่ถูกพอร์ตแล้ว (เอาต์พุตแบบ `stylish` และ `json`, การแก้ไขอัตโนมัติ, คำแนะนำ) | การค้นหา `eslint.config.*` (`eslint .`), ไฟล์และโฟลเดอร์, core rules ที่เหลืออีก 202 ข้อ, ปลั๊กอิน, พาร์สเซอร์, ฟอร์แมตเตอร์อื่นๆ, parse errors |

ไบนารีรองรับ macOS (arm64, x64) และ Linux (x64, arm64, glibc) บน Windows และสภาพแวดล้อมอื่นๆ หรือเมื่อใช้ `RUST_ANYTHING_MODE=old` ตัว `eslint-rs` จะรัน ESLint ดั้งเดิมแทน (ได้ผลลัพธ์เหมือนเดิมแต่ไม่ได้รับความเร็วเพิ่ม) เมื่อติดตั้งจาก tarball บน macOS arm64 ตัว `eslint-rs` ให้ผลลัพธ์ตรงกับ ESLint 10.15.0 ที่ติดตั้งข้างเคียงกันครบทั้ง 10,585 การรันที่บันทึกไว้ ([อ่านเพิ่มเติม](packaging.md#a-whole-cli-front-door))

สำหรับ Prettier พอร์ตถูกแพ็กเกจในลักษณะเดียวกัน (`prettier-rs`, Prettier 3.6.2) และผ่านการรันซ้ำ 17,224/17,224 กรณีที่บันทึกไว้จาก tarball โดยมีเพียง 7.9% ที่ถูกส่งต่อไปยัง Prettier แต่ยังไม่ได้เผยแพร่บน npm ทั้งนี้คำสั่ง `rust-anything package` สามารถสร้างแพ็กเกจแบบนี้จาก CLI front door ใดก็ได้

<h2 id="quick-start">เริ่มต้นใช้งานด่วน</h2>

คัดลอกข้อความด้านล่างนี้แล้ววางลงใน Claude Code, Codex, Cursor, Gemini CLI หรือ OpenCode เพื่อให้ agent ของคุณติดตั้งให้โดยอัตโนมัติ:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

หรือติดตั้งด้วยตนเอง:

```sh
cargo install rust-anything   # ติดตั้งจาก crates.io
rust-anything setup           # ลงทะเบียนกับ Claude Code, Codex, Cursor, Gemini CLI และ OpenCode
```

หากยังไม่ได้ติดตั้ง Rust toolchain บนเครื่อง สามารถดาวน์โหลดไบนารีสำเร็จรูปสำหรับ macOS, Linux และ Windows ได้จาก [GitHub Releases](https://github.com/erkamyaman/rust-anything/releases) และสำหรับโปรเจกต์ JavaScript สามารถใช้ `npm install -g rust-anything` เพื่อติดตั้งไบนารีเดียวกันได้

สำหรับ Claude Code คุณยังสามารถติดตั้งในรูปแบบปลั๊กอิน (ประกอบด้วย MCP server และ skill; โดย CLI ด้านบนต้องอยู่ใน `PATH` ของคุณ):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

สำหรับ Codex สามารถติดตั้งจาก agents marketplace ในคลังนี้ (`.agents/plugins/marketplace.json` ปลั๊กอินเดียวกันภายใต้โฟลเดอร์ `plugin/` พร้อม skill และ MCP server):

```sh
codex plugin marketplace add erkamyaman/rust-anything
codex plugin add rust-anything@rust-anything
```

คำสั่ง `setup` จะเพิ่ม MCP server ให้กับแต่ละ agent ที่ตรวจพบ และติดตั้ง workflow skill ลงในตำแหน่งที่ agent นั้นอ่านข้อมูล โดยจะคงการตั้งค่าอื่นๆ ของคุณไว้ สามารถสั่งรันซ้ำได้อย่างปลอดภัย และสามารถใช้คำสั่ง `--remove` เพื่อยกเลิกการตั้งค่าได้

จากนั้นคุณสามารถสั่ง agent ของคุณได้ เช่น:

- *"Rewrite this CLI in Rust with rust-anything, end to end, and prove it behaves the same."*
- *"This search feels slow. Profile it with rust-anything and move the hot loop to Rust."*
- *"Plan a migration of `src/parser/` to Rust with rust-anything; port the first unit and prove it."*
- *"Bench the Rust port in `rust/text-rs` against the original and tell me whether to ship it."*

<h3 id="works-with">ใช้งานร่วมกับ</h3>

| Agent | สิ่งที่ `rust-anything setup` เพิ่มเข้าไป |
| --- | --- |
| Claude Code | MCP server ใน `~/.claude.json`, skill ใน `~/.claude/skills/` |
| Codex | MCP server ใน `~/.codex/config.toml`, skill ใน `~/.codex/skills/` |
| Cursor | MCP server ใน `~/.cursor/mcp.json` |
| Gemini CLI | MCP server ใน `~/.gemini/settings.json` |
| OpenCode | MCP server ใน `~/.config/opencode/opencode.json` (`mcp.servers`), skill ใน `~/.config/opencode/skills/` |
| MCP Client อื่นๆ | เพิ่ม `rust-anything mcp` เป็น stdio server ด้วยตนเอง (ดู [วิธีติดตั้ง](install.md#install)) |

ก่อนจะแก้ไขไฟล์การตั้งค่าที่มีอยู่เดิม `setup` จะบันทึกไฟล์สำรองไว้ข้างกันเสมอ (`<file>.rust-anything.bak`); สามารถใส่ `--no-backup` เพื่อข้ามการสร้างไฟล์สำรองได้

สำหรับการตั้งค่า OpenCode ด้วยตนเอง ให้ใส่การกำหนดค่าเซิร์ฟเวอร์ใน `~/.config/opencode/opencode.json` (ช่อง `command` ระบุ argv ทั้งหมด) แล้วคัดลอกโฟลเดอร์ [`skills/rust-anything/`](../skills/rust-anything) ไปไว้ที่ `~/.config/opencode/skills/rust-anything/`:

```json
{
  "mcp": {
    "servers": {
      "rust-anything": { "type": "local", "command": ["rust-anything", "mcp"] }
    }
  }
}
```

หรือสั่งรันขั้นตอนต่างๆ ด้วยตัวเอง:

```sh
rust-anything profile . -- node bench.mjs              # ตรวจดูว่าจุดไหนช้า และใครเป็นผู้เรียกใช้
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# เขียนโค้ดใน src/port.rs (หรือให้ agent เขียนให้) จากนั้น:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # ทุกกรณีที่บันทึกไว้ต้องตรงกัน
rust-anything fuzz rust/fuzzy-rs --time 60 --keep      # สุ่มอินพุตใหม่เพื่อหาจุดที่บันทึกตกหล่น
rust-anything bench rust/fuzzy-rs                      # วัดความเร็วเพื่อเปรียบเทียบ
rust-anything gate rust/fuzzy-rs                       # พร้อมใช้งานเมื่อคำสั่งนี้ผ่าน
```

rust-anything ไม่ได้ทำหน้าที่แปลภาษาโค้ด: coding agent ของคุณ (Claude Code, Codex, Cursor, OpenCode, ...) หรือตัวคุณเองจะเป็นผู้เขียน Rust โดยที่ rust-anything จะทำหน้าที่เติมเต็มสิ่งที่ agent ไม่สามารถคาดเดาได้อย่างแม่นยำ ได้แก่: เวลาหมดไปกับจุดไหน, ควรพอร์ตส่วนใดก่อนและจัดลำดับอย่างไร, สร้างสะพานเชื่อม (bridge) เพื่อให้โค้ดส่วนอื่นยังเรียกใช้ Rust ได้โดย API ไม่เปลี่ยนแปลง, พฤติกรรมที่แน่นอนที่พอร์ตต้องทำตาม, วิธีการปล่อยใช้งานอย่างปลอดภัย และ gate สำหรับชี้วัดว่าพอร์ตเสร็จสมบูรณ์แล้วหรือไม่

เพียงแค่ผลลัพธ์เหมือนกันยังไม่พอ ให้บันทึกสิ่งที่แอปส่งออกไปด้วย ตัวอย่างเช่น การเขียนใหม่ที่ส่งอีเมลซ้ำสองครั้งจะถูกตรวจจับว่าล้มเหลว:

<p align="center">
  <img src="assets/effects.gif" alt="app record พร้อม --effects smtp บันทึกอีเมลต้อนรับหนึ่งฉบับต่อการสมัคร; app replay ของโค้ดที่เขียนใหม่ซึ่งพิมพ์เอาต์พุตตรงกันแต่ส่งอีเมลซ้ำสองครั้งจะรายงาน 0/2 กรณีผ่าน และแสดงอีเมลส่วนเกินที่ถูกส่ง" width="820">
  <br>
  <sub>การรันจริงของคำสั่ง <code>app record --effects</code> และ <code>app replay</code> ดูเพิ่มเติมที่ <a href="whole-apps.md#side-effects-email-webhooks-and-databases">Side effects</a></sub>
</p>

<h2 id="languages">ภาษาที่รองรับ</h2>

| ภาษา | ยูนิต (Unit) | สะพานเชื่อม (Bridge) | บันทึกและรันซ้ำโดย |
| --- | --- | --- | --- |
| TypeScript/JavaScript | โฟลเดอร์หรือไฟล์ | napi-rs หรือ wasm-bindgen พร้อมไบนารี wire ที่สร้างขึ้น; รองรับทั้ง classes และ async | Node, รันจากไฟล์ซอร์สโค้ด `.ts` |
| Python | แพ็กเกจหรือโมดูล | PyO3 (abi3), แปลงชนิดข้อมูลโดยตรง, รักษา typed exceptions | Python, นำเข้าโมดูลต้นฉบับ |
| C | ไฟล์ `.c` | ฟังก์ชัน `extern "C"` โดยใช้ชื่อเดิมพร้อมเฮดเดอร์ไฟล์ | โค้ดต้นฉบับคอมไพล์เป็น shared library เรียกผ่าน ctypes โดยแยกหนึ่ง fork ต่อหนึ่งกรณี |
| Go | แพ็กเกจ | การเรียกแบบ typed cgo และไบนารี wire ที่สร้างขึ้น อยู่ในแพ็กเกจที่คง API เดิมไว้ | ไดรเวอร์ Go ที่สร้างขึ้น ซึ่งคอมไพล์เป็นโมดูลของตัวเอง |
| Ruby | ไฟล์หรือโฟลเดอร์ของฟังก์ชันโมดูล | ส่วนขยาย magnus ภายใต้โมดูลเดิม | ไดรเวอร์ Ruby ที่สร้างขึ้น |
| Java | แพ็กเกจของเมธอด `public static` | typed JNI และไบนารี wire ในคลาส wrapper ที่มีเมธอดเดิม | ไดรเวอร์ reflection ที่สร้างขึ้น |
| Kotlin | โมดูล Gradle ของฟังก์ชันระดับบนสุดและเมธอด `@JvmStatic` | Java bridge: typed JNI ในคลาส wrapper ของ Java ที่เรียก Kotlin statics | ไดรเวอร์ reflection ที่สร้างขึ้น โดยคอมไพล์ซอร์สโค้ด Kotlin ด้วย `kotlinc` |
| PHP | ไฟล์หรือโฟลเดอร์ของฟังก์ชันระดับบนสุด (เมธอด `public static` จะคงอยู่ใน PHP) | JSON dispatcher ผ่าน `ext-ffi` ในไฟล์ที่ประกาศฟังก์ชันซ้ำด้วยพารามิเตอร์เดิม | ไดรเวอร์ PHP ที่สร้างขึ้น; สำหรับทั้งแอปใช้ `app record`, `app replay` และ front door ทั้ง CLI และเซิร์ฟเวอร์ (`php -S`) |
| PowerShell | โมดูล `.psm1` หรือสคริปต์ `.ps1` ที่มีบล็อก `param()` (ส่งออกตาม `Export-ModuleMember`) | JSON dispatcher ผ่าน P/Invoke ที่ประกาศตอนโหลดด้วย Reflection.Emit ในไฟล์ `.ps1` ที่ประกาศฟังก์ชันเดิมพร้อมบล็อก `param()` | ไดรเวอร์ `pwsh` ที่สร้างขึ้น (PowerShell 7.4+) |
| C# | ไฟล์หรือโฟลเดอร์ของเมธอด `public static` | JSON dispatcher ผ่าน P/Invoke ในคลาส `<namespace>.Rust` ที่มีเมธอดเดิม | ไดรเวอร์ reflection ที่คอมไพล์ร่วมกับซอร์สโค้ดของโปรเจกต์ด้วย `dotnet build`; สำหรับทั้งแอปใช้ `app record`, `app replay` และ front door สำหรับ CLI และ ASP.NET servers (`dotnet <app>.dll`) |

ทุกภาษายกเว้น C, PHP, C# และ PowerShell จะได้รับคลาสแบบ batch (`--batch`): ข้อมูลจะส่งข้ามสะพานเชื่อมเพียงครั้งเดียวและคงอยู่ในฝั่ง Rust และการเรียกใช้แต่ละครั้งจะประมวลผลข้อมูลทั้งหมดพร้อมกันบนทุกคอร์ซีพียู

ระบบจะตรวจจับภาษาของโปรเจกต์จากไฟล์ในคลังโค้ดโดยอัตโนมัติ (`go.mod`, `pom.xml`, `build.gradle`/`build.gradle.kts`, `.csproj` หรือ `.sln`, `pyproject.toml`, `Gemfile`, `composer.json`, `package.json` หรือนามสกุลไฟล์ซอร์สโค้ดที่พบบ่อยที่สุด) หากโมดูล Gradle มีไฟล์ `.kt` โดยไม่มีไฟล์ `.java` จะนับว่าเป็น Kotlin แต่หากผสมกันจะถือว่าเป็น Java สามารถใช้ `--lang` เพื่อระบุภาษาด้วยตนเองได้ ชนิดข้อมูลจะถูกดึงมาจากซอร์สโค้ด: ไทป์ของ TS, type hints ของ Python, ซิกเนเจอร์ของ C, Go, Java, Kotlin และ C#, YARD tags ของ Ruby, types และ phpdoc ของ PHP; สำหรับพารามิเตอร์ที่ไม่ระบุชนิดข้อมูลแต่ถูกนำไปใช้เป็นตัวเลข ระบบจะอนุมานว่าเป็นชนิดตัวเลข

สำหรับ Android คำสั่ง `rust-anything build <crate> --profile android` จะบิลด์ไลบรารี JNI ด้วย NDK สำหรับสถาปัตยกรรม `arm64-v8a`, `armeabi-v7a` และ `x86_64` (หรือตามค่า `--abi` ที่ระบุ) และติดตั้งลงใน `<crate>/jniLibs/<abi>/`; พารามิเตอร์ `--api` กำหนดระดับ API (ขั้นต่ำ 26 ตามที่ Java runtime ที่สร้างขึ้นต้องการ) ระบบจะค้นหา NDK จาก `ANDROID_NDK_HOME`/`ANDROID_NDK_ROOT` หรือ SDK และจำเป็นต้องติดตั้ง Rust targets (`rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`) เชื่อมโยงไลบรารี 64-bit ด้วยการจัดแนวหน้าหน่วยความจำ 16 KB และตรวจสอบว่าทุก JNI entry point ถูก export ออกมาอย่างถูกต้อง โดยปกติ Gradle จะค้นหาใน `src/main/jniLibs` ดังนั้นควรระบุ `--jni-libs app/src/main/jniLibs` หรือเพิ่ม `<crate>/jniLibs` ลงใน `sourceSets.main.jniLibs.srcDirs`
คำสั่ง `rust-anything record <crate> --profile android` จะรันยูนิต Kotlin ดั้งเดิมภายใต้ ART บนอุปกรณ์ที่เชื่อมต่อ (adb) แปลงไดรเวอร์ที่สร้างขึ้นเป็น dex ด้วย `d8` จาก SDK build-tools แล้วดึงผลลัพธ์กลับมาเป็นไฟล์ `cases.jsonl` เดียวกัน จากนั้นคำสั่ง `diff <crate> --profile android` จะรันซ้ำเทียบกับไฟล์ `.so` ที่บิลด์ขึ้นสำหรับ ABI ของอุปกรณ์นั้น ทั้งคู่ต้องการ `adb`, อุปกรณ์ทดสอบ, `kotlinc`, SDK ที่มี build-tools และแพลตฟอร์ม และใส่ `--device <serial>` เมื่อมีอุปกรณ์เชื่อมต่อมากกว่าหนึ่งเครื่อง โปรไฟล์นี้ครอบคลุมยูนิตเชิงตรรกะ: ส่วนของ UI อย่าง Activity/Compose/XML ตลอดจนคลาส lifecycle และ SDK-only จะยังคงอยู่ใน Kotlin และเนื่องจากการเรียกใช้แต่ละครั้งจะต้องข้ามผ่าน JNI จึงแนะนำให้ใช้ batch wrapper สำหรับลูปที่มีการทำงานหนัก บนเครื่องโฮสต์ คำสั่ง `record` และ `diff` จะยังคงใช้ JVM บนเดสก์ท็อป

สำหรับ PHP คำสั่ง `analyze` จะวางแผนการพอร์ตแอป PHP (ตรวจสอบแพ็กเกจ Composer และการโหลดอัตโนมัติ PSR-4, ความเสี่ยงเช่นการเปรียบเทียบแบบหลวม loose comparison และ integer overflow กลายเป็น float, และจับคู่แพ็กเกจ Composer เข้ากับ crates) นอกจากนี้ยังสามารถบันทึก รันซ้ำ และวางไว้หลัง Rust front door ได้ สำหรับฟังก์ชันระดับบนสุดของยูนิตที่มีพารามิเตอร์แบบ scalar, string, list และ associative array จะเชื่อมต่อผ่าน PHP FFI extension (`php` พร้อม `ext-ffi` ซึ่งเปิดใช้งานเป็นค่าเริ่มต้นสำหรับ CLI): เมื่อ `diff` ผ่านแล้ว ให้เปลี่ยนไป `require` ไฟล์ `php/<crate>.php` ที่สร้างขึ้น (ชื่อไฟล์คือชื่อ crate ที่เปลี่ยน `-` เป็น `_` เช่น `foo-rs` จะได้ `php/foo_rs.php`) ในจุดที่แอปเคยเรียกไฟล์เดิม และวางไฟล์ทั้งสองไว้ที่เดิมเนื่องจาก wrapper จะโหลด native library แบบ relative ตามโฟลเดอร์ของมัน ตัว wrapper จะประกาศฟังก์ชันที่เชื่อมต่อไว้โดยตรงโดยไม่มีเงื่อนไข `function_exists` จึงเป็นการแทนที่ไฟล์เดิมไม่ใช่ใช้งานร่วมกัน สำหรับส่วนอื่นๆ ในไฟล์เดิม เช่น คลาส, เมธอด `public static`, ฟังก์ชันที่ตัวเชื่อมต่อยังคงไว้ใน PHP (เช่น พารามิเตอร์ `callable` หรือ `Closure`) และคำสั่งระดับบนสุดเช่น `define('FEATURE_ENABLED', true)` จะไม่ถูกนำเข้ามาใน wrapper และการ require ทั้งสองไฟล์จะทำให้เกิดการประกาศฟังก์ชันซ้ำซ้อน ดังนั้นควรแยกส่วนดังกล่าวไว้ในไฟล์ที่ต้องเรียกใช้งานต่างหาก ทั้งนี้การเรียกใช้แต่ละครั้งจะส่งข้อมูลเป็น JSON ใช้เวลาประมาณ 1 µs ดังนั้นคำสั่ง `bench` จะแสดงให้เห็นว่าฟังก์ชันเล็กๆ อาจทำงานช้ากว่า PHP ดั้งเดิม และยังไม่มีฟังก์ชัน `shadow` สำหรับ PHP ในขณะนี้ ([issue #200](https://github.com/erkamyaman/rust-anything/issues/200))

สำหรับ C# มีการทำงานในรูปแบบเดียวกัน: `analyze` จะอ่านโปรเจกต์ `.csproj` และแพ็กเกจ NuGet (จับคู่กับ crates), แปลงคำสั่ง `using` ให้สอดคล้องกับโฟลเดอร์ และแจ้งเตือนสิ่งที่มีพฤติกรรมเฉพาะตัวใน C# (`decimal`, `dynamic`, LINQ deferred queries, `Task`, `lock`, reflection) แอปพลิเคชัน CLI หรือ ASP.NET server ในภาษา C# สามารถถูกบันทึก รันซ้ำ และนำไปไว้หลัง Rust front door ได้ เมธอด public static ของยูนิตที่มีพารามิเตอร์เป็น scalar, string, array, list, dictionary และ record หรือคลาสทั่วไปจะเชื่อมต่อผ่าน P/Invoke (ต้องมี `dotnet` บน PATH สำหรับคำสั่ง record และ diff): เมื่อ `diff` ผ่านแล้ว ให้คอมไพล์ wrapper ในโฟลเดอร์ `cs/` (ไฟล์เดี่ยวตามชื่อ crate เช่น `foo-rs` จะได้ `cs/FooRs.cs`) เข้าสู่โปรเจกต์ และวาง native library ที่บิลด์แล้วไว้ข้างตัวแอป จากนั้นเรียกใช้คลาสที่สร้างขึ้นในเนมสเปซ `<namespace>.Rust` (`Acme.Text.Slug.Slugify` กลายเป็น `Acme.Text.Rust.Slug.Slugify`) หรือใช้นามแฝงด้วยคำสั่ง `using` แต่ละการเรียกใช้จะส่งข้อมูลเป็น JSON ใช้เวลาประมาณ 1-3 µs ทำให้ฟังก์ชันเล็กๆ อาจช้ากว่า .NET และยังไม่มี `shadow` สำหรับ C# เช่นกัน ([issue #200](https://github.com/erkamyaman/rust-anything/issues/200))

สำหรับ PowerShell คำสั่ง `analyze` จะอ่านโมดูล `.psm1`, สคริปต์ `.ps1` และไฟล์ manifest `.psd1` (`RequiredModules`), ติดตามการใช้งาน `Import-Module`, `using module` และไฟล์ที่ dot-source พร้อมแจ้งเตือนพฤติกรรมเฉพาะของ PowerShell เช่น implicit output และ array unrolling, case-insensitive `-eq`, `-match` และคีย์ของ hashtable, `$null`, และการแปลงชนิดข้อมูลอัตโนมัติ (`5 / 2` ได้ 2.5 แต่ `[int]2.5` ได้ 2) ฟังก์ชันในยูนิตที่มีพารามิเตอร์แบบ `[string]`, ตัวเลขจำนวนเต็ม, `[double]`, `[bool]`/`[switch]`, array, `[hashtable]` หรือไม่ระบุชนิด จะเชื่อมต่อผ่าน P/Invoke จากไฟล์ `ps/<crate>.ps1` ที่สร้างขึ้น ส่วน filters, บล็อก `begin`/`process`, อินพุตแบบ pipeline, `[scriptblock]`, `[ref]` และชนิดข้อมูลอื่นๆ ของ .NET จะคงไว้ใน PowerShell พร้อมระบุเหตุผล เมื่อ `diff` ผ่านแล้ว คำสั่ง `rust-anything enable <crate>` จะทำการ dot-source ตัว wrapper ไว้ที่ท้ายไฟล์เดิม ข้อมูลจะถูกเข้ารหัสเป็น JSON โดย PowerShell ใช้เวลาประมาณ 0.1 ms สำหรับสเกลาร์ทั่วไปบน Apple M4 (และใช้เวลาเพิ่มขึ้นสำหรับ array และ hashtable) ดังนั้นเฉพาะฟังก์ชันที่มีภาระงานมากกว่าระยะเวลาดังกล่าวจึงจะเร็วขึ้นอย่างเห็นได้ชัด (คำสั่ง `bench` จะช่วยวัดผลให้) ทั้งนี้ความช้าส่วนใหญ่ของสคริปต์ PowerShell ในเครื่องมือต่างๆ มักเกิดจากการเริ่มโปรเซสใหม่ของ `pwsh` หรือ `powershell.exe` ในแต่ละคำสั่ง (ใช้เวลา 0.24 ถึง 0.45 วินาทีสำหรับ `pwsh -NoProfile -c 1` บน M4 เดียวกัน) ซึ่งการพอร์ตโค้ดไม่สามารถแก้ไขจุดนี้ได้: แนะนำให้เปิดเชลล์ทิ้งไว้และส่งคำสั่งเข้าไปรัน (ดูเพิ่มเติมที่ [FAQ](faq.md))

<h2 id="learn-more">เรียนรู้เพิ่มเติม</h2>

- [การทำงานของระบบ (How it works)](how-it-works.md): ลูปการทำงาน, การตัดสินใจของ `bridge` และ `analyze`, hot paths, สิ่งที่ไม่สามารถตรวจจับได้, สิ่งที่ต้องใช้ในการพอร์ตจริง, และการอัปเดตให้ทันโค้ดต้นฉบับ (`map`)
- [ส่งมอบพอร์ตเป็นแพ็กเกจ (Ship a port as a package)](packaging.md): maturin wheels และแพ็กเกจ npm แบบแยกแพลตฟอร์มของ napi-rs พร้อมตัวอย่าง CI ในการบิลด์
- [พอร์ตทั้งแอปพลิเคชัน (Port a whole app)](whole-apps.md): เซิร์ฟเวอร์, CLI และไลบรารี ตั้งแต่หน้าบ้านจนถึง `app lift`
- [ตัวอย่างและกรณีศึกษา (Examples and case studies)](examples.md): x18n และตัวอย่างโค้ดจริงใน 8 ภาษา
- [การพอร์ตในโลกจริง (Real-world ports)](real-world.md): json-server, Fuse.js, vercel/ms พร้อมผลเบนช์มาร์ก
- [การติดตั้ง อัปเดต และการพัฒนา (Install, update and develop)](install.md): ทุกวิธีการติดตั้ง, การใช้งานบน Windows และการบิลด์จากซอร์สโค้ด
- [คำถามที่พบบ่อย (FAQ)](faq.md) · [สถานะโครงการ (Status)](STATUS.md) · [เครื่องมือ MCP (MCP tools)](tools.md)

<h2 id="sponsors">ผู้สนับสนุน</h2>

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> เป็นโครงการโอเพนซอร์สที่ใช้งานได้ฟรี หากโครงการนี้ช่วยงานของคุณ โปรดพิจารณาช่วยสนับสนุน<br>
  การสนับสนุนของคุณช่วยให้การพัฒนาดำเนินต่อไปได้อย่างต่อเนื่อง
</p>

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

## การมีส่วนร่วม

ยินดีต้อนรับทุก Issues และ Pull Requests; สามารถอ่าน [CONTRIBUTING.md](../CONTRIBUTING.md) สำหรับการตั้งค่าสภาพแวดล้อม ตำแหน่งไฟล์ทดสอบ และวิธีเพิ่มภาษาใหม่ ส่วนการตัดสินใจเชิงออกแบบและเหตุผลสามารถดูได้ใน [DECISIONS.md](../DECISIONS.md) เรายินดีเป็นพิเศษสำหรับกรณีศึกษาการพอร์ตในโปรเจกต์จริง หากพบปัญหาด้านความปลอดภัยกรุณาแจ้งอย่างเป็นส่วนตัวตาม [SECURITY.md](../SECURITY.md) และโปรดอ่าน [แนวทางปฏิบัติของชุมชน (Code of Conduct)](../CODE_OF_CONDUCT.md)

## สัญญาอนุญาต

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman) โค้ดที่พอร์ตและโค้ดที่สร้างขึ้นในคลังของคุณเป็นกรรมสิทธิ์ของคุณทั้งหมด
