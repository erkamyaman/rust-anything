<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything qisqichbaqasi">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">Muallif: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · Oʻzbekcha · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a></p>
<p align="center"><sub>Bu tarjima hamjamiyat tomonidan yuritiladi va <a href="../README.md">inglizcha README</a>dan ortda qolishi mumkin.</sub></p>

<p align="center"><em>Butun ilovangizni Rust'da qayta yozing. Butun yo'l davomida u ishlab tursin.</em></p>

<p align="center"><em>CLI'lar, serverlar va kutubxonalar, qismma-qism, har qadamda asl nusxa bilan tekshirilgan holda.</em></p>

<p align="center">Node.js va TypeScript · Python · C · Go · Ruby · Java · PHP va C# preview'da</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io versiyasi"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm versiyasi"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 til, yana 2 tasi preview'da">
  <a href="#qollab-quvvatlanadigan-agentlar"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 ta agent bilan ishlaydi"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Testlar"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT litsenziyasi"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub yulduzlari"></a>
</p>

<p align="center">
  <a href="#tezkor-boshlash">Tezkor boshlash</a> ·
  <a href="#tillar">Tillar</a> ·
  <a href="how-it-works.md#how-it-works">Qanday ishlaydi</a> ·
  <a href="how-it-works.md#the-loop">Sikl</a> ·
  <a href="whole-apps.md#port-a-whole-app">Butun ilovani port qilish</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path'lar</a> ·
  <a href="install.md#install">O'rnatish</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Nimalarni aniqlay olmaydi</a> ·
  <a href="faq.md#faq">Ko'p so'raladigan savollar</a>
</p>

<p align="center">20x gacha tezroq · 38,251 ta yozib olingan holat qayta ijro etildi · 0 ta nomuvofiqlik · bitta buyruq</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n'ning 50,000 kalit bo'yicha qidiruviga user.profile yozilmoqda: TypeScript 143 ms sarflaydi, rust-anything batch'i 6.9 ms, har bir tugma bosilganda bir xil natijalar bilan." width="820">
  <br>
  <sub>x18n'ning 50,000 kalit bo'yicha qidiruviga so'rov yozilmoqda: TypeScript asl nusxasi rust-anything batch'iga qarshi, har bir tugma bosilganda bir xil natijalar.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Yozib olish</b><br>
      <code>record</code> asl nusxani generatsiya qilingan chegaraviy holatlar, semantik tuzoqlar va o'z kirish ma'lumotlaringiz ustida ishga tushiradi hamda u nima qaytarishi yoki qanday xato tashlashini saqlaydi.
    </td>
    <td width="25%" valign="top">
      <b>Port</b><br>
      <code>bridge</code> Rust crate'ini va wrapper'ni generatsiya qiladi, shunda chaqiruvchilar asl API'ni saqlab qoladi. Rust kodini siz yoki agentingiz yozadi.
    </td>
    <td width="25%" valign="top">
      <b>Tekshirish</b><br>
      <code>diff</code> har bir yozib olingan holatni Rust'ga qarshi qayta ijro etadi va ularning hammasi mos kelmaguncha hamda <code>gate</code> o'tmaguncha hech narsa chiqarilmaydi.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Qayta ijro fallback'siz o'tganda va har bir birlik gate'dan o'tganda, <code>app lift</code> uni toza Rust ilovasiga aylantiradi. Asl nusxaga hech qachon tegilmaydi.
    </td>
  </tr>
</table>

**Butun ilovangizni Rust'da, qismma-qism qayta yozing.** rust-anything Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby yoki Java ilovangiz bugun nima qilayotganini yozib oladi,
so'ng Rust'da qayta yozilgan har bir qismni keyingisiga o'tishdan oldin shu yozuv bilan tekshiradi,
toki ilova to'liq Rust bo'lguncha. CLI'lar, HTTP serverlar va kutubxonalar; Rust kodini kodlash
agentingiz (yoki o'zingiz) yozadi. U faqat yozuv qamrab olgan narsani ushlay oladi:
[Nimalarni aniqlay olmaydi](how-it-works.md#what-it-cant-catch) bo'limiga qarang. Faqat sekin qismni
tezlashtirish kerakmi? Faqat uni, bir vaqtda bitta hot path'dan port qiling:

| Haqiqiy ish yuki, asl nusxa bilan bir xil natijalar | Asl nusxa | rust-anything bilan |
| --- | ---: | ---: |
| x18n'ning yon panel qidiruvi, 50,000 kalit (TypeScript, Node) | har so'rovga 10 ms | **0.5 ms: 20x tezroq** |
| Xuddi shu qidiruv brauzerda (wasm, bitta thread) | 10 ms | **0.95 ms: 11.5x** |
| 20,000 string ustida `slugify` (Python) | 98 ms | **17x tezroq** |
| Ruby standart kutubxonasidan `Shellwords.shellescape`, 50,000 qiymat | 88 ms | **11.8x tezroq** |
| 50,000 string ustida `Slugify` (Go) | 49 ms | **4x tezroq** |
| 50,000 string ustida `slugify` (Java) | 43 ms | **3x tezroq** |

4 yadroda o'lchangan; har bir raqamning [`demo/`](../demo) ichida avval natijalar bir xilligini
tekshiradigan benchmark'i bor. Har bir kichik funksiya uchun Rust'ni alohida chaqirish odatda JIT'dan
*sekinroq*; rust-anything ko'chirishga arziydigan siklni topadi va uni ko'chiradi.

> rust-anything mustaqil loyiha bo'lib, Rust Project yoki Rust Foundation bilan bog'liq emas va ular
> tomonidan qo'llab-quvvatlanmaydi. "Rust" Rust Foundation'ning savdo belgisidir.

> [!WARNING]
> `record` va `app record` asl kodingizni ishga tushiradi, har bir funksiyani (yoki butun ilovani)
> ko'plab generatsiya qilingan kirish ma'lumotlari bilan chaqiradi, `app record` esa har bir holatni ikki marta
> ishga tushiradi. `diff`, `gate` va `app replay` xuddi shu holatlarni port yoki asl nusxada yana ishga tushiradi.
> Fayl yozadigan, ma'lumotlar bazasiga murojaat qiladigan yoki tarmoqqa chiqadigan kod buni har bir holat uchun
> bir marta qiladi. Ularni loyihaning nusxasida, test ma'lumotlar bazasi bilan va muhitda production hisob
> ma'lumotlarisiz ishga tushiring.

## Tezkor boshlash

Buni Claude Code, Codex, Cursor yoki Gemini CLI'ga joylashtiring va o'rnatishni agentingizga topshiring:

```text
rust-anything'ni o'rnat: `cargo install rust-anything`, so'ng `rust-anything setup` ni ishga tushir.
Seni qachon qayta ishga tushirishim kerakligini ayt.
```

Yoki o'zingiz:

```sh
cargo install rust-anything   # crates.io'dan
rust-anything setup           # uni Claude Code, Codex, Cursor va Gemini CLI'da ro'yxatdan o'tkazadi
```

Hali Rust toolchain'ingiz yo'qmi? macOS, Linux va Windows uchun tayyor binary fayllar har bir
[GitHub release](https://github.com/erkamyaman/rust-anything/releases)'ga biriktirilgan, JavaScript
loyihalari uchun esa `npm install -g rust-anything` xuddi shu binary'ni o'rnatadi.

Claude Code'da uni plugin sifatida ham o'rnatishingiz mumkin (MCP server va skill; yuqoridagi CLI
`PATH` ichida bo'lishi kerak):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` topgan har bir agentiga MCP serverni qo'shadi va workflow skill'ini agent skill'larni o'qiydigan
joyga o'rnatadi. U boshqa sozlamalaringizni saqlaydi, qayta ishga tushirish xavfsiz, `--remove` esa
qilganlarini bekor qiladi. So'ng agentingizdan, masalan, quyidagilarni so'rang:

- *"Bu CLI'ni rust-anything bilan boshidan oxirigacha Rust'da qayta yoz va u xuddi shunday ishlashini isbotla."*
- *"Bu qidiruv sekin tuyulyapti. rust-anything bilan profilini ol va issiq siklni Rust'ga ko'chir."*
- *"`src/parser/` ni rust-anything bilan Rust'ga ko'chirishni rejalashtir; birinchi birlikni port qil va isbotla."*
- *"`rust/text-rs` dagi Rust portini asl nusxaga qarshi benchmark qil va uni chiqarish kerakmi, ayt."*

### Qo'llab-quvvatlanadigan agentlar

| Agent | `rust-anything setup` qo'shadi |
| --- | --- |
| Claude Code | `~/.claude.json` da MCP server, `~/.claude/skills/` da skill |
| Codex | `~/.codex/config.toml` da MCP server, `~/.codex/skills/` da skill |
| Cursor | `~/.cursor/mcp.json` da MCP server |
| Gemini CLI | `~/.gemini/settings.json` da MCP server |
| Istalgan boshqa MCP mijozi | `rust-anything mcp` ni qo'lda stdio server sifatida qo'shing ([O'rnatish](install.md#install) ga qarang) |

Mavjud faylni o'zgartirishdan oldin `setup` uning yoniga nusxasini saqlaydi (`<file>.rust-anything.bak`).

Yoki qadamlarni o'zingiz bajaring:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile sikl ichida fuzzyScore'ni topadi, bridge --batch Rust crate'ini yaratadi, gate 2000/2000 yozib olingan holat bilan o'tadi va benchmark TS'dan 20.7x tezroq ekanini ko'rsatadi." width="820">
  <br>
  <sub>x18n'ning <code>fuzzyScore</code> funksiyasidagi qadamlar: <code>profile</code> issiq siklni topadi, <code>bridge --batch</code> crate'ni generatsiya qiladi, <code>gate</code> o'tadi va benchmark uni TypeScript bilan solishtiradi.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # nima issiq va uni kim chaqiradi
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs ni yozing (yoki agentingizga yozdiring), so'ng:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # har bir yozib olingan holat mos kelishi kerak
rust-anything bench rust/fuzzy-rs                      # tezroqmi? hukm aytadi
rust-anything gate rust/fuzzy-rs                       # tayyor degani, bu o'tadi degani
```

rust-anything kodni tarjima qilmaydi: Rust'ni kodlash agentingiz (Claude Code, Codex, Cursor, ...)
yoki o'zingiz yozasiz. rust-anything agent ishonchli taxmin qila olmaydigan narsalarni beradi: vaqt
qayerga ketayotganini, avval nimani va qanday tartibda port qilishni, kodning qolgan qismi Rust'ni
API'si o'zgarmagan holda chaqirishi uchun ko'prikni (bridge), port mos kelishi shart bo'lgan aniq
xatti-harakatni, uni chiqarishning xavfsiz yo'lini va port qachon tayyor ekanini aytadigan gate'ni.

## Tillar

| Til | Birlik | Ko'prik | Yozib oluvchi va qayta ijro etuvchi |
| --- | --- | --- | --- |
| TypeScript/JavaScript | papka yoki fayl | generatsiya qilingan binary wire bilan napi-rs yoki wasm-bindgen; klasslar va async ham | `.ts` manbalarini ishga tushiradigan Node |
| Python | paket yoki modul | PyO3 (abi3), to'g'ridan-to'g'ri konvertatsiya, tiplangan exception'lar | asl nusxani import qiladigan Python |
| C | `.c` fayl | asl nomlar bilan `extern "C"` funksiyalar va header | umumiy kutubxona sifatida kompilyatsiya qilingan asl nusxa, ctypes orqali chaqiriladi, har bir holat uchun bitta fork |
| Go | paket | asl API'li paketda tiplangan cgo chaqiruvlari va generatsiya qilingan binary wire | o'z moduli sifatida build qilinadigan generatsiya qilingan Go drayveri |
| Ruby | modul funksiyalaridan iborat fayl yoki papka | asl modullar ostida magnus kengaytmasi | generatsiya qilingan Ruby drayveri |
| Java | `public static` metodlardan iborat paket | asl metodlarga ega wrapper klasslarda tiplangan JNI va generatsiya qilingan binary wire | generatsiya qilingan reflection drayveri |
| PHP (preview) | funksiyalar va `public static` metodlardan iborat papka | navbatda: C ABI orqali PHP FFI | bugun butun ilovalar: CLI'lar va serverlar (`php -S`) uchun `app record`, `app replay` va front door |
| C# (preview) | `public static` metodlardan iborat papka | navbatda: C ABI orqali P/Invoke | bugun butun ilovalar: CLI'lar va ASP.NET serverlari (`dotnet <app>.dll`) uchun `app record`, `app replay` va front door |

C'dan tashqari har bir til batch klasslarini ham oladi (`--batch`): elementlar ko'prikdan bir marta
o'tadi va Rust'da qoladi, har bir chaqiruv esa barcha yadrolarda ularning hammasi ustida ishlaydi.

Til repozitoriydan aniqlanadi (`go.mod`, `pom.xml`, `.csproj` yoki `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, aks holda eng ko'p uchraydigan manba fayllari); `--lang`
buni almashtiradi. Tiplar manbadan olinadi: TS tiplari, Python type hint'lari, C, Go, Java va C#
signaturalari, Ruby YARD teglari, PHP tiplari va phpdoc; son sifatida ishlatilgan tipsiz parametrlar
son deb aniqlanadi.

PHP preview'da: `analyze` PHP ilovasini rejalashtiradi (Composer paketlari va PSR-4 autoloading,
erkin taqqoslash va butun sonning float'ga to'lib o'tishi kabi xavflar, crate'larga moslangan Composer
paketlari), PHP CLI yoki serverini esa yozib olish, qayta ijro etish va Rust front door'i ortiga qo'yish
mumkin. Uning birliklarini PHP FFI orqali Rust'ga port qilish keyingi qadam
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# ham xuddi shunday preview'da: `analyze` `.csproj` loyihalarini va ularning NuGet paketlarini
o'qiydi (crate'larga moslangan), `using` direktivalarini papkalarga bog'laydi va C# boshqacha
qiladigan narsalarni belgilaydi (`decimal`, `dynamic`, LINQ'ning kechiktirilgan so'rovlari, `Task`,
`lock`, reflection). C# CLI yoki ASP.NET serverini yozib olish, qayta ijro etish va Rust front door'i
ortiga qo'yish mumkin; birliklarni P/Invoke orqali port qilish navbatda.

## Batafsil

- [Qanday ishlaydi](how-it-works.md): sikl, `bridge` va `analyze` nimani hal qiladi, hot path'lar va u nimalarni aniqlay olmaydi
- [Butun ilovani port qilish](whole-apps.md): serverlar, CLI'lar va kutubxonalar, front door'dan `app lift` gacha
- [Misollar va amaliy tadqiqotlar](examples.md): x18n va olti tildagi haqiqiy kod
- [Haqiqiy portlar](real-world.md): json-server, Fuse.js, vercel/ms, benchmark'lar bilan
- [O'rnatish, yangilash va ishlab chiqish](install.md): barcha o'rnatish usullari, Windows va manba kodidan build qilish
- [Ko'p so'raladigan savollar](faq.md) · [Holat](STATUS.md) · [MCP vositalari](tools.md)

## Homiylar

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> bepul va ochiq manbali. Agar u ishingizga yordam bersa, iltimos, unga homiylik qilishni o'ylab ko'ring.<br>
  Qo'llab-quvvatlashingiz ishlab chiqishni davom ettirishga yordam beradi.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Kompaniya homiysi: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Homiylar: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Homiy_bo%27lish-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Homiy bo'lish"></a>
</p>

## Hissa qo'shish

Issue'lar va pull request'lar mamnuniyat bilan qabul qilinadi; [CONTRIBUTING.md](../CONTRIBUTING.md)
sozlash, testlar qayerga joylashishi va yangi til qanday qo'shilishini tushuntiradi. Dizayn qarorlari va
ularning sabablari [DECISIONS.md](../DECISIONS.md) da. Haqiqiy portlar bo'yicha amaliy misollar ayniqsa
mamnuniyat bilan qabul qilinadi. Xavfsizlik muammolari haqida iltimos maxfiy xabar bering
([SECURITY.md](../SECURITY.md)) va [xulq-atvor kodeksi](../CODE_OF_CONDUCT.md) bilan tanishing.

## Litsenziya

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Repozitoriyingizdagi portlar va generatsiya qilingan kod sizniki.
