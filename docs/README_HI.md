<p align="center">
  <a href="https://rustanything.ai/">
    <img src="assets/logo.png" width="140" height="140" alt="rust-anything का केकड़ा">
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
अपने ऐप को Rust में दोबारा लिखें, और साबित करें कि वह अब भी ठीक वैसे ही चलता है।
</p>

<p align="center">
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/daily?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the day on Trendshift" width="250" height="55"/></a>
<a href="https://trendshift.io/repositories/363468" target="_blank" rel="noopener noreferrer"><img src="https://trendshift.io/api/badge/trendshift/repositories/363468/weekly?language=Rust" alt="erkamyaman/rust-anything: #1 Rust repository of the week on Trendshift" width="250" height="55"/></a>
</p>

<p align="center"><b>दस्तावेज़:</b> <a href="https://rustanything.ai/hi">https://rustanything.ai/hi</a></p>

<p align="center"><a href="translations.md">🌐 अन्य भाषाएँ</a></p>

<p align="center">किसी कोडबेस को Rust में ले जा रहे हैं? मैं Rust माइग्रेशन के लिए कंसल्टिंग देता हूँ: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center"><sub>इस अनुवाद का रखरखाव समुदाय करता है, इसलिए यह <a href="../README.md">अंग्रेज़ी README</a> से पीछे हो सकता है।</sub></p>

## प्रायोजक

<p align="center">Türkiye 🇹🇷 में <a href="https://github.com/erkamyaman">erKam Yaman</a> द्वारा बनाया गया<br>
Rust Anything मुफ़्त और ओपन सोर्स है। अगर यह आपके काम आता है, तो आपका प्रायोजन इसे आगे बढ़ाता है।</p>

<p align="center">
  <a href="https://coderabbit.link/er-kam">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="कंपनी प्रायोजक: CodeRabbit">
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
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="प्रायोजक: Santosh Yadav, Hakan Soytürk, Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş, bugrcvi">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Become_a_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="स्पॉन्सर बनें"></a>
  <a href="https://buymeacoffee.com/erkamyaman"><img src="https://img.shields.io/badge/Buy_me_a_coffee-%E2%98%95-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black" alt="Buy me a coffee"></a>
</p>

<p align="center">कंपनी के रूप में प्रायोजन, या कुछ और? <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

<p align="center">किसी कोडबेस को Rust में ले जा रहे हैं? मैं Rust माइग्रेशन के लिए कंसल्टिंग देता हूँ: <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a></p>

## त्वरित शुरुआत

इसे अपने कोडिंग एजेंट में पेस्ट करें:

```text
Install Rust Anything with npm i -g rust-anything, then run rust-anything setup. Run rust-anything go -- <slow command of this app> and finish the port until rust-anything gate passes.
```

या इसे खुद इंस्टॉल करें:

```sh
cargo install rust-anything   # or: npm install -g rust-anything
rust-anything setup           # adds it to Claude Code, Codex, Cursor, Gemini CLI and OpenCode
```

> [!TIP]
> **एजेंट:** इंस्टॉल के चरण, सभी कमांड और पोर्टिंग का तरीका जानने के लिए [rustanything.ai/llms.txt](https://rustanything.ai/llms.txt) पढ़ें।

## यह कैसे काम करता है

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/how-it-works/hi-dark.svg">
    <img src="assets/how-it-works/hi-light.svg" width="880" alt="analyze योजना बनाता है कि क्या पोर्ट करना है; rust-anything go ब्रिज बनाता है, रिकॉर्ड करता है, आपके एजेंट से पोर्ट करवाता है और gate से जाँचता है, तब तक दोहराता है जब तक रिकॉर्ड किया गया हर केस मेल न खाए; फिर enable कॉल को Rust पर स्विच करता है">
  </picture>
</p>

**किसे पूरा माना जाता है।** व्यवहार, आपके टेस्ट नहीं। `record` मूल कोड को जनरेट किए गए और दिए गए इनपुट पर चलाता है, और हर कॉल जो लौटाती है, फेंकती है और प्रिंट करती है, उसे सहेजता है। पोर्ट तब पूरा होता है जब `gate` पास हो जाए: रिकॉर्ड किया गया हर केस मेल खाता है, और fmt, clippy, `cargo test`, `unsafe` जाँच, स्टब जाँच और पोर्ट में चिपकाए गए रिकॉर्ड किए गए जवाबों की जाँच, सब पास होते हैं। आपके अपने टेस्ट भी चलते हैं।

**रिपोर्ट या माइग्रेशन, आप चुनें।** `analyze` और `profile` सिर्फ़ रिपोर्ट करते हैं: क्या पोर्ट करना है उसकी प्राथमिकता के क्रम में योजना, और असली वर्कलोड में कौन सा हिस्सा सबसे ज़्यादा समय लेता है। ये साधारण कमांड हैं, इसलिए रिपोर्ट में कोई टोकन खर्च नहीं होता। `go` एक यूनिट माइग्रेट करता है: सबसे ज़्यादा समय लेने वाले फ़ंक्शन के लिए ब्रिज बनाता है, उसे रिकॉर्ड करता है, पोर्ट आपके एजेंट को सौंपता है, फिर उसे बिल्ड करता है, diff करता है और `gate` से गुज़ारता है, और सब कुछ मेल खाने पर ही उसे चालू करता है (TS/JS, Python और PowerShell; `disable` उसे फिर से बंद करता है)।

**एक रन में कितना समय लगता है।** एक `go` रन एक यूनिट पोर्ट करता है, ज़्यादा से ज़्यादा `--rounds` एजेंट टर्न में (डिफ़ॉल्ट 3): एक पोर्ट के लिए, फिर हर फ़ेल हुए diff या gate के लिए एक। पूरा ऐप हर यूनिट के लिए एक रन है, इसलिए आप किसी भी रन के बाद रुक सकते हैं।

**योजना के लिए सस्ता मॉडल, पोर्ट के लिए मज़बूत मॉडल।** योजना को किसी मॉडल की ज़रूरत नहीं। पोर्ट के लिए कोई भी एजेंट कमांड दें: `rust-anything go --agent "claude --model <model>" -- <workload>`।

**एजेंट।** `rust-anything setup` इसे Claude Code, Codex, Cursor, Gemini CLI और OpenCode में जोड़ता है। कोई भी दूसरा MCP क्लाइंट `rust-anything mcp` चला सकता है, और स्किल `npx skills add erkamyaman/rust-anything` से अलग से भी इंस्टॉल होती है ([इंस्टॉल](install.md) देखें)।

## लाइसेंस

[MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) © [Erkam Yaman](https://github.com/erkamyaman)। आपकी रिपॉज़िटरी में पोर्ट और जनरेट किया गया कोड
आपका है।
