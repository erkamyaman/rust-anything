<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="rust-anything のカニ">
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
アプリを Rust で書き直し、元と同じように動くことを証明する。
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>ドキュメント:</b> <a href="https://rustanything.ai/ja">https://rustanything.ai/ja</a></p>

<p align="center"><a href="translations.md">🌐 他の言語</a></p>

<p align="center">コードベースを Rust に移行しますか？ Rust 移行のコンサルティングを行っています: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>この翻訳はコミュニティによって管理されており、<a href="../README.md">英語版 README</a> より古い場合があります。</sub></p>

## スポンサー

<p align="center">Türkiye 🇹🇷 で <a href="https://github.com/erkamyaman">erKam Yaman</a> が制作<br>
Rust Anything は無料のオープンソースです。お仕事の役に立っているなら、スポンサーが開発を支えます。</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企業スポンサー: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="スポンサー: Santosh Yadav、Hakan Soytürk、Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş、bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E3%82%B9%E3%83%9D%E3%83%B3%E3%82%B5%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="スポンサーになる"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">企業としてのスポンサー、またはその他のご用件は？ <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">コードベースを Rust に移行しますか？ Rust 移行のコンサルティングを行っています: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## クイックスタート

これをコーディングエージェントに貼り付けてください:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

または自分でインストール:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **エージェント向け:** インストール手順、すべてのコマンド、移植の流れは [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) を読んでください。

## 仕組み

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/ja-dark.svg">
    <img src="assets/how-it-works/ja-light.svg" width="880" alt="analyze が何を移植するかのプランを立て、rust-anything go がブリッジ、記録、エージェントによる移植、gate を、記録したすべてのケースが一致するまで繰り返し、最後に enable が呼び出しを Rust に切り替えます">
  </picture>
</p>

**何をもって完了とするか。** あなたのテストではなく、振る舞いです。`record` は生成した入力と与えた入力でオリジナルを実行し、各呼び出しが返す値、投げる例外、出力する内容を保存します。`gate` が通れば移植は完了です。つまり、記録したすべてのケースが一致し、fmt、clippy、`cargo test`、`unsafe` チェック、スタブチェック、記録済みの答えを移植版に貼り付けていないかのチェックがすべて通ることです。あなた自身のテストも実行されます。

**レポートか移行か、選べます。** `analyze` と `profile` はレポートだけを出します。何を移植すべきかの優先順位付きプランと、実際のワークロードで何がホットかです。普通のコマンドなので、レポートにトークンはかかりません。`go` は 1 つのユニットを移行します。最もホットな関数をブリッジし、記録し、移植をエージェントに渡し、ビルド、diff、`gate` を実行して、すべて一致したときだけ有効にします（TS/JS、Python、PowerShell。`disable` で再び無効にできます）。

**1 回の実行にかかる時間。** `go` の 1 回の実行で 1 つのユニットを移植し、エージェントのターンは最大 `--rounds` 回（デフォルト 3）です。移植に 1 回、その後は失敗した diff や gate ごとに 1 回です。アプリ全体はユニットごとに 1 回の実行なので、どの実行の後でも止められます。

**プランには安いモデル、移植には強いモデル。** プランにモデルは不要です。移植には任意のエージェントコマンドを渡します: `rust-anything go --agent "claude --model <model>" -- <workload>`。

**エージェント。** `rust-anything setup` は Claude Code、Codex、Cursor、Gemini CLI、OpenCode に追加します。その他の MCP クライアントは `rust-anything mcp` を実行でき、スキルは `npx skills add erkamyaman/rust-anything` で単体でもインストールできます（[インストール](install.md)を参照）。

## ライセンス

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman)。あなたのリポジトリ内の移植版および生成コードはすべてあなたのものです。
