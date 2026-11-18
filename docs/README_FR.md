<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="Le crabe de rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">par <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · Français · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · <a href="README_PT.md">Português</a></p>

<p align="center"><sub>Cette traduction est maintenue par la communauté et peut être en retard sur le <a href="../README.md">README anglais</a>.</sub></p>

<p align="center"><em>Réécrivez toute votre application en Rust. Gardez-la fonctionnelle tout du long.</em></p>

<p align="center"><em>CLI, serveurs et bibliothèques, morceau par morceau, vérifiés par rapport à l'original à chaque étape.</em></p>

<p align="center">Node.js et TypeScript · Python · C · Go · Ruby · Java · PHP et C# en aperçu</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="version crates.io"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="version npm"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 langages, plus 2 en aperçu">
  <a href="#fonctionne-avec"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="Fonctionne avec 4 agents"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Tests"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="Licence MIT"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="Étoiles GitHub"></a>
</p>

<p align="center">
  <a href="#démarrage-rapide">Démarrage rapide</a> ·
  <a href="#langages">Langages</a> ·
  <a href="how-it-works.md#how-it-works">Fonctionnement</a> ·
  <a href="how-it-works.md#the-loop">La boucle</a> ·
  <a href="whole-apps.md#port-a-whole-app">Porter toute une application</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Chemins chauds</a> ·
  <a href="install.md#install">Installation</a> ·
  <a href="how-it-works.md#what-it-cant-catch">Ce qui lui échappe</a> ·
  <a href="faq.md#faq">FAQ</a>
</p>

<p align="center">jusqu'à 20x plus rapide · 38 251 cas enregistrés rejoués · 0 divergence · une seule commande</p>

<p align="center">
  <img src="assets/race.gif" alt="Saisie de user.profile dans la recherche de x18n sur 50 000 clés : TypeScript met 143 ms, le traitement par lots rust-anything met 6,9 ms, avec les mêmes résultats à chaque frappe." width="820">
  <br>
  <sub>Saisie d'une requête dans la recherche de x18n sur 50 000 clés : l'original TypeScript face au traitement par lots rust-anything, avec les mêmes résultats à chaque frappe.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Enregistrer</b><br>
      <code>record</code> exécute l'original sur des cas limites générés, des pièges sémantiques et vos propres entrées, et sauvegarde ce qu'il renvoie ou lève.
    </td>
    <td width="25%" valign="top">
      <b>Porter</b><br>
      <code>bridge</code> génère la crate Rust et un wrapper, pour que les appelants gardent l'API d'origine. Vous ou votre agent écrivez le Rust.
    </td>
    <td width="25%" valign="top">
      <b>Vérifier</b><br>
      <code>diff</code> rejoue chaque cas enregistré contre le Rust, et rien n'est livré tant qu'ils ne correspondent pas tous et que <code>gate</code> ne passe pas.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Quand le replay passe sans fallback et que chaque unité passe le gate, <code>app lift</code> en fait une application entièrement en Rust. L'original n'est jamais modifié.
    </td>
  </tr>
</table>

**Réécrivez toute votre application en Rust, un morceau à la fois.** rust-anything enregistre ce que
fait aujourd'hui votre application Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ou Java, puis
vérifie chaque morceau de la réécriture Rust par rapport à cet enregistrement avant de passer à la
suite, jusqu'à ce que l'application soit entièrement en Rust. CLI, serveurs HTTP et bibliothèques ;
votre agent de programmation (ou vous) écrit le code Rust. Il ne peut détecter que ce que
l'enregistrement couvre : voir [Ce qui lui échappe](how-it-works.md#what-it-cant-catch). Vous voulez seulement
accélérer la partie lente ? Ne portez que celle-ci, un chemin chaud à la fois :

| Charge réelle, mêmes résultats que l'original | Original | Avec rust-anything |
| --- | ---: | ---: |
| Recherche dans la barre latérale de x18n, 50 000 clés (TypeScript, Node) | 10 ms par requête | **0,5 ms : 20x plus rapide** |
| La même recherche dans le navigateur (wasm, un thread) | 10 ms | **0,95 ms : 11,5x** |
| `slugify` sur 20 000 chaînes (Python) | 98 ms | **17x plus rapide** |
| `Shellwords.shellescape` de la bibliothèque standard Ruby, 50 000 valeurs | 88 ms | **11,8x plus rapide** |
| `Slugify` sur 50 000 chaînes (Go) | 49 ms | **4x plus rapide** |
| `slugify` sur 50 000 chaînes (Java) | 43 ms | **3x plus rapide** |

Mesuré sur 4 cœurs ; chaque chiffre dispose d'un benchmark dans [`demo/`](../demo) qui vérifie d'abord
que les résultats sont identiques. Appeler Rust une fois par petite fonction est généralement *plus lent*
qu'un JIT ; rust-anything identifie la boucle qui vaut la peine d'être déplacée, et la déplace.

> rust-anything est un projet indépendant, qui n'est ni affilié au Rust Project ou à la Rust Foundation,
> ni approuvé par eux. « Rust » est une marque déposée de la Rust Foundation.

> [!WARNING]
> `record` et `app record` exécutent votre code d'origine, en appelant chaque fonction (ou toute l'application) sur
> de nombreuses entrées générées, et `app record` exécute chaque cas deux fois. `diff`, `gate` et `app replay`
> rejouent les mêmes cas, sur le portage ou sur l'original. Du code qui écrit des fichiers, parle à une
> base de données ou appelle le réseau le fait une fois par cas. Lancez-les sur une copie du projet, avec une
> base de test et aucun identifiant de production dans l'environnement.

## Démarrage rapide

Collez ceci dans Claude Code, Codex, Cursor ou Gemini CLI et laissez votre agent l'installer :

```text
Installe rust-anything : exécute `cargo install rust-anything`, puis `rust-anything setup`.
Dis-moi quand je dois te redémarrer.
```

Ou faites-le vous-même :

```sh
cargo install rust-anything   # depuis crates.io
rust-anything setup           # l'enregistre auprès de Claude Code, Codex, Cursor et Gemini CLI
```

Pas encore de chaîne d'outils Rust ? Des binaires précompilés pour macOS, Linux et Windows sont joints
à chaque [release GitHub](https://github.com/erkamyaman/rust-anything/releases), et pour les projets
JavaScript, `npm install -g rust-anything` installe le même binaire.

Dans Claude Code, vous pouvez aussi l'installer en tant que plugin (le serveur MCP et le skill ; le CLI
ci-dessus doit se trouver dans votre `PATH`) :

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

`setup` ajoute le serveur MCP à chaque agent détecté et installe le skill de workflow là où l'agent
lit ses skills. Il conserve vos autres paramètres, peut être relancé sans risque, et `--remove` l'annule.
Demandez ensuite à votre agent, par exemple :

- *« Réécris ce CLI en Rust avec rust-anything, de bout en bout, et prouve qu'il se comporte à l'identique. »*
- *« Cette recherche semble lente. Profile-la avec rust-anything et déplace la boucle critique vers Rust. »*
- *« Planifie une migration de `src/parser/` vers Rust avec rust-anything ; porte la première unité et prouve-la. »*
- *« Compare les performances du portage Rust dans `rust/text-rs` face à l'original et dis-moi s'il faut le livrer. »*

### Fonctionne avec

| Agent | Ce que `rust-anything setup` ajoute |
| --- | --- |
| Claude Code | Serveur MCP dans `~/.claude.json`, skill dans `~/.claude/skills/` |
| Codex | Serveur MCP dans `~/.codex/config.toml`, skill dans `~/.codex/skills/` |
| Cursor | Serveur MCP dans `~/.cursor/mcp.json` |
| Gemini CLI | Serveur MCP dans `~/.gemini/settings.json` |
| Tout autre client MCP | ajouter `rust-anything mcp` manuellement comme serveur stdio (voir [Installation](install.md#install)) |

Avant de modifier un fichier existant, `setup` en enregistre une copie à côté (`<file>.rust-anything.bak`).

Ou exécutez les étapes vous-même :

<p align="center">
  <img src="assets/terminal.gif" alt="rust-anything profile repère fuzzyScore dans une boucle, bridge --batch génère la crate Rust, gate passe avec 2000/2000 cas enregistrés, et le benchmark affiche 20,7x plus rapide que TS." width="820">
  <br>
  <sub>Les étapes sur le <code>fuzzyScore</code> de x18n : <code>profile</code> trouve la boucle chaude, <code>bridge --batch</code> génère la crate, <code>gate</code> passe, et le benchmark la compare au TypeScript.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # ce qui consomme du temps, et qui l'appelle
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# écrivez src/port.rs (ou laissez votre agent le faire), puis :
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # chaque cas enregistré doit correspondre
rust-anything bench rust/fuzzy-rs                      # plus rapide ? le verdict tranche
rust-anything gate rust/fuzzy-rs                       # terminé signifie que ce contrôle passe
```

rust-anything ne traduit pas le code automatiquement : votre agent de programmation (Claude Code, Codex,
Cursor, ...) ou vous-même écrivez le Rust. rust-anything fournit ce qu'un agent ne peut pas deviner de
manière fiable : où passe le temps, quoi porter en premier et dans quel ordre, un pont pour que le reste
du code appelle le Rust sans changer son API, le comportement exact que le portage doit reproduire, un
moyen sûr de le déployer et un contrôle final indiquant quand un portage est terminé.

## Langages

| Langage | Unité | Pont | Enregistré et rejoué par |
| --- | --- | --- | --- |
| TypeScript/JavaScript | dossier ou fichier | napi-rs ou wasm-bindgen avec un protocole binaire généré ; classes et asynchrone inclus | Node, en exécutant les sources `.ts` |
| Python | paquet ou module | PyO3 (abi3), conversion directe, exceptions typées | Python, en important l'original |
| C | fichier `.c` | fonctions `extern "C"` portant les noms d'origine et un fichier d'en-tête | l'original compilé en bibliothèque partagée, appelé via ctypes, un fork par cas |
| Go | paquet | appels cgo typés et protocole binaire généré, dans un paquet conservant l'API d'origine | un pilote Go généré, compilé comme son propre module |
| Ruby | fichier ou dossier de fonctions de module | une extension magnus sous les modules d'origine | un pilote Ruby généré |
| Java | paquet de méthodes `public static` | JNI typé et protocole binaire généré, dans des classes enveloppes conservant les méthodes d'origine | un pilote par réflexion généré |
| PHP (aperçu) | dossier de fonctions et de méthodes `public static` | à venir : PHP FFI sur une ABI C | applications complètes dès aujourd'hui : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs (`php -S`) |
| C# (aperçu) | dossier de méthodes `public static` | à venir : P/Invoke sur une ABI C | applications complètes dès aujourd'hui : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs ASP.NET (`dotnet <app>.dll`) |

Chaque langage sauf C obtient également des classes de traitement par lots (`--batch`) : les éléments
traversent la frontière une seule fois et restent en Rust, et chaque appel s'exécute sur l'ensemble des
éléments sur tous les cœurs.

Le langage est détecté à partir du dépôt (`go.mod`, `pom.xml`, un `.csproj` ou `.sln`, `pyproject.toml`,
`Gemfile`, `composer.json`, `package.json`, sinon les fichiers sources les plus courants) ; `--lang`
permet de le forcer. Les types proviennent des sources : types TS, annotations Python, signatures C, Go,
Java et C#, balises Ruby YARD, types PHP et phpdoc ; les paramètres non typés utilisés comme des nombres
sont inférés comme nombres.

PHP est en aperçu : `analyze` planifie une application PHP (paquets Composer et chargement automatique
PSR-4, risques tels que la comparaison souple et le débordement d'entiers en flottants, paquets Composer
mappés vers des crates), et un CLI ou serveur PHP peut être enregistré, rejoué et placé derrière une porte
d'entrée Rust. Le portage de ses unités en Rust via la FFI de PHP est l'étape suivante
([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

C# est en aperçu de la même manière : `analyze` lit les projets `.csproj` et leurs paquets NuGet (mappés
vers des crates), résout les directives `using` vers les dossiers correspondants et signale ce que C# gère
différemment (`decimal`, `dynamic`, requêtes différées LINQ, `Task`, `lock`, réflexion). Un CLI C# ou un
serveur ASP.NET peut être enregistré, rejoué et placé derrière une porte d'entrée Rust ; le portage
d'unités via P/Invoke viendra ensuite.

## En savoir plus

- [Fonctionnement](how-it-works.md) : la boucle, ce que décident `bridge` et `analyze`, les chemins chauds, et ce qui lui échappe
- [Porter toute une application](whole-apps.md) : serveurs, CLI et bibliothèques, de la porte d'entrée à `app lift`
- [Exemples et études de cas](examples.md) : x18n, et du code réel dans six langages
- [Portages réels](real-world.md) : json-server, Fuse.js, vercel/ms, avec benchmarks
- [Installer, mettre à jour et développer](install.md) : toutes les méthodes d'installation, Windows, et la compilation depuis les sources
- [FAQ](faq.md) · [État](STATUS.md) · [Outils MCP](tools.md)

## Sponsors

<p align="center">
  <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> est gratuit et open source. S'il vous aide dans votre travail, envisagez de le sponsoriser.<br>
  Votre soutien permet de poursuivre le développement.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Sponsor entreprise : CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Sponsors : Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Devenir_sponsor-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Devenir sponsor"></a>
</p>

## Contribuer

Les issues et pull requests sont les bienvenues ; [CONTRIBUTING.md](../CONTRIBUTING.md) couvre la mise en
place, l'emplacement des tests et la manière d'ajouter un langage. Les choix de conception et leurs raisons
se trouvent dans [DECISIONS.md](../DECISIONS.md). Les études de cas de portages réels sont particulièrement
appréciées. Merci de signaler les failles de sécurité en privé ([SECURITY.md](../SECURITY.md)), et de
consulter le [code de conduite](../CODE_OF_CONDUCT.md).

## Licence

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). Les portages et le code généré dans votre dépôt vous appartiennent.
