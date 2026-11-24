<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything का केकड़ा">
</p>

<h1 align="center">rust-anything</h1>

<p align="center"><sub><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a> · <a href="README_ES.md">Español</a> · <a href="README_IT.md">Italiano</a> · <a href="README_ID.md">Bahasa Indonesia</a> · <a href="README_AR.md">العربية</a> · <a href="README_FA.md">فارسی</a> · हिन्दी</sub></p>
<p align="center"><sub>इस अनुवाद का रखरखाव समुदाय करता है, इसलिए यह <a href="../README.md">अंग्रेज़ी README</a> से पीछे हो सकता है।</sub></p>

<p align="center"><b>अपने ऐप को Rust में दोबारा लिखें, और साबित करें कि वह अब भी ठीक वैसे ही चलता है।</b><br>
यह कोड का अनुवाद नहीं करता: Rust आपका कोडिंग एजेंट (या आप) लिखते हैं, और rust-anything उसे मूल कोड से मिलाकर जाँचता है।</p>

<p align="center"><b>दस्तावेज़:</b> <a href="https://rust-anything.com/hi">https://rust-anything.com/hi</a></p>

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

<p align="center"><b><a href="#त्वरित-शुरुआत">त्वरित शुरुआत</a></b> · <a href="https://github.com/erkamyaman/rust-anything/releases">Releases</a> · <a href="#त्वरित-शुरुआत">Claude Code plugin</a> · <a href="https://registry.modelcontextprotocol.io/v0.1/servers?search=rust-anything">MCP Registry</a> · <a href="real-world.md">असली पोर्ट</a> · <a href="faq.md#faq">अक्सर पूछे जाने वाले प्रश्न</a><br>
<sub><a href="#भाषाएँ">भाषाएँ</a> · <a href="how-it-works.md#how-it-works">यह कैसे काम करता है</a> · <a href="whole-apps.md#port-a-whole-app">पूरे ऐप</a> · <a href="../LICENSE">MIT</a></sub><br>
तुर्की 🇹🇷 में <a href="https://github.com/erkamyaman">erKam Yaman</a> द्वारा बनाया गया · <a href="../LICENSE">MIT</a> · <a href="https://github.com/sponsors/erkamyaman">❤️ मुझे स्पॉन्सर करें</a> · प्रायोजक: <a href="https://coderabbit.link/er-kam"><img src="assets/brands/coderabbit.png" width="22" height="22" align="absmiddle" alt=""> <b>CodeRabbit</b></a></p>

<p align="center">2.0 में नया: <a href="real-world.md#prettier">Rust में पोर्ट किया गया Prettier</a>। 19 गुना तेज़ · 17,224 रिकॉर्ड की गई रन में से 15,861 पूरी तरह Rust में · सभी बाइट-दर-बाइट एक जैसी</p>

<p align="center">
  <img src="assets/prettier.gif" alt="Prettier और उसका Rust पोर्ट वही 59 असली फ़ाइलें फ़ॉर्मैट करते हैं, हर फ़ाइल के लिए एक प्रोसेस: Prettier को 3.63 सेकंड लगते हैं और Rust पोर्ट को 0.20 सेकंड, आउटपुट बाइट-दर-बाइट एक जैसा।" width="820">
  <br>
  <sub>ms, Fuse.js और json-server की 59 असली फ़ाइलों पर Prettier बनाम उसका Rust पोर्ट (Apple M4)। आउटपुट बाइट-दर-बाइट एक जैसा। <a href="real-world.md#prettier">नापा कैसे गया</a>।</sub>
</p>

<p align="center"><sub>PHP और C#: रिकॉर्ड, रीप्ले और front door, साथ में FFI और P/Invoke के ज़रिए यूनिट पोर्ट।</sub></p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>रिकॉर्ड</b><br>
      <code>record</code> मूल कोड को जनरेट किए गए किनारे के मामलों, अर्थ-संबंधी जालों और आपके अपने इनपुट के साथ चलाता है, और जो वह लौटाता या फेंकता है उसे सहेज लेता है।
    </td>
    <td width="25%" valign="top">
      <b>पोर्ट</b><br>
      <code>bridge</code> Rust क्रेट और एक रैपर बनाता है ताकि कॉल करने वाले मूल API ही इस्तेमाल करते रहें। Rust आप या आपका एजेंट लिखता है।
    </td>
    <td width="25%" valign="top">
      <b>जाँच</b><br>
      <code>diff</code> हर रिकॉर्ड किए गए मामले को Rust पर दोबारा चलाता है, और जब तक सब मेल न खाएँ और <code>gate</code> पास न हो, कुछ भी शिप नहीं होता।
    </td>
    <td width="25%" valign="top">
      <b>लिफ़्ट</b><br>
      जब रीप्ले बिना फ़ॉलबैक के पास हो और हर यूनिट gate पास कर ले, तो <code>app lift</code> उसे शुद्ध Rust ऐप में बदल देता है। मूल कोड को कभी छुआ नहीं जाता।
    </td>
  </tr>
</table>

**अपना पूरा ऐप टुकड़े-टुकड़े करके Rust में दोबारा लिखें।** rust-anything रिकॉर्ड करता है कि आपका
Node.js (TypeScript/JavaScript), Python, C, Go, Ruby या Java ऐप आज क्या करता है, फिर आगे बढ़ने से पहले
Rust पुनर्लेखन के हर हिस्से को उसी रिकॉर्डिंग से जाँचता है, जब तक ऐप शुद्ध Rust न हो जाए। CLI, HTTP सर्वर
और लाइब्रेरी; Rust आपका कोडिंग एजेंट (या आप) लिखते हैं। यह केवल वही पकड़ सकता है जो रिकॉर्डिंग में
आया हो: देखें [यह क्या नहीं पकड़ सकता](how-it-works.md#what-it-cant-catch)। पूरे टूल और सर्वर, या सिर्फ़ धीमा हिस्सा,
एक-एक हॉट पाथ करके:

| असली प्रोजेक्ट, मूल जैसा ही आउटपुट | मूल | rust-anything के साथ |
| --- | ---: | ---: |
| [Prettier](real-world.md#prettier) 3.6.2, कमांड लाइन से 2,360 JS/TS फ़ाइलें | 128.6 से. | **6.9 से.: 19 गुना तेज़** |
| Prettier, कमांड लाइन से 6,305 Markdown फ़ाइलें | 352 से. | **22.5 से.: 15.6 गुना तेज़** |
| [json-server](real-world.md#json-server), पूरा Node सर्वर: निष्क्रिय मेमोरी | 74 MB | **3.5 MB**, एक ही 5.3 MB बाइनरी |
| json-server, शुरू से पहले जवाब तक | 79 से 85 ms | **8 से 10 ms** |
| [Fuse.js](real-world.md#fusejs) खोज, 50,000 स्ट्रिंग पर, प्रति अनुरोध | 91.2 ms | **2.8 ms: 32 गुना तेज़** |

Prettier: 17,224 रिकॉर्ड की गई रन में से 15,861 पूरी तरह Rust में फ़ॉर्मैट होती हैं, 10 भाषाओं में
(JavaScript, TypeScript, JSX, JSON, GraphQL, YAML, Markdown, CSS, SCSS, Less), और सभी 17,224
Prettier से बाइट-दर-बाइट मेल खाती हैं, क्योंकि जो अभी पोर्ट नहीं हुआ वह Prettier पर लौट जाता है।
json-server: 514/514 रिकॉर्ड किए गए HTTP मामले शुद्ध Rust बाइनरी पर मेल खाते हैं।
Apple M4 पर नापा गया, हर एक की दो रन, और समय नापने से पहले हर आउटपुट की तुलना की गई
([विवरण](real-world.md))। हर छोटे फ़ंक्शन के लिए अलग से Rust को बुलाना अक्सर JIT से *धीमा* होता है;
rust-anything वह लूप ढूँढता है जिसे ले जाना फ़ायदेमंद है और वही ले जाता है।

## सिर्फ़ धीमा हिस्सा

आपको पूरा ऐप पोर्ट करने की ज़रूरत नहीं। `go` को ऐसे कमांड पर चलाएँ जो बहुत धीमा है, और वह समय खाने वाले
उस एक फ़ंक्शन को पोर्ट करेगा:

```sh
rust-anything go --agent claude -- python3 app.py
```

यह रन की प्रोफ़ाइलिंग करता है, सबसे बाहरी वह फ़ंक्शन चुनता है जिसे ले जाना फ़ायदेमंद है (लूप, उसके अंदर का
छोटा कॉल नहीं, जहाँ प्रोफ़ाइल सुझाए वहाँ बैच में), उसे जनरेट किए गए इनपुट और आपकी रन के कॉल के साथ
रिकॉर्ड करता है, और `src/port.rs` आपके एजेंट को सौंपता है (`claude`, `codex`, या कोई भी कमांड जो
stdin से प्रॉम्प्ट पढ़े; `--agent` के बिना यह आपके लिए `TASK.md` लिखकर रुक जाता है)। फ़ंक्शन तब तक चालू
नहीं होता जब तक हर रिकॉर्ड किया गया मामला मेल न खाए और `gate` पास न हो। उसके बाद:

```sh
rust-anything report rust/dedupe-rs     # pull request के लिए REPORT.md और card.svg
rust-anything disable rust/dedupe-rs    # मूल पर वापसी, बाइट-दर-बाइट
```

ऊपर की [Fuse.js](real-world.md#fusejs) पंक्ति ठीक इसी रूप की है: एक स्ट्रिंग के लिए Fuse का स्कोर, सूची पर बैच में,
Rust में ले जाया गया, जबकि ऑब्जेक्ट कीज़, एक्सटेंडेड सर्च और बाकी सब JavaScript में रहा, 50,000 स्ट्रिंग पर
प्रति अनुरोध 91.2 ms से 2.8 ms। `go` को
[`tests/fixtures/go-fuzzy`](../tests/fixtures/go-fuzzy) पर आज़माएँ, जो 20 पंक्तियों का Python डीडुप है। किसी पोर्ट को चालू और बंद करने के सभी तरीके:
[Switch a port on and off](how-it-works.md#switch-a-port-on-and-off)।

<p align="center">
  <img src="assets/json-server.gif" alt="json-server Node सर्वर के रूप में और एक ही Rust बाइनरी के रूप में: निष्क्रिय मेमोरी 74 MB और 3.5 MB, शुरू से पहले जवाब तक 79 से 85 ms और 8 से 10 ms, और 514/514 रिकॉर्ड किए गए HTTP मामले मेल खाते हैं।" width="820">
  <br>
  <sub>json-server, पूरा Node सर्वर, शुद्ध Rust बाइनरी में लिफ़्ट किया गया (Apple M4)। <a href="real-world.md#json-server">नापा कैसे गया</a>।</sub>
</p>

> rust-anything एक स्वतंत्र प्रोजेक्ट है, जो Rust Project या Rust Foundation से जुड़ा नहीं है और न ही उनके द्वारा
> समर्थित है। "Rust" Rust Foundation का ट्रेडमार्क है।

> [!WARNING]
> `record` और `app record` आपका मूल कोड चलाते हैं और हर फ़ंक्शन (या पूरे ऐप) को ढेरों जनरेट किए गए
> इनपुट के साथ बुलाते हैं, और `app record` हर मामले को दो बार चलाता है। `diff`, `gate` और
> `app replay` वही मामले दोबारा चलाते हैं, पोर्ट पर या मूल पर। जो कोड फ़ाइलें लिखता है, डेटाबेस से बात करता है
> या नेटवर्क को कॉल करता है, वह हर मामले में एक बार ऐसा करेगा। इन्हें प्रोजेक्ट की कॉपी पर चलाएँ, टेस्ट डेटाबेस के साथ
> और वातावरण में प्रोडक्शन क्रेडेंशियल के बिना।

## त्वरित शुरुआत

इसे Claude Code, Codex, Cursor, Gemini CLI या OpenCode में चिपकाएँ और अपने एजेंट से इंस्टॉल करवाएँ:

```text
rust-anything इंस्टॉल करो: `cargo install rust-anything` चलाओ, फिर `rust-anything setup`।
बताओ कि मुझे तुम्हें कब रीस्टार्ट करना है।
```

या खुद:

```sh
cargo install rust-anything   # crates.io से
rust-anything setup           # इसे Claude Code, Codex, Cursor, Gemini CLI और OpenCode में दर्ज करता है
```

अभी Rust टूलचेन नहीं है? macOS, Linux और Windows के पहले से बने बाइनरी हर
[GitHub रिलीज़](https://github.com/erkamyaman/rust-anything/releases) के साथ जुड़े होते हैं, और JavaScript प्रोजेक्ट के लिए
`npm install -g rust-anything` वही बाइनरी इंस्टॉल करता है।

Claude Code में आप इसे प्लगइन के रूप में भी इंस्टॉल कर सकते हैं (MCP सर्वर और स्किल; ऊपर वाला CLI
आपके `PATH` में होना चाहिए):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` हर मिले हुए एजेंट में MCP सर्वर दर्ज करता है और वर्कफ़्लो स्किल वहाँ इंस्टॉल करता है जहाँ एजेंट स्किल पढ़ता है।
आपकी बाकी सेटिंग्स बनी रहती हैं, इसे दोबारा चलाना सुरक्षित है, और `--remove` इसे पलट देता है। फिर अपने एजेंट से कहें, जैसे:

- *"इस CLI को rust-anything से पूरी तरह Rust में दोबारा लिखो और साबित करो कि यह बिल्कुल वैसे ही चलता है।"*
- *"यह खोज धीमी लगती है। rust-anything से प्रोफ़ाइल करो और हॉट लूप को Rust में ले जाओ।"*
- *"rust-anything से `src/parser/` को Rust में माइग्रेट करने की योजना बनाओ; पहली यूनिट पोर्ट करके जाँचो।"*
- *"`rust/text-rs` में Rust पोर्ट का मूल से बेंचमार्क करो और बताओ कि क्या हम इसे शिप कर सकते हैं।"*

### इनके साथ काम करता है

| एजेंट | `rust-anything setup` क्या जोड़ता है |
| --- | --- |
| Claude Code | `~/.claude.json` में MCP सर्वर, `~/.claude/skills/` में स्किल |
| Codex | `~/.codex/config.toml` में MCP सर्वर, `~/.codex/skills/` में स्किल |
| Cursor | `~/.cursor/mcp.json` में MCP सर्वर |
| Gemini CLI | `~/.gemini/settings.json` में MCP सर्वर |
| OpenCode | `~/.config/opencode/opencode.json` (`mcp.servers`) में MCP सर्वर, `~/.config/opencode/skills/` में स्किल |
| कोई भी दूसरा MCP क्लाइंट | `rust-anything mcp` को हाथ से stdio सर्वर के रूप में जोड़ें (देखें [इंस्टॉल](install.md#install)) |

किसी मौजूदा फ़ाइल को बदलने से पहले `setup` उसके बगल में एक कॉपी रख देता है (`<file>.rust-anything.bak`)।

OpenCode को हाथ से सेट करने के लिए, सर्वर को `~/.config/opencode/opencode.json` में जोड़ें (`command`
पूरा argv है) और [`skills/rust-anything/`](../skills/rust-anything) को
`~/.config/opencode/skills/rust-anything/` में कॉपी करें:

```json
{
  "mcp": {
    "servers": {
      "rust-anything": { "type": "local", "command": ["rust-anything", "mcp"] }
    }
  }
}
```

या कदम खुद चलाएँ:

```sh
rust-anything profile . -- node bench.mjs              # क्या गर्म है और उसे कौन बुलाता है
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs लिखें (या एजेंट से लिखवाएँ), फिर:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # हर रिकॉर्ड किया गया मामला मेल खाना चाहिए
rust-anything bench rust/fuzzy-rs                      # क्या यह तेज़ है? फ़ैसला बता देगा
rust-anything gate rust/fuzzy-rs                       # पास का मतलब है: तैयार
```

rust-anything कोड का अनुवाद नहीं करता: Rust आपका कोडिंग एजेंट (Claude Code, Codex, Cursor, OpenCode, …) या
आप लिखते हैं। rust-anything वही देता है जो एजेंट भरोसे के साथ अंदाज़ा नहीं लगा सकता:
समय कहाँ जाता है, पहले क्या और किस क्रम में पोर्ट करना है, एक ब्रिज ताकि बाकी कोड Rust को
वही API रखकर बुला सके, वह ठीक-ठीक व्यवहार जिससे पोर्ट मेल खाना चाहिए, शिप करने का एक सुरक्षित रास्ता, और एक gate जो बताता है कि
पोर्ट कब पूरा हुआ।

एक जैसा आउटपुट काफ़ी नहीं है। यह भी रिकॉर्ड करें कि ऐप बाहर क्या भेजता है, और जो पुनर्लेखन दो बार ईमेल भेजता है वह फ़ेल हो जाएगा:

<p align="center">
  <img src="assets/effects.gif" alt="--effects smtp के साथ app record हर साइन-अप पर एक वेलकम ईमेल रिकॉर्ड करता है; ऐसे पुनर्लेखन पर app replay जो वही आउटपुट छापता है पर दो बार मेल भेजता है, 0/2 पास मामले बताता है, प्रभावों पर फ़ेल, और रिपोर्ट अतिरिक्त ईमेल दिखाती है।" width="820">
  <br>
  <sub><code>app record --effects</code> और <code>app replay</code> की एक असली रन। देखें <a href="whole-apps.md#side-effects-email-webhooks-and-databases">साइड इफ़ेक्ट</a>।</sub>
</p>

## भाषाएँ

| भाषा | यूनिट | ब्रिज | इसके द्वारा रिकॉर्ड और रीप्ले |
| --- | --- | --- | --- |
| TypeScript/JavaScript | फ़ोल्डर या फ़ाइल | napi-rs या wasm-bindgen, जनरेट किए गए बाइनरी वायर फ़ॉर्मैट के साथ; क्लास और async शामिल | Node, जो `.ts` सोर्स सीधे चलाता है |
| Python | पैकेज या मॉड्यूल | PyO3 (abi3), सीधा रूपांतरण, टाइप्ड अपवाद | Python, जो मूल को import करता है |
| C | `.c` फ़ाइल | मूल नामों और एक हेडर के साथ `extern "C"` फ़ंक्शन | मूल, शेयर्ड लाइब्रेरी के रूप में कंपाइल होकर ctypes से बुलाया जाता है, हर मामले के लिए एक fork |
| Go | पैकेज | टाइप्ड cgo कॉल और मूल API वाले पैकेज में जनरेट किया गया बाइनरी वायर फ़ॉर्मैट | एक जनरेट किया गया Go ड्राइवर, जो अलग मॉड्यूल के रूप में बनता है |
| Ruby | मॉड्यूल फ़ंक्शन वाली फ़ाइल या फ़ोल्डर | मूल मॉड्यूल के नीचे एक magnus एक्सटेंशन | एक जनरेट किया गया Ruby ड्राइवर |
| Java | `public static` मेथड वाला पैकेज | टाइप्ड JNI और मूल मेथड वाली रैपर क्लास में जनरेट किया गया बाइनरी वायर फ़ॉर्मैट | एक जनरेट किया गया रिफ़्लेक्शन ड्राइवर |
| Kotlin | टॉप-लेवल फ़ंक्शन और `@JvmStatic` मेथड वाला Gradle मॉड्यूल | Java ब्रिज: Java रैपर क्लास में टाइप्ड JNI, जो कंपाइल किए गए Kotlin स्टैटिक को बुलाती हैं | एक जनरेट किया गया रिफ़्लेक्शन ड्राइवर, Kotlin सोर्स `kotlinc` से कंपाइल होकर |
| PHP | टॉप-लेवल फ़ंक्शन वाली फ़ाइल या फ़ोल्डर (`public static` मेथड PHP में रहते हैं) | `ext-ffi` के ज़रिए एक JSON डिस्पैचर, एक ऐसी फ़ाइल में जो फ़ंक्शनों को उनके मूल पैरामीटर के साथ दोबारा घोषित करती है | एक जनरेट किया गया PHP ड्राइवर; `app record`, `app replay` और front door से पूरे ऐप, CLI और सर्वर (`php -S`) के लिए |
| PowerShell | फ़ंक्शन और `param()` ब्लॉक वाला `.psm1` मॉड्यूल या `.ps1` स्क्रिप्ट (`Export-ModuleMember` से निर्यात) | P/Invoke के ज़रिए एक JSON डिस्पैचर, लोड होने पर Reflection.Emit से घोषित, एक `.ps1` में जो हर फ़ंक्शन को उसके मूल `param()` ब्लॉक के साथ दोबारा घोषित करती है | एक जनरेट किया गया `pwsh` ड्राइवर (PowerShell 7.4+) |
| C# | `public static` मेथड वाली फ़ाइल या फ़ोल्डर | P/Invoke के ज़रिए एक JSON डिस्पैचर, मूल मेथड वाली `<namespace>.Rust` क्लास में | एक जनरेट किया गया रिफ़्लेक्शन ड्राइवर, प्रोजेक्ट के सोर्स के साथ `dotnet build` से कंपाइल; `app record`, `app replay` और front door से पूरे ऐप, CLI और ASP.NET सर्वर (`dotnet <app>.dll`) के लिए |

C, PHP, C# और PowerShell को छोड़कर हर भाषा को बैच क्लास (`--batch`) भी मिलती हैं: तत्व सीमा को एक बार
पार करते हैं और Rust में रहते हैं, और हर कॉल उन सबको सभी कोर पर प्रोसेस करती है।

भाषा रिपॉज़िटरी से पहचानी जाती है (`go.mod`, `pom.xml`, `.csproj` या `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, वरना सबसे आम सोर्स फ़ाइलें);
`--lang` इसे बदल देता है। टाइप सोर्स से लिए जाते हैं: TS टाइप, Python टाइप हिंट, C, Go,
Java और C# सिग्नेचर, Ruby YARD टैग, PHP टाइप और phpdoc; बिना टाइप वाले पैरामीटर, जो
संख्या की तरह इस्तेमाल होते हैं, संख्या माने जाते हैं।

PHP के लिए `analyze` एक PHP ऐप की योजना बनाता है (Composer पैकेज और PSR-4 ऑटोलोडिंग, ढीली तुलना और
पूर्णांक के फ़्लोट में ओवरफ़्लो जैसे जोखिम, Composer पैकेज को क्रेट से मिलाना), और एक
PHP CLI या सर्वर को रिकॉर्ड, रीप्ले और Rust front door के पीछे रखा जा सकता है। यूनिट के टॉप-लेवल फ़ंक्शन,
स्केलर, स्ट्रिंग, सूची और एसोसिएटिव ऐरे पैरामीटर के साथ, PHP के FFI एक्सटेंशन से चलते हैं
(`ext-ffi` वाला `php`, जो CLI में डिफ़ॉल्ट रूप से चालू होता है): `diff` पास होने के बाद मूल फ़ाइल की जगह
जनरेट की गई `php/<crate>.php` को include करें। हर कॉल सीमा को JSON के रूप में पार करती है, लगभग 1 µs में,
इसलिए `bench` छोटे फ़ंक्शन को PHP से धीमा दिखाता है; PHP के लिए अभी `shadow` नहीं है
([issue #200](https://github.com/erkamyaman/rust-anything/issues/200))।

C# भी इसी तरह काम करता है: `analyze` `.csproj` प्रोजेक्ट और उनके NuGet पैकेज पढ़ता है
(क्रेट से मैप किए गए), `using` निर्देशों को फ़ोल्डर तक पहुँचाता है, और C# की ख़ास बातें चिह्नित करता है
(`decimal`, `dynamic`, देर से चलने वाली LINQ क्वेरी, `Task`, `lock`, रिफ़्लेक्शन)। एक C# CLI या
ASP.NET सर्वर को रिकॉर्ड, रीप्ले और Rust front door के पीछे रखा जा सकता है। यूनिट के पब्लिक स्टैटिक मेथड,
स्केलर, स्ट्रिंग, ऐरे, सूची, डिक्शनरी और रिकॉर्ड या सादी क्लास पैरामीटर के साथ, P/Invoke से चलते हैं
(रिकॉर्ड और तुलना के लिए `PATH` में `dotnet`): `diff` पास होने के बाद जनरेट की गई `<namespace>.Rust` क्लास
को बुलाएँ या `using` से उन्हें उपनाम दें। हर कॉल सीमा को JSON के रूप में पार करती है, लगभग
1-3 µs में, इसलिए `bench` छोटे मेथड को .NET से धीमा दिखाता है; C# के लिए अभी `shadow` नहीं है
([issue #200](https://github.com/erkamyaman/rust-anything/issues/200))।

## और जानें

- [यह कैसे काम करता है](how-it-works.md): लूप, `bridge` और `analyze` क्या तय करते हैं, हॉट पाथ, यह क्या नहीं पकड़ सकता, असली पोर्ट के लिए क्या चाहिए, और मूल के साथ तालमेल कैसे रखें (`map`)
- [पूरा ऐप पोर्ट करना](whole-apps.md): सर्वर, CLI और लाइब्रेरी, front door से `app lift` तक
- [उदाहरण और केस स्टडी](examples.md): x18n और छह भाषाओं में असली कोड
- [असली पोर्ट](real-world.md): json-server, Fuse.js, vercel/ms, बेंचमार्क के साथ
- [इंस्टॉल, अपडेट और डेवलप](install.md): हर इंस्टॉल तरीका, Windows, और सोर्स से बिल्ड
- [अक्सर पूछे जाने वाले प्रश्न](faq.md) · [स्थिति](STATUS.md) · [MCP टूल](tools.md)

## प्रायोजक

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> मुफ़्त और ओपन सोर्स है। अगर यह आपके काम में मदद करता है, तो कृपया स्पॉन्सर करने पर विचार करें।<br>
  आपका सहयोग विकास को चालू रखता है।
</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="कंपनी प्रायोजक: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="प्रायोजक: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="स्पॉन्सर बनें"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

## योगदान

Issue और pull request का स्वागत है; [CONTRIBUTING.md](../CONTRIBUTING.md) में सेटअप, टेस्ट कहाँ रखने हैं
और नई भाषा कैसे जोड़नी है, यह बताया गया है। डिज़ाइन के फ़ैसले और उनके कारण
[DECISIONS.md](../DECISIONS.md) में हैं। असली पोर्ट की केस स्टडी का विशेष स्वागत है।
सुरक्षा संबंधी समस्याओं की सूचना गोपनीय रूप से दें ([SECURITY.md](../SECURITY.md)), और
[आचार संहिता](../CODE_OF_CONDUCT.md) का पालन करें।

## लाइसेंस

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman)। आपकी रिपॉज़िटरी में पोर्ट और जनरेट किया गया कोड
आपका है।
