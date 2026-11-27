<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="Краб rust-anything">
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
Перепишите своё приложение на Rust и докажите, что оно ведёт себя так же.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Документация:</b> <a href="https://rustanything.ai/ru">https://rustanything.ai/ru</a></p>

<p align="center"><a href="translations.md">🌐 Другие языки</a></p>

<p align="center">Переводите кодовую базу на Rust? Я консультирую по миграции на Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Этот перевод поддерживается сообществом и может отставать от <a href="../README.md">английского README</a>.</sub></p>

## Спонсоры

<p align="center">Сделано в Türkiye 🇹🇷, автор <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything бесплатный и с открытым исходным кодом. Если он помогает в работе, ваша поддержка помогает ему развиваться.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Спонсор-компания: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Спонсоры: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%D0%A1%D1%82%D0%B0%D1%82%D1%8C_%D1%81%D0%BF%D0%BE%D0%BD%D1%81%D0%BE%D1%80%D0%BE%D0%BC-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Стать спонсором"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">Спонсорство от компании или что-то другое? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">Переводите кодовую базу на Rust? Я консультирую по миграции на Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Быстрый старт

Вставьте это в своего агента для кода:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

Или установите сами:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **Агентам:** прочитайте [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt), там шаги установки, все команды и порядок портирования.

## Как это работает

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/ru-dark.svg">
    <img src="assets/how-it-works/ru-light.svg" width="880" alt="analyze планирует, что портировать; rust-anything go строит мост, записывает, отдаёт порт вашему агенту и прогоняет через gate, повторяя, пока каждый записанный случай не совпадёт; затем enable переключает вызовы на Rust">
  </picture>
</p>

**Что считается готовым.** Поведение, а не ваши тесты. `record` запускает оригинал на сгенерированных и заданных входных данных и сохраняет, что каждый вызов возвращает, выбрасывает и печатает. Порт готов, когда проходит `gate`: каждый записанный случай совпадает, а fmt, clippy, `cargo test`, проверка `unsafe`, проверка заглушек и проверка на записанные ответы, вставленные в порт, тоже проходят. Ваши собственные тесты тоже запускаются.

**Отчёт или миграция, выбирать вам.** `analyze` и `profile` только сообщают: ранжированный план того, что портировать, и что горячее всего под реальной нагрузкой. Это обычные команды, поэтому отчёт не тратит токенов. `go` переносит один модуль: строит мост к самой горячей функции, записывает её, передаёт порт вашему агенту, затем собирает, сравнивает и прогоняет через `gate`, и включает его, только когда всё совпадает (TS/JS, Python и PowerShell; `disable` выключает обратно).

**Сколько длится запуск.** Один запуск `go` портирует один модуль, максимум за `--rounds` ходов агента (по умолчанию 3): один на порт, затем по одному на каждый непройденный diff или gate. Целое приложение переносится по одному запуску на модуль, так что можно остановиться после любого из них.

**Дешёвая модель для плана, сильная для порта.** Плану модель не нужна. Для порта передайте любую команду агента: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Агенты.** `rust-anything setup` добавляет его в Claude Code, Codex, Cursor, Gemini CLI и OpenCode. Любой другой MCP-клиент может запустить `rust-anything mcp`, а навык также ставится отдельно через `npx skills add erkamyaman/rust-anything` (см. [установку](install.md)).

## Лицензия

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Порты и сгенерированный код в вашем репозитории принадлежат вам.
