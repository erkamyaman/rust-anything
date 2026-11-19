<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything краб">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">автору: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · Кыргызча · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> ? <a href="README_VI.md">Vietnamese</a></p>

<p align="center"><sub>Бул котормону коомчулук колдойт, ошондуктан ал <a href="../README.md">англисче README</a> нускасынан артта калышы мүмкүн.</sub></p>

<p align="center"><em>Бүт колдонмоңузду Rust тилинде кайра жазыңыз. Бүт жол бою анын иштеп турушун сактаңыз.</em></p>

<p align="center"><em>CLI'лар, серверлер жана китепканалар, бөлүк-бөлүгү менен, ар бир кадамда түп нуска менен салыштырылып текшерилет.</em></p>

<p align="center">Node.js жана TypeScript · Python · C · Go · Ruby · Java · PHP жана C# preview абалында</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io версиясы"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm версиясы"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 тил, дагы 2 preview абалында">
  <a href="#шайкеш-агенттер"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 агент менен иштейт"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Тесттер"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT лицензиясы"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub жылдыздары"></a>
</p>

<p align="center">
  <a href="#тез-баштоо">Тез баштоо</a> ·
  <a href="#тилдер">Тилдер</a> ·
  <a href="how-it-works.md#how-it-works">Кантип иштейт</a> ·
  <a href="how-it-works.md#the-loop">Цикл</a> ·
  <a href="whole-apps.md#port-a-whole-app">Бүт колдонмону порттоо</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path'тар</a> ·
  <a href="install.md#install">Орнотуу</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Ал эмнени кармай албайт</a> ·
  <a href="faq.md#faq">Көп берилүүчү суроолор</a>
</p>

<p align="center">20x чейин ылдамыраак · 38,251 жазылган кейс кайра ойнотулду · 0 дал келбестик · бир буйрук</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n издөөсүнө 50,000 ачкыч боюнча user.profile терилүүдө: TypeScript 143 ms коротот, rust-anything batch 6.9 ms, ар бир баскычты басканда натыйжалар бирдей." width="820">
  <br>
  <sub>x18n'дин 50,000 ачкыч боюнча издөөсүнө суроо терилүүдө: TypeScript түп нускасы rust-anything batch'ына каршы, ар бир баскычты басканда натыйжалар бирдей.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Жазып алуу</b><br>
      <code>record</code> түп нусканы генерацияланган чектик кейстерде, семантикалык тузактарда жана өзүңүздүн киргизүүлөрүңүздө иштетип, ал эмне кайтарарын же кандай ката ыргытарын сактайт.
    </td>
    <td width="25%" valign="top">
      <b>Порттоо</b><br>
      <code>bridge</code> Rust crate'ин жана wrapper'ди генерациялайт, ошентип чакыруучулар түп нуска API'син сакташат. Rust кодун сиз же агентиңиз жазат.
    </td>
    <td width="25%" valign="top">
      <b>Текшерүү</b><br>
      <code>diff</code> ар бир жазылган кейсти Rust'ка каршы кайра ойнотот, алардын баары дал келмейинче жана <code>gate</code> өтмөйүнчө эч нерсе чыгарылбайт.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Replay fallback'сиз өтүп, ар бир бирдик gate'тен өткөндө, <code>app lift</code> аны таза Rust колдонмосуна айлантат. Түп нускага эч качан тийбейт.
    </td>
  </tr>
</table>

**Бүт колдонмоңузду Rust тилинде бөлүк-бөлүгү менен кайра жазыңыз.** rust-anything сиздин Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby же Java колдонмоңуз бүгүн эмне кылып жатканын жазып
алат, андан кийин Rust нускасынын ар бир бөлүгүн кийинкисине өтүүдөн мурун ошол жазуу менен салыштырып
текшерет, колдонмо толугу менен таза Rust болгонго чейин. CLI'лар, HTTP серверлер жана китепканалар;
Rust кодун сиздин коддоо агентиңиз (же өзүңүз) жазат. Ал жазуу камтыган нерсени гана кармай алат:
[Ал эмнени кармай албайт](how-it-works.md#what-it-cant-catch) бөлүмүн караңыз. Жай бөлүгүн гана
ылдамдатуу керекпи? Ошону гана порттоңуз, бир убакта бир hot path:

| Чыныгы жүктөм, натыйжалары түп нуска менен бирдей | Түп нуска | rust-anything менен |
| --- | ---: | ---: |
| x18n каптал панелиндеги издөө, 50,000 ачкыч (TypeScript, Node) | бир суроого 10 ms | **0.5 ms: 20x ылдамыраак** |
| Ошол эле издөө браузерде (wasm, бир агым) | 10 ms | **0.95 ms: 11.5x** |
| `slugify`, 20,000 сап (Python) | 98 ms | **17x ылдамыраак** |
| Ruby стандарттык китепканасындагы `Shellwords.shellescape`, 50,000 маани | 88 ms | **11.8x ылдамыраак** |
| `Slugify`, 50,000 сап (Go) | 49 ms | **4x ылдамыраак** |
| `slugify`, 50,000 сап (Java) | 43 ms | **3x ылдамыраак** |

4 ядродо өлчөндү; ар бир сан үчүн [`demo/`](../demo) ичинде адегенде натыйжалардын бирдей экенин
текшерген бенчмарк бар. Ар бир кичинекей функция үчүн Rust'ту өзүнчө чакыруу адатта JIT'тен *жайыраак*;
rust-anything көчүрүүгө арзый турган циклди таап, ошону көчүрөт.

> rust-anything көз карандысыз долбоор, ал Rust Project же Rust Foundation уюмдары менен байланышкан
> эмес жана алар тарабынан колдоого алынган эмес. "Rust" Rust Foundation уюмунун соода белгиси.

> [!WARNING]
> `record` жана `app record` түп нуска кодуңузду иштетет: ар бир функцияны (же бүт колдонмону) көптөгөн
> генерацияланган киргизүүлөр менен чакырат, ал эми `app record` ар бир кейсти эки жолу иштетет. `diff`,
> `gate` жана `app replay` ошол эле кейстерди портто же түп нускада кайрадан иштетет. Файл жазган,
> маалымат базасы менен иштеген же тармакка кайрылган код муну ар бир кейс үчүн бир жолу аткарат. Аларды
> долбоордун көчүрмөсүндө, тесттик маалымат базасы менен жана чөйрөдө production эсептик маалыматтары жок
> абалда иштетиңиз.

## Тез баштоо

Муну Claude Code, Codex, Cursor же Gemini CLI ичине чаптап, орнотууну агентиңизге тапшырыңыз:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Же өзүңүз:

```sh
cargo install rust-anything   # from crates.io
rust-anything setup           # registers it with Claude Code, Codex, Cursor and Gemini CLI
```

Rust toolchain азырынча жокпу? macOS, Linux жана Windows үчүн даяр бинардык файлдар ар бир
[GitHub release](https://github.com/erkamyaman/rust-anything/releases) барагына тиркелген, ал эми
JavaScript долбоорлору үчүн `npm install -g rust-anything` ошол эле бинардык файлды орнотот.

Claude Code ичинде аны плагин катары да орнотсоңуз болот (MCP сервер жана skill; жогорудагы CLI сиздин
`PATH` ичинде болушу керек):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` тапкан ар бир агентке MCP серверди кошот жана workflow skill'ин агент skill'дерди окуган жерге
орнотот. Ал башка жөндөөлөрүңүздү сактайт, аны кайра иштетүү коопсуз, ал эми `--remove` баарын артка
кайтарат. Андан кийин агентиңизден, мисалы, мындай сураңыз:

- *"Бул CLI'ды rust-anything менен башынан аягына чейин Rust тилинде кайра жаз жана анын так ошондой иштээрин далилде."*
- *"Бул издөө жай иштеп жаткандай. Аны rust-anything менен профилде жана hot loop'ту Rust'ка көчүр."*
- *"rust-anything менен `src/parser/` папкасын Rust'ка көчүрүү планын түз; биринчи бирдикти порттоп, далилде."*
- *"`rust/text-rs` ичиндеги Rust портун түп нуска менен бенчмарк аркылуу салыштыр да, аны чыгаруу керекпи, айт."*

### Шайкеш агенттер

| Агент | `rust-anything setup` кошкону |
| --- | --- |
| Claude Code | `~/.claude.json` ичинде MCP сервер, `~/.claude/skills/` ичинде skill |
| Codex | `~/.codex/config.toml` ичинде MCP сервер, `~/.codex/skills/` ичинде skill |
| Cursor | `~/.cursor/mcp.json` ичинде MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` ичинде MCP сервер |
| Башка каалаган MCP кардар | `rust-anything mcp` буйругун stdio сервер катары кол менен кошуңуз ([Орнотуу](install.md#install) бөлүмүн караңыз) |

Бар файлды өзгөртүүдөн мурун `setup` анын көчүрмөсүн жанына сактайт (`<file>.rust-anything.bak`).

Же кадамдарды өзүңүз аткарыңыз:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile циклдеги fuzzyScore функциясын табат, bridge --batch Rust crate'ин генерациялайт, gate 2000/2000 жазылган кейс менен өтөт, ал эми бенчмарк TS'тен 20.7x ылдамыраак экенин көрсөтөт." width="820">
  <br>
  <sub>x18n'дин <code>fuzzyScore</code> функциясындагы кадамдар: <code>profile</code> hot loop'ту табат, <code>bridge --batch</code> crate'ти генерациялайт, <code>gate</code> өтөт, ал эми бенчмарк аны TypeScript менен салыштырат.</sub>
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

rust-anything кодду которбойт: Rust кодун сиздин коддоо агентиңиз (Claude Code, Codex, Cursor, ...)
же өзүңүз жазасыз. rust-anything агент ишенимдүү түрдө болжолдой албаган нерселерди берет: убакыт
кайда кетет, адегенде эмнени жана кандай тартипте порттоо керек, калган код Rust'ту API'син өзгөртпөй
чакырышы үчүн bridge, порт дал келиши керек болгон так жүрүм-турум, аны чыгаруунун коопсуз жолу жана
порт качан даяр болгонун айткан gate.

## Тилдер

| Тил | Бирдик | Bridge | Ким жазып алат жана кайра ойнотот |
| --- | --- | --- | --- |
| TypeScript/JavaScript | папка же файл | генерацияланган бинардык wire менен napi-rs же wasm-bindgen; класстар жана async да бар | Node, `.ts` баштапкы файлдарын түз иштетет |
| Python | пакет же модуль | PyO3 (abi3), түз айландыруу, типтелген exception'дор | Python, түп нусканы импорттойт |
| C | `.c` файлы | түп нуска аталыштары жана header менен `extern "C"` функциялары | shared library катары компиляцияланган түп нуска, ctypes аркылуу чакырылат, ар бир кейске бир fork |
| Go | пакет | типтелген cgo чакыруулары жана генерацияланган бинардык wire, түп нуска API'си бар пакетте | генерацияланган Go драйвери, өзүнчө модуль катары курулат |
| Ruby | модуль функцияларынан турган файл же папка | түп нуска модулдардын астындагы magnus кеңейтүүсү | генерацияланган Ruby драйвери |
| Java | `public static` методдорунан турган пакет | типтелген JNI жана генерацияланган бинардык wire, түп нуска методдору бар wrapper класстарында | генерацияланган reflection драйвери |
| PHP (preview) | функциялар жана `public static` методдорунан турган папка | кийинки кезекте: C ABI аркылуу PHP FFI | бүгүн бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана серверлер үчүн (`php -S`) |
| C# (preview) | `public static` методдорунан турган папка | кийинки кезекте: C ABI аркылуу P/Invoke | бүгүн бүт колдонмолор: `app record`, `app replay` жана front door, CLI'лар жана ASP.NET серверлери үчүн (`dotnet <app>.dll`) |

C'дан башка ар бир тил batch класстарын да алат (`--batch`): элементтер чектен бир жолу өтүп, Rust
ичинде калат, ал эми ар бир чакыруу алардын баарын бардык ядролордо иштетет.

Тил репозиторийден аныкталат (`go.mod`, `pom.xml`, `.csproj` же `.sln`, `pyproject.toml`, `Gemfile`,
`composer.json`, `package.json`, болбосо эң көп кездешкен баштапкы файлдар боюнча); `--lang` аны кол
менен көрсөтөт. Типтер баштапкы коддон алынат: TS типтери, Python hint'тери, C, Go, Java жана C#
сигнатуралары, Ruby YARD тегдери, PHP типтери жана phpdoc; сан катары колдонулган типсиз параметрлер
сан деп эсептелет.

PHP preview абалында: `analyze` PHP колдонмосун пландайт (Composer пакеттери жана PSR-4 autoloading,
loose comparison жана integer'дин float'ка толуп кетиши сыяктуу тобокелдиктер, crate'терге шайкештелген
Composer пакеттери), ал эми PHP CLI же серверин жазып алууга, кайра ойнотууга жана Rust front door'дун
артына коюуга болот. Анын бирдиктерин PHP FFI аркылуу Rust'ка порттоо кийинки кадам
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# да ушундай эле preview абалында: `analyze` `.csproj` долбоорлорун жана алардын NuGet пакеттерин
окуйт (crate'терге шайкештелет), `using` директиваларын папкаларга чечет жана C# башкача кылган
нерселерди белгилейт (`decimal`, `dynamic`, LINQ'тун кийинкиге калтырылган суроо-талаптары, `Task`,
`lock`, reflection). C# CLI же ASP.NET серверин жазып алууга, кайра ойнотууга жана Rust front door'дун
артына коюуга болот; бирдиктерди P/Invoke аркылуу порттоо кийинки кезекте.

## Көбүрөөк билүү

- [Кантип иштейт](how-it-works.md): цикл, `bridge` жана `analyze` эмнени чечет, hot path'тар жана ал эмнени кармай албайт
- [Бүт колдонмону порттоо](whole-apps.md): серверлер, CLI'лар жана китепканалар, front door'дон `app lift`'ке чейин
- [Мисалдар жана практикалык изилдөөлөр](examples.md): x18n жана алты тилдеги чыныгы код
- [Чыныгы порттор](real-world.md): json-server, Fuse.js, vercel/ms, бенчмарктары менен
- [Орнотуу, жаңыртуу жана иштеп чыгуу](install.md): орнотуунун бардык жолдору, Windows жана баштапкы коддон куруу
- [Көп берилүүчү суроолор](faq.md) · [Абалы](STATUS.md) · [MCP куралдары](tools.md)

## Демөөрчүлөр

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> акысыз жана ачык булак коддуу долбоор. Эгер ал ишиңизге жардам берсе, ага демөөрчү болууну карап көрүңүз.<br>
  Сиздин колдооңуз иштеп чыгууну улантууга жардам берет.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Компания демөөрчүсү: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Демөөрчүлөр: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%D0%94%D0%B5%D0%BC%D3%A9%D3%A9%D1%80%D1%87%D2%AF_%D0%B1%D0%BE%D0%BB%D1%83%D1%83-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Демөөрчү болуу"></a>
</p>

## Салым кошуу

Issue'лар жана pull request'тер кубаттуу кабыл алынат; [CONTRIBUTING.md](../CONTRIBUTING.md) чөйрөнү
жөндөөнү, тесттер кайда жайгашарын жана жаңы тилди кантип кошууну түшүндүрөт. Дизайн чечимдери жана
алардын себептери [DECISIONS.md](../DECISIONS.md) ичинде. Чыныгы порттордун практикалык мисалдары
өзгөчө кубаттуу кабыл алынат. Коопсуздук көйгөйлөрү жөнүндө жеке кабарлаңыз
([SECURITY.md](../SECURITY.md)), ошондой эле [жүрүм-турум кодексин](../CODE_OF_CONDUCT.md) караңыз.

## Лицензия

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Репозиторийиңиздеги порттор жана генерацияланган код сиздики.
