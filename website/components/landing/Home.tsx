import { REPO_URL } from '../../lib/site'
import { messages, withLocale, fill } from '../../lib/i18n'
import Rich from '../Rich'
import Hl from '../Hl'
import CONTRIBUTORS from '../../content/contributors.json'
import './landing.css'

const STEP_CMDS = ['record', 'bridge', 'diff, gate', 'app lift']

const HOW_DOCS = [
  [['Record', '/docs/concepts/record'], ['Coverage', '/docs/concepts/coverage']],
  [['Bridge', '/docs/concepts/bridge'], ['Port', '/docs/concepts/port']],
  [['Diff', '/docs/concepts/diff'], ['Gate', '/docs/concepts/gate']],
  [['Switch on and off', '/docs/concepts/switch'], ['Port a whole app', '/docs/whole-apps/overview']],
]

function Head({ id, title, desc, locale, dark }: { id: string; title: string; desc: string; locale: string; dark?: boolean }) {
  const cut = /^(.+?[.。!?])\s*([\s\S]+)$/.exec(desc)
  const short = !!cut && cut[1].length <= 34 && !cut[1].includes('`')
  const lead = short && cut ? cut[1] : ''
  const rest = short && cut ? cut[2] : desc
  return (
    <div className={`section-head${dark ? ' on-dark' : ''}`}>
      <h2 id={id} className="section-title">
        {title}
        {lead && (
          <>
            {/[.。!?]$/.test(title) ? ' ' : /[\u3040-\u30ff\u4e00-\u9fff]/.test(title) ? '。' : '. '}
            <span className="soft">{lead}</span>
          </>
        )}
      </h2>
      <p className="section-desc">
        <Rich text={rest} locale={locale} />
      </p>
    </div>
  )
}

const LANGS = [
  { key: 'typescript', name: 'TypeScript/JavaScript' },
  { key: 'python', name: 'Python' },
  { key: 'c', name: 'C' },
  { key: 'go', name: 'Go' },
  { key: 'ruby', name: 'Ruby' },
  { key: 'java', name: 'Java' },
  { key: 'php', name: 'PHP' },
  { key: 'csharp', name: 'C#' },
  { key: 'kotlin', name: 'Kotlin', tested: true },
  { key: 'powershell', name: 'PowerShell', tested: true },
] as const

const CODERABBIT = 'https://coderabbit.link/er-kam'
const ATAK_DOMAIN = 'https://www.atakdomain.com/'

const SPONSORS = [
  { login: 'santoshyadavdev', name: 'Santosh Yadav' },
  { login: 'hakansoyturk', name: 'Hakan Soytürk' },
  { login: 'aliyilmazco', name: 'Ali Yilmaz' },
  { login: 'ashley-hunter', name: 'Ashley Hunter' },
  { login: 'fatihuzuntas', name: 'Fatih Uzuntaş' },
  { login: 'bugrcvi', name: 'bugrcvi' },
]

const BENCH = [
  ['GET /posts', '11.1k / 10.8k', '27.3k / 23.9k', '19.7k / 20.4k', '115.0k / 110.7k'],
  ['GET /posts/1', '11.0k / 11.3k', '27.7k / 25.4k', '21.9k / 22.6k', '119.9k / 99.0k'],
  ['?_embed=comments', '8.7k / 6.6k', '22.5k / 21.8k', '13.8k / 11.0k', '98.3k / 100.6k'],
]

export default function Home({ hero, install, locale = 'en' }: { hero: React.ReactNode; install: React.ReactNode; locale?: string }) {
  const m = messages(locale).home
  const L = (p: string) => withLocale(p, locale)
  return (
    <div className="home">
      {hero}
      <div className="wrap">
        <dl className="ledger">
          {m.stats.map((x) => (
            <div key={x.k} className="ledger-row">
              <dt>
                {x.v}
                {x.unit && <small>{x.unit}</small>}
              </dt>
              <dd>{x.k}</dd>
            </div>
          ))}
        </dl>
      </div>
      <div className="band">
        <div className="wrap">
          <p>
            {m.sponsoredBy} <a href={CODERABBIT}>CodeRabbit</a> and <a href={ATAK_DOMAIN}>Atak Domain</a>.
          </p>
          <p className="band-sub">
            {m.bandText} <a href="https://github.com/sponsors/erkamyaman">{m.bandLink}</a>
          </p>
        </div>
      </div>

      <section className="section" aria-labelledby="prettier-title">
        <div className="wrap">
          <Head id="prettier-title" title={m.prettierTitle} desc={m.prettierDesc} locale={locale} />
          <div className="race">
            <div className="race-card">
              <div className="race-lane">
                <div className="race-top">
                  <b>Prettier</b>
                  <span className="race-chip">Node 24</span>
                </div>
                <code className="race-cmd">$ prettier &lt;file&gt;</code>
                <div className="race-time">3.63 s</div>
                <div className="race-bar">
                  <span className="slow" />
                </div>
                <div className="race-meta">59 / 59 {m.raceFiles}</div>
              </div>
              <div className="race-lane rust">
                <div className="race-top">
                  <b>
                    <span className="rust-word">Rust</span> port
                  </b>
                  <span className="race-chip">one binary</span>
                </div>
                <code className="race-cmd">$ prettier-rs &lt;file&gt;</code>
                <div className="race-time">0.20 s</div>
                <div className="race-bar">
                  <span className="fast" />
                </div>
                <div className="race-meta">59 / 59 {m.raceFiles}</div>
              </div>
            </div>
            <div className="race-result">
              <span className="what">{m.raceSame}</span>
              <code>59 / 59</code>
              <span className="state ok">match</span>
              <span className="race-x">
                <b>18x</b> {m.raceFaster}
              </span>
              <a href="https://github.com/erkamyaman/rust-anything/blob/main/docs/real-world.md#prettier">{m.raceHow}</a>
            </div>
          </div>
        </div>
      </section>

      <section className="section dark-band" aria-labelledby="install-title">
        <div className="wrap">
          <Head id="install-title" title={m.installTitle} desc={m.installDesc} locale={locale} dark />
          <ol className="install-steps">
            {m.installSteps.map((st, i) => (
              <li key={st.title}>
                <span className="n">{String(i + 1).padStart(2, '0')}</span>
                <h3>{st.title}</h3>
                {i === 0 ? <div className="install">{install}</div> : null}
                <p>
                  <Rich text={st.text} />
                </p>
              </li>
            ))}
          </ol>
          <div className="cmd-table">
            {m.commands.map((c) => (
              <div key={c.cmd} className="cmd-row">
                <code>{c.cmd}</code>
                <span className={`chip c-${c.tag.toLowerCase()}`}>{c.tag}</span>
                <span className="cmd-text">{c.text}</span>
              </div>
            ))}
          </div>
          <p className="note">
            <Rich text={fill(m.installNote, { releases: `${REPO_URL}/releases` })} />
          </p>
        </div>
      </section>

      <section className="section" aria-labelledby="loop-title">
        <div className="wrap">
          <Head id="loop-title" title={m.loopTitle} desc={m.loopDesc} locale={locale} />
          <div className="how">
            {m.steps.map((_, i) => (
              <input key={i} type="radio" name="ra-steps" id={`ra-step-${i}`} className="how-radio" defaultChecked={i === 0} />
            ))}
            <div className="how-tabs">
              {m.howTabs.map((t, i) => (
                <label key={t} htmlFor={`ra-step-${i}`} className={`how-tab ra-st-${i}`}>
                  <span className="n">{String(i + 1).padStart(2, '0')}</span>
                  {t}
                </label>
              ))}
            </div>
            <div className="how-panel">
              <div className="how-canvas">
                <div className="how-flow">
                  <div className="how-node">
                    <b>{m.howYourCode}</b>
                    <span>{m.howYourCodeSub}</span>
                  </div>
                  <div className="how-link" aria-hidden />
                  <div className="how-node core">
                    <div className="how-swap">
                      {m.steps.map((_, i) => (
                        <div key={i} className={`ra-sv-${i}`}>
                          <b>
                            <span className="rust">Rust</span> Anything <code>{STEP_CMDS[i]}</code>
                          </b>
                          <ul>
                            {m.howBullets[i].map((x) => (
                              <li key={x}>{x}</li>
                            ))}
                          </ul>
                        </div>
                      ))}
                    </div>
                  </div>
                  <div className="how-link" aria-hidden />
                  <div className="how-node">
                    <b>{m.howPort}</b>
                    <span>{m.howPortSub}</span>
                  </div>
                </div>
                <div className="how-example how-swap">
                  {m.steps.map((_, i) => (
                    <code key={i} className={`ra-sv-${i}`}>
                      <Hl code={m.howExample[i]} />
                    </code>
                  ))}
                </div>
              </div>
              <div className="how-aside how-swap">
                {m.steps.map((s, i) => (
                  <aside key={i} className={`ra-sv-${i}`}>
                    <h3>{s.title}</h3>
                    <p>
                      <Rich text={s.text} />
                    </p>
                    <div className="how-docs-label">{m.howDocs}</div>
                    <ul className="how-docs">
                      {HOW_DOCS[i].map(([t, href]) => (
                        <li key={href}>
                          <a href={L(href)}>{t}</a>
                        </li>
                      ))}
                    </ul>
                  </aside>
                ))}
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="section" aria-labelledby="demo-title">
        <div className="wrap">
          <Head id="demo-title" title={m.demoTitle} desc={m.demoDesc} locale={locale} />
          <div className="panel">
            <div className="titlebar">
              <span>~/x18n</span>
            </div>
            <div className="split">
              <div>
                <pre>
                  <span className="c">{m.demoComments.profile}</span>
                  {'\n'}
                  <span className="p">$</span> <Hl code="rust-anything profile . -- node bench.mjs" />{'\n'}
                  <span className="p">$</span> <Hl code="rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text" />{'\n'}
                  <span className="c">{m.demoComments.write}</span>
                  {'\n'}
                  <span className="p">$</span> <Hl code="rust-anything record rust/fuzzy-rs" />{'\n'}
                  <span className="p">$</span> <Hl code="rust-anything build rust/fuzzy-rs" />{'\n'}
                  <span className="p">$</span> <Hl code="rust-anything diff rust/fuzzy-rs" />{'\n'}
                  <span className="p">$</span> <Hl code="rust-anything bench rust/fuzzy-rs" />{'\n'}
                  <span className="p">$</span> <Hl code="rust-anything gate rust/fuzzy-rs" />
                </pre>
              </div>
              <div>
                <pre>
                  <span className="c">{m.demoComments.first}</span>
                  {'\n'}
                  {m.demoComments.matchLine}
                  {'\n'}
                  <span className="bad">- fuzzyScore("b😀", "a😀b😀c"): expected {'{"ok":1997}'}, got {'{"ok":1998}'}</span>
                  {'\n\n'}
                  <span className="c">{m.demoComments.why}</span>
                  {'\n\n'}
                  <span className="ok">{m.demoComments.fixed}</span>
                </pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="section" aria-labelledby="ports-title">
        <div className="wrap">
          <Head id="ports-title" title={m.portsTitle} desc={m.portsDesc} locale={locale} />

          <div className="ports">
            {m.ports.map((p) => (
              <a key={p.href} href={L(p.href)} className="port-card">
                <div className="port-top">
                  <span className="port-kind">{p.kind}</span>
                  <span className="badge">{m.verified}</span>
                </div>
                <div className="big">{p.big}</div>
                <p>
                  <Rich text={p.text} />
                </p>
                <dl>
                  {p.rows.map((r) => (
                    <Row key={r.k} k={r.k} v={r.v} slow={'slow' in r && !!r.slow} />
                  ))}
                </dl>
              </a>
            ))}
          </div>

          <p className="ports-legend">
            <span className="slow-dot" aria-hidden /> {m.slowLegend}
          </p>

          <details className="panel bench-panel">
            <summary className="titlebar">
              <span>{m.benchTitle}</span>
              <svg className="chev" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" aria-hidden>
                <path d="M6 9l6 6 6-6" />
              </svg>
            </summary>
            <div className="overflow-x-auto">
              <table className="bench">
                <thead>
                  <tr>
                    {m.benchHead.map((h) => (
                      <th key={h}>{h}</th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {BENCH.map((r) => (
                    <tr key={r[0]}>
                      <td>{r[0]}</td>
                      <td>{r[1]}</td>
                      <td className="rust">{r[2]}</td>
                      <td>{r[3]}</td>
                      <td className="rust">{r[4]}</td>
                    </tr>
                  ))}
                  <tr>
                    <td>{m.benchPostRow}</td>
                    <td></td>
                    <td></td>
                    <td>2,973 / 2,467</td>
                    <td className="rust">5,821 / 5,976</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <p className="panel-note">
              <Rich text={m.benchNote} locale={locale} />
            </p>
          </details>
        </div>
      </section>

      <section className="section" aria-labelledby="langs-title">
        <div className="wrap">
          <Head id="langs-title" title={m.langsTitle} desc={m.langsDesc} locale={locale} />
          <ul className="langs">
            {LANGS.map((l) => (
              <li key={l.key}>
                <a href={'tested' in l ? `${REPO_URL}#languages` : L(`/docs/languages/${l.key}`)} className="lang">
                  <span className="lang-top">
                    <span className={`badge${'tested' in l ? ' preview' : ''}`}>{'tested' in l ? m.tested : m.verified}</span>
                  </span>
                  <span className="name">{l.name.replace('/', '/\u200b')}</span>
                  <LangMeta text={m.langMeta[l.key]} />
                </a>
              </li>
            ))}
          </ul>
        </div>
      </section>

      <section className="section" aria-labelledby="people-title">
        <div className="wrap">
          <Head id="people-title" title={fill(m.contributorsTitle, { n: String(CONTRIBUTORS.length) })} desc={m.contributorsDesc} locale={locale} />
          <div className="people-row">
            <ul className="people">
              {CONTRIBUTORS.slice(0, 24).map((p) => (
                <li key={p.login}>
                  <a href={`https://github.com/${p.login}`} className="person" title={p.login}>
                    <img src={`https://github.com/${p.login}.png?size=64`} width={36} height={36} alt={p.login} loading="lazy" decoding="async" />
                  </a>
                </li>
              ))}
            </ul>
            <a className="people-all" href={`${REPO_URL}/graphs/contributors`}>
              {fill(m.contributorsAll, { n: String(CONTRIBUTORS.length) })}
            </a>
          </div>
        </div>
      </section>

      <section className="section last" aria-labelledby="sponsors-title">
        <div className="wrap">
          <Head id="sponsors-title" title={m.sponsorsTitle} desc={m.sponsorsDesc} locale={locale} />
          <div className="sponsor-tiers">
            <a href={CODERABBIT} className="sponsor-tier">
              <span className="sponsor-co">
                <img src="/brands/coderabbit.png" width={36} height={36} alt="" loading="lazy" />
                CodeRabbit
              </span>
              <span className="sponsor-role">{m.companySponsor}</span>
            </a>
            <a href={ATAK_DOMAIN} className="sponsor-tier">
              <span className="sponsor-co">
                <img className="only-light" src="/brands/atak-domain.svg" width={150} height={34} alt="Atak Domain" loading="lazy" />
                <img className="only-dark" src="/brands/atak-domain-dark.svg" width={150} height={34} alt="Atak Domain" loading="lazy" />
              </span>
              <span className="sponsor-role">{m.domainSponsor}</span>
            </a>
          </div>
          <div className="sponsor-people-row">
            <span className="sponsor-role">{m.peopleSponsors}</span>
            <ul className="sponsor-people">
              {SPONSORS.map((p) => (
                <li key={p.login}>
                  <a href={`https://github.com/${p.login}`}>
                    <img src={`https://github.com/${p.login}.png?size=64`} width={28} height={28} alt="" loading="lazy" decoding="async" />
                    {p.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>
          <div className="sponsor-cta">
            <a className="btn btn-primary" href="https://github.com/sponsors/erkamyaman">
              {m.becomeSponsor}
            </a>
            <a className="btn btn-quiet" href="https://buymeacoffee.com/erkamyaman">
              {m.buyCoffee}
            </a>
          </div>
          <p className="sponsor-mail">
            {m.sponsorEmail} <a href="mailto:hello@rustanything.ai">hello@rustanything.ai</a>
          </p>
        </div>
      </section>

      <section className="cta" aria-labelledby="cta-title">
        <div className="wrap">
          <h2 id="cta-title">
            {m.ctaTitle1} <span className="soft">{m.ctaTitle2}</span>
          </h2>
          <p>{m.ctaSub}</p>
          <div className="cta-actions">
            <a className="btn btn-light" href={L('/docs/introduction/quick-start')}>
              {messages(locale).hero.cta}
            </a>
            <a className="btn btn-ghost" href={L('/docs/introduction/what-is-rust-anything')}>
              {m.ctaDocs}
            </a>
          </div>
        </div>
      </section>
    </div>
  )
}

function Row({ k, v, slow }: { k: string; v: string; slow: boolean }) {
  return (
    <>
      <dt className={slow ? 'slow' : undefined}>{k}</dt>
      <dd className={slow ? 'slow' : undefined}>{v}</dd>
    </>
  )
}

function LangMeta({ text }: { text: string }) {
  const cut = /\.\s|。/.exec(text)
  const bridge = cut ? text.slice(0, cut.index) : ''
  const rest = cut ? text.slice(cut.index + cut[0].length).trim() : text
  const count = /([\d][\d,.\s]*\/\s*[\d][\d,.\s]*)$/.exec(rest)
  const what = count ? rest.slice(0, count.index).replace(/[:：]\s*$/, '').trim() : rest
  return (
    <>
      {count && <span className="count">{count[1].trim()}</span>}
      <span className="what">{what}</span>
      {bridge && <span className="bridge">{bridge}</span>}
    </>
  )
}
