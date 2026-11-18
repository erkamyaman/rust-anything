<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything крабы">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">авторы: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · Татарча · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a></p>

<p align="center"><sub>Бу тәрҗемәне җәмәгатьчелек алып бара, шуңа күрә ул <a href="../README.md">инглизчә README</a> файлыннан калышырга мөмкин.</sub></p>

<p align="center"><em>Бөтен кушымтагызны Rust телендә яңадан языгыз. Һәм ул юл буе эшләп торсын.</em></p>

<p align="center"><em>CLI, серверлар һәм китапханәләр, өлешләп, һәр адымда оригинал белән чагыштырып тикшерелә.</em></p>

<p align="center">Node.js һәм TypeScript · Python · C · Go · Ruby · Java · PHP һәм C# preview рәвешендә</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io версиясе"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm версиясе"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 тел, өстәвенә 2 тел preview рәвешендә">
  <a href="#кайсы-агентлар-белән-эшли"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 агент белән эшли"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Тестлар"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT лицензиясе"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub йолдызлары"></a>
</p>

<p align="center">
  <a href="#тиз-башлау">Тиз башлау</a> ·
  <a href="#телләр">Телләр</a> ·
  <a href="how-it-works.md#how-it-works">Ничек эшли</a> ·
  <a href="how-it-works.md#the-loop">Цикл</a> ·
  <a href="whole-apps.md#port-a-whole-app">Бөтен кушымтаны күчерү</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Кайнар юллар</a> ·
  <a href="install.md#install">Урнаштыру</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Ул нәрсәне тотып кала алмый</a> ·
  <a href="faq.md#faq">Еш бирелә торган сораулар</a>
</p>

<p align="center">20x кадәр тизрәк · язып алынган 38,251 очрак кабат уйнатылды · 0 туры килмәү · бер команда</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n эзләвенә 50,000 ачкыч буенча user.profile язу: TypeScript 143 ms сарыф итә, rust-anything batch 6.9 ms, һәр төймә басылганда нәтиҗәләр бертөрле." width="820">
  <br>
  <sub>x18n эзләвенә 50,000 ачкыч буенча сорау язу: TypeScript оригиналы rust-anything batch'ына каршы, һәр төймә басылганда нәтиҗәләр бертөрле.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Язып алу</b><br>
      <code>record</code> оригиналны ясалган чик очракларда, семантик тозакларда һәм сезнең үз кертмәләрегездә җибәрә һәм ул нәрсә кайтарганын яки нинди хата ыргытканын саклый.
    </td>
    <td width="25%" valign="top">
      <b>Күчерү</b><br>
      <code>bridge</code> Rust crate'ын һәм wrapper ясый, шуңа күрә чакыручылар оригинал API'ны саклый. Rust кодын сез яки агентыгыз яза.
    </td>
    <td width="25%" valign="top">
      <b>Тикшерү</b><br>
      <code>diff</code> язып алынган һәр очракны Rust'та кабат уйната, һәм алар барысы да туры килмичә һәм <code>gate</code> узмыйча, бернәрсә дә чыгарылмый.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Кабат уйнату fallback'сыз узса һәм һәр берәмлек gate'ны узса, <code>app lift</code> аны саф Rust кушымтасына әйләндерә. Оригиналга беркайчан да кагылмый.
    </td>
  </tr>
</table>

**Бөтен кушымтагызны Rust телендә өлешләп яңадан языгыз.** rust-anything сезнең Node.js (TypeScript/JavaScript),
Python, C, Go, Ruby яки Java кушымтагыз бүген нәрсә эшләгәнен язып ала, аннары Rust телендәге
яңа версиянең һәр өлешен, алга барганчы, шул язма белән чагыштырып тикшерә, кушымта саф Rust булганчы. CLI, HTTP
серверлар һәм китапханәләр; Rust кодын сезнең программалау агенты (яки сез үзегез) яза. Ул язма
нәрсәне колачлаганын гына тотып кала ала: [Ул нәрсәне тотып кала алмый](how-it-works.md#what-it-cant-catch) бүлеген карагыз. Бары тик әкрен өлешне генә тизләтергә кирәкме?
Шуны гына күчерегез, бер тапкырда бер кайнар юл (hot path):

| Чын эш йөкләмәсе, нәтиҗәләр оригиналдагы кебек | Оригинал | rust-anything белән |
| --- | ---: | ---: |
| x18n ян панелендәге эзләү, 50,000 ачкыч (TypeScript, Node) | бер сорауга 10 ms | **0.5 ms: 20x тизрәк** |
| Шул ук эзләү браузерда (wasm, бер агым) | 10 ms | **0.95 ms: 11.5x** |
| 20,000 юлга `slugify` (Python) | 98 ms | **17x тизрәк** |
| Ruby стандарт китапханәсеннән `Shellwords.shellescape`, 50,000 кыйммәт | 88 ms | **11.8x тизрәк** |
| 50,000 юлга `Slugify` (Go) | 49 ms | **4x тизрәк** |
| 50,000 юлга `slugify` (Java) | 43 ms | **3x тизрәк** |

4 ядрода үлчәнде; һәр санның [`demo/`](../demo) эчендә бенчмаркы бар, ул башта
нәтиҗәләрнең бертөрле булуын тикшерә. Һәр кечкенә функция өчен Rust'ны аерым чакыру гадәттә JIT'тан *әкренрәк*;
rust-anything күчерергә лаеклы циклны таба һәм аны күчерә.

> rust-anything бәйсез проект, ул Rust Project яки Rust Foundation белән бәйле түгел һәм алар тарафыннан хупланмаган.
> "Rust" Rust Foundation'ның сәүдә билгесе.

> [!WARNING]
> `record` һәм `app record` сезнең оригинал кодны җибәрә: һәр функцияне (яки бөтен кушымтаны) күп санлы
> ясалган кертмәләр белән чакыра, ә `app record` һәр очракны ике тапкыр җибәрә. `diff`, `gate` һәм `app replay`
> шул ук очракларны портта яки оригиналда тагын җибәрә. Файл язучы, мәгълүмат базасы белән эшләүче яки
> челтәргә мөрәҗәгать итүче код моны һәр очрак саен бер тапкыр эшли. Аларны проектның күчермәсендә,
> тест базасы белән һәм мохиттә production сертификатлары булмаганда җибәрегез.

## Тиз башлау

Моны Claude Code, Codex, Cursor яки Gemini CLI эченә куегыз һәм агентыгыз аны урнаштырсын:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Яки үзегез:

```sh
cargo install rust-anything   # crates.io'дан
rust-anything setup           # аны Claude Code, Codex, Cursor һәм Gemini CLI'да теркәп куя
```

Әле Rust toolchain юкмы? macOS, Linux һәм Windows өчен әзер бинар файллар һәр
[GitHub релизына](https://github.com/erkamyaman/rust-anything/releases) беркетелгән, ә JavaScript
проектлары өчен `npm install -g rust-anything` шул ук бинар файлны урнаштыра.

Claude Code'та аны плагин итеп тә урнаштырып була (MCP сервер һәм skill; өстәге CLI
сезнең `PATH` эчендә булырга тиеш):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` тапкан һәр агентка MCP серверны өсти һәм workflow skill'ны агент skill'ларны укыган урынга
урнаштыра. Ул сезнең башка көйләнмәләрегезне саклый, аны кабат җибәрү куркынычсыз, ә `--remove` аны кире кайтара.
Аннары агентыгыздан сорагыз, мәсәлән:

- *"Бу CLI'ны rust-anything ярдәмендә башыннан ахырына кадәр Rust телендә яңадан яз һәм аның шулай ук эшләвен исбатла."*
- *"Бу эзләү әкрен кебек. Аны rust-anything белән профильлә һәм кайнар циклны Rust'ка күчер."*
- *"rust-anything белән `src/parser/` папкасын Rust'ка күчерүне планлаштыр; беренче берәмлекне күчер һәм исбатла."*
- *"`rust/text-rs` эчендәге Rust портын оригинал белән бенчмаркта чагыштыр һәм аны чыгарыргамы икәнен әйт."*

### Кайсы агентлар белән эшли

| Агент | `rust-anything setup` нәрсә өсти |
| --- | --- |
| Claude Code | `~/.claude.json` эчендә MCP сервер, `~/.claude/skills/` эчендә skill |
| Codex | `~/.codex/config.toml` эчендә MCP сервер, `~/.codex/skills/` эчендә skill |
| Cursor | `~/.cursor/mcp.json` эчендә MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` эчендә MCP сервер |
| Башка теләсә нинди MCP клиент | `rust-anything mcp` командасын stdio сервер итеп кулдан өстәгез ([Урнаштыру](install.md#install) бүлеген карагыз) |

Булган файлны үзгәртер алдыннан `setup` аның күчермәсен янәшәдә саклый (`<file>.rust-anything.bak`).

Яки адымнарны үзегез башкарыгыз:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile циклда fuzzyScore'ны таба, bridge --batch Rust crate ясый, gate язып алынган 2000/2000 очрак белән уза, ә бенчмарк TS'тан 20.7x тизрәк икәнен күрсәтә." width="820">
  <br>
  <sub>x18n'ның <code>fuzzyScore</code> функциясендә адымнар: <code>profile</code> кайнар циклны таба, <code>bridge --batch</code> crate ясый, <code>gate</code> уза, ә бенчмарк аны TypeScript белән чагыштыра.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # нәрсә кайнар һәм аны кем чакыра
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs языгыз (яки агентка тапшырыгыз), аннары:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # язып алынган һәр очрак туры килергә тиеш
rust-anything bench rust/fuzzy-rs                      # тизрәкме? нәтиҗә әйтә
rust-anything gate rust/fuzzy-rs                       # бу узса, эш бетте
```

rust-anything кодны тәрҗемә итми: Rust кодын сезнең программалау агенты (Claude Code, Codex, Cursor, ...) яки сез
үзегез язасыз. rust-anything агент ышанычлы итеп чамалый алмаган нәрсәләрне бирә: вакыт кая китә, нәрсәне
беренче һәм нинди тәртиптә күчерергә, калган код Rust'ны API үзгәрмичә чакырсын өчен bridge,
порт туры килергә тиешле төгәл тәртип, аны чыгаруның куркынычсыз ысулы һәм порт кайчан
әзер икәнен әйтүче gate.

## Телләр

| Тел | Берәмлек | Bridge | Кем язып ала һәм кабат уйната |
| --- | --- | --- | --- |
| TypeScript/JavaScript | папка яки файл | ясалган бинар wire форматлы napi-rs яки wasm-bindgen; класслар һәм async та керә | Node, `.ts` чыганакларын җибәреп |
| Python | пакет яки модуль | PyO3 (abi3), туры үзгәртү, типлы исключениеләр | Python, оригиналны импортлап |
| C | `.c` файл | оригинал исемле `extern "C"` функцияләр һәм header | уртак китапханәгә компиляцияләнгән оригинал, ctypes аша чакырыла, һәр очракка бер fork |
| Go | пакет | типлы cgo чакырулары һәм ясалган бинар wire формат, оригинал API'лы пакетта | ясалган Go драйверы, аерым модуль итеп җыела |
| Ruby | модуль функцияләре булган файл яки папка | оригинал модульләр астында magnus киңәйтмәсе | ясалган Ruby драйверы |
| Java | `public static` методлар пакеты | типлы JNI һәм ясалган бинар wire формат, оригинал методлы wrapper класслар эчендә | ясалган reflection драйверы |
| PHP (preview) | функцияләр һәм `public static` методлар папкасы | киләсе адым: C ABI өстендә PHP FFI | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм серверлар өчен (`php -S`) |
| C# (preview) | `public static` методлар папкасы | киләсе адым: C ABI өстендә P/Invoke | бүген үк бөтен кушымталар: `app record`, `app replay` һәм front door, CLI һәм ASP.NET серверлар өчен (`dotnet <app>.dll`) |

C'тан кала һәр тел batch класслар да ала (`--batch`): элементлар бер тапкыр күчә һәм
Rust'та кала, ә һәр чакыру аларның барысын да барлык ядроларда эшкәртә.

Тел репозиторийдан билгеләнә (`go.mod`, `pom.xml`, `.csproj` яки `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, юкса иң күп очрый торган чыганак файллар);
`--lang` аны алыштыра. Типлар чыганак кодтан алына: TS типлары, Python type hints, C, Go, Java һәм C#
сигнатуралары, Ruby YARD теглары, PHP типлары һәм phpdoc; сан итеп кулланылган типсыз параметрлар
сан дип чыгарыла.

PHP preview рәвешендә: `analyze` PHP кушымтасын планлаштыра (Composer пакетлары һәм PSR-4 autoloading, катгый булмаган
чагыштыру һәм бөтен саннарның float'ка ташып чыгуы кебек куркынычлар, Composer пакетларына туры килгән crate'лар),
ә PHP CLI яки серверны язып алып, кабат уйнатып, Rust front door артына куеп була. Аның
берәмлекләрен PHP FFI аша Rust'ка күчерү киләсе адым ([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# та шулай ук preview рәвешендә: `analyze` `.csproj` проектларын һәм аларның NuGet пакетларын укый
(crate'ларга туры китереп), `using` директиваларын папкаларга бәйли һәм C# нәрсәне башкача эшләгәнен билгели
(`decimal`, `dynamic`, LINQ'ның кичектерелгән сораулары, `Task`, `lock`, reflection). C# CLI яки ASP.NET
серверын язып алып, кабат уйнатып, Rust front door артына куеп була; берәмлекләрне P/Invoke аша күчерү
киләсе адым.

## Күбрәк белү

- [Ничек эшли](how-it-works.md): цикл, `bridge` һәм `analyze` нәрсәне хәл итә, кайнар юллар һәм ул нәрсәне тотып кала алмый
- [Бөтен кушымтаны күчерү](whole-apps.md): серверлар, CLI һәм китапханәләр, front door'дан `app lift`'ка кадәр
- [Мисаллар һәм тикшеренүләр](examples.md): x18n һәм алты телдәге чын код
- [Чын портлар](real-world.md): json-server, Fuse.js, vercel/ms, бенчмарклар белән
- [Урнаштыру, яңарту һәм эшләнмә](install.md): урнаштыруның һәр ысулы, Windows һәм чыганак коддан җыю
- [Еш бирелә торган сораулар](faq.md) · [Халәт](STATUS.md) · [MCP кораллары](tools.md)

## Спонсорлар

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> бушлай һәм ачык чыганаклы. Әгәр ул эшегездә ярдәм итсә, зинһар, аңа спонсорлык итүне уйлап карагыз.<br>
  Сезнең ярдәм эшләнмәне дәвам итәргә мөмкинлек бирә.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Компания спонсоры: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Спонсорлар: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%D0%A1%D0%BF%D0%BE%D0%BD%D1%81%D0%BE%D1%80_%D0%B1%D1%83%D0%BB%D1%83-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Спонсор булу"></a>
</p>

## Катнашу

Issue'лар һәм pull request'лар хуплана; [CONTRIBUTING.md](../CONTRIBUTING.md) көйләүне, тестларны
кая куярга һәм яңа телне ничек өстәргә икәнен аңлата. Проект карарлары һәм аларның сәбәпләре
[DECISIONS.md](../DECISIONS.md) эчендә. Чын портлар турында мисаллар аеруча хуплана. Зинһар, куркынычсызлык
проблемалары турында шәхси рәвештә хәбәр итегез ([SECURITY.md](../SECURITY.md)) һәм
[тәртип кодексы](../CODE_OF_CONDUCT.md) белән танышыгыз.

## Лицензия

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Репозиторийыгыздагы портлар һәм ясалган код сезнеке.
