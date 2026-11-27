<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="سلطعون rust-anything">
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

<div dir="rtl">

<p align="center">
أعد كتابة تطبيقك بلغة Rust، وأثبت أنه ما زال يعمل بالطريقة نفسها.
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>التوثيق:</b> <a href="https://rustanything.ai/ar">https://rustanything.ai/ar</a></p>

<p align="center"><a href="translations.md">🌐 لغات أخرى</a></p>

<p align="center"><sub>هذه الترجمة يحافظ عليها المجتمع، وقد تتأخر عن <a href="../README.md">README الإنجليزي</a>.</sub></p>

## الرعاة

<p align="center">صُنع في Türkiye 🇹🇷 بواسطة <a href="https://github.com/erkamyaman">erKam Yaman</a><br>
Rust Anything مجاني ومفتوح المصدر. إن كان يفيدك في عملك، فرعايتك تبقيه مستمرًا.</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="راعي الشركات: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="الرعاة: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="كن راعيًا"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">رعاية بصفتك شركة، أو أي أمر آخر؟ <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">تنقل قاعدة شيفرة إلى Rust؟ أقدّم استشارات لعمليات الترحيل إلى Rust: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## البدء السريع

الصق هذا في وكيل البرمجة لديك:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

أو ثبّته بنفسك:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> **للوكلاء:** اقرأ [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) لخطوات التثبيت وكل الأوامر وسير عمل النقل.

## كيف يعمل

**ما الذي يُعدّ منجزًا.** السلوك، لا اختباراتك. يشغّل `record` الأصل على مدخلات مولّدة ومعطاة، ويحفظ ما يعيده كل استدعاء وما يرميه وما يطبعه. يكتمل النقل عندما ينجح `gate`: تتطابق كل حالة مسجّلة، وتنجح جميعها: fmt وclippy و`cargo test` وفحص `unsafe` وفحص البدائل الفارغة (stub) وفحص الإجابات المسجّلة الملصوقة في النقل. وتُشغَّل اختباراتك أيضًا.

**تقرير أو ترحيل، الخيار لك.** يكتفي `analyze` و`profile` بالتقرير: خطة مرتّبة لما يجب نقله، وما هو الأكثر استهلاكًا للوقت تحت حمل عمل حقيقي. إنهما أمران عاديان، فلا يكلّف التقرير أي رموز (tokens). أما `go` فيرحّل وحدة واحدة: يبني جسرًا لأكثر دالة استهلاكًا للوقت، ويسجّلها، ويسلّم النقل إلى وكيلك، ثم يُجمّعه ويقارنه ويمرّره على `gate`، ولا يفعّله إلا عندما يتطابق كل شيء (TS/JS وPython وPowerShell؛ و`disable` يوقفه من جديد).

**كم يستغرق التشغيل.** تشغيل واحد لـ `go` ينقل وحدة واحدة، بحدّ أقصى `--rounds` من أدوار الوكيل (3 افتراضيًا): دور للنقل، ثم دور لكل diff أو gate يفشل. التطبيق كله تشغيل واحد لكل وحدة، فيمكنك التوقف بعد أيٍّ منها.

**نموذج رخيص للخطة، ونموذج قوي للنقل.** الخطة لا تحتاج إلى نموذج. للنقل، مرّر أي أمر وكيل: `rust-anything go --agent "claude --model <model>" -- <workload>`.

**الوكلاء.** يضيفه `rust-anything setup` إلى Claude Code وCodex وCursor وGemini CLI وOpenCode. ويمكن لأي عميل MCP آخر تشغيل `rust-anything mcp`، وتُثبَّت المهارة أيضًا وحدها عبر `npx skills add erkamyaman/rust-anything` (انظر [التثبيت](install.md)).

## الرخصة

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman). النقول والشيفرة المولّدة في
مستودعك ملكك.

</div>
