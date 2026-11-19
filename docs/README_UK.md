<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="Краб rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">автор: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · Українська · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> ? <a href="README_VI.md">Vietnamese</a></p>

<p align="center"><sub>Цей переклад підтримує спільнота, тому він може відставати від <a href="../README.md">англійського README</a>.</sub></p>

<p align="center"><em>Перепишіть увесь застосунок на Rust. І щоб він працював на кожному кроці.</em></p>

<p align="center"><em>CLI, сервери та бібліотеки, частина за частиною, з перевіркою щодо оригіналу на кожному кроці.</em></p>

<p align="center">Node.js і TypeScript · Python · C · Go · Ruby · Java · PHP і C# у preview</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="Версія на crates.io"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="Версія в npm"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 мов, плюс 2 у preview">
  <a href="#працює-з"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="Працює з 4 агентами"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Тести"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="Ліцензія MIT"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="Зірки на GitHub"></a>
</p>

<p align="center">
  <a href="#швидкий-старт">Швидкий старт</a> ·
  <a href="#мови">Мови</a> ·
  <a href="how-it-works.md#how-it-works">Як це працює</a> ·
  <a href="how-it-works.md#the-loop">Цикл</a> ·
  <a href="whole-apps.md#port-a-whole-app">Портування всього застосунку</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Гарячі шляхи</a> ·
  <a href="install.md#install">Встановлення</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Чого він не може виявити</a> ·
  <a href="faq.md#faq">Часті запитання</a>
</p>

<p align="center">до 20x швидше · 38 251 записаний випадок відтворено · 0 розбіжностей · одна команда</p>

<p align="center">
  <img src="assets/race.gif" alt="Введення user.profile у пошук x18n по 50 000 ключів: TypeScript витрачає 143 мс, batch від rust-anything 6,9 мс, з однаковими результатами після кожного натискання клавіші." width="820">
  <br>
  <sub>Введення запиту в пошук x18n по 50 000 ключів: оригінал на TypeScript проти batch від rust-anything, однакові результати після кожного натискання клавіші.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Запис</b><br>
      <code>record</code> запускає оригінал на згенерованих граничних випадках, семантичних пастках і ваших власних вхідних даних і зберігає, що він повертає або кидає.
    </td>
    <td width="25%" valign="top">
      <b>Портування</b><br>
      <code>bridge</code> генерує Rust crate і обгортку, тож код, що викликає, зберігає оригінальний API. Rust пишете ви або ваш агент.
    </td>
    <td width="25%" valign="top">
      <b>Перевірка</b><br>
      <code>diff</code> відтворює кожен записаний випадок на Rust, і нічого не йде в реліз, доки всі вони не збігаються і <code>gate</code> не проходить.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Коли відтворення проходить без fallback і кожна одиниця проходить gate, <code>app lift</code> робить із нього чистий застосунок на Rust. Оригінал ніколи не змінюється.
    </td>
  </tr>
</table>

**Перепишіть увесь застосунок на Rust, частина за частиною.** rust-anything записує, що сьогодні робить
ваш застосунок на Node.js (TypeScript/JavaScript), Python, C, Go, Ruby чи Java, а потім перевіряє кожну частину
переписаного на Rust коду щодо цього запису, перш ніж рухатися далі, доки застосунок не стане чистим Rust. CLI, HTTP-сервери
та бібліотеки; Rust пише ваш агент для програмування (або ви самі). Він може виявити лише те, що
покриває запис: див. [Чого він не може виявити](how-it-works.md#what-it-cant-catch). Потрібно пришвидшити лише повільну частину?
Портуйте тільки її, один гарячий шлях за раз:

| Реальне навантаження, ті самі результати, що й в оригіналу | Оригінал | З rust-anything |
| --- | ---: | ---: |
| Пошук у бічній панелі x18n, 50 000 ключів (TypeScript, Node) | 10 мс на запит | **0,5 мс: у 20x швидше** |
| Той самий пошук у браузері (wasm, один потік) | 10 мс | **0,95 мс: 11,5x** |
| `slugify` по 20 000 рядків (Python) | 98 мс | **у 17x швидше** |
| `Shellwords.shellescape` зі стандартної бібліотеки Ruby, 50 000 значень | 88 мс | **у 11,8x швидше** |
| `Slugify` по 50 000 рядків (Go) | 49 мс | **у 4x швидше** |
| `slugify` по 50 000 рядків (Java) | 43 мс | **у 3x швидше** |

Виміряно на 4 ядрах; для кожного числа в [`demo/`](../demo) є бенчмарк, який спершу перевіряє,
що результати ідентичні. Викликати Rust окремо для кожної маленької функції зазвичай *повільніше*, ніж JIT;
rust-anything знаходить цикл, який варто перенести, і переносить його.

> rust-anything є незалежним проєктом, не пов'язаним із Rust Project чи Rust Foundation і не схваленим ними.
> «Rust» є торговельною маркою Rust Foundation.

> [!WARNING]
> `record` і `app record` запускають ваш оригінальний код, викликаючи кожну функцію (або весь застосунок) на
> багатьох згенерованих вхідних даних, а `app record` запускає кожен випадок двічі. `diff`, `gate` і `app replay`
> знову запускають ті самі випадки, на порті або на оригіналі. Код, що пише файли, звертається до
> бази даних чи мережі, робить це один раз на кожен випадок. Запускайте їх на копії проєкту, з
> тестовою базою даних і без продакшн-облікових даних у середовищі.

## Швидкий старт

Вставте це в Claude Code, Codex, Cursor або Gemini CLI, і нехай ваш агент усе встановить:

```text
Встанови rust-anything: виконай `cargo install rust-anything`, потім `rust-anything setup`.
Скажи мені, коли тебе треба перезапустити.
```

Або самостійно:

```sh
cargo install rust-anything   # з crates.io
rust-anything setup           # реєструє його в Claude Code, Codex, Cursor і Gemini CLI
```

Ще немає Rust toolchain? Готові бінарні файли для macOS, Linux і Windows прикріплено до кожного
[релізу на GitHub](https://github.com/erkamyaman/rust-anything/releases), а для JavaScript-проєктів
`npm install -g rust-anything` встановлює той самий бінарний файл.

У Claude Code його також можна встановити як плагін (MCP-сервер і skill; CLI вище
має бути у вашому `PATH`):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` додає MCP-сервер до кожного агента, якого знаходить, і встановлює workflow skill туди, звідки агент
читає skills. Він зберігає інші ваші налаштування, його безпечно запускати повторно, а `--remove` скасовує зміни.
Потім попросіть агента, наприклад:

- *«Перепиши цей CLI на Rust за допомогою rust-anything, від початку до кінця, і доведи, що він поводиться так само.»*
- *«Цей пошук здається повільним. Запусти профілювання з rust-anything і перенеси гарячий цикл у Rust.»*
- *«Сплануй міграцію `src/parser/` на Rust з rust-anything; портуй першу одиницю і доведи її правильність.»*
- *«Порівняй у бенчмарку Rust-порт у `rust/text-rs` з оригіналом і скажи, чи варто його випускати.»*

### Працює з

| Агент | `rust-anything setup` додає |
| --- | --- |
| Claude Code | MCP-сервер у `~/.claude.json`, skill у `~/.claude/skills/` |
| Codex | MCP-сервер у `~/.codex/config.toml`, skill у `~/.codex/skills/` |
| Cursor | MCP-сервер у `~/.cursor/mcp.json` |
| Gemini CLI | MCP-сервер у `~/.gemini/settings.json` |
| Будь-який інший MCP-клієнт | додайте `rust-anything mcp` як stdio-сервер вручну (див. [Встановлення](install.md#install)) |

Перед зміною наявного файлу `setup` зберігає його копію поруч (`<file>.rust-anything.bak`).

Або виконайте кроки самостійно:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile знаходить fuzzyScore у циклі, bridge --batch генерує Rust crate, gate проходить з 2000/2000 записаних випадків, а бенчмарк показує, що це у 20,7x швидше за TS." width="820">
  <br>
  <sub>Кроки на <code>fuzzyScore</code> з x18n: <code>profile</code> знаходить гарячий цикл, <code>bridge --batch</code> генерує crate, <code>gate</code> проходить, а бенчмарк порівнює результат із TypeScript.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # що гаряче і хто це викликає
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# напишіть src/port.rs (або доручіть агенту), потім:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # кожен записаний випадок має збігтися
rust-anything bench rust/fuzzy-rs                      # швидше? вердикт скаже
rust-anything gate rust/fuzzy-rs                       # готово означає, що це проходить
```

rust-anything не перекладає код: Rust пише ваш агент для програмування (Claude Code, Codex, Cursor, ...) або ви
самі. rust-anything дає те, що агент не може надійно вгадати: куди йде час, що портувати
першим і в якому порядку, bridge, щоб решта коду викликала Rust з незмінним API,
точну поведінку, якій має відповідати порт, безпечний спосіб його випустити і gate, який каже, коли порт
готовий.

## Мови

| Мова | Одиниця | Bridge | Хто записує та відтворює |
| --- | --- | --- | --- |
| TypeScript/JavaScript | тека або файл | napi-rs або wasm-bindgen зі згенерованим бінарним wire-форматом; класи та async теж | Node, запускає вихідні файли `.ts` |
| Python | пакет або модуль | PyO3 (abi3), пряме перетворення, типізовані винятки | Python, імпортує оригінал |
| C | файл `.c` | функції `extern "C"` з оригінальними іменами та заголовком | оригінал, скомпільований у спільну бібліотеку, викликається через ctypes, один fork на випадок |
| Go | пакет | типізовані виклики cgo та згенерований бінарний wire-формат, у пакеті з оригінальним API | згенерований Go-драйвер, зібраний як окремий модуль |
| Ruby | файл або тека з функціями модулів | розширення magnus під оригінальними модулями | згенерований Ruby-драйвер |
| Java | пакет методів `public static` | типізований JNI та згенерований бінарний wire-формат, у класах-обгортках з оригінальними методами | згенерований драйвер на reflection |
| PHP (preview) | тека з функціями та методами `public static` | наступний крок: PHP FFI поверх C ABI | цілі застосунки вже зараз: `app record`, `app replay` і front door, для CLI та серверів (`php -S`) |
| C# (preview) | тека з методами `public static` | наступний крок: P/Invoke поверх C ABI | цілі застосунки вже зараз: `app record`, `app replay` і front door, для CLI та серверів ASP.NET (`dotnet <app>.dll`) |

Кожна мова, крім C, також отримує batch-класи (`--batch`): елементи передаються один раз і залишаються в
Rust, а кожен виклик обробляє їх усі на всіх ядрах.

Мова визначається за репозиторієм (`go.mod`, `pom.xml`, `.csproj` або `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, інакше за найпоширенішими вихідними файлами);
`--lang` перевизначає її. Типи беруться з коду: типи TS, підказки типів Python, сигнатури C, Go, Java і C#,
теги YARD у Ruby, типи PHP і phpdoc; нетипізовані параметри, які використовуються як числа, виводяться
як числа.

PHP у preview: `analyze` планує PHP-застосунок (пакети Composer і автозавантаження PSR-4, ризики
на кшталт нестрогого порівняння та переповнення цілих чисел у float, відповідність пакетів Composer crate'ам),
а PHP CLI чи сервер можна записати, відтворити і поставити за front door на Rust. Портування його
одиниць у Rust через PHP FFI є наступним кроком ([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# у preview так само: `analyze` читає проєкти `.csproj` та їхні пакети NuGet
(з відповідними crate'ами), зводить директиви `using` до тек і позначає, що C# робить інакше
(`decimal`, `dynamic`, відкладені запити LINQ, `Task`, `lock`, reflection). C# CLI чи сервер ASP.NET
можна записати, відтворити і поставити за front door на Rust; портування одиниць через P/Invoke
буде наступним.

## Дізнатися більше

- [Як це працює](how-it-works.md): цикл, що вирішують `bridge` і `analyze`, гарячі шляхи і чого він не може виявити
- [Портування всього застосунку](whole-apps.md): сервери, CLI та бібліотеки, від front door до `app lift`
- [Приклади та розбори](examples.md): x18n і реальний код шістьма мовами
- [Реальні портування](real-world.md): json-server, Fuse.js, vercel/ms, з бенчмарками
- [Встановлення, оновлення і розробка](install.md): усі способи встановлення, Windows і збирання з вихідного коду
- [Часті запитання](faq.md) · [Статус](STATUS.md) · [Інструменти MCP](tools.md)

## Спонсори

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> безкоштовний і з відкритим кодом. Якщо він допомагає вам у роботі, будь ласка, подумайте про спонсорство.<br>
  Ваша підтримка дає змогу продовжувати розробку.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Спонсор-компанія: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Спонсори: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%D0%A1%D1%82%D0%B0%D1%82%D0%B8_%D1%81%D0%BF%D0%BE%D0%BD%D1%81%D0%BE%D1%80%D0%BE%D0%BC-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Стати спонсором"></a>
</p>

## Участь у розробці

Issues і pull requests вітаються; [CONTRIBUTING.md](../CONTRIBUTING.md) описує налаштування, куди
класти тести і як додати мову. Проєктні рішення та їхні причини описано в
[DECISIONS.md](../DECISIONS.md). Особливо вітаються розбори реальних портувань. Будь ласка, повідомляйте
про проблеми безпеки приватно ([SECURITY.md](../SECURITY.md)) і ознайомтеся з
[кодексом поведінки](../CODE_OF_CONDUCT.md).

## Ліцензія

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Порти і згенерований код у вашому репозиторії належать вам.
