import { useEffect, useRef, useState } from 'react'
import { GitHubIcon } from '../site/icons'
import { REPO_URL } from '../../lib/site'
import { fill, withLocale } from '../../lib/locales'
import type { Messages } from '../../lib/messages/en'

type Lang = {
  name: string
  file: string
  bridge: string
  proof?: { what: string; total: number }
}

const LANGS: Lang[] = [
  {
    name: 'TypeScript',
    file: 'src/words.ts',
    bridge: 'napi-rs',
    proof: { what: 'Fuse.js', total: 3414 },
  },
  {
    name: 'JavaScript',
    file: 'lib/words.js',
    bridge: 'napi-rs',
    proof: { what: 'json-server', total: 514 },
  },
  {
    name: 'Python',
    file: 'words.py',
    bridge: 'PyO3',
    proof: { what: 'CPython colorsys', total: 2400 },
  },
  {
    name: 'Go',
    file: 'words.go',
    bridge: 'cgo',
    proof: { what: 'Go path', total: 2642 },
  },
  {
    name: 'Ruby',
    file: 'words.rb',
    bridge: 'magnus',
    proof: { what: 'shellwords', total: 1617 },
  },
  {
    name: 'Java',
    file: 'Words.java',
    bridge: 'JNI',
    proof: { what: 'JDK facade', total: 4492 },
  },
  {
    name: 'C',
    file: 'words.c',
    bridge: 'extern "C"',
    proof: { what: 'glibc facade', total: 1859 },
  },
  {
    name: 'PHP',
    file: 'words.php',
    bridge: 'ext-ffi',
    proof: { what: 'PEAR XML_Util', total: 1838 },
  },
  {
    name: 'C#',
    file: 'Words.cs',
    bridge: 'P/Invoke',
    proof: { what: 'Google.Protobuf', total: 2941 },
  },
  {
    name: 'Kotlin',
    file: 'Words.kt',
    bridge: 'JNI',
  },
  {
    name: 'PowerShell',
    file: 'Words.psm1',
    bridge: 'P/Invoke',
  },
]

const SHOWN = [0, 2, 3, 5]

function useCount(target: number, run: number, delay: number) {
  const [n, setN] = useState(target)
  useEffect(() => {
    if (matchMedia('(prefers-reduced-motion: reduce)').matches) {
      setN(target)
      return
    }
    setN(0)
    let raf = 0
    const start = performance.now() + delay
    const tick = (t: number) => {
      const k = Math.min(1, Math.max(0, (t - start) / 1300))
      setN(Math.round(target * (1 - Math.pow(1 - k, 3))))
      if (k < 1) raf = requestAnimationFrame(tick)
    }
    raf = requestAnimationFrame(tick)
    return () => cancelAnimationFrame(raf)
  }, [target, run, delay])
  return n
}

export default function Hero({ t, locale = 'en' }: { t: Messages['hero']; locale?: string }) {
  const [i, setI] = useState(0)
  const [run, setRun] = useState(0)
  const [copied, setCopied] = useState(false)
  const tabs = useRef<(HTMLButtonElement | null)[]>([])
  const lang = LANGS[i]
  const count = useCount(lang.proof?.total ?? 0, run, 250)

  useEffect(() => {
    if (matchMedia('(prefers-reduced-motion: reduce)').matches) return
    const t = window.setTimeout(() => {
      setI((v) => SHOWN[(SHOWN.indexOf(v) + 1) % SHOWN.length])
      setRun((r) => r + 1)
    }, 5200)
    return () => clearTimeout(t)
  }, [i, run])

  const pick = (n: number) => {
    setI(n)
    setRun((r) => r + 1)
  }

  const onKey = (e: React.KeyboardEvent) => {
    if (!['ArrowRight', 'ArrowLeft', 'ArrowDown', 'ArrowUp'].includes(e.key)) return
    e.preventDefault()
    const at = SHOWN.indexOf(i)
    const row = (at + (e.key === 'ArrowDown' || e.key === 'ArrowRight' ? 1 : SHOWN.length - 1)) % SHOWN.length
    pick(SHOWN[row])
    tabs.current[row]?.focus()
  }

  return (
    <section className="hero" aria-labelledby="hero-title">
      <div className="wrap hero-grid">
        <div className="hero-copy">
          <h1 id="hero-title">
            {t.title1}
            <span className="line2">{t.title2}</span>
          </h1>
          <p className="sub">{t.sub}</p>
          <div className="actions">
            <a className="btn btn-primary" href={withLocale('/docs/introduction/quick-start', locale)}>
              {t.cta}
            </a>
            <a className="btn btn-quiet" href={REPO_URL}>
              <GitHubIcon />
              {t.source}
            </a>
          </div>
          <div className="agent-prompt">
            <span className="agent-trail" aria-hidden>
              <svg>
                <rect pathLength={100} />
              </svg>
              <span className="walker">
                <img src="/logo-mark.png" alt="" width={16} height={16} />
              </span>
            </span>
            <div className="agent-head">
              <span className="agent-label">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
                  <rect x="4" y="8" width="16" height="12" rx="3" />
                  <path d="M12 8V4M10 4h4M9 13v1.5M15 13v1.5M2 13v3M22 13v3" />
                </svg>
                {t.agentLabel}
              </span>
              <button
                type="button"
                onClick={() => {
                  navigator.clipboard?.writeText(t.agentPrompt).then(
                    () => {
                      setCopied(true)
                      setTimeout(() => setCopied(false), 1800)
                    },
                    () => {},
                  )
                }}
              >
                <span className="swap-label">
                  <span aria-hidden={copied} data-on={!copied}>{t.copy}</span>
                  <span aria-hidden={!copied} data-on={copied}>{t.copied}</span>
                </span>
              </button>
            </div>
            <p>{t.agentPrompt}</p>
          </div>
        </div>

        <div className="flow">
          <div className="flow-card">
            <div className="flow-grid">
              <div className="flow-col">
                <div className="flow-label">{t.sourceLanguage}</div>
                <div role="tablist" aria-label={t.sourceLanguage} onKeyDown={onKey} className="flow-list">
                  {SHOWN.map((n, row) => {
                    const l = LANGS[n]
                    return (
                      <button
                        key={l.name}
                        ref={(el) => {
                          tabs.current[row] = el
                        }}
                        role="tab"
                        type="button"
                        aria-selected={n === i}
                        tabIndex={n === i ? 0 : -1}
                        className={`flow-node${n === i ? ' on' : ''}`}
                        onClick={() => pick(n)}
                      >
                        {l.name}
                        {n === i && <span className="timer" key={run} aria-hidden />}
                      </button>
                    )
                  })}
                </div>
              </div>

              <div className="flow-core">
                <div className="flow-core-title">
                  <img src="/logo-mark.png" width={22} height={22} alt="" />
                  <span>
                    <span className="rust">Rust</span> Anything
                  </span>
                </div>
                <div className="flow-step done">
                  record<span>{lang.proof ? lang.proof.total.toLocaleString('en-US') : '...'}</span>
                </div>
                <div className={`flow-step${lang.proof && count < lang.proof.total ? ' on' : ' done'}`}>
                  diff
                  <span>{lang.proof ? `${count.toLocaleString('en-US')} / ${lang.proof.total.toLocaleString('en-US')}` : '...'}</span>
                </div>
                <div className={`flow-step${lang.proof && count === lang.proof.total ? ' on' : ''}`}>
                  gate<span>{lang.proof && count === lang.proof.total ? 'pass' : ''}</span>
                </div>
              </div>

              <div className="flow-col">
                <div className="flow-label">{t.bridge}</div>
                <div className="flow-list">
                  {SHOWN.map((n) => (
                    <div key={n} className={`flow-node out${n === i ? ' on' : ''}`}>
                      {LANGS[n].bridge}
                    </div>
                  ))}
                </div>
              </div>
            </div>
            <div className="flow-also">
              {LANGS.filter((_, n) => !SHOWN.includes(n))
                .map((l) => l.name)
                .join(', ')}
            </div>
          </div>

          <div className="flow-result">
            <span className="flow-chip">{lang.name}</span>
            <code>{lang.file}</code>
            <span className="what">
              {lang.proof
                ? fill(t.verified, {
                    lang: lang.name,
                    what: lang.proof.what,
                    count: `${count.toLocaleString('en-US')} / ${lang.proof.total.toLocaleString('en-US')}`,
                  })
                : fill(t.tested, { lang: lang.name })}
            </span>
            <span className={`state${lang.proof && count === lang.proof.total ? ' ok' : ''}`}>
              {lang.proof && count === lang.proof.total ? 'match' : 'replaying'}
            </span>
          </div>
          <div className="flow-foot">
            Prettier, 2,360 files <b>128.6 s → 6.9 s</b> <b>19x faster</b>
          </div>
        </div>
      </div>
    </section>
  )
}
