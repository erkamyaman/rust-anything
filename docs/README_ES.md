<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="El cangrejo de rust-anything">
  </a>
</p>

<h1 align="center">Rust Anything</h1>

<p align="center">
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io" alt="crates.io version"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm" alt="npm version"></a>
<a href="https://github.com/erkamyaman/rust-anything/stargazers"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=stars" alt="GitHub stars"></a>
<a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/discord-join-111111?style=flat&colorA=111111&colorB=b34a1a" alt="Discord"></a>
<a href="https://github.com/erkamyaman/rust-anything#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-111111?style=flat&colorA=111111&colorB=b34a1a" alt="MIT or Apache-2.0 license"></a><br>
<a href="https://rustanything.ai/"><img src="https://img.shields.io/endpoint?url=https%3A%2F%2Frustanything.ai%2Fbadge-downloads.json&style=flat" alt="Total downloads, crates.io and npm"></a>
<a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/d/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=crates.io%20downloads" alt="crates.io downloads"></a>
<a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/d18m/rust-anything?style=flat&colorA=111111&colorB=b34a1a&label=npm%20downloads" alt="npm downloads"></a>
</p>

<p align="center">
Reescribe tu aplicación en Rust y demuestra que sigue comportándose igual.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>Documentación:</b> <a href="https://rustanything.ai/es">https://rustanything.ai/es</a></p>

<p align="center"><a href="translations.md">🌐 Otros idiomas</a></p>

<p align="center">¿Migrando una base de código a Rust? Ofrezco consultoría para migraciones a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>Esta traducción la mantiene la comunidad y puede quedarse atrás respecto al <a href="../README.md">README en inglés</a>.</sub></p>

## Patrocinadores

<p align="center">Hecho en Türkiye 🇹🇷 por <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything es gratuito y de código abierto. Si te ayuda en tu trabajo, tu patrocinio lo mantiene en marcha.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Patrocinador de la empresa: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Patrocinadores: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Become a sponsor"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">¿Patrocinio como empresa, o cualquier otra cosa? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">¿Migrando una base de código a Rust? Ofrezco consultoría para migraciones a Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## Inicio rápido

Pega esto en tu agente de código:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

O instálalo tú mismo:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **Agentes:** lee [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) para los pasos de instalación, todos los comandos y el flujo de migración.

## Cómo funciona

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/es-dark.svg">
    <img src="assets/how-it-works/es-light.svg" width="880" alt="analyze planifica qué portar; rust-anything go crea el puente, graba, deja que tu agente porte y lo pasa por gate, repitiendo hasta que todos los casos grabados coinciden; luego enable cambia las llamadas a Rust">
  </picture>
</p>

**Qué cuenta como terminado.** El comportamiento, no tus tests. `record` ejecuta el original con entradas generadas y dadas, y guarda lo que cada llamada devuelve, lanza e imprime. Un porte está terminado cuando `gate` pasa: todos los casos grabados coinciden, y fmt, clippy, `cargo test`, la comprobación de `unsafe`, la de stubs y la de respuestas grabadas pegadas en el porte pasan todas. Tus propios tests también se ejecutan.

**Informe o migración, tú eliges.** `analyze` y `profile` solo informan: un plan ordenado de qué portar y de qué está caliente bajo una carga real. Son comandos normales, así que el informe no cuesta tokens. `go` migra una unidad: crea el puente para la función más caliente, la graba, entrega el porte a tu agente, luego lo compila, lo compara y lo pasa por `gate`, y solo lo activa cuando todo coincide (TS/JS, Python y PowerShell; `disable` lo vuelve a desactivar).

**Cuánto dura una ejecución.** Una ejecución de `go` porta una unidad, con como máximo `--rounds` turnos del agente (3 por defecto): uno para el porte y luego uno por cada diff o gate que falle. Una aplicación entera es una ejecución por unidad, así que puedes parar después de cualquiera.

**Modelo barato para el plan, modelo potente para el porte.** El plan no necesita modelo. Para el porte, pasa cualquier comando de agente: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**Agentes.** `rust-anything setup` lo añade a Claude Code, Codex, Cursor, Gemini CLI y OpenCode. Cualquier otro cliente MCP puede ejecutar `rust-anything mcp`, y la skill también se instala por sí sola con `npx skills add erkamyaman/rust-anything` (ver [instalación](install.md)).

## Licencia

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). Los portes y el código generado en tu repositorio son tuyos.
