<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything のカニ">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">作者: <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · 日本語 · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> ? <a href="README_VI.md">Vietnamese</a></p>

<p align="center"><sub>この翻訳はコミュニティによって管理されており、<a href="../README.md">英語版 README</a> より古い場合があります。</sub></p>

<p align="center"><em>アプリ全体を Rust で書き直す。その間ずっと動く状態を保ったまま。</em></p>

<p align="center"><em>CLI、サーバー、ライブラリを一つずつ移植し、あらゆる段階で元の実装と照合して検証します。</em></p>

<p align="center">Node.js / TypeScript · Python · C · Go · Ruby · Java · PHP と C# はプレビュー版</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io バージョン"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm バージョン"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 言語対応、さらに 2 言語がプレビュー版">
  <a href="#対応エージェント"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 つのエージェントに対応"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="テスト"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT ライセンス"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub スター"></a>
</p>

<p align="center">
  <a href="#クイックスタート">クイックスタート</a> ·
  <a href="#対応言語">対応言語</a> ·
  <a href="how-it-works.md#how-it-works">仕組み</a> ·
  <a href="how-it-works.md#the-loop">移植ループ</a> ·
  <a href="whole-apps.md#port-a-whole-app">アプリ全体の移植</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">ホットパス</a> ·
  <a href="install.md#install">インストール</a> ·
  <a href="how-it-works.md#what-it-cant-catch">検出できないもの</a> ·
  <a href="faq.md#faq">よくある質問</a>
</p>

<p align="center">最大 20 倍高速化 · 38,251 件の記録ケースをリプレイ · 0 件の不一致 · コマンドひとつで</p>

<p align="center">
  <img src="assets/race.gif" alt="50,000 キーに対する x18n の検索で user.profile を入力した場合: TypeScript は 143 ms、rust-anything のバッチ処理は 6.9 ms で、すべてのキー入力において同一の結果を返します。" width="820">
  <br>
  <sub>50,000 キーに対する x18n の検索にクエリを入力: TypeScript の元の実装と rust-anything のバッチを比較。すべてのキー入力で結果は同一です。</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>記録</b><br>
      <code>record</code> は生成したエッジケース、セマンティックトラップ、あなた自身の入力で元の実装を実行し、返り値や例外を保存します。
    </td>
    <td width="25%" valign="top">
      <b>移植</b><br>
      <code>bridge</code> は Rust クレートとラッパーを生成するので、呼び出し元は元の API をそのまま使えます。Rust を書くのはあなたかエージェントです。
    </td>
    <td width="25%" valign="top">
      <b>検証</b><br>
      <code>diff</code> は記録したすべてのケースを Rust に対してリプレイします。すべて一致して <code>gate</code> に合格するまで何も出荷されません。
    </td>
    <td width="25%" valign="top">
      <b>リフト</b><br>
      フォールバックなしでリプレイに合格し、すべてのユニットが gate に合格すると、<code>app lift</code> がアプリを純粋な Rust アプリにします。元の実装には一切手を加えません。
    </td>
  </tr>
</table>

**アプリ全体を、一つずつ Rust に書き直しましょう。** rust-anything は、既存の Node.js（TypeScript/JavaScript）、Python、C、Go、Ruby、または Java アプリが現在どう振る舞うかを記録し、Rust で書き直した各パーツを次へ進む前にその記録と照合して、最終的にアプリ全体が純粋な Rust になるまで検証を続けます。CLI、HTTP サーバー、ライブラリに対応しており、Rust コードを書くのはコーディングエージェント（あるいはあなた自身）です。記録がカバーしている範囲しか検出できない点には注意してください（[検出できないもの](how-it-works.md#what-it-cant-catch)を参照）。遅い部分だけを高速化したい場合は、ホットパスだけを一つずつ移植することもできます:

| 実ワークロード（元の実装と同一の結果） | 元の実装 | rust-anything 使用時 |
| --- | ---: | ---: |
| x18n のサイドバー検索、50,000 キー（TypeScript、Node） | 1 クエリあたり 10 ms | **0.5 ms: 20 倍高速** |
| 同じ検索をブラウザ上で実行（wasm、シングルスレッド） | 10 ms | **0.95 ms: 11.5 倍** |
| 20,000 文字列に対する `slugify`（Python） | 98 ms | **17 倍高速** |
| Ruby 標準ライブラリの `Shellwords.shellescape`、50,000 値 | 88 ms | **11.8 倍高速** |
| 50,000 文字列に対する `Slugify`（Go） | 49 ms | **4 倍高速** |
| 50,000 文字列に対する `slugify`（Java） | 43 ms | **3 倍高速** |

4 コア環境での計測結果です。すべての数値は [`demo/`](../demo) 内のベンチマークに基づいており、事前に結果が完全に一致することを確認しています。小さな関数ごとに毎回 Rust を呼び出すと、通常は JIT より*遅く*なります。rust-anything は移植する価値のあるループを見つけ出し、そのループごと移行します。

> rust-anything は独立したプロジェクトであり、Rust Project または Rust Foundation と提携・承認関係にあるものではありません。"Rust" は Rust Foundation の商標です。

> [!WARNING]
> `record` と `app record` は元のコードを実行し、各関数（またはアプリ全体）を多数の生成入力で呼び出します。`app record` はすべてのケースを 2 回実行します。`diff`、`gate`、`app replay` は同じケースを移植版または元の実装で再度実行します。ファイルを書き込む、データベースにアクセスする、ネットワークを呼び出すコードは、ケースごとにその処理を行います。プロジェクトのコピー上で、テスト用データベースを使い、本番環境の認証情報が環境に含まれない状態で実行してください。

## クイックスタート

Claude Code、Codex、Cursor、または Gemini CLI に以下を貼り付けて、エージェントにインストールを任せてください:

```text
rust-anything をインストールして: `cargo install rust-anything` を実行し、次に `rust-anything setup` を実行してください。
あなたを再起動すべきタイミングが来たら教えてください。
```

または自分で実行します:

```sh
cargo install rust-anything   # crates.io からインストール
rust-anything setup           # Claude Code、Codex、Cursor、Gemini CLI に登録
```

Rust ツールチェーンをまだ導入していない場合は、各 [GitHub release](https://github.com/erkamyaman/rust-anything/releases) に macOS、Linux、Windows 向けのビルド済みバイナリが添付されています。JavaScript プロジェクトの場合は `npm install -g rust-anything` でも同じバイナリをインストールできます。

Claude Code ではプラグインとしてインストールすることもできます（MCP サーバーとワークフロースキルが含まれます。上記の CLI が `PATH` に通っている必要があります）:

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` は見つかった各エージェントに MCP サーバーを追加し、エージェントがスキルを読み込む場所にワークフロースキルをインストールします。他の設定は保持され、何度実行しても安全で、`--remove` で元に戻せます。あとはエージェントに次のように依頼するだけです:

- *"rust-anything を使ってこの CLI をエンドツーエンドで Rust に書き直し、元の実装と同じ挙動であることを証明して。"*
- *"この検索が遅く感じる。rust-anything でプロファイルを取って、ホットループを Rust に移して。"*
- *"rust-anything で `src/parser/` の Rust 移行計画を立てて。最初のユニットを移植して検証して。"*
- *"`rust/text-rs` の Rust 移植版と元の実装をベンチマーク比較して、出荷すべきか教えて。"*

### 対応エージェント

| エージェント | `rust-anything setup` が追加する内容 |
| --- | --- |
| Claude Code | `~/.claude.json` の MCP サーバー、`~/.claude/skills/` のスキル |
| Codex | `~/.codex/config.toml` の MCP サーバー、`~/.codex/skills/` のスキル |
| Cursor | `~/.cursor/mcp.json` の MCP サーバー |
| Gemini CLI | `~/.gemini/settings.json` の MCP サーバー |
| その他の MCP クライアント | 手動で `rust-anything mcp` を stdio サーバーとして追加（[インストール](install.md#install)を参照） |

既存のファイルを変更する前に、`setup` は同じ場所にバックアップ（`<file>.rust-anything.bak`）を保存します。

自分で各ステップを実行することもできます:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile がループ内の fuzzyScore を検出し、bridge --batch が Rust クレートを生成し、gate が 2000/2000 件の記録ケースに合格し、ベンチマークで TS より 20.7 倍高速であることを示しています。" width="820">
  <br>
  <sub>x18n の <code>fuzzyScore</code> での手順: <code>profile</code> がホットループを見つけ、<code>bridge --batch</code> がクレートを生成し、<code>gate</code> に合格し、ベンチマークで TypeScript と比較します。</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # どこがホットスポットで、誰が呼んでいるか
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs を書く（またはエージェントに書かせる）。その後:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # 記録されたすべてのケースが一致する必要がある
rust-anything bench rust/fuzzy-rs                      # 速くなったか？判定結果が出る
rust-anything gate rust/fuzzy-rs                       # これに通れば完了
```

rust-anything 自体はコードを自動翻訳しません。Rust コードを書くのはあなたのコーディングエージェント（Claude Code、Codex、Cursor など）またはあなた自身です。rust-anything が提供するのは、エージェントが推測では確実に当てられない部分です: どこに時間がかかっているか、何をどの順番で移植すべきか、既存のコードが API を変えずに Rust を呼び出すためのブリッジ、移植版が一致させるべき正確な挙動、安全に出荷する手段、そして移植の完了を判定するゲートです。

## 対応言語

| 言語 | 移植単位 | ブリッジ | 記録とリプレイの実行方法 |
| --- | --- | --- | --- |
| TypeScript/JavaScript | フォルダまたはファイル | napi-rs または wasm-bindgen（自動生成されたバイナリワイヤプロトコル使用、クラスと async にも対応） | Node で `.ts` ソースを直接実行 |
| Python | パッケージまたはモジュール | PyO3（abi3）、直接型変換、型付き例外 | Python で元のモジュールをインポート |
| C | `.c` ファイル | 元の関数名を持つ `extern "C"` 関数とヘッダ | 元のコードを共有ライブラリにコンパイルし、ctypes 経由でケースごとに fork して呼び出し |
| Go | パッケージ | 型付き cgo 呼び出しと自動生成バイナリワイヤ、元の API を持つパッケージとして提供 | 独立したモジュールとしてビルドされる自動生成 Go ドライバ |
| Ruby | モジュール関数を含むファイルまたはフォルダ | 元のモジュール配下に配置される magnus 拡張 | 自動生成 Ruby ドライバ |
| Java | `public static` メソッドのパッケージ | 型付き JNI と自動生成バイナリワイヤ、元のメソッドを持つラッパークラスとして提供 | 自動生成リフレクションドライバ |
| PHP（プレビュー） | 関数および `public static` メソッドのフォルダ | 近日対応予定: C ABI 越しの PHP FFI | 現時点でアプリ全体に対応: CLI およびサーバー（`php -S`）向けの `app record`、`app replay`、フロントドア |
| C#（プレビュー） | `public static` メソッドのフォルダ | 近日対応予定: C ABI 越しの P/Invoke | 現時点でアプリ全体に対応: CLI および ASP.NET サーバー（`dotnet <app>.dll`）向けの `app record`、`app replay`、フロントドア |

C 言語を除くすべての言語でバッチクラス（`--batch`）も生成されます。要素は一度だけ境界を越えて Rust 側に保持され、呼び出しのたびに全コアを使って全要素を一括処理します。

言語はリポジトリから自動検出されます（`go.mod`、`pom.xml`、`.csproj` または `.sln`、`pyproject.toml`、`Gemfile`、`composer.json`、`package.json` の順で判定し、該当しない場合は最も多いソースファイルから判断）。`--lang` で上書きもできます。型情報はソースから取得します: TS の型、Python の型ヒント、C/Go/Java/C# のシグネチャ、Ruby の YARD タグ、PHP の型および phpdoc。型注釈がなく数値として使われている引数は数値型と推論されます。

PHP はプレビュー版です: `analyze` で PHP アプリの移行計画を立てられ（Composer パッケージと PSR-4 オートローディング、緩い比較や整数オーバーフローによる float 化などのリスク、Composer パッケージからクレートへのマッピング）、PHP の CLI やサーバーを記録・リプレイして Rust 製フロントドアの背後に配置できます。PHP の FFI を通じたユニット単位の Rust 移植は次のステップです（[issue #2](https://github.com/erkamyaman/rust-anything/issues/2)）。

C# も同様にプレビュー版です: `analyze` は `.csproj` プロジェクトとその NuGet パッケージ（クレートにマッピング）を読み取り、`using` ディレクティブをフォルダに解決して、C# 特有の挙動の違い（`decimal`、`dynamic`、LINQ の遅延クエリ、`Task`、`lock`、リフレクション）を指摘します。C# の CLI や ASP.NET サーバーは記録・リプレイ・Rust フロントドア化が可能で、P/Invoke を用いたユニット単位の移植が次のステップです。

## 詳しく知る

- [仕組み](how-it-works.md): 移植ループ、`bridge` と `analyze` が判断すること、ホットパス、検出できないもの
- [アプリ全体の移植](whole-apps.md): サーバー、CLI、ライブラリ。フロントドアから `app lift` まで
- [例とケーススタディ](examples.md): x18n と、6 つの言語の実コード
- [実プロジェクトの移植](real-world.md): json-server、Fuse.js、vercel/ms とベンチマーク
- [インストール、更新、開発](install.md): すべてのインストール方法、Windows、ソースからのビルド
- [よくある質問](faq.md) · [ステータス](STATUS.md) · [MCP ツール](tools.md)

## スポンサー

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> は無料のオープンソースです。お役に立ちましたら、ぜひスポンサーをご検討ください。<br>
  皆様のご支援が開発を支えます。
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="企業スポンサー: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="スポンサー: Ali Yilmaz、Ashley Hunter、Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/%E3%82%B9%E3%83%9D%E3%83%B3%E3%82%B5%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="スポンサーになる"></a>
</p>

## 貢献

Issue や Pull Request を歓迎します。[CONTRIBUTING.md](../CONTRIBUTING.md) にセットアップ手順、テストの配置場所、新しい言語の追加方法が記載されています。設計上の選択とその理由は [DECISIONS.md](../DECISIONS.md) にあります。実際の移植事例（ケーススタディ）も特に歓迎しています。セキュリティ上の問題は非公開でご報告いただき（[SECURITY.md](../SECURITY.md)）、[行動規範](../CODE_OF_CONDUCT.md)をご確認ください。

## ライセンス

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman)。あなたのリポジトリ内の移植版および生成コードはすべてあなたのものです。
