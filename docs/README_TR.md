<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="rust-anything yengeci">
</p>

<h1 align="center">rust-anything</h1>

<p align="center"><a href="https://github.com/erkamyaman">erKam Yaman</a> tarafından</p>

<p align="center"><a href="../README.md">English</a> · Türkçe · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a> · <a href="README_VI.md">Tiếng Việt</a></p>
<p align="center"><sub>Bu çeviri topluluk tarafından sürdürülmektedir ve <a href="../README.md">İngilizce README</a>'nin gerisinde kalabilir.</sub></p>

<p align="center"><em>Uygulamanızın tamamını Rust ile yeniden yazın. Yol boyunca çalışır durumda kalsın.</em></p>

<p align="center"><em>CLI'lar, sunucular ve kütüphaneler, parça parça, her adımda orijinaline karşı doğrulanarak.</em></p>

<p align="center">Node.js ve TypeScript · Python · C · Go · Ruby · Java · PHP ve C# önizlemede</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io sürümü"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm sürümü"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 dil, artı 2 dil önizlemede">
  <a href="#desteklenen-ajanlar"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="4 ajanla çalışır"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Testler"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT lisansı"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub yıldızları"></a>
</p>

<p align="center">
  <a href="#hızlı-başlangıç">Hızlı başlangıç</a> ·
  <a href="#diller">Diller</a> ·
  <a href="how-it-works.md#how-it-works">Nasıl çalışır</a> ·
  <a href="how-it-works.md#the-loop">Döngü</a> ·
  <a href="whole-apps.md#port-a-whole-app">Tüm uygulamayı port etmek</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot path'ler</a> ·
  <a href="install.md#install">Kurulum</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Neyi yakalayamaz</a> ·
  <a href="faq.md#faq">SSS</a>
</p>

<p align="center">20x'e kadar daha hızlı · 38,251 kayıtlı vaka yeniden oynatıldı · 0 uyuşmazlık · tek komut</p>

<p align="center">
  <img src="assets/race.gif" alt="x18n'in 50,000 anahtar üzerindeki aramasına user.profile yazılıyor: TypeScript 143 ms harcıyor, rust-anything batch'i 6.9 ms, her tuş vuruşunda aynı sonuçlarla." width="820">
  <br>
  <sub>x18n'in 50,000 anahtar üzerindeki aramasına bir sorgu yazılıyor: TypeScript orijinali rust-anything batch'ine karşı, her tuş vuruşunda aynı sonuçlar.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Kaydet</b><br>
      <code>record</code>, orijinali üretilmiş uç durumlar, anlamsal tuzaklar ve sizin girdilerinizle çalıştırır; döndürdüğünü ya da fırlattığını kaydeder.
    </td>
    <td width="25%" valign="top">
      <b>Port et</b><br>
      <code>bridge</code>, Rust crate'ini ve bir wrapper üretir; böylece çağıranlar orijinal API'yi kullanmaya devam eder. Rust'ı siz ya da ajanınız yazar.
    </td>
    <td width="25%" valign="top">
      <b>Kontrol et</b><br>
      <code>diff</code>, kaydedilen her vakayı Rust'a karşı yeniden oynatır; hepsi eşleşip <code>gate</code> geçene kadar hiçbir şey yayınlanmaz.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Yeniden oynatma fallback olmadan geçtiğinde ve her birim gate'i geçtiğinde, <code>app lift</code> onu saf bir Rust uygulamasına dönüştürür. Orijinale asla dokunulmaz.
    </td>
  </tr>
</table>

**Uygulamanızın tamamını Rust ile, parça parça yeniden yazın.** rust-anything, Node.js
(TypeScript/JavaScript), Python, C, Go, Ruby veya Java uygulamanızın bugün ne yaptığını kaydeder,
ardından Rust ile yeniden yazılan her parçayı bir sonrakine geçmeden önce bu kayda karşı kontrol
eder; ta ki uygulama tamamen Rust olana kadar. CLI'lar, HTTP sunucuları ve kütüphaneler; Rust'ı
kodlama ajanınız (ya da siz) yazar. Yalnızca kaydın kapsadığını yakalayabilir: bkz.
[Neyi yakalayamaz](how-it-works.md#what-it-cant-catch). Sadece yavaş kısmın hızlanması mı gerekiyor?
Yalnızca onu, her seferinde bir hot path olacak şekilde port edin:

| Gerçek iş yükü, orijinaliyle aynı sonuçlar | Orijinal | rust-anything ile |
| --- | ---: | ---: |
| x18n'in kenar çubuğu araması, 50,000 anahtar (TypeScript, Node) | sorgu başına 10 ms | **0.5 ms: 20x daha hızlı** |
| Aynı arama tarayıcıda (wasm, tek thread) | 10 ms | **0.95 ms: 11.5x** |
| 20,000 string üzerinde `slugify` (Python) | 98 ms | **17x daha hızlı** |
| Ruby standart kütüphanesinden `Shellwords.shellescape`, 50,000 değer | 88 ms | **11.8x daha hızlı** |
| 50,000 string üzerinde `Slugify` (Go) | 49 ms | **4x daha hızlı** |
| 50,000 string üzerinde `slugify` (Java) | 43 ms | **3x daha hızlı** |

4 çekirdekte ölçülmüştür; her sayının [`demo/`](../demo) içinde, önce sonuçların aynı olduğunu
kontrol eden bir benchmark'ı vardır. Her küçük fonksiyon için Rust'ı ayrı ayrı çağırmak genellikle
bir JIT'ten *daha yavaştır*; rust-anything taşımaya değer döngüyü bulur ve onu taşır.

> rust-anything bağımsız bir projedir; Rust Project veya Rust Foundation ile bağlantılı değildir ve
> onlar tarafından desteklenmez. "Rust", Rust Foundation'ın ticari markasıdır.

> [!WARNING]
> `record` ve `app record` orijinal kodunuzu çalıştırır; her fonksiyonu (ya da uygulamanın
> tamamını) çok sayıda üretilmiş girdiyle çağırır ve `app record` her vakayı iki kez çalıştırır.
> `diff`, `gate` ve `app replay` aynı vakaları port ya da orijinal üzerinde yeniden çalıştırır.
> Dosya yazan, bir veritabanıyla konuşan veya ağa çağrı yapan kod bunu her vaka için bir kez yapar.
> Bunları projenin bir kopyası üzerinde, bir test veritabanıyla ve ortamda hiçbir production
> kimlik bilgisi olmadan çalıştırın.

## Hızlı başlangıç

Bunu Claude Code, Codex, Cursor veya Gemini CLI'a yapıştırın ve kurulumu ajanınıza bırakın:

```text
rust-anything'i kur: `cargo install rust-anything`, ardından `rust-anything setup` çalıştır.
Seni ne zaman yeniden başlatmam gerektiğini söyle.
```

Ya da kendiniz:

```sh
cargo install rust-anything   # crates.io'dan
rust-anything setup           # Claude Code, Codex, Cursor ve Gemini CLI'a kaydeder
```

Henüz Rust toolchain'iniz yok mu? macOS, Linux ve Windows için hazır derlenmiş binary'ler her
[GitHub release](https://github.com/erkamyaman/rust-anything/releases)'e eklenmiştir; JavaScript
projeleri için `npm install -g rust-anything` aynı binary'yi kurar.

Claude Code'da bunu bir plugin olarak da kurabilirsiniz (MCP sunucusu ve skill; yukarıdaki CLI
`PATH` içinde olmalıdır):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup`, bulduğu her ajana MCP sunucusunu ekler ve iş akışı skill'ini ajanın skill'leri okuduğu
yere kurar. Diğer ayarlarınızı korur, tekrar çalıştırmak güvenlidir ve `--remove` yaptıklarını geri alır.
Ardından ajanınıza örneğin şunları sorun:

- *"Bu CLI'ı rust-anything ile baştan sona Rust'ta yeniden yaz ve aynı davrandığını kanıtla."*
- *"Bu arama yavaş geliyor. rust-anything ile profilini çıkar ve sıcak döngüyü Rust'a taşı."*
- *"`src/parser/`'ın rust-anything ile Rust'a geçişini planla; ilk birimi port et ve kanıtla."*
- *"`rust/text-rs` içindeki Rust portunu orijinaline karşı benchmark'la ve yayınlamaya değer mi söyle."*

### Desteklenen ajanlar

| Ajan | `rust-anything setup` şunları ekler |
| --- | --- |
| Claude Code | `~/.claude.json` içinde MCP sunucusu, `~/.claude/skills/` içinde skill |
| Codex | `~/.codex/config.toml` içinde MCP sunucusu, `~/.codex/skills/` içinde skill |
| Cursor | `~/.cursor/mcp.json` içinde MCP sunucusu |
| Gemini CLI | `~/.gemini/settings.json` içinde MCP sunucusu |
| Diğer herhangi bir MCP istemcisi | `rust-anything mcp`'yi elle stdio sunucusu olarak ekleyin (bkz. [Kurulum](install.md#install)) |

Var olan bir dosyayı değiştirmeden önce `setup` yanına bir kopyasını kaydeder (`<file>.rust-anything.bak`).

Ya da adımları kendiniz çalıştırın:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile bir döngü içinde fuzzyScore'u buluyor, bridge --batch Rust crate'ini üretiyor, gate 2000/2000 kayıtlı vakayla geçiyor ve benchmark TS'ten 20.7x daha hızlı olduğunu gösteriyor." width="820">
  <br>
  <sub>x18n'in <code>fuzzyScore</code> fonksiyonu üzerinde adımlar: <code>profile</code> sıcak döngüyü bulur, <code>bridge --batch</code> crate'i üretir, <code>gate</code> geçer ve benchmark onu TypeScript ile karşılaştırır.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # sıcak olan ne, onu kim çağırıyor
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs'i yazın (ya da ajanınıza yazdırın), ardından:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # kaydedilen her vaka eşleşmeli
rust-anything bench rust/fuzzy-rs                      # daha mı hızlı? karar söyler
rust-anything gate rust/fuzzy-rs                       # bitti demek, bunun geçmesi demek
```

rust-anything kodu çevirmez: Rust'ı kodlama ajanınız (Claude Code, Codex, Cursor, ...) ya da siz
yazarsınız. rust-anything, bir ajanın güvenilir biçimde tahmin edemeyeceği şeyleri sağlar: zamanın
nereye gittiğini, önce neyin ve hangi sırayla port edileceğini, kodun geri kalanının Rust'ı API'si
değişmeden çağırabilmesi için bir köprüyü (bridge), portun eşleşmesi gereken davranışın tam halini,
onu yayına almanın güvenli bir yolunu ve bir portun ne zaman bittiğini söyleyen bir gate'i.

## Diller

| Dil | Birim | Köprü | Kaydeden ve yeniden oynatan |
| --- | --- | --- | --- |
| TypeScript/JavaScript | klasör veya dosya | üretilmiş bir binary wire ile napi-rs veya wasm-bindgen; sınıflar ve async dahil | `.ts` kaynaklarını çalıştıran Node |
| Python | paket veya modül | PyO3 (abi3), doğrudan dönüşüm, tipli exception'lar | orijinali import eden Python |
| C | `.c` dosyası | orijinal adlarla `extern "C"` fonksiyonlar ve bir header | paylaşımlı kütüphane olarak derlenmiş orijinal, ctypes üzerinden çağrılır, vaka başına bir fork |
| Go | paket | orijinal API'ye sahip bir pakette tipli cgo çağrıları ve üretilmiş bir binary wire | kendi modülü olarak derlenen, üretilmiş bir Go sürücüsü |
| Ruby | modül fonksiyonlarından oluşan dosya veya klasör | orijinal modüllerin altında bir magnus eklentisi | üretilmiş bir Ruby sürücüsü |
| Java | `public static` metotlardan oluşan paket | orijinal metotlara sahip wrapper sınıflarda tipli JNI ve üretilmiş bir binary wire | üretilmiş bir reflection sürücüsü |
| PHP (önizleme) | fonksiyonlar ve `public static` metotlardan oluşan klasör | sırada: C ABI üzerinden PHP FFI | bugün tüm uygulamalar: CLI'lar ve sunucular (`php -S`) için `app record`, `app replay` ve front door |
| C# (önizleme) | `public static` metotlardan oluşan klasör | sırada: C ABI üzerinden P/Invoke | bugün tüm uygulamalar: CLI'lar ve ASP.NET sunucuları (`dotnet <app>.dll`) için `app record`, `app replay` ve front door |

C hariç her dil batch sınıfları da alır (`--batch`): öğeler köprüyü bir kez geçer ve Rust'ta kalır,
her çağrı tüm çekirdeklerde hepsinin üzerinden çalışır.

Dil, repodan tespit edilir (`go.mod`, `pom.xml`, bir `.csproj` veya `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, aksi halde en yaygın kaynak dosyalar); `--lang` bunu
geçersiz kılar. Tipler kaynaktan gelir: TS tipleri, Python type hint'leri, C, Go, Java ve C#
imzaları, Ruby YARD etiketleri, PHP tipleri ve phpdoc; sayı olarak kullanılan tipsiz parametreler
sayı olarak çıkarılır.

PHP önizlemededir: `analyze` bir PHP uygulamasını planlar (Composer paketleri ve PSR-4 autoloading,
gevşek karşılaştırma ve tam sayı taşmasının float'a dönüşmesi gibi riskler, crate'lere eşlenmiş
Composer paketleri) ve bir PHP CLI'ı ya da sunucusu kaydedilebilir, yeniden oynatılabilir ve bir
Rust front door'unun arkasına konabilir. Birimlerini PHP'nin FFI'ı üzerinden Rust'a port etmek bir
sonraki adımdır ([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# da aynı şekilde önizlemededir: `analyze` `.csproj` projelerini ve NuGet paketlerini okur
(crate'lere eşlenmiş olarak), `using` yönergelerini klasörlere çözümler ve C#'ın farklı yaptığı
şeyleri işaretler (`decimal`, `dynamic`, LINQ'in ertelenmiş sorguları, `Task`, `lock`, reflection).
Bir C# CLI'ı veya ASP.NET sunucusu kaydedilebilir, yeniden oynatılabilir ve bir Rust front door'unun
arkasına konabilir; birimleri P/Invoke üzerinden port etmek sırada.

## Daha fazlası

- [Nasıl çalışır](how-it-works.md): döngü, `bridge` ve `analyze` neye karar verir, hot path'ler ve neyi yakalayamaz
- [Tüm uygulamayı port etmek](whole-apps.md): sunucular, CLI'lar ve kütüphaneler, front door'dan `app lift`'e kadar
- [Örnekler ve vaka çalışmaları](examples.md): x18n ve altı dilde gerçek kod
- [Gerçek dünya portları](real-world.md): json-server, Fuse.js, vercel/ms, benchmark'larıyla
- [Kurulum, güncelleme ve geliştirme](install.md): tüm kurulum yöntemleri, Windows ve kaynaktan derleme
- [SSS](faq.md) · [Durum](STATUS.md) · [MCP araçları](tools.md)

## Sponsorlar

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> ücretsiz ve açık kaynaklıdır. İşinize yarıyorsa lütfen sponsor olmayı düşünün.<br>
  Desteğiniz geliştirmenin sürmesini sağlar.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Şirket sponsoru: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsorlar: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Sponsor_olun-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Sponsor olun"></a>
</p>

## Katkıda bulunma

Issue'lar ve pull request'ler memnuniyetle karşılanır; [CONTRIBUTING.md](../CONTRIBUTING.md) kurulumu,
testlerin nereye gittiğini ve yeni bir dilin nasıl ekleneceğini anlatır. Tasarım kararları ve
gerekçeleri [DECISIONS.md](../DECISIONS.md) içindedir. Gerçek portların vaka çalışmaları özellikle
memnuniyetle karşılanır. Lütfen güvenlik sorunlarını gizli olarak bildirin ([SECURITY.md](../SECURITY.md))
ve [davranış kurallarına](../CODE_OF_CONDUCT.md) göz atın.

## Lisans

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Reponuzdaki portlar ve üretilmiş kod sizindir.
