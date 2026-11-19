<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything xərçəngi">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">Müəllif: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · Azərbaycanca · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> ? <a href="README_VI.md">Vietnamese</a></p>
<p align="center"><sub>Bu tərcümə icma tərəfindən dəstəklənir və <a href="../README.md">ingiliscə README</a>-dən geri qala bilər.</sub></p>

<p align="center"><em>Bütün tətbiqinizi Rust-da yenidən yazın. Bütün yol boyu işlək qalsın.</em></p>

<p align="center"><em>CLI-lar, serverlər və kitabxanalar, hissə-hissə, hər addımda orijinalla yoxlanılaraq.</em></p>

<p align="center">Node.js və TypeScript · Python · C · Go · Ruby · Java · PHP və C# önizləmədə</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io versiyası"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm versiyası"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 dil, üstəgəl 2 önizləmədə">
  <a href="#dəstəklənən-agentlər"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 agentlə işləyir"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Testlər"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT lisenziyası"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub ulduzları"></a>
</p>

<p align="center">
  <a href="#sürətli-başlanğıc">Sürətli başlanğıc</a> ·
  <a href="#dillər">Dillər</a> ·
  <a href="how-it-works.md#how-it-works">Necə işləyir</a> ·
  <a href="how-it-works.md#the-loop">Dövr</a> ·
  <a href="whole-apps.md#port-a-whole-app">Bütöv tətbiqi port etmək</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path-lar</a> ·
  <a href="install.md#install">Quraşdırma</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Nəyi tuta bilmir</a> ·
  <a href="faq.md#faq">Tez-tez verilən suallar</a>
</p>

<p align="center">20x-ə qədər daha sürətli · 38,251 qeydə alınmış hal təkrar oynadılıb · 0 uyğunsuzluq · bir əmr</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n-in 50,000 açar üzrə axtarışına user.profile yazılır: TypeScript 143 ms sərf edir, rust-anything batch-i 6.9 ms, hər düymə basışında eyni nəticələrlə." width="820">
  <br>
  <sub>x18n-in 50,000 açar üzrə axtarışına sorğu yazılır: TypeScript orijinalı rust-anything batch-inə qarşı, hər düymə basışında eyni nəticələr.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Qeyd</b><br>
      <code>record</code> orijinalı yaradılmış kənar hallar, semantik tələlər və sizin öz girişlərinizlə işə salır və nə qaytardığını və ya hansı xətanı atdığını saxlayır.
    </td>
    <td width="25%" valign="top">
      <b>Port</b><br>
      <code>bridge</code> Rust crate-ini və wrapper yaradır ki, çağıranlar orijinal API-ni saxlasın. Rust-u siz və ya agentiniz yazır.
    </td>
    <td width="25%" valign="top">
      <b>Yoxlama</b><br>
      <code>diff</code> hər qeydə alınmış halı Rust-a qarşı təkrar oynadır və hamısı uyğun gəlib <code>gate</code> keçənə qədər heç nə buraxılmır.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Təkrar oynatma fallback olmadan keçəndə və hər vahid gate-dən keçəndə, <code>app lift</code> onu təmiz Rust tətbiqinə çevirir. Orijinala heç vaxt toxunulmur.
    </td>
  </tr>
</table>

**Bütün tətbiqinizi Rust-da, hissə-hissə yenidən yazın.** rust-anything Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby və ya Java tətbiqinizin bu gün nə etdiyini qeydə alır,
sonra Rust-da yenidən yazılmış hər hissəni növbətiyə keçməzdən əvvəl bu qeydlə yoxlayır, tətbiq
tamamilə Rust olana qədər. CLI-lar, HTTP serverləri və kitabxanalar; Rust kodunu kodlaşdırma
agentiniz (və ya siz) yazır. O, yalnız qeydin əhatə etdiyini tuta bilər: bax
[Nəyi tuta bilmir](how-it-works.md#what-it-cant-catch). Yalnız yavaş hissənin sürətlənməsi lazımdır? Yalnız onu,
hər dəfə bir hot path olmaqla port edin:

| Real iş yükü, orijinalla eyni nəticələr | Orijinal | rust-anything ilə |
| --- | ---: | ---: |
| x18n-in yan panel axtarışı, 50,000 açar (TypeScript, Node) | sorğu başına 10 ms | **0.5 ms: 20x daha sürətli** |
| Eyni axtarış brauzerdə (wasm, bir thread) | 10 ms | **0.95 ms: 11.5x** |
| 20,000 string üzərində `slugify` (Python) | 98 ms | **17x daha sürətli** |
| Ruby-nin standart kitabxanasından `Shellwords.shellescape`, 50,000 dəyər | 88 ms | **11.8x daha sürətli** |
| 50,000 string üzərində `Slugify` (Go) | 49 ms | **4x daha sürətli** |
| 50,000 string üzərində `slugify` (Java) | 43 ms | **3x daha sürətli** |

4 nüvədə ölçülüb; hər rəqəmin [`demo/`](../demo) qovluğunda əvvəlcə nəticələrin eyni olduğunu
yoxlayan bir benchmark-ı var. Hər kiçik funksiya üçün Rust-u ayrıca çağırmaq adətən JIT-dən *daha
yavaşdır*; rust-anything köçürməyə dəyən dövrü tapır və onu köçürür.

> rust-anything müstəqil layihədir, Rust Project və ya Rust Foundation ilə əlaqəli deyil və onlar
> tərəfindən dəstəklənmir. "Rust" Rust Foundation-ın əmtəə nişanıdır.

> [!WARNING]
> `record` və `app record` orijinal kodunuzu işə salır, hər funksiyanı (və ya bütöv tətbiqi) çoxlu
> yaradılmış girişlə çağırır, `app record` isə hər halı iki dəfə işlədir. `diff`, `gate` və `app replay`
> eyni halları port və ya orijinal üzərində yenidən işə salır. Fayl yazan, verilənlər bazası ilə
> danışan və ya şəbəkəyə müraciət edən kod bunu hər hal üçün bir dəfə edir. Onları layihənin
> surətində, test verilənlər bazası ilə və mühitdə production etimadnamələri olmadan işə salın.

## Sürətli başlanğıc

Bunu Claude Code, Codex, Cursor və ya Gemini CLI-a yapışdırın və quraşdırmanı agentinizə həvalə edin:

```text
rust-anything-i quraşdır: `cargo install rust-anything`, sonra `rust-anything setup` işə sal.
Səni nə vaxt yenidən başlatmalı olduğumu de.
```

Və ya özünüz:

```sh
cargo install rust-anything   # crates.io-dan
rust-anything setup           # Claude Code, Codex, Cursor və Gemini CLI-da qeydiyyatdan keçirir
```

Hələ Rust toolchain-iniz yoxdur? macOS, Linux və Windows üçün hazır binary fayllar hər
[GitHub release](https://github.com/erkamyaman/rust-anything/releases)-ə əlavə olunub, JavaScript
layihələri üçün isə `npm install -g rust-anything` eyni binary-ni quraşdırır.

Claude Code-da onu plugin kimi də quraşdıra bilərsiniz (MCP server və skill; yuxarıdakı CLI
`PATH`-da olmalıdır):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` tapdığı hər agentə MCP serveri əlavə edir və iş axını skill-ini agentin skill-ləri oxuduğu
yerə quraşdırır. Digər ayarlarınızı saxlayır, yenidən işə salmaq təhlükəsizdir, `--remove` isə
etdiklərini geri qaytarır. Sonra agentinizdən, məsələn, bunları istəyin:

- *"Bu CLI-ı rust-anything ilə başdan-başa Rust-da yenidən yaz və eyni davrandığını sübut et."*
- *"Bu axtarış yavaş görünür. rust-anything ilə profilini çıxar və isti dövrü Rust-a köçür."*
- *"`src/parser/`-in rust-anything ilə Rust-a miqrasiyasını planlaşdır; ilk vahidi port et və sübut et."*
- *"`rust/text-rs`-dəki Rust portunu orijinala qarşı benchmark et və onu buraxmağa dəyib-dəymədiyini de."*

### Dəstəklənən agentlər

| Agent | `rust-anything setup` əlavə edir |
| --- | --- |
| Claude Code | `~/.claude.json`-da MCP server, `~/.claude/skills/`-də skill |
| Codex | `~/.codex/config.toml`-da MCP server, `~/.codex/skills/`-də skill |
| Cursor | `~/.cursor/mcp.json`-da MCP server |
| Gemini CLI | `~/.gemini/settings.json`-da MCP server |
| İstənilən digər MCP müştərisi | `rust-anything mcp`-ni əllə stdio server kimi əlavə edin (bax [Quraşdırma](install.md#install)) |

Mövcud faylı dəyişməzdən əvvəl `setup` onun yanında bir surətini saxlayır (`<file>.rust-anything.bak`).

Və ya addımları özünüz icra edin:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile dövr daxilində fuzzyScore-u tapır, bridge --batch Rust crate-ini yaradır, gate 2000/2000 qeydə alınmış halla keçir və benchmark TS-dən 20.7x daha sürətli olduğunu göstərir." width="820">
  <br>
  <sub>x18n-in <code>fuzzyScore</code> funksiyası üzərində addımlar: <code>profile</code> isti dövrü tapır, <code>bridge --batch</code> crate-i yaradır, <code>gate</code> keçir və benchmark onu TypeScript ilə müqayisə edir.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # isti olan nədir və onu kim çağırır
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs-i yazın (və ya agentinizə yazdırın), sonra:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # hər qeydə alınmış hal uyğun gəlməlidir
rust-anything bench rust/fuzzy-rs                      # daha sürətlidir? hökm deyəcək
rust-anything gate rust/fuzzy-rs                       # hazırdır, yəni bu keçir
```

rust-anything kodu tərcümə etmir: Rust-u kodlaşdırma agentiniz (Claude Code, Codex, Cursor, ...) və ya
siz yazırsınız. rust-anything agentin etibarlı şəkildə təxmin edə bilmədiyini təmin edir: vaxtın hara
getdiyini, əvvəlcə nəyin və hansı ardıcıllıqla port ediləcəyini, kodun qalan hissəsinin Rust-u API-si
dəyişmədən çağıra bilməsi üçün körpünü (bridge), portun uyğun gəlməli olduğu dəqiq davranışı, onu
buraxmağın təhlükəsiz yolunu və portun nə vaxt hazır olduğunu deyən gate-i.

## Dillər

| Dil | Vahid | Körpü | Qeydə alan və təkrar oynadan |
| --- | --- | --- | --- |
| TypeScript/JavaScript | qovluq və ya fayl | yaradılmış binary wire ilə napi-rs və ya wasm-bindgen; siniflər və async daxil | `.ts` mənbələrini işlədən Node |
| Python | paket və ya modul | PyO3 (abi3), birbaşa çevirmə, tipli exception-lar | orijinalı import edən Python |
| C | `.c` faylı | orijinal adlarla `extern "C"` funksiyaları və header | paylaşılan kitabxana kimi kompilyasiya olunmuş orijinal, ctypes ilə çağırılır, hər hal üçün bir fork |
| Go | paket | orijinal API-li paketdə tipli cgo çağırışları və yaradılmış binary wire | öz modulu kimi qurulan yaradılmış Go drayveri |
| Ruby | modul funksiyalarından ibarət fayl və ya qovluq | orijinal modulların altında magnus genişlənməsi | yaradılmış Ruby drayveri |
| Java | `public static` metodlardan ibarət paket | orijinal metodlara malik wrapper siniflərdə tipli JNI və yaradılmış binary wire | yaradılmış reflection drayveri |
| PHP (önizləmə) | funksiyalardan və `public static` metodlardan ibarət qovluq | növbədə: C ABI üzərindən PHP FFI | bu gün bütöv tətbiqlər: CLI-lar və serverlər (`php -S`) üçün `app record`, `app replay` və front door |
| C# (önizləmə) | `public static` metodlardan ibarət qovluq | növbədə: C ABI üzərindən P/Invoke | bu gün bütöv tətbiqlər: CLI-lar və ASP.NET serverləri (`dotnet <app>.dll`) üçün `app record`, `app replay` və front door |

C-dən başqa hər dil batch sinifləri də alır (`--batch`): elementlər körpünü bir dəfə keçir və
Rust-da qalır, hər çağırış isə bütün nüvələrdə hamısının üzərində işləyir.

Dil repozitoriyadan müəyyən edilir (`go.mod`, `pom.xml`, `.csproj` və ya `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, əks halda ən çox rast gəlinən mənbə faylları); `--lang`
bunu əvəz edir. Tiplər mənbədən gəlir: TS tipləri, Python type hint-ləri, C, Go, Java və C#
imzaları, Ruby YARD teqləri, PHP tipləri və phpdoc; rəqəm kimi istifadə olunan tipsiz parametrlər
rəqəm kimi müəyyən edilir.

PHP önizləmədədir: `analyze` PHP tətbiqini planlaşdırır (Composer paketləri və PSR-4 autoloading,
sərbəst müqayisə və tam ədədin float-a daşması kimi risklər, crate-lərə uyğunlaşdırılmış Composer
paketləri), PHP CLI və ya serveri isə qeydə alına, təkrar oynadıla və Rust front door-unun arxasına
qoyula bilər. Onun vahidlərini PHP-nin FFI-ı vasitəsilə Rust-a port etmək növbəti addımdır
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# da eyni şəkildə önizləmədədir: `analyze` `.csproj` layihələrini və onların NuGet paketlərini
oxuyur (crate-lərə uyğunlaşdırılmış), `using` direktivlərini qovluqlara həll edir və C#-ın fərqli
etdiyi şeyləri işarələyir (`decimal`, `dynamic`, LINQ-in təxirə salınmış sorğuları, `Task`, `lock`,
reflection). C# CLI və ya ASP.NET serveri qeydə alına, təkrar oynadıla və Rust front door-unun
arxasına qoyula bilər; vahidləri P/Invoke vasitəsilə port etmək növbədədir.

## Ətraflı

- [Necə işləyir](how-it-works.md): dövr, `bridge` və `analyze` nəyə qərar verir, hot path-lar və nəyi tuta bilmir
- [Bütöv tətbiqi port etmək](whole-apps.md): serverlər, CLI-lar və kitabxanalar, front door-dan `app lift`-ə qədər
- [Nümunələr və nümunə araşdırmaları](examples.md): x18n və altı dildə real kod
- [Real portlar](real-world.md): json-server, Fuse.js, vercel/ms, benchmark-larla
- [Quraşdırma, yeniləmə və inkişaf](install.md): bütün quraşdırma üsulları, Windows və mənbədən qurma
- [Tez-tez verilən suallar](faq.md) · [Vəziyyət](STATUS.md) · [MCP alətləri](tools.md)

## Sponsorlar

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> pulsuz və açıq mənbəlidir. Əgər işinizə yarayırsa, lütfən ona sponsorluq etməyi düşünün.<br>
  Dəstəyiniz inkişafın davam etməsinə kömək edir.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Şirkət sponsoru: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsorlar: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Sponsor_olun-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Sponsor olun"></a>
</p>

## Töhfə vermək

Issue-lar və pull request-lər xoş qarşılanır; [CONTRIBUTING.md](../CONTRIBUTING.md) quraşdırmanı,
testlərin harada yerləşdiyini və yeni dilin necə əlavə edildiyini izah edir. Dizayn qərarları və
onların səbəbləri [DECISIONS.md](../DECISIONS.md)-dədir. Real portların nümunə araşdırmaları xüsusilə
xoş qarşılanır. Təhlükəsizlik problemlərini lütfən gizli bildirin ([SECURITY.md](../SECURITY.md)) və
[davranış kodeksinə](../CODE_OF_CONDUCT.md) baxın.

## Lisenziya

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Repozitoriyanızdakı portlar və yaradılmış kod sizindir.
