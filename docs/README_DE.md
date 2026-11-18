<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="Die rust-anything-Krabbe">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">von <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · Deutsch · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a></p>

<p align="center"><sub>Diese Übersetzung wird von der Community gepflegt und kann hinter dem <a href="../README.md">englischen README</a> zurückliegen.</sub></p>

<p align="center"><em>Schreibe deine gesamte App in Rust neu. Halte sie dabei durchgehend lauffähig.</em></p>

<p align="center"><em>CLIs, Server und Bibliotheken, Stück für Stück, bei jedem Schritt gegen das Original geprüft.</em></p>

<p align="center">Node.js und TypeScript · Python · C · Go · Ruby · Java · PHP und C# in der Vorschau</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="crates.io-Version"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="npm-Version"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 Sprachen, plus 2 in der Vorschau">
  <a href="#funktioniert-mit"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="Funktioniert mit 4 Agenten"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Tests"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="MIT-Lizenz"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="GitHub-Sterne"></a>
</p>

<p align="center">
  <a href="#schnellstart">Schnellstart</a> ·
  <a href="#sprachen">Sprachen</a> ·
  <a href="how-it-works.md#how-it-works">So funktioniert es</a> ·
  <a href="how-it-works.md#the-loop">Der Ablauf</a> ·
  <a href="whole-apps.md#port-a-whole-app">Eine ganze App portieren</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Hot Paths</a> ·
  <a href="install.md#install">Installation</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Was es nicht abfangen kann</a> ·
  <a href="faq.md#faq">FAQ</a>
</p>

<p align="center">bis zu 20× schneller · 38.251 aufgezeichnete Fälle wiedergegeben · 0 Abweichungen · ein Befehl</p>

<p align="center">
  <img src="assets/race.gif" alt="Eingabe von user.profile in die x18n-Suche über 50.000 Schlüssel: TypeScript benötigt 143 ms, der rust-anything-Batch 6,9 ms, bei jedem Tastenanschlag mit exakt denselben Ergebnissen." width="820">
  <br>
  <sub>Eine Suchanfrage wird in die x18n-Suche über 50.000 Schlüssel getippt: das TypeScript-Original gegen den rust-anything-Batch, bei jedem Tastenanschlag mit denselben Ergebnissen.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Aufzeichnen</b><br>
      <code>record</code> führt das Original mit generierten Randfällen, semantischen Fallen und deinen eigenen Eingaben aus und speichert, was es zurückgibt oder wirft.
    </td>
    <td width="25%" valign="top">
      <b>Portieren</b><br>
      <code>bridge</code> erzeugt das Rust-Crate und einen Wrapper, sodass Aufrufer die ursprüngliche API behalten. Du oder dein Agent schreibt das Rust.
    </td>
    <td width="25%" valign="top">
      <b>Prüfen</b><br>
      <code>diff</code> spielt jeden aufgezeichneten Fall gegen das Rust ab, und nichts wird ausgeliefert, bevor alle übereinstimmen und <code>gate</code> besteht.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Wenn die Wiedergabe ohne Fallback besteht und jede Einheit gate besteht, macht <code>app lift</code> daraus eine reine Rust-App. Das Original wird nie angefasst.
    </td>
  </tr>
</table>

**Schreibe deine gesamte App Stück für Stück in Rust neu.** rust-anything zeichnet auf, was deine
Node.js- (TypeScript/JavaScript), Python-, C-, Go-, Ruby- oder Java-App heute tut, und prüft dann
jeden Teil der Rust-Neuimplementierung gegen diese Aufzeichnung, bevor es weitergeht, bis die App
reines Rust ist. CLIs, HTTP-Server und Bibliotheken; dein Coding-Agent (oder du selbst) schreibt
das Rust. Es kann nur erfassen, was die Aufzeichnung abdeckt: siehe
[Was es nicht abfangen kann](how-it-works.md#what-it-cant-catch). Du brauchst nur den langsamen Teil schnell?
Portiere genau diesen, einen Hot Path nach dem anderen:

| Reale Arbeitslast, gleiche Ergebnisse wie das Original | Original | Mit rust-anything |
| --- | ---: | ---: |
| x18n-Seitenleistensuche, 50.000 Schlüssel (TypeScript, Node) | 10 ms pro Anfrage | **0,5 ms: 20× schneller** |
| Dieselbe Suche im Browser (wasm, ein Thread) | 10 ms | **0,95 ms: 11,5×** |
| `slugify` über 20.000 Strings (Python) | 98 ms | **17× schneller** |
| `Shellwords.shellescape` aus Rubys Standardbibliothek, 50.000 Werte | 88 ms | **11,8× schneller** |
| `Slugify` über 50.000 Strings (Go) | 49 ms | **4× schneller** |
| `slugify` über 50.000 Strings (Java) | 43 ms | **3× schneller** |

Gemessen auf 4 Kernen; zu jedem Wert gibt es einen Benchmark in [`demo/`](../demo), der zuerst
prüft, dass die Ergebnisse identisch sind. Rust für jede kleine Funktion einzeln aufzurufen ist meist
*langsamer* als ein JIT; rust-anything findet die Schleife, deren Auslagerung sich lohnt, und
verschiebt sie.

> rust-anything ist ein unabhängiges Projekt und steht in keiner Verbindung zum Rust Project oder
> der Rust Foundation und wird von diesen nicht unterstützt. „Rust“ ist eine Marke der Rust Foundation.

> [!WARNING]
> `record` und `app record` führen deinen Originalcode aus und rufen jede Funktion (oder die ganze App)
> mit vielen generierten Eingaben auf, und `app record` führt jeden Fall zweimal aus. `diff`, `gate` und
> `app replay` führen dieselben Fälle erneut aus, auf der Portierung oder dem Original. Code, der Dateien
> schreibt, mit einer Datenbank spricht oder das Netzwerk aufruft, tut das einmal pro Fall. Führe sie auf
> einer Kopie des Projekts aus, mit einer Testdatenbank und ohne Produktionszugangsdaten in der Umgebung.

## Schnellstart

Füge dies in Claude Code, Codex, Cursor oder Gemini CLI ein und lass deinen Agenten es installieren:

```text
Installiere rust-anything: Führe `cargo install rust-anything` aus und danach `rust-anything setup`.
Sag mir Bescheid, wann ich dich neu starten soll.
```

Oder selbst:

```sh
cargo install rust-anything   # von crates.io
rust-anything setup           # registriert es bei Claude Code, Codex, Cursor und Gemini CLI
```

Noch keine Rust-Toolchain? Vorgefertigte Binärdateien für macOS, Linux und Windows hängen an jedem
[GitHub-Release](https://github.com/erkamyaman/rust-anything/releases), und für JavaScript-Projekte
installiert `npm install -g rust-anything` dieselbe Binärdatei.

In Claude Code kannst du es auch als Plugin installieren (der MCP-Server und der Skill; das obige CLI
muss in deinem `PATH` liegen):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` trägt den MCP-Server bei jedem gefundenen Agenten ein und installiert den Workflow-Skill dort,
wo der Agent Skills einliest. Deine übrigen Einstellungen bleiben erhalten, ein erneutes Ausführen ist
gefahrlos, und `--remove` macht es rückgängig. Frage deinen Agenten anschließend zum Beispiel:

- *"Schreibe dieses CLI mit rust-anything vollständig in Rust neu und beweise, dass es sich identisch verhält."*
- *"Diese Suche fühlt sich langsam an. Erstelle ein Profil mit rust-anything und verlagere die Hot Loop nach Rust."*
- *"Plane mit rust-anything eine Migration von `src/parser/` nach Rust; portiere die erste Einheit und verifiziere sie."*
- *"Vergleiche die Rust-Portierung in `rust/text-rs` im Benchmark mit dem Original und sag mir, ob wir sie ausliefern können."*

### Funktioniert mit

| Agent | `rust-anything setup` ergänzt |
| --- | --- |
| Claude Code | MCP-Server in `~/.claude.json`, Skill in `~/.claude/skills/` |
| Codex | MCP-Server in `~/.codex/config.toml`, Skill in `~/.codex/skills/` |
| Cursor | MCP-Server in `~/.cursor/mcp.json` |
| Gemini CLI | MCP-Server in `~/.gemini/settings.json` |
| Jeder andere MCP-Client | `rust-anything mcp` manuell als stdio-Server hinzufügen (siehe [Installation](install.md#install)) |

Bevor eine bestehende Datei geändert wird, legt `setup` daneben eine Kopie an (`<file>.rust-anything.bak`).

Oder führe die Schritte selbst aus:

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile findet fuzzyScore in einer Schleife, bridge --batch erzeugt das Rust-Crate, gate besteht mit 2000/2000 aufgezeichneten Fällen, und der Benchmark zeigt 20,7× schneller als TS." width="820">
  <br>
  <sub>Die Schritte an x18ns <code>fuzzyScore</code>: <code>profile</code> findet die heiße Schleife, <code>bridge --batch</code> erzeugt das Crate, <code>gate</code> besteht, und der Benchmark vergleicht es mit dem TypeScript.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # was heiß läuft und wer es aufruft
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# src/port.rs schreiben (oder vom Agenten schreiben lassen), dann:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # jeder aufgezeichnete Fall muss übereinstimmen
rust-anything bench rust/fuzzy-rs                      # schneller? Das Urteil sagt es dir
rust-anything gate rust/fuzzy-rs                       # bestanden heißt: bereit
```

rust-anything übersetzt keinen Code automatisch: Dein Coding-Agent (Claude Code, Codex, Cursor, …) oder
du selbst schreibst das Rust. rust-anything liefert genau das, was ein Agent nicht verlässlich erraten
kann: wo die Zeit bleibt, was zuerst und in welcher Reihenfolge portiert werden sollte, eine Bridge,
damit der übrige Code das Rust mit unveränderter API aufruft, das exakte Verhalten, dem die Portierung
entsprechen muss, einen sicheren Weg zur Auslieferung und ein Gate, das anzeigt, wann eine Portierung
fertig ist.

## Sprachen

| Sprache | Einheit | Bridge | Aufgezeichnet und wiedergegeben durch |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Ordner oder Datei | napi-rs oder wasm-bindgen mit einem generierten binären Wire-Format; Klassen und async inklusive | Node, führt die `.ts`-Quellen direkt aus |
| Python | Paket oder Modul | PyO3 (abi3), direkte Konvertierung, typisierte Exceptions | Python, importiert das Original |
| C | `.c`-Datei | `extern "C"`-Funktionen mit den Originalnamen und einem Header | das Original, kompiliert als Shared Library, aufgerufen über ctypes, ein Fork pro Fall |
| Go | Paket | typisierte cgo-Aufrufe und ein generiertes binäres Wire-Format in einem Paket mit der Original-API | einen generierten Go-Treiber, gebaut als eigenes Modul |
| Ruby | Datei oder Ordner mit Modulfunktionen | eine magnus-Erweiterung unter den ursprünglichen Modulen | einen generierten Ruby-Treiber |
| Java | Paket mit `public static`-Methoden | typisiertes JNI und ein generiertes binäres Wire-Format in Wrapper-Klassen mit den Originalmethoden | einen generierten Reflection-Treiber |
| PHP (Vorschau) | Ordner mit Funktionen und `public static`-Methoden | als Nächstes geplant: PHP FFI über ein C-ABI | heute schon ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und Server (`php -S`) |
| C# (Vorschau) | Ordner mit `public static`-Methoden | als Nächstes geplant: P/Invoke über ein C-ABI | heute schon ganze Apps: `app record`, `app replay` und die Front Door, für CLIs und ASP.NET-Server (`dotnet <app>.dll`) |

Jede Sprache außer C erhält außerdem Batch-Klassen (`--batch`): Die Elemente überqueren die Grenze
einmalig und bleiben in Rust, und jeder Aufruf verarbeitet sie alle auf allen Kernen.

Die Sprache wird anhand des Repositorys erkannt (`go.mod`, `pom.xml`, eine `.csproj` oder `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, sonst die häufigsten Quelldateien);
`--lang` überschreibt dies. Typen stammen aus dem Quellcode: TS-Typen, Python-Type-Hints, C-, Go-,
Java- und C#-Signaturen, Ruby-YARD-Tags, PHP-Typen und phpdoc; untypisierte Parameter, die als Zahlen
verwendet werden, werden als Zahlen abgeleitet.

PHP ist in der Vorschau: `analyze` plant eine PHP-App (Composer-Pakete und PSR-4-Autoloading, Risiken
wie lose Vergleiche und Integer-Überläufe in Floats, Zuordnung von Composer-Paketen zu Crates), und ein
PHP-CLI oder -Server lässt sich aufzeichnen, wiedergeben und hinter eine Rust-Front-Door stellen. Das
Portieren einzelner Einheiten nach Rust über das PHP-FFI ist der nächste Schritt
([Issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# ist auf dieselbe Weise in der Vorschau: `analyze` liest `.csproj`-Projekte und ihre NuGet-Pakete
(auf Crates abgebildet), löst `using`-Direktiven zu Ordnern auf und markiert C#-Eigenheiten
(`decimal`, `dynamic`, verzögerte LINQ-Abfragen, `Task`, `lock`, Reflection). Ein C#-CLI oder
ASP.NET-Server lässt sich aufzeichnen, wiedergeben und hinter eine Rust-Front-Door stellen; das
Portieren einzelner Einheiten über P/Invoke folgt als Nächstes.

## Mehr erfahren

- [So funktioniert es](how-it-works.md): der Ablauf, was `bridge` und `analyze` entscheiden, Hot Paths und was es nicht abfangen kann
- [Eine ganze App portieren](whole-apps.md): Server, CLIs und Bibliotheken, von der Front Door bis `app lift`
- [Beispiele und Fallstudien](examples.md): x18n und echter Code in sechs Sprachen
- [Echte Portierungen](real-world.md): json-server, Fuse.js, vercel/ms, mit Benchmarks
- [Installieren, aktualisieren und entwickeln](install.md): jede Installationsmethode, Windows und das Bauen aus dem Quellcode
- [FAQ](faq.md) · [Status](STATUS.md) · [MCP-Tools](tools.md)

## Sponsoren

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> ist kostenlos und Open Source. Wenn es dir bei der Arbeit hilft, ziehe bitte ein Sponsoring in Betracht.<br>
  Deine Unterstützung hält die Entwicklung am Laufen.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Unternehmenssponsor: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsoren: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Sponsor_werden-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Sponsor werden"></a>
</p>

## Mitwirken

Issues und Pull Requests sind willkommen; [CONTRIBUTING.md](../CONTRIBUTING.md) behandelt die
Einrichtung, wohin Tests gehören und wie man eine Sprache hinzufügt. Design-Entscheidungen und ihre
Gründe stehen in [DECISIONS.md](../DECISIONS.md). Fallstudien echter Portierungen sind besonders
willkommen. Sicherheitsprobleme bitte vertraulich melden ([SECURITY.md](../SECURITY.md)), und beachte
den [Verhaltenskodex](../CODE_OF_CONDUCT.md).

## Lizenz

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Portierungen und generierter Code in
deinem Repository gehören dir.
