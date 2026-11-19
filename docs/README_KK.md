<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">авторы: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · Қазақша · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a></p>

<p align="center"><sub>Бұл аударманы қауымдастық жүргізеді, сондықтан ол <a href="../README.md">ағылшынша README</a> нұсқасынан артта қалуы мүмкін.</sub></p>

<p align="center"><em>Бүкіл қолданбаңызды Rust тілінде қайта жазыңыз. Бүкіл жол бойы оны жұмыс күйінде сақтаңыз.</em></p>

<p align="center"><em>CLI-лар, серверлер мен кітапханалар, бөлік-бөлігімен, әр қадамда түпнұсқамен салыстырылып тексеріледі.</em></p>

<p align="center">Node.js және TypeScript · Python · C · Go · Ruby · Java · PHP және C# preview күйінде</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io нұсқасы"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm нұсқасы"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 тіл, тағы 2 preview күйінде">
  <a href="#үйлесімді-агенттер"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 агентпен жұмыс істейді"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Тесттер"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT лицензиясы"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub жұлдыздары"></a>
</p>

<p align="center">
  <a href="#жылдам-бастау">Жылдам бастау</a> ·
  <a href="#тілдер">Тілдер</a> ·
  <a href="how-it-works.md#how-it-works">Қалай жұмыс істейді</a> ·
  <a href="how-it-works.md#the-loop">Цикл</a> ·
  <a href="whole-apps.md#port-a-whole-app">Бүкіл қолданбаны порттау</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path-тар</a> ·
  <a href="install.md#install">Орнату</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Ол нені ұстай алмайды</a> ·
  <a href="faq.md#faq">Жиі қойылатын сұрақтар</a>
</p>

<p align="center">20x дейін жылдамырақ · 38,251 жазылған кейс қайта ойнатылды · 0 сәйкессіздік · бір команда</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n іздеуіне 50,000 кілт бойынша user.profile теріліп жатыр: TypeScript 143 ms жұмсайды, rust-anything batch 6.9 ms, әр пернені басқан сайын нәтижелер бірдей." width="820">
  <br>
  <sub>x18n іздеуіне 50,000 кілт бойынша сұраныс теріліп жатыр: TypeScript түпнұсқасы rust-anything batch-іне қарсы, әр пернені басқан сайын нәтижелер бірдей.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Жазып алу</b><br>
      <code>record</code> түпнұсқаны генерацияланған шеткі кейстерде, семантикалық тұзақтарда және өз кірістеріңізде іске қосып, оның не қайтаратынын немесе не лақтыратынын сақтайды.
    </td>
    <td width="25%" valign="top">
      <b>Порттау</b><br>
      <code>bridge</code> Rust crate-ін және wrapper-ді генерациялайды, сондықтан шақырушылар түпнұсқа API-ді сақтайды. Rust кодын сіз немесе агентіңіз жазады.
    </td>
    <td width="25%" valign="top">
      <b>Тексеру</b><br>
      <code>diff</code> әр жазылған кейсті Rust-қа қарсы қайта ойнатады, және барлығы сәйкес келіп, <code>gate</code> өтпейінше ештеңе шығарылмайды.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Қайта ойнату fallback-сіз өтіп, әр бірлік gate-тен өткенде, <code>app lift</code> оны таза Rust қолданбасына айналдырады. Түпнұсқаға ешқашан қол тигізілмейді.
    </td>
  </tr>
</table>

**Бүкіл қолданбаңызды Rust тілінде бөлік-бөлігімен қайта жазыңыз.** rust-anything сіздің Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby немесе Java қолданбаңыздың бүгін не істейтінін жазып
алады, содан кейін Rust нұсқасының әр бөлігін келесіге өтпес бұрын сол жазбамен салыстырып тексереді,
қолданба толығымен таза Rust болғанша. CLI-лар, HTTP серверлер және кітапханалар; Rust кодын сіздің
кодинг-агентіңіз (немесе өзіңіз) жазады. Ол тек жазба қамтитын нәрсені ғана ұстай алады:
[Ол нені ұстай алмайды](how-it-works.md#what-it-cant-catch) бөлімін қараңыз. Тек баяу бөлікті жылдамдату керек пе?
Тек соны порттаңыз, бір уақытта бір hot path:

| Нақты жүктеме, нәтижелері түпнұсқамен бірдей | Түпнұсқа | rust-anything-пен |
| --- | ---: | ---: |
| x18n бүйірлік панеліндегі іздеу, 50,000 кілт (TypeScript, Node) | бір сұранысқа 10 ms | **0.5 ms: 20x жылдамырақ** |
| Сол іздеу браузерде (wasm, бір ағын) | 10 ms | **0.95 ms: 11.5x** |
| `slugify`, 20,000 жол (Python) | 98 ms | **17x жылдамырақ** |
| Ruby стандартты кітапханасындағы `Shellwords.shellescape`, 50,000 мән | 88 ms | **11.8x жылдамырақ** |
| `Slugify`, 50,000 жол (Go) | 49 ms | **4x жылдамырақ** |
| `slugify`, 50,000 жол (Java) | 43 ms | **3x жылдамырақ** |

4 ядрода өлшенді; әр сан үшін [`demo/`](../demo) ішінде алдымен нәтижелердің бірдей екенін тексеретін
бенчмарк бар. Әр кішкентай функция үшін Rust-ты жеке-жеке шақыру әдетте JIT-тен *баяуырақ*;
rust-anything көшіруге тұрарлық циклды тауып, соны көшіреді.

> rust-anything тәуелсіз жоба, ол Rust Project немесе Rust Foundation ұйымдарымен байланысты емес
> және олар тарапынан мақұлданбаған. «Rust» Rust Foundation ұйымының сауда белгісі.

> [!WARNING]
> `record` және `app record` түпнұсқа кодыңызды іске қосады: әр функцияны (немесе бүкіл қолданбаны)
> көптеген генерацияланған кірістермен шақырады, ал `app record` әр кейсті екі рет іске қосады. `diff`,
> `gate` және `app replay` сол кейстерді портта немесе түпнұсқада қайта іске қосады. Файлдарға жазатын,
> дерекқормен жұмыс істейтін немесе желіге қатынайтын код мұны әр кейс үшін бір рет істейді. Оларды
> жобаның көшірмесінде, тесттік дерекқормен және ортада production тіркелгі деректері жоқ күйде іске қосыңыз.

## Жылдам бастау

Мынаны Claude Code, Codex, Cursor немесе Gemini CLI ішіне қойып, орнатуды агентіңізге тапсырыңыз:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Немесе өзіңіз:

```sh
cargo install rust-anything   # from crates.io
rust-anything setup           # registers it with Claude Code, Codex, Cursor and Gemini CLI
```

Rust toolchain әлі жоқ па? macOS, Linux және Windows үшін дайын бинарлық файлдар әр
[GitHub release](https://github.com/erkamyaman/rust-anything/releases) бетіне тіркелген, ал JavaScript
жобалары үшін `npm install -g rust-anything` сол бинарлық файлды орнатады.

Claude Code ішінде оны плагин ретінде де орнатуға болады (MCP сервер және skill; жоғарыдағы CLI сіздің
`PATH` ішінде болуы керек):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` тапқан әр агентке MCP серверді қосады және workflow skill-ін агент skill-дерді оқитын жерге
орнатады. Ол басқа баптауларыңызды сақтайды, оны қайта іске қосу қауіпсіз, ал `--remove` барлығын кері
қайтарады. Содан кейін агентіңізден, мысалы, былай сұраңыз:

- *"Осы CLI-ды rust-anything көмегімен басынан аяғына дейін Rust тілінде қайта жаз және оның дәл солай жұмыс істейтінін дәлелде."*
- *"Бұл іздеу баяу сияқты. Оны rust-anything көмегімен профайлда және hot loop-ты Rust-қа көшір."*
- *"rust-anything көмегімен `src/parser/` бумасын Rust-қа көшіру жоспарын жаса; бірінші бірлікті порттап, дәлелде."*
- *"`rust/text-rs` ішіндегі Rust портын түпнұсқамен бенчмарк арқылы салыстыр да, оны шығару керек пе, айт."*

### Үйлесімді агенттер

| Агент | `rust-anything setup` қосатыны |
| --- | --- |
| Claude Code | `~/.claude.json` ішінде MCP сервер, `~/.claude/skills/` ішінде skill |
| Codex | `~/.codex/config.toml` ішінде MCP сервер, `~/.codex/skills/` ішінде skill |
| Cursor | `~/.cursor/mcp.json` ішінде MCP сервер |
| Gemini CLI | `~/.gemini/settings.json` ішінде MCP сервер |
| Кез келген басқа MCP клиент | `rust-anything mcp` командасын stdio сервер ретінде қолмен қосыңыз ([Орнату](install.md#install) бөлімін қараңыз) |

Бар файлды өзгертпес бұрын `setup` оның көшірмесін қасына сақтайды (`<file>.rust-anything.bak`).

Немесе қадамдарды өзіңіз орындаңыз:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile циклдағы fuzzyScore функциясын табады, bridge --batch Rust crate-ін генерациялайды, gate 2000/2000 жазылған кейспен өтеді, ал бенчмарк TS-тен 20.7x жылдамырақ екенін көрсетеді." width="820">
  <br>
  <sub>x18n-нің <code>fuzzyScore</code> функциясындағы қадамдар: <code>profile</code> hot loop-ты табады, <code>bridge --batch</code> crate-ті генерациялайды, <code>gate</code> өтеді, ал бенчмарк оны TypeScript-пен салыстырады.</sub>
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

rust-anything кодты аудармайды: Rust кодын сіздің кодинг-агентіңіз (Claude Code, Codex, Cursor, ...)
немесе өзіңіз жазасыз. rust-anything агент сенімді түрде болжай алмайтын нәрселерді береді: уақыт қайда
кетеді, алдымен нені және қандай ретпен порттау керек, қалған код Rust-ты API-ін өзгертпей шақыруы үшін
bridge, порт сәйкес келуге тиіс нақты мінез-құлық, оны шығарудың қауіпсіз жолы және порттың қашан дайын
болғанын айтатын gate.

## Тілдер

| Тіл | Бірлік | Bridge | Жазып алатын және қайта ойнататын |
| --- | --- | --- | --- |
| TypeScript/JavaScript | бума немесе файл | генерацияланған бинарлық wire бар napi-rs немесе wasm-bindgen; кластар мен async те бар | Node, `.ts` бастапқы файлдарын тікелей іске қосады |
| Python | пакет немесе модуль | PyO3 (abi3), тікелей түрлендіру, типтелген exception-дар | Python, түпнұсқаны импорттайды |
| C | `.c` файлы | түпнұсқа атаулары мен header бар `extern "C"` функциялары | shared library ретінде компиляцияланған түпнұсқа, ctypes арқылы шақырылады, әр кейске бір fork |
| Go | пакет | типтелген cgo шақырулары және генерацияланған бинарлық wire, түпнұсқа API бар пакетте | генерацияланған Go драйвері, жеке модуль ретінде құрастырылады |
| Ruby | модуль функцияларынан тұратын файл немесе бума | түпнұсқа модульдер астындағы magnus кеңейтімі | генерацияланған Ruby драйвері |
| Java | `public static` әдістерінен тұратын пакет | типтелген JNI және генерацияланған бинарлық wire, түпнұсқа әдістері бар wrapper кластарында | генерацияланған reflection драйвері |
| PHP (preview) | функциялар мен `public static` әдістерінен тұратын бума | келесі кезекте: C ABI арқылы PHP FFI | бүгін бүкіл қолданбалар: `app record`, `app replay` және front door, CLI-лар мен серверлер үшін (`php -S`) |
| C# (preview) | `public static` әдістерінен тұратын бума | келесі кезекте: C ABI арқылы P/Invoke | бүгін бүкіл қолданбалар: `app record`, `app replay` және front door, CLI-лар мен ASP.NET серверлері үшін (`dotnet <app>.dll`) |

C-дан басқа әр тіл batch кластарын да алады (`--batch`): элементтер шекарадан бір рет өтіп, Rust ішінде
қалады, ал әр шақыру олардың барлығын барлық ядроларда өңдейді.

Тіл репозиторийден анықталады (`go.mod`, `pom.xml`, `.csproj` немесе `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, әйтпесе ең көп кездесетін бастапқы файлдар бойынша);
`--lang` оны қолмен көрсетеді. Типтер бастапқы кодтан алынады: TS типтері, Python hint-тері, C, Go, Java
және C# сигнатуралары, Ruby YARD тегтері, PHP типтері және phpdoc; сан ретінде қолданылатын типсіз
параметрлер сан деп есептеледі.

PHP preview күйінде: `analyze` PHP қолданбасын жоспарлайды (Composer пакеттері және PSR-4 autoloading,
loose comparison және integer-дің float-қа толып кетуі сияқты тәуекелдер, crate-терге сәйкестендірілген
Composer пакеттері), ал PHP CLI немесе серверін жазып алуға, қайта ойнатуға және Rust front door-ының
артына қоюға болады. Оның бірліктерін PHP FFI арқылы Rust-қа порттау келесі қадам
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# та дәл солай preview күйінде: `analyze` `.csproj` жобалары мен олардың NuGet пакеттерін оқиды
(crate-терге сәйкестендіріледі), `using` директиваларын бумаларға шешеді және C# басқаша істейтін
нәрселерді белгілейді (`decimal`, `dynamic`, LINQ-тің кейінге қалдырылған сұраныстары, `Task`, `lock`,
reflection). C# CLI немесе ASP.NET серверін жазып алуға, қайта ойнатуға және Rust front door-ының
артына қоюға болады; бірліктерді P/Invoke арқылы порттау келесі кезекте.

## Толығырақ

- [Қалай жұмыс істейді](how-it-works.md): цикл, `bridge` пен `analyze` нені шешеді, hot path-тар және ол нені ұстай алмайды
- [Бүкіл қолданбаны порттау](whole-apps.md): серверлер, CLI-лар мен кітапханалар, front door-дан `app lift`-ке дейін
- [Мысалдар мен тәжірибелер](examples.md): x18n және алты тілдегі нақты код
- [Нақты порттар](real-world.md): json-server, Fuse.js, vercel/ms, бенчмарктарымен
- [Орнату, жаңарту және әзірлеу](install.md): орнатудың барлық тәсілдері, Windows және бастапқы кодтан құрастыру
- [Жиі қойылатын сұрақтар](faq.md) · [Күйі](STATUS.md) · [MCP құралдары](tools.md)

## Демеушілер

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> тегін және ашық бастапқы кодты жоба. Егер ол жұмысыңызға көмектессе, оған демеуші болуды қарастырыңыз.<br>
  Сіздің қолдауыңыз әзірлеуді жалғастыруға мүмкіндік береді.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Компания демеушісі: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Демеушілер: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%D0%94%D0%B5%D0%BC%D0%B5%D1%83%D1%88%D1%96_%D0%B1%D0%BE%D0%BB%D1%83-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Демеуші болу"></a>
</p>

## Үлес қосу

Issue-лер мен pull request-тер құпталады; [CONTRIBUTING.md](../CONTRIBUTING.md) ортаны баптауды,
тесттердің қайда орналасатынын және жаңа тілді қалай қосуды түсіндіреді. Дизайн шешімдері және олардың
себептері [DECISIONS.md](../DECISIONS.md) ішінде. Нақты порттардың тәжірибелік мысалдары әсіресе
құпталады. Қауіпсіздік мәселелері туралы жеке хабарлаңыз ([SECURITY.md](../SECURITY.md)), сондай-ақ
[мінез-құлық кодексін](../CODE_OF_CONDUCT.md) қараңыз.

## Лицензия

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Репозиторийіңіздегі порттар мен генерацияланған код сіздікі.
