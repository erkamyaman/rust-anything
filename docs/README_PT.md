<p align="center">
  <img src="assets/logo.png" width="160" height="160" alt="O caranguejo do rust-anything">
</p>

<h1 align="center">rust-anything</h1>

<p align="center">por <a href="https://github.com/erkamyaman">erKam Yaman</a></p>

<p align="center"><a href="../README.md">English</a> · <a href="README_TR.md">Türkçe</a> · <a href="README_UK.md">Українська</a> · <a href="README_ZH.md">简体中文</a> · <a href="README_ZH-TW.md">繁體中文</a> · <a href="README_DE.md">Deutsch</a> · <a href="README_FR.md">Français</a> · <a href="README_RU.md">Русский</a> · <a href="README_JA.md">日本語</a> · <a href="README_KO.md">한국어</a> · <a href="README_AZ.md">Azərbaycanca</a> · <a href="README_UZ.md">Oʻzbekcha</a> · <a href="README_KK.md">Қазақша</a> · <a href="README_KY.md">Кыргызча</a> · <a href="README_TK.md">Türkmençe</a> · <a href="README_TT.md">Татарча</a> · <a href="README_UG.md">ئۇيغۇرچە</a> · Português ? <a href="README_VI.md">Vietnamese</a></p>

<p align="center"><sub>Esta tradução é mantida pela comunidade e pode estar desatualizada em relação ao <a href="../README.md">README em inglês</a>.</sub></p>

<p align="center"><em>Reescreva toda a sua aplicação em Rust. Mantenha-a funcional durante todo o processo.</em></p>

<p align="center"><em>CLIs, servidores e bibliotecas, peça por peça, comparados com o original em cada etapa.</em></p>

<p align="center">Node.js e TypeScript · Python · C · Go · Ruby · Java · PHP e C# em versão de pré-visualização</p>

<p align="center">
  <a href="https://crates.io/crates/rust-anything"><img src="https://img.shields.io/crates/v/rust-anything?color=E2862A&label=crates.io" alt="Versão no crates.io"></a>
  <a href="https://www.npmjs.com/package/rust-anything"><img src="https://img.shields.io/npm/v/rust-anything?color=E2862A&label=npm" alt="Versão no npm"></a>
  <img src="https://img.shields.io/badge/languages-6%20%2B%202%20preview-E2862A" alt="6 linguagens, mais 2 em pré-visualização">
  <a href="#compatibilidade-com-agentes"><img src="https://img.shields.io/badge/works%20with-4%20agents-E2862A" alt="Compatível com 4 agentes"></a>
  <a href="https://github.com/erkamyaman/rust-anything/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/erkamyaman/rust-anything/ci.yml?branch=main&label=tests" alt="Testes"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-E2862A" alt="Licença MIT"></a>
  <a href="https://github.com/erkamyaman/rust-anything"><img src="https://img.shields.io/github/stars/erkamyaman/rust-anything?style=social" alt="Estrelas no GitHub"></a>
</p>

<p align="center">
  <a href="#início-rápido">Início rápido</a> ·
  <a href="#linguagens">Linguagens</a> ·
  <a href="how-it-works.md#how-it-works">Como funciona</a> ·
  <a href="how-it-works.md#the-loop">O ciclo</a> ·
  <a href="whole-apps.md#port-a-whole-app">Migrar uma aplicação completa</a> ·
  <a href="how-it-works.md#hot-paths-keep-the-data-in-rust">Caminhos críticos</a> ·
  <a href="install.md#install">Instalação</a> ·
  <a href="how-it-works.md#what-it-cant-catch">O que não consegue detetar</a> ·
  <a href="faq.md#faq">Perguntas frequentes</a>
</p>

<p align="center">Até 20x mais rápido · 38.251 casos registados e reproduzidos · 0 divergências · um único comando</p>

<p align="center">
  <img src="assets/race.gif" alt="Pesquisa de user.profile no x18n entre 50.000 chaves: TypeScript demora 143 ms, enquanto o processamento em lote do rust-anything demora 6,9 ms, com os mesmos resultados em cada tecla." width="820">
  <br>
  <sub>Escrever uma consulta na pesquisa do x18n entre 50.000 chaves: o original em TypeScript contra o lote do rust-anything, com os mesmos resultados em cada tecla.</sub>
</p>

<table>
  <tr>
    <td width="25%" valign="top">
      <b>Registar</b><br>
      <code>record</code> executa o original sobre casos limite gerados, armadilhas semânticas e as suas próprias entradas, e guarda o que ele devolve ou lança.
    </td>
    <td width="25%" valign="top">
      <b>Migrar</b><br>
      <code>bridge</code> gera o crate Rust e um wrapper, para que quem chama mantenha a API original. Você ou o seu agente escrevem o Rust.
    </td>
    <td width="25%" valign="top">
      <b>Verificar</b><br>
      <code>diff</code> reproduz cada caso registado contra o Rust, e nada é publicado até todos coincidirem e o <code>gate</code> passar.
    </td>
    <td width="25%" valign="top">
      <b>Lift</b><br>
      Quando o replay passa sem fallback e todas as unidades passam o gate, <code>app lift</code> transforma-a numa aplicação em Rust puro. O original nunca é alterado.
    </td>
  </tr>
</table>

**Reescreva toda a sua aplicação em Rust, uma peça de cada vez.** O rust-anything regista o comportamento atual da sua aplicação Node.js (TypeScript/JavaScript), Python, C, Go, Ruby ou Java e, em seguida, compara cada parte da reescrita em Rust com esse registo antes de avançar, até que a aplicação seja totalmente escrita em Rust. CLIs, servidores HTTP e bibliotecas; o seu agente de programação (ou você) escreve o código Rust. A ferramenta só consegue detetar aquilo que o registo abrange: consulte [O que não consegue detetar](how-it-works.md#what-it-cant-catch). Só precisa de acelerar a parte mais lenta? Migre apenas essa parte, um caminho crítico de cada vez:

| Carga de trabalho real, mesmos resultados do original | Original | Com rust-anything |
| --- | ---: | ---: |
| Pesquisa na barra lateral do x18n, 50.000 chaves (TypeScript, Node) | 10 ms por consulta | **0,5 ms: 20x mais rápido** |
| A mesma pesquisa no navegador (wasm, uma thread) | 10 ms | **0,95 ms: 11,5x** |
| `slugify` em 20.000 strings (Python) | 98 ms | **17x mais rápido** |
| `Shellwords.shellescape` da biblioteca padrão do Ruby, 50.000 valores | 88 ms | **11,8x mais rápido** |
| `Slugify` em 50.000 strings (Go) | 49 ms | **4x mais rápido** |
| `slugify` em 50.000 strings (Java) | 43 ms | **3x mais rápido** |

Medições realizadas em 4 núcleos; cada valor tem um benchmark em [`demo/`](../demo) que primeiro verifica se os resultados são idênticos. Chamar Rust uma vez por cada pequena função costuma ser *mais lento* do que utilizar um JIT; o rust-anything identifica o ciclo que realmente vale a pena migrar e transfere-o.

> O rust-anything é um projeto independente, sem qualquer afiliação ou aprovação do Rust Project ou da Rust Foundation. "Rust" é uma marca registada da Rust Foundation.

> [!WARNING]
> `record` e `app record` executam o seu código original, chamando cada função (ou a aplicação completa) com
> muitas entradas geradas, e `app record` executa cada caso duas vezes. `diff`, `gate` e `app replay`
> voltam a executar os mesmos casos, na migração ou no original. Código que escreve ficheiros, comunica com uma
> base de dados ou acede à rede fá-lo uma vez por caso. Execute-os numa cópia do projeto, com uma
> base de dados de teste e sem credenciais de produção no ambiente.

## Início rápido

Cole isto no Claude Code, Codex, Cursor ou Gemini CLI e deixe o seu agente instalá-lo:

```text
Install rust-anything: run `cargo install rust-anything`, then `rust-anything setup`.
Tell me when to restart you.
```

Ou faça a instalação manualmente:

```sh
cargo install rust-anything   # a partir do crates.io
rust-anything setup           # regista a ferramenta no Claude Code, Codex, Cursor e Gemini CLI
```

Ainda não tem a toolchain do Rust? Os binários pré-compilados para macOS, Linux e Windows estão disponíveis em cada [lançamento no GitHub](https://github.com/erkamyaman/rust-anything/releases). Para projetos JavaScript, `npm install -g rust-anything` instala o mesmo binário.

No Claude Code, também pode instalá-lo como plugin (o servidor MCP e a skill; a CLI acima deve estar disponível no seu `PATH`):

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

O comando `setup` adiciona o servidor MCP a cada agente que encontrar e instala a skill do fluxo de trabalho no local onde o agente procura as suas skills. Preserva as suas outras configurações, pode ser executado novamente em segurança e `--remove` desfaz as alterações. Depois, peça ao seu agente algo como:

- *"Reescreve esta CLI em Rust com rust-anything, de ponta a ponta, e comprova que se comporta da mesma forma."*
- *"Esta pesquisa parece lenta. Analisa o desempenho com rust-anything e transfere o ciclo mais pesado para Rust."*
- *"Planeia a migração de `src/parser/` para Rust com rust-anything; migra a primeira unidade e comprova que funciona."*
- *"Compara o port Rust em `rust/text-rs` com o original e diz-me se está pronto para produção."*

### Compatibilidade com agentes

| Agente | O que `rust-anything setup` adiciona |
| --- | --- |
| Claude Code | Servidor MCP em `~/.claude.json`, skill em `~/.claude/skills/` |
| Codex | Servidor MCP em `~/.codex/config.toml`, skill em `~/.codex/skills/` |
| Cursor | Servidor MCP em `~/.cursor/mcp.json` |
| Gemini CLI | Servidor MCP em `~/.gemini/settings.json` |
| Qualquer outro cliente MCP | Adicione manualmente `rust-anything mcp` como servidor stdio (consulte [Instalação](install.md#install)) |

Antes de modificar um ficheiro existente, `setup` guarda uma cópia ao lado dele (`<file>.rust-anything.bak`).

Ou execute as etapas manualmente:

<p align="center">
  <img src="assets/terminal.gif" alt="O perfil do rust-anything encontra fuzzyScore num ciclo, bridge --batch gera o crate Rust, gate é aprovado com 2000/2000 casos registados e o benchmark mostra um desempenho 20,7x superior ao TypeScript." width="820">
  <br>
  <sub>As etapas no <code>fuzzyScore</code> do x18n: <code>profile</code> encontra o ciclo crítico, <code>bridge --batch</code> gera o crate, <code>gate</code> é aprovado e o benchmark compara-o com o TypeScript.</sub>
</p>

```sh
rust-anything profile . -- node bench.mjs              # o que está lento e quem o chama
rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text
# escreva src/port.rs (ou deixe o seu agente fazê-lo) e depois:
rust-anything record rust/fuzzy-rs && rust-anything build rust/fuzzy-rs
rust-anything diff rust/fuzzy-rs                       # todos os casos registados devem corresponder
rust-anything bench rust/fuzzy-rs                      # está mais rápido? o resultado dirá
rust-anything gate rust/fuzzy-rs                       # concluído significa que este teste passa
```

O rust-anything não traduz código: o seu agente de programação (Claude Code, Codex, Cursor, ...) ou você escreve o código Rust. O rust-anything fornece aquilo que um agente não consegue adivinhar de forma fiável: onde o tempo é gasto, o que deve ser migrado primeiro e em que ordem, uma ponte para que o restante código continue a chamar Rust com a mesma API, o comportamento exato que a migração deve preservar, uma forma segura de disponibilizar a nova implementação e uma verificação que determina quando a migração está concluída.

## Linguagens

| Linguagem | Unidade | Ponte | Registado e reproduzido por |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Pasta ou ficheiro | napi-rs ou wasm-bindgen com um protocolo binário gerado; inclui classes e operações assíncronas | Node, executando diretamente os ficheiros `.ts` |
| Python | Pacote ou módulo | PyO3 (abi3), conversão direta, exceções tipadas | Python, importando o original |
| C | Ficheiro `.c` | Funções `extern "C"` com os nomes originais e um header | O original compilado como biblioteca partilhada, chamado através de ctypes, uma execução independente por caso |
| Go | Pacote | Chamadas cgo tipadas e um protocolo binário gerado, num pacote com a API original | Um driver Go gerado, compilado como módulo independente |
| Ruby | Ficheiro ou pasta de funções de módulo | Uma extensão magnus nos módulos originais | Um driver Ruby gerado |
| Java | Pacote de métodos `public static` | JNI tipado e um protocolo binário gerado, em classes wrapper com os métodos originais | Um driver de reflexão gerado |
| PHP (pré-visualização) | Pasta de funções e métodos `public static` | Próximo passo: PHP FFI através de uma ABI C | Aplicações completas atualmente: `app record`, `app replay` e a front door, para CLIs e servidores (`php -S`) |
| C# (pré-visualização) | Pasta de métodos `public static` | Próximo passo: P/Invoke através de uma ABI C | Aplicações completas atualmente: `app record`, `app replay` e a front door, para CLIs e servidores ASP.NET (`dotnet <app>.dll`) |

Todas as linguagens, exceto C, também recebem classes de processamento em lote (`--batch`): os elementos atravessam a ponte uma única vez e permanecem em Rust, e cada chamada processa todos eles em todos os núcleos.

A linguagem é detetada a partir do repositório (`go.mod`, `pom.xml`, um `.csproj` ou `.sln`, `pyproject.toml`, `Gemfile`, `composer.json`, `package.json` ou, caso contrário, os ficheiros de código-fonte mais comuns). A opção `--lang` permite substituir essa deteção. Os tipos são obtidos a partir do código-fonte: tipos de TS, anotações de Python, assinaturas de C, Go, Java e C#, tags YARD de Ruby, tipos PHP e phpdoc. Parâmetros sem tipo que são utilizados como números são inferidos como números.

O PHP está em pré-visualização: `analyze` planeia uma aplicação PHP (pacotes Composer e carregamento automático PSR-4, riscos como comparações flexíveis e overflow de inteiros para floats, pacotes Composer associados a crates), e uma CLI ou servidor PHP pode ser registado, reproduzido e colocado atrás de uma front door em Rust. A migração das suas unidades para Rust através do FFI do PHP é o próximo passo ([issue #2](https://github.com/erkamyaman/rust-anything/issues/2)).

O C# está em pré-visualização da mesma forma: `analyze` lê projetos `.csproj` e os respetivos pacotes NuGet (associados a crates), resolve diretivas `using` para pastas e identifica diferenças do C# (`decimal`, `dynamic`, consultas adiadas do LINQ, `Task`, `lock` e reflexão). Uma CLI C# ou um servidor ASP.NET pode ser registado, reproduzido e colocado atrás de uma front door em Rust; a migração de unidades através de P/Invoke vem a seguir.

## Saber mais

- [Como funciona](how-it-works.md): o ciclo, o que `bridge` e `analyze` decidem, caminhos críticos e o que não consegue detetar
- [Migrar uma aplicação completa](whole-apps.md): servidores, CLIs e bibliotecas, da front door até ao `app lift`
- [Exemplos e estudos de caso](examples.md): x18n e código real em seis linguagens
- [Migrações reais](real-world.md): json-server, Fuse.js, vercel/ms, com benchmarks
- [Instalar, atualizar e desenvolver](install.md): todos os métodos de instalação, Windows e compilação a partir do código-fonte
- [Perguntas frequentes](faq.md) · [Estado](STATUS.md) · [Ferramentas MCP](tools.md)

## Patrocinadores

<p align="center">
  O <a href="https://github.com/erkamyaman/rust-anything">rust-anything</a> é gratuito e de código aberto. Se for útil para o seu trabalho, considere patrociná-lo.<br>
  O seu apoio ajuda a manter o desenvolvimento.
</p>

<p align="center">
  <a href="https://coderabbit.ai">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/company-dark.svg">
      <img src="assets/sponsors/company-light.svg" width="800" alt="Patrocinador empresarial: CodeRabbit">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/sponsors/sponsors-dark.svg">
      <img src="assets/sponsors/sponsors-light.svg" width="800" alt="Patrocinadores: Ali Yilmaz, Ashley Hunter, Fatih Uzuntaş">
    </picture>
  </a>
</p>

<p align="center">
  <a href="https://github.com/sponsors/erkamyaman"><img src="https://img.shields.io/badge/Tornar--se_patrocinador-%E2%9D%A4-db61a2?style=for-the-badge&logo=githubsponsors&logoColor=white" alt="Tornar-se patrocinador"></a>
</p>

## Contribuir

Issues e pull requests são bem-vindos; [CONTRIBUTING.md](../CONTRIBUTING.md) explica a configuração, onde colocar os testes e como adicionar uma linguagem. As decisões de design e as respetivas justificações estão em [DECISIONS.md](../DECISIONS.md). Estudos de caso de migrações reais são especialmente bem-vindos. Por favor, comunique problemas de segurança de forma privada ([SECURITY.md](../SECURITY.md)) e consulte o [código de conduta](../CODE_OF_CONDUCT.md).

## Licença

[MIT](../LICENSE) © [Erkam Yaman](https://github.com/erkamyaman). As migrações e o código gerado no seu repositório pertencem-lhe.
