<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="سلطعون rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center"><sub><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a> · <a href="README_ES.md">Español</a> · <a href="README_IT.md">Italiano</a> · <a href="README_ID.md">Bahasa Indonesia</a> · العربية · <a href="README_FA.md">فارسی</a> · <a href="README_HI.md">हिन्दी</a> · <a href="README_TH.md">ไทย</a></sub></p>
<p align="center"><sub>هذه الترجمة يحافظ عليها المجتمع، وقد تتأخر عن <a href="../README.md">README الإنجليزي</a>.</sub></p>

<div dir="rtl">

<p align="center"><b>أعد كتابة تطبيقك بلغة Rust، وأثبت أنه ما زال يعمل بالطريقة نفسها.</b><br>
هو لا يترجم الشيفرة: وكيل البرمجة لديك (أو أنت) يكتب Rust، ويتحقق rust-anything منه مقابل الأصل.</p>

<p align="center"><b>التوثيق:</b> <a href="https://rust-anything.com/ar">https://rust-anything.com/ar</a></p>

</div>

<p align="center">
  <img src="https://img.shields.io/github/languages/top/erkamyaman/rust-anything?logo=rust&label=Rust&color=CE422B" alt="Rust share of the code">
  <img src="https://img.shields.io/badge/macOS%20·%20Linux%20·%20Windows-native-2563EB" alt="Native binaries for macOS, Linux and Windows">
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-555" alt="MIT license"></a>
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io version"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm version"></a>
  <br>
  <a href="https://coderabbit.link/er-kam"><img src="https://img.shields.io/coderabbit/prs/github/erkamyaman/rust-anything?labelColor=171717&color=FF570A&label=Reviewed+by+CodeRabbit" alt="Pull requests reviewed by CodeRabbit"></a>
  <a href="https://github.com/rust-unofficial/awesome-rust#development-tools"><img src="https://awesome.re/mentioned-badge.svg" alt="Mentioned in Awesome Rust"></a>
  <a href="https://github.com/AIshihzong/awesome-reverse-engineer-anything"><img src="https://img.shields.io/badge/Featured%20in-Awesome%20REA-f0b429" alt="Featured in Awesome REA"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub stars"></a>
</p>

<p align="center"><a href="https://github.com/erkamyaman/rust-anything/discussions/127"><img src="https://img.shields.io/badge/Join%20the%20discussion-6E40C9?style=for-the-badge&logo=github&logoColor=white" alt="Join the discussion"></a> <a href="https://discord.gg/FXhUEA8eBR"><img src="https://img.shields.io/badge/Join%20the%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Join the Discord"></a></p>

<p align="center"><a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a></p>

<p align="center"><b><a href="#البدء-السريع">البدء السريع</a></b> · <a href="https://github.com/erkamyaman/rust-anything/releases">Releases</a> · <a href="#البدء-السريع">Claude Code plugin</a> · <a href="https://registry.modelcontextprotocol.io/v0.1/servers?search=rust-anything">MCP Registry</a> · <a href="real-world.md">نقول حقيقية</a> · <a href="faq.md#faq">الأسئلة الشائعة</a><br>
<sub><a href="#اللغات">اللغات</a> · <a href="how-it-works.md#how-it-works">كيف يعمل</a> · <a href="whole-apps.md#port-a-whole-app">تطبيقات كاملة</a> · <a href="../LICENSE">MIT</a></sub><br>
صُنع في تركيا 🇹🇷 بواسطة <a href="https://github.com/erkamyaman">erKam Yaman</a> · <a href="../LICENSE">MIT</a> · <a href="https://github.com/sponsors/erkamyaman">❤️ ادعمني</a> · برعاية <a href="https://coderabbit.link/er-kam"><img src="assets/brands/coderabbit.png" width="22" height="22" align="absmiddle" alt=""> <b>CodeRabbit</b></a></p>

<p align="center">جديد في 2.0: <a href="real-world.md#prettier">Prettier بعد نقله إلى Rust</a>. أسرع 19 مرة · 15,861 من أصل 17,224 تشغيلًا مسجلًا تعمل بالكامل في Rust · وكلها مطابقة بايتًا ببايت</p>

<p align="center">
  <img src="assets/prettier.gif" alt="Prettier ونسخته المنقولة إلى Rust ينسقان الملفات الحقيقية نفسها (59 ملفًا)، عملية لكل ملف: يستغرق Prettier 3.63 ثانية والنسخة المنقولة 0.20 ثانية، بمخرجات مطابقة بايتًا ببايت." width="820">
  <br>
  <sub>Prettier مقابل نسخته المنقولة إلى Rust على 59 ملفًا حقيقيًا من ms وFuse.js وjson-server (Apple M4). المخرجات نفسها بايتًا ببايت. <a href="real-world.md#prettier">كيف قيس ذلك</a>.</sub>
</p>

<p align="center"><sub>PHP وC#: التسجيل والإعادة والواجهة الأمامية (front door)، مع نقل الوحدات عبر FFI وP/Invoke.</sub></p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>سجّل</b><br>
      <code>record</code> يشغّل الأصل بحالات حدّية مولّدة وفخاخ دلالية ومدخلاتك الخاصة، ويحفظ ما يعيده أو يرميه.
    </td>
    <td width="25%" valign="top">
      <b>انقل</b><br>
      <code>bridge</code> يولّد حزمة Rust وغلافًا ليحتفظ المستدعون بالواجهة الأصلية. أنت أو وكيلك يكتب Rust.
    </td>
    <td width="25%" valign="top">
      <b>تحقق</b><br>
      <code>diff</code> يعيد كل حالة مسجلة على Rust، ولا يُشحن شيء قبل أن تتطابق كلها ويجتاز <code>gate</code>.
    </td>
    <td width="25%" valign="top">
      <b>ارفع (Lift)</b><br>
      عندما تنجح الإعادة دون أي رجوع إلى الأصل وتجتاز كل وحدة gate، يحوّلها <code>app lift</code> إلى تطبيق Rust خالص. لا يُمَسّ الأصل أبدًا.
    </td>
  </tr>
</table>

**أعد كتابة تطبيقك كاملًا بلغة Rust قطعةً قطعة.** يسجّل rust-anything ما يفعله تطبيق
Node.js (TypeScript/JavaScript) أو Python أو C أو Go أو Ruby أو Java لديك اليوم، ثم يتحقق من كل جزء من
إعادة التنفيذ بلغة Rust مقابل ذلك التسجيل قبل الانتقال إلى ما بعده، إلى أن يصبح التطبيق Rust خالصًا.
أدوات سطر الأوامر وخوادم HTTP والمكتبات؛ ووكيل البرمجة لديك (أو أنت) يكتب Rust. لا يستطيع أن يلتقط إلا
ما يغطيه التسجيل: انظر [ما لا يستطيع التقاطه](how-it-works.md#what-it-cant-catch). أدوات وخوادم كاملة،
أو الجزء البطيء فقط، مسارًا ساخنًا تلو الآخر:

| مشروع حقيقي، المخرجات نفسها كالأصل | الأصل | مع rust-anything |
| --- | ---: | ---: |
| [Prettier](real-world.md#prettier) 3.6.2، ‏2,360 ملف JS/TS عبر سطر الأوامر | 128.6 ث | **6.9 ث: أسرع 19 مرة** |
| Prettier، ‏6,305 ملف Markdown عبر سطر الأوامر | 352 ث | **22.5 ث: أسرع 15.6 مرة** |
| [json-server](real-world.md#json-server)، خادم Node كامل: الذاكرة في وضع الخمول | 74 MB | **3.5 MB**، ملف تنفيذي واحد بحجم 5.3 MB |
| json-server، من البدء إلى أول استجابة | من 79 إلى 85 ms | **من 8 إلى 10 ms** |
| بحث [Fuse.js](real-world.md#fusejs) في 50,000 نص، لكل طلب | 91.2 ms | **2.8 ms: أسرع 32 مرة** |

Prettier: ‏15,861 من أصل 17,224 تشغيلًا مسجلًا تُنسَّق بالكامل في Rust، عبر 10 لغات
(JavaScript وTypeScript وJSX وJSON وGraphQL وYAML وMarkdown وCSS وSCSS وLess)، وكل الـ 17,224
تطابق Prettier بايتًا ببايت، لأن كل ما لم يُنقل بعد يعود إلى Prettier.
json-server: ‏514 من 514 حالة HTTP مسجلة تتطابق على ملف تنفيذي Rust خالص.
القياس على Apple M4، بتشغيلين لكل حالة، مع مقارنة كل مخرجات قبل أخذ الزمن
([التفاصيل](real-world.md)). استدعاء Rust لكل دالة صغيرة على حدة أبطأ في الغالب من JIT؛
يجد rust-anything الحلقة التي يستحق نقلها وينقلها.

## الجزء البطيء فقط

لا تحتاج إلى نقل التطبيق كله. وجّه `go` إلى أمر بطيء جدًا، وسينقل الدالة الواحدة التي تستهلك الوقت:

```sh
rust-anything go --agent claude -- python3 app.py
```

يقيس أداء التشغيل، ويختار أبعد دالة خارجية يستحق نقلها (الحلقة، لا الاستدعاء الصغير بداخلها، على
دفعات حيث يشير الملف التعريفي إلى ذلك)، ويسجلها بمدخلات مولّدة وبالاستدعاءات التي حدثت في تشغيلك،
ثم يسلّم `src/port.rs` إلى وكيلك (`claude` أو `codex` أو أي أمر يقرأ الموجّه من stdin؛ بدون `--agent`
يكتب `TASK.md` ويتوقف). لا يفعّل الدالة إلا بعد أن تتطابق كل حالة مسجلة ويجتاز `gate`. بعد ذلك:

```sh
rust-anything report rust/dedupe-rs     # REPORT.md وcard.svg لطلب السحب
rust-anything disable rust/dedupe-rs    # العودة إلى الأصل، بايتًا ببايت
```

صف [Fuse.js](real-world.md#fusejs) أعلاه له هذا الشكل بالضبط: نُقلت درجة Fuse لنص واحد، على دفعات عبر
القائمة، إلى Rust، بينما بقيت مفاتيح الكائنات والبحث المتقدم وغيرها في JavaScript، من 91.2 ms إلى 2.8 ms
لكل طلب على 50,000 نص. جرّب `go` على
[`tests/fixtures/go-fuzzy`](../tests/fixtures/go-fuzzy)، وهو برنامج Python من 20 سطرًا لإزالة التكرار. كل الطرق
لتفعيل النقل وتعطيله: [Switch a port on and off](how-it-works.md#switch-a-port-on-and-off).

<p align="center">
  <img src="assets/json-server.gif" alt="json-server كخادم Node وكملف تنفيذي Rust واحد: الذاكرة في وضع الخمول 74 MB و3.5 MB، ومن البدء إلى أول استجابة 79 إلى 85 ms و8 إلى 10 ms، و514 من 514 حالة HTTP مسجلة تتطابق." width="820">
  <br>
  <sub>json-server، خادم Node كامل، مرفوع إلى ملف تنفيذي Rust خالص (Apple M4). <a href="real-world.md#json-server">كيف قيس ذلك</a>.</sub>
</p>

> rust-anything مشروع مستقل، وليس مرتبطًا بمشروع Rust ولا بمؤسسة Rust Foundation ولا مدعومًا منهما.
> "Rust" علامة تجارية لمؤسسة Rust Foundation.

> [!WARNING]
> يشغّل `record` و`app record` شيفرتك الأصلية ويستدعيان كل دالة (أو التطبيق كله) بمدخلات مولّدة
> كثيرة، ويشغّل `app record` كل حالة مرتين. ويعيد `diff` و`gate` و`app replay` تشغيل الحالات نفسها،
> على النسخة المنقولة أو على الأصل. الشيفرة التي تكتب ملفات أو تتحدث إلى قاعدة بيانات أو تستدعي الشبكة
> ستفعل ذلك مرة لكل حالة. شغّلها على نسخة من المشروع، مع قاعدة بيانات اختبار وبدون
> بيانات اعتماد الإنتاج في البيئة.

## البدء السريع

الصق هذا في Claude Code أو Codex أو Cursor أو Gemini CLI أو OpenCode ودع وكيلك يثبّته:

```text
ثبّت rust-anything: شغّل `cargo install rust-anything` ثم `rust-anything setup`.
أخبرني متى يجب أن أعيد تشغيلك.
```

أو بنفسك:

```sh
cargo install rust-anything   # من crates.io
rust-anything setup           # يسجله في Claude Code وCodex وCursor وGemini CLI وOpenCode
```

لا تملك سلسلة أدوات Rust بعد؟ الملفات التنفيذية الجاهزة لـ macOS وLinux وWindows مرفقة بكل
[إصدار على GitHub](https://github.com/erkamyaman/rust-anything/releases)، ولمشاريع JavaScript
يثبّت `npm install -g rust-anything` الملف التنفيذي نفسه.

في Claude Code يمكنك أيضًا تثبيته كإضافة (خادم MCP والمهارة؛ يجب أن يكون سطر الأوامر أعلاه
في `PATH`):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

يضيف `setup` خادم MCP إلى كل وكيل يجده، ويثبّت مهارة سير العمل حيث يقرأ الوكيل المهارات. تبقى
بقية إعداداتك كما هي، ولا ضرر من إعادة تشغيله، ويتراجع عنه `--remove`. بعد ذلك اطلب من وكيلك مثلًا:

- *"أعد كتابة سطر الأوامر هذا بالكامل في Rust باستخدام rust-anything وأثبت أنه يتصرف بالطريقة نفسها."*
- *"هذا البحث بطيء. قس الأداء باستخدام rust-anything وانقل الحلقة الساخنة إلى Rust."*
- *"خطط بـ rust-anything لترحيل `src/parser/` إلى Rust؛ انقل الوحدة الأولى وتحقق منها."*
- *"قارن أداء النسخة المنقولة إلى Rust في `rust/text-rs` بالأصل وأخبرني هل يمكننا شحنها."*

### يعمل مع

| الوكيل | ما يضيفه `rust-anything setup` |
| --- | --- |
| Claude Code | خادم MCP في `~/.claude.json`، والمهارة في `~/.claude/skills/` |
| Codex | خادم MCP في `~/.codex/config.toml`، والمهارة في `~/.codex/skills/` |
| Cursor | خادم MCP في `~/.cursor/mcp.json` |
| Gemini CLI | خادم MCP في `~/.gemini/settings.json` |
| OpenCode | خادم MCP في `~/.config/opencode/opencode.json` (`mcp.servers`)، والمهارة في `~/.config/opencode/skills/` |
| أي عميل MCP آخر | أضف `rust-anything mcp` يدويًا كخادم stdio (انظر [التثبيت](install.md#install)) |

قبل تعديل أي ملف موجود، ينشئ `setup` نسخة بجانبه (`<file>.rust-anything.bak`).

لإعداد OpenCode يدويًا، أضف الخادم إلى `~/.config/opencode/opencode.json` (`command` هو
argv كاملًا) وانسخ [`skills/rust-anything/`](../skills/rust-anything) إلى
`~/.config/opencode/skills/rust-anything/`:

```json
{
  "mcp": {
    "servers": {
      "rust-anything": { "type": "local", "command": ["rust-anything", "mcp"] }
    }
  }
}
```

أو نفّذ الخطوات بنفسك:

```sh
rust-anything profile . -- node bench.mjs              # ما الذي يسخن ومن يستدعيه
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# اكتب src/port.rs (أو دع الوكيل يكتبه)، ثم:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # يجب أن تتطابق كل حالة مسجلة
rust-anything bench rust/fuzzy-rs                      # هل هو أسرع؟ الحكم يخبرك
rust-anything gate rust/fuzzy-rs                       # النجاح يعني: جاهز
```

لا يترجم rust-anything الشيفرة: وكيل البرمجة لديك (Claude Code أو Codex أو Cursor أو OpenCode، …) أو
أنت من يكتب Rust. ما يقدمه rust-anything هو ما لا يستطيع الوكيل تخمينه بموثوقية:
أين يذهب الوقت، وماذا ينقل أولًا وبأي ترتيب، وجسرًا ليستدعي بقية الشيفرة Rust بالواجهة
نفسها دون تغيير، والسلوك الدقيق الذي يجب أن يطابقه النقل، وطريقة آمنة للشحن، وبوابة تبيّن متى
يكتمل النقل.

تطابق المخرجات وحده لا يكفي. سجّل أيضًا ما يرسله التطبيق إلى الخارج، وستفشل أي إعادة تنفيذ ترسل بريدًا إلكترونيًا مرتين:

<p align="center">
  <img src="assets/effects.gif" alt="يسجل app record مع --effects smtp رسالة ترحيب واحدة لكل تسجيل؛ وapp replay لإعادة تنفيذ تطبع المخرجات نفسها لكنها ترسل البريد مرتين يبلغ عن 0/2 حالة ناجحة، مع فشل في التأثيرات، ويعرض التقرير الرسالة الزائدة." width="820">
  <br>
  <sub>تشغيل حقيقي لـ <code>app record --effects</code> و<code>app replay</code>. انظر <a href="whole-apps.md#side-effects-email-webhooks-and-databases">التأثيرات الجانبية</a>.</sub>
</p>

## اللغات

| اللغة | الوحدة | الجسر (Bridge) | يُسجَّل ويُعاد بواسطة |
| --- | --- | --- | --- |
| TypeScript/JavaScript | مجلد أو ملف | napi-rs أو wasm-bindgen مع صيغة تبادل ثنائية مولّدة؛ تشمل الفئات وasync | Node، يشغّل مصادر `.ts` مباشرة |
| Python | حزمة أو وحدة | PyO3 (abi3)، تحويل مباشر، استثناءات مصنّفة | Python، يستورد الأصل |
| C | ملف `.c` | دوال `extern "C"` بالأسماء الأصلية مع ملف رأس | الأصل مُجمّعًا كمكتبة مشتركة ويُستدعى عبر ctypes، مع fork لكل حالة |
| Go | حزمة | استدعاءات cgo مصنّفة وصيغة تبادل ثنائية مولّدة في حزمة بالواجهة الأصلية | مشغّل Go مولّد، يُبنى كوحدة مستقلة |
| Ruby | ملف أو مجلد بدوال وحدات | امتداد magnus تحت الوحدات الأصلية | مشغّل Ruby مولّد |
| Java | حزمة بطرق `public static` | JNI مصنّف وصيغة تبادل ثنائية مولّدة في فئات غلاف بالطرق الأصلية | مشغّل انعكاس (reflection) مولّد |
| Kotlin | وحدة Gradle بدوال المستوى الأعلى وطرق `@JvmStatic` | جسر Java: JNI مصنّف في فئات غلاف Java تستدعي الدوال الساكنة المُجمّعة من Kotlin | مشغّل انعكاس مولّد، مع تجميع مصادر Kotlin عبر `kotlinc` |
| PHP | ملف أو مجلد بدوال المستوى الأعلى (تبقى طرق `public static` في PHP) | موزّع JSON عبر `ext-ffi`، في ملف يعيد التصريح بالدوال بمعاملاتها الأصلية | مشغّل PHP مولّد؛ تطبيقات كاملة عبر `app record` و`app replay` والواجهة الأمامية، لأدوات سطر الأوامر والخوادم (`php -S`) |
| PowerShell | وحدة `.psm1` أو سكربت `.ps1` بدوال وكتل `param()` (مُصدَّرة عبر `Export-ModuleMember`) | موزّع JSON عبر P/Invoke، يُصرَّح به عند التحميل بـ Reflection.Emit، في ملف `.ps1` يعيد التصريح بكل دالة بكتلة `param()` الأصلية | مشغّل `pwsh` مولّد (PowerShell 7.4+) |
| C# | ملف أو مجلد بطرق `public static` | موزّع JSON عبر P/Invoke، في فئات `<namespace>.Rust` بالطرق الأصلية | مشغّل انعكاس مولّد، يُجمَّع مع مصادر المشروع عبر `dotnet build`؛ تطبيقات كاملة عبر `app record` و`app replay` والواجهة الأمامية، لأدوات سطر الأوامر وخوادم ASP.NET (`dotnet <app>.dll`) |

تحصل كل لغة عدا C وPHP وC# وPowerShell أيضًا على فئات الدفعات (`--batch`): تعبر العناصر الحدّ
مرة واحدة وتبقى في Rust، وكل استدعاء يعالجها جميعًا على كل الأنوية.

تُكتشف اللغة من المستودع (`go.mod` أو `pom.xml` أو `.csproj` أو `.sln` أو
`pyproject.toml` أو `Gemfile` أو `composer.json` أو `package.json`، وإلا فأكثر ملفات المصدر شيوعًا)؛
ويتجاوز `--lang` ذلك. تُؤخذ الأنواع من المصدر: أنواع TS وتلميحات أنواع Python وتواقيع C وGo
وJava وC# ووسوم YARD في Ruby وأنواع PHP وphpdoc؛ أما المعاملات غير المصنّفة التي تُستخدم كأرقام
فتُستنتج أرقامًا.

بالنسبة إلى PHP، يخطط `analyze` لتطبيق PHP (حزم Composer والتحميل التلقائي PSR-4، ومخاطر مثل
المقارنات الفضفاضة وفيضان الأعداد الصحيحة إلى أعداد عشرية، وربط حزم Composer بالحزم المقابلة في Rust)،
ويمكن تسجيل أداة سطر أوامر أو خادم PHP وإعادته ووضعه خلف واجهة أمامية بلغة Rust. دوال المستوى الأعلى
في الوحدة، بمعاملات عددية ونصية وقوائم ومصفوفات ترابطية، تعمل عبر امتداد FFI في PHP (`php` مع
`ext-ffi`، المفعّل افتراضيًا في سطر الأوامر): ضمّن `php/<crate>.php` المولّد بدل الملف الأصلي بعد
نجاح `diff`. كل استدعاء يعبر الحدّ بصيغة JSON، بنحو 1 µs، ولهذا يُظهر `bench` الدوال الصغيرة أبطأ
من PHP؛ ولا يوجد `shadow` لـ PHP بعد
([المسألة #200](https://github.com/erkamyaman/rust-anything/issues/200)).

يعمل C# بالطريقة نفسها: يقرأ `analyze` مشاريع `.csproj` وحزم NuGet الخاصة بها
(مربوطة بحزم Rust)، ويحلّ تعليمات `using` إلى مجلدات، وينبّه إلى خصوصيات C#
(`decimal` و`dynamic` واستعلامات LINQ المؤجلة و`Task` و`lock` والانعكاس). يمكن تسجيل أداة سطر أوامر C# أو
خادم ASP.NET وإعادته ووضعه خلف واجهة أمامية بلغة Rust. الطرق الساكنة العامة في الوحدة، بمعاملات عددية
ونصية ومصفوفات وقوائم وقواميس وسجلات (record) أو فئات بسيطة، تعمل عبر P/Invoke (`dotnet` في
`PATH` للتسجيل والمقارنة): استدعِ فئات `<namespace>.Rust` المولّدة أو أعطها اسمًا مستعارًا عبر
`using` بعد نجاح `diff`. كل استدعاء يعبر الحدّ بصيغة JSON، بنحو
1-3 µs، ولهذا يُظهر `bench` الطرق الصغيرة أبطأ من .NET؛ ولا يوجد `shadow` لـ C# بعد
([المسألة #200](https://github.com/erkamyaman/rust-anything/issues/200)).

## اعرف المزيد

- [كيف يعمل](how-it-works.md): الحلقة، وما يقرره `bridge` و`analyze`، والمسارات الساخنة، وما لا يمكنه التقاطه، وما يتطلبه النقل الحقيقي، وكيف تبقى متزامنًا مع الأصل (`map`)
- [نقل تطبيق كامل](whole-apps.md): الخوادم وأدوات سطر الأوامر والمكتبات، من الواجهة الأمامية إلى `app lift`
- [أمثلة ودراسات حالة](examples.md): x18n وشيفرة حقيقية بست لغات
- [نقول حقيقية](real-world.md): json-server وFuse.js وvercel/ms، مع قياسات الأداء
- [التثبيت والتحديث والتطوير](install.md): كل طرق التثبيت، وWindows، والبناء من المصدر
- [الأسئلة الشائعة](faq.md) · [الحالة](STATUS.md) · [أدوات MCP](tools.md)

## الرعاة

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> مجاني ومفتوح المصدر. إن كان يساعدك في عملك، ففكّر في رعايته.<br>
  دعمك يُبقي التطوير مستمرًا.
</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="راعي الشركات: CodeRabbit">
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

## المساهمة

المسائل (issues) وطلبات السحب مرحّب بها؛ يشرح [CONTRIBUTING.md](../CONTRIBUTING.md) الإعداد،
وأين تُوضع الاختبارات، وكيف تضيف لغة. قرارات التصميم وأسبابها في
[DECISIONS.md](../DECISIONS.md). دراسات حالة النقول الحقيقية مرحّب بها بشكل خاص. أبلغ عن
المشكلات الأمنية بسرّية ([SECURITY.md](../SECURITY.md))، والتزم
[بمدونة السلوك](../CODE_OF_CONDUCT.md).

## الرخصة

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). النقول والشيفرة المولّدة في
مستودعك ملكك.
