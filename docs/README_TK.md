<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">awtory: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · Türkmençe · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a></p>

<p align="center"><sub>Bu terjimäni jemgyýetçilik goldaýar, şonuň üçin ol <a href="../README.md">iňlisçe README</a> faýlyndan yzda galyp biler.</sub></p>

<p align="center"><em>Tutuş programmaňyzy Rust-da täzeden ýazyň. Ýoluň dowamynda onuň işleýşini saklaň.</em></p>

<p align="center"><em>CLI-ler, serwerler we kitaphanalar, bölek-bölekden, her ädimde asyl nusga bilen deňeşdirilip barlanýar.</em></p>

<p align="center">Node.js we TypeScript · Python · C · Go · Ruby · Java · PHP we C# preview ýagdaýynda</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io wersiýasy"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm wersiýasy"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 dil, üstesine 2 dil preview ýagdaýynda">
  <a href="#haýsy-agentler-bilen-işleýär"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 agent bilen işleýär"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Testler"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT ygtyýarnamasy"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub ýyldyzlary"></a>
</p>

<p align="center">
  <a href="#çalt-başlangyç">Çalt başlangyç</a> ·
  <a href="#diller">Diller</a> ·
  <a href="how-it-works.md#how-it-works">Nähili işleýär</a> ·
  <a href="how-it-works.md#the-loop">Aýlaw</a> ·
  <a href="whole-apps.md#port-a-whole-app">Tutuş programmany geçirmek</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot paths</a> ·
  <a href="install.md#install">Gurnamak</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Onuň tutup bilmeýän zatlary</a> ·
  <a href="faq.md#faq">Köp soralýan soraglar</a>
</p>

<p align="center">20x çenli has çalt · ýazga alnan 38,251 ýagdaý gaýtadan işledildi · 0 gabat gelmezlik · bir buýruk</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n gözleginde 50,000 açar boýunça user.profile ýazylýar: TypeScript 143 ms sarp edýär, rust-anything batch 6.9 ms, her düwme basylanda netijeler birmeňzeş." width="820">
  <br>
  <sub>x18n-iň 50,000 açar boýunça gözleginde sorag ýazylýar: TypeScript asyl nusgasy rust-anything batch-a garşy, her düwme basylanda netijeler birmeňzeş.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Ýazga almak</b><br>
      <code>record</code> asyl nusgany döredilen çäk ýagdaýlarynda, semantik duzaklarda we siziň öz girişleriňizde işledýär we onuň näme gaýtarýanyny ýa-da näme taşlaýanyny saklaýar.
    </td>
    <td width="25%" valign="top">
      <b>Geçirmek</b><br>
      <code>bridge</code> Rust crate-ini we wrapper döredýär, şonuň üçin çagyryjylar asyl API-ni saklaýar. Rust koduny siz ýa-da agentiňiz ýazýar.
    </td>
    <td width="25%" valign="top">
      <b>Barlamak</b><br>
      <code>diff</code> ýazga alnan her ýagdaýy Rust-a garşy gaýtadan işledýär we olaryň hemmesi gabat gelýänçä we <code>gate</code> geçýänçä hiç zat çykarylmaýar.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Replay hiç hili fallback bolmazdan geçende we her birlik gate-den geçende, <code>app lift</code> ony arassa Rust programmasyna öwürýär. Asyl nusga hiç haçan üýtgedilmeýär.
    </td>
  </tr>
</table>

**Tutuş programmaňyzy Rust-da bölek-bölekden täzeden ýazyň.** rust-anything siziň Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby ýa-da Java programmaňyzyň häzir näme edýändigini ýazga
alýar, soňra Rust nusgasynyň her bölegini indiki bölege geçmezden öň şol ýazgy bilen deňeşdirip
barlaýar, programma doly arassa Rust bolýança. CLI-ler, HTTP serwerler we kitaphanalar; Rust koduny
siziň kodlaýyş agentiňiz (ýa-da özüňiz) ýazýar. Ol diňe ýazgynyň öz içine alýan zadyny tutup bilýär:
[Onuň tutup bilmeýän zatlary](how-it-works.md#what-it-cant-catch) bölümine serediň. Diňe haýal bölegi
çaltlandyrmak gerekmi? Diňe şony geçiriň, bir gezekde bir hot path:

| Hakyky iş ýüki, netijeleri asyl nusga bilen birmeňzeş | Asyl nusga | rust-anything bilen |
| --- | ---: | ---: |
| x18n gapdal panelindäki gözleg, 50,000 açar (TypeScript, Node) | bir soraga 10 ms | **0.5 ms: 20x has çalt** |
| Şol gözleg brauzerde (wasm, bir akym) | 10 ms | **0.95 ms: 11.5x** |
| `slugify`, 20,000 setir (Python) | 98 ms | **17x has çalt** |
| Ruby-nyň standart kitaphanasyndaky `Shellwords.shellescape`, 50,000 baha | 88 ms | **11.8x has çalt** |
| `Slugify`, 50,000 setir (Go) | 49 ms | **4x has çalt** |
| `slugify`, 50,000 setir (Java) | 43 ms | **3x has çalt** |

4 ýadroda ölçüldi; her san üçin [`demo/`](../demo) içinde ilki netijeleriň birmeňzeşdigini barlaýan
benchmark bar. Her kiçijik funksiýa üçin Rust-y aýratyn çagyrmak adatça JIT-den *haýalrak*;
rust-anything göçürmäge degýän aýlawy tapýar we şony göçürýär.

> rust-anything garaşsyz taslama, ol Rust Project ýa-da Rust Foundation bilen baglanyşykly däl we
> olar tarapyndan goldanmaýar. "Rust" Rust Foundation-yň söwda belgisidir.

> [!WARNING]
> `record` we `app record` siziň asyl kodyňyzy işledýär, her funksiýany (ýa-da tutuş programmany) köp
> sanly döredilen girişde çagyrýar, `app record` bolsa her ýagdaýy iki gezek işledýär. `diff`, `gate` we
> `app replay` şol ýagdaýlary portda ýa-da asyl nusgada ýene işledýär. Faýl ýazýan, maglumat binasy bilen
> işleýän ýa-da tora ýüz tutýan kod muny her ýagdaý üçin bir gezek edýär. Olary taslamanyň nusgasynda,
> synag maglumat binasy bilen we gurşawda production maglumatlary bolmazdan işlediň.

## Çalt başlangyç

Muny Claude Code, Codex, Cursor ýa-da Gemini CLI içine goýuň we gurnamagy agentiňize tabşyryň:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Ýa-da özüňiz:

```sh
cargo install rust-anything   # from crates.io
rust-anything setup           # registers it with Claude Code, Codex, Cursor and Gemini CLI
```

Entek Rust toolchain ýokmy? macOS, Linux we Windows üçin taýýar binar faýllar her
[GitHub release](https://github.com/erkamyaman/rust-anything/releases) sahypasyna goşulan, JavaScript
taslamalary üçin bolsa `npm install -g rust-anything` şol bir binar faýly gurnaýar.

Claude Code-da ony plugin hökmünde hem gurnap bilersiňiz (MCP serwer we skill; ýokardaky CLI siziň
`PATH` içiňizde bolmaly):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` tapan her agentine MCP serweri goşýar we workflow skill-ini agentiň skill-leri okaýan ýerine
gurnaýar. Ol beýleki sazlamalaryňyzy saklaýar, ony gaýtadan işletmek howpsuz, `--remove` bolsa ony yzyna
gaýtarýar. Soňra agentiňizden, meselem, şeýle soraň:

- *"Bu CLI-ni rust-anything bilen başdan-aýak Rust-da täzeden ýaz we onuň edil şonuň ýaly işleýändigini subut et."*
- *"Bu gözleg haýal ýaly. Ony rust-anything bilen profille we hot loop-y Rust-a göçür."*
- *"rust-anything bilen `src/parser/` bukjasyny Rust-a geçirmegiň meýilnamasyny düz; birinji birligi geçir we subut et."*
- *"`rust/text-rs` içindäki Rust portuny asyl nusga bilen benchmark arkaly deňeşdir we ony çykarmalymy, aýt."*

### Haýsy agentler bilen işleýär

| Agent | `rust-anything setup` näme goşýar |
| --- | --- |
| Claude Code | `~/.claude.json` içinde MCP serwer, `~/.claude/skills/` içinde skill |
| Codex | `~/.codex/config.toml` içinde MCP serwer, `~/.codex/skills/` içinde skill |
| Cursor | `~/.cursor/mcp.json` içinde MCP serwer |
| Gemini CLI | `~/.gemini/settings.json` içinde MCP serwer |
| Islendik başga MCP müşderi | `rust-anything mcp` buýrugyny stdio serwer hökmünde el bilen goşuň ([Gurnamak](install.md#install) bölümine serediň) |

Bar bolan faýly üýtgetmezden öň `setup` onuň nusgasyny ýanynda saklaýar (`<file>.rust-anything.bak`).

Ýa-da ädimleri özüňiz ýerine ýetiriň:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile aýlawdaky fuzzyScore funksiýasyny tapýar, bridge --batch Rust crate-ini döredýär, gate ýazga alnan 2000/2000 ýagdaý bilen geçýär, benchmark bolsa TS-den 20.7x has çaltdygyny görkezýär." width="820">
  <br>
  <sub>x18n-iň <code>fuzzyScore</code> funksiýasyndaky ädimler: <code>profile</code> hot loop-y tapýar, <code>bridge --batch</code> crate-i döredýär, <code>gate</code> geçýär, benchmark bolsa ony TypeScript bilen deňeşdirýär.</sub>
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

rust-anything kody terjime etmeýär: Rust koduny siziň kodlaýyş agentiňiz (Claude Code, Codex, Cursor,
...) ýa-da özüňiz ýazýarsyňyz. rust-anything agentiň ygtybarly çaklap bilmeýän zatlaryny berýär: wagt
nirä gidýär, ilki näme we haýsy tertipde geçirmeli, galan kodyň Rust-y API-sini üýtgetmezden çagyrmagy
üçin bridge, portuň gabat gelmeli takyk özüni alyp barşy, ony çykarmagyň howpsuz ýoly we portuň haçan
taýýardygyny aýdýan gate.

## Diller

| Dil | Birlik | Bridge | Kim ýazga alýar we gaýtadan işledýär |
| --- | --- | --- | --- |
| TypeScript/JavaScript | bukja ýa-da faýl | döredilen binar wire bilen napi-rs ýa-da wasm-bindgen; klaslar we async hem bar | Node, `.ts` çeşme faýllaryny göni işledýär |
| Python | paket ýa-da modul | PyO3 (abi3), göni öwrülişik, tipli exception-lar | Python, asyl nusgany import edýär |
| C | `.c` faýly | asyl atlary we header bilen `extern "C"` funksiýalary | shared library hökmünde kompilirlenen asyl nusga, ctypes arkaly çagyrylýar, her ýagdaý üçin bir fork |
| Go | paket | tipli cgo çagyryşlary we döredilen binar wire, asyl API-li paketde | döredilen Go draýweri, aýratyn modul hökmünde gurulýar |
| Ruby | modul funksiýalaryndan ybarat faýl ýa-da bukja | asyl modullaryň aşagyndaky magnus giňeltmesi | döredilen Ruby draýweri |
| Java | `public static` usullaryndan ybarat paket | tipli JNI we döredilen binar wire, asyl usullary bolan wrapper klaslarynda | döredilen reflection draýweri |
| PHP (preview) | funksiýalardan we `public static` usullaryndan ybarat bukja | indiki nobatda: C ABI arkaly PHP FFI | häzir tutuş programmalar: `app record`, `app replay` we front door, CLI-ler we serwerler üçin (`php -S`) |
| C# (preview) | `public static` usullaryndan ybarat bukja | indiki nobatda: C ABI arkaly P/Invoke | häzir tutuş programmalar: `app record`, `app replay` we front door, CLI-ler we ASP.NET serwerleri üçin (`dotnet <app>.dll`) |

C-den başga her dil batch klaslaryny hem alýar (`--batch`): elementler serhetden bir gezek geçýär we
Rust-da galýar, her çagyryş bolsa olaryň hemmesini ähli ýadrolarda işleýär.

Dil repozitoriýden kesgitlenýär (`go.mod`, `pom.xml`, `.csproj` ýa-da `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, bolmasa iň köp duş gelýän çeşme faýllary boýunça); `--lang`
ony el bilen görkezýär. Tipler çeşme kodundan alynýar: TS tipleri, Python hint-leri, C, Go, Java we C#
signaturalary, Ruby YARD tegleri, PHP tipleri we phpdoc; san hökmünde ulanylýan tipsiz parametrler san
diýlip kabul edilýär.

PHP preview ýagdaýynda: `analyze` PHP programmasyny meýilleşdirýär (Composer paketleri we PSR-4
autoloading, loose comparison we integer-iň float-a doly geçmegi ýaly töwekgelçilikler, crate-lere
laýyklaşdyrylan Composer paketleri), PHP CLI-ni ýa-da serwerini bolsa ýazga alyp, gaýtadan işledip we
Rust front door-yň arkasyna goýup bolýar. Onuň birliklerini PHP FFI arkaly Rust-a geçirmek indiki ädim
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# hem edil şeýle preview ýagdaýynda: `analyze` `.csproj` taslamalaryny we olaryň NuGet paketlerini
okaýar (crate-lere laýyklaşdyrylýar), `using` direktiwalaryny bukjalara çözýär we C#-yň başgaça
edýän zatlaryny belleýär (`decimal`, `dynamic`, LINQ-iň yza süýşürilen soraglary, `Task`, `lock`,
reflection). C# CLI-ni ýa-da ASP.NET serwerini ýazga alyp, gaýtadan işledip we Rust front door-yň
arkasyna goýup bolýar; birlikleri P/Invoke arkaly geçirmek indiki nobatda.

## Has giňişleýin

- [Nähili işleýär](how-it-works.md): aýlaw, `bridge` we `analyze` näme karar berýär, hot paths we onuň tutup bilmeýän zatlary
- [Tutuş programmany geçirmek](whole-apps.md): serwerler, CLI-ler we kitaphanalar, front door-dan `app lift` çenli
- [Mysallar we tejribe mysallary](examples.md): x18n we alty dilde hakyky kod
- [Hakyky portlar](real-world.md): json-server, Fuse.js, vercel/ms, benchmark-lar bilen
- [Gurnamak, täzelemek we işläp düzmek](install.md): gurnamagyň ähli usullary, Windows we çeşme kodundan gurmak
- [Köp soralýan soraglar](faq.md) · [Ýagdaý](STATUS.md) · [MCP gurallary](tools.md)

## Hemaýatkärler

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> mugt we açyk çeşmeli taslama. Eger ol işiňize kömek edýän bolsa, oňa hemaýatkär bolmagy göz öňünde tutuň.<br>
  Siziň goldawyňyz işläp düzmegiň dowam etmegine kömek edýär.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Kompaniýa hemaýatkäri: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Hemaýatkärler: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Hema%C3%BDatk%C3%A4r_bolu%C5%88-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Hemaýatkär boluň"></a>
</p>

## Goşant goşmak

Issue-lar we pull request-ler hoş görülýär; [CONTRIBUTING.md](../CONTRIBUTING.md) gurşawy sazlamagy,
testleriň nirede ýerleşýändigini we täze dili nähili goşmalydygyny düşündirýär. Dizaýn kararlary we
olaryň sebäpleri [DECISIONS.md](../DECISIONS.md) içinde. Hakyky portlaryň tejribe mysallary aýratyn
hoş görülýär. Howpsuzlyk meseleleri barada gizlin habar beriň ([SECURITY.md](../SECURITY.md)), şeýle
hem [özüňi alyp barş kodeksine](../CODE_OF_CONDUCT.md) serediň.

## Ygtyýarnama

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Repozitoriýiňizdäki portlar we döredilen kod siziňki.
