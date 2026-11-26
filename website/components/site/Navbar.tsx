import { useEffect, useRef, useState } from 'react'
import { createPortal } from 'react-dom'
import Logo from '../Logo'
import ThemeToggle from './ThemeToggle'
import SearchDialog, { SearchButton } from './SearchDialog'
import { CloseIcon, GitHubIcon, HeartIcon, MenuIcon } from './icons'
import type { NavGroup } from '../../lib/nav'
import type { Messages } from '../../lib/messages/en'
import { LOCALES, stripLocale, withLocale } from '../../lib/locales'
import { REPO_URL } from '../../lib/site'

function LanguageMenu({ currentPath, locale, label }: { currentPath: string; locale: string; label: string }) {
  const rest = stripLocale(currentPath)
  const current = LOCALES.find((l) => l.code === locale) ?? LOCALES[0]
  const ref = useRef<HTMLDetailsElement>(null)

  useEffect(() => {
    const close = () => ref.current?.removeAttribute('open')
    const onDown = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) close()
    }
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') close()
    }
    document.addEventListener('mousedown', onDown)
    document.addEventListener('keydown', onKey)
    return () => {
      document.removeEventListener('mousedown', onDown)
      document.removeEventListener('keydown', onKey)
    }
  }, [])

  return (
    <details ref={ref} className="lang-menu relative">
      <summary
        className="inline-flex h-9 cursor-pointer list-none items-center gap-1.5 rounded-md px-2 text-sm text-fg/80 hover:bg-bg-soft hover:text-fg"
        aria-label={label}
        title={label}
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden>
          <circle cx="12" cy="12" r="9" />
          <path d="M3 12h18M12 3c2.5 2.7 3.8 5.7 3.8 9s-1.3 6.3-3.8 9c-2.5-2.7-3.8-5.7-3.8-9S9.5 5.7 12 3z" />
        </svg>
        <span className="font-mono text-xs uppercase tracking-wide">{current.code.split('-')[0]}</span>
      </summary>
      <div className="absolute end-0 top-11 z-[70] w-[24rem] max-sm:fixed max-sm:inset-x-3 max-sm:top-[4.25rem] max-sm:w-auto rounded-xl border border-line bg-bg p-2 shadow-2xl">
        <div className="px-2 pb-1.5 pt-1 text-sm font-medium text-muted">{label}</div>
        <ul className="thin-scrollbar grid max-h-[75vh] grid-cols-2 gap-0.5 overflow-y-auto sm:max-h-none sm:overflow-visible">
          {LOCALES.map((l) => {
            const on = l.code === locale
            return (
              <li key={l.code}>
                <a
                  href={withLocale(rest, l.code)}
                  lang={l.lang}
                  hrefLang={l.lang}
                  aria-current={on ? 'true' : undefined}
                  className={`flex items-center gap-2 rounded-md px-2 py-1.5 text-sm ${on ? 'bg-brand-soft font-semibold text-brand-ink' : 'text-fg/85 hover:bg-bg-soft hover:text-fg'}`}
                  onClick={() => {
                    try {
                      localStorage.setItem('locale', l.code)
                    } catch {}
                  }}
                >
                  <span className="min-w-0 flex-1 truncate">{l.name}</span>
                  {on && (
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" aria-hidden>
                      <path d="M5 12l5 5L20 7" />
                    </svg>
                  )}
                </a>
              </li>
            )
          })}
        </ul>
      </div>
    </details>
  )
}

export default function Navbar({
  currentPath,
  locale = 'en',
  themeToggle = false,
  ui,
  nav,
}: {
  currentPath: string
  locale?: string
  themeToggle?: boolean
  ui: Messages['ui']
  nav: NavGroup[]
}) {
  const L = (p: string) => withLocale(p, locale)
  const LINKS = [
    { title: ui.docs, href: L('/docs/introduction/what-is-rust-anything'), match: L('/docs/') },
    { title: ui.languages, href: L('/docs/languages/overview'), match: L('/docs/languages/') },
    { title: ui.realWorld, href: L('/docs/real-world/overview'), match: L('/docs/real-world/') },
  ]
  const [searchOpen, setSearchOpen] = useState(false)
  const [menuOpen, setMenuOpen] = useState(false)

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault()
        setSearchOpen((o) => !o)
      } else if (e.key === '/' && !(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)) {
        e.preventDefault()
        setSearchOpen(true)
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  useEffect(() => {
    document.body.style.overflow = menuOpen ? 'hidden' : ''
  }, [menuOpen])

  const active = (match: string) =>
    match === LINKS[0].match
      ? currentPath.startsWith('/docs/') && !LINKS.slice(1).some((l) => currentPath.startsWith(l.match))
      : currentPath.startsWith(match)

  return (
    <div className="mx-auto flex h-16 max-w-[1440px] items-center gap-4 px-4 sm:px-6">
      <a href={L('/')} className="shrink-0 font-display text-[17px] font-bold" aria-label={ui.home}>
        <Logo size={32} />
      </a>
      <div className="ms-auto flex items-center gap-0.5 sm:gap-1">
        <nav className="me-3 hidden items-center gap-1 border-line pe-3 lg:flex lg:border-e" aria-label={ui.docs}>
          {LINKS.map((l) => (
            <a
              key={l.href}
              href={l.href}
              className={`whitespace-nowrap rounded-md px-3 py-1.5 text-sm ${active(l.match) ? 'font-semibold text-fg' : 'text-muted hover:text-fg'}`}
              aria-current={active(l.match) ? 'page' : undefined}
            >
              {l.title}
            </a>
          ))}
        </nav>
        <SearchButton ui={ui} compact onOpen={() => setSearchOpen(true)} />
        <LanguageMenu currentPath={currentPath} locale={locale} label={ui.language} />
        <a
          href={REPO_URL}
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg"
          aria-label={ui.github}
        >
          <GitHubIcon className="h-[22px] w-[22px]" />
        </a>
        <a
          href="https://github.com/sponsors/erkamyaman"
          className="group inline-flex h-9 items-center justify-center gap-1.5 rounded-md px-2 text-sm text-fg/80 hover:text-fg"
          aria-label={ui.sponsor}
          title={ui.sponsor}
        >
          <HeartIcon className="h-[18px] w-[18px] fill-transparent stroke-current stroke-2 transition-colors group-hover:fill-current motion-reduce:transition-none" />
        </a>
        <button
          type="button"
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg lg:hidden"
          onClick={() => setMenuOpen(true)}
          aria-label={ui.openMenu}
          aria-expanded={menuOpen}
        >
          <MenuIcon />
        </button>
      </div>

      {menuOpen &&
        createPortal(
        <div className="fixed inset-0 z-[65] lg:hidden" role="dialog" aria-modal="true" aria-label={ui.menu} onKeyDown={(e) => e.key === 'Escape' && setMenuOpen(false)}>
          <button type="button" tabIndex={-1} aria-label={ui.closeMenu} className="absolute inset-0 cursor-default bg-black/50" onClick={() => setMenuOpen(false)} />
          <div className="absolute inset-y-0 end-0 flex w-[min(20rem,88vw)] flex-col border-s border-line bg-bg text-fg">
            <div className="flex h-16 items-center justify-between border-b border-line px-4">
              <span className="font-display font-bold"><Logo size={26} /></span>
              <button
                type="button"
                className="inline-flex h-9 w-9 items-center justify-center rounded-md text-muted hover:text-fg"
                autoFocus
                onClick={() => setMenuOpen(false)}
                aria-label={ui.closeMenu}
              >
                <CloseIcon />
              </button>
            </div>
            <nav className="thin-scrollbar flex-1 overflow-y-auto px-3 py-4" aria-label={ui.docs}>
              <div className="mb-4 flex flex-col md:hidden">
                {LINKS.map((l) => (
                  <a key={l.href} href={l.href} className="rounded-md px-2 py-1.5 text-sm font-semibold">
                    {l.title}
                  </a>
                ))}
              </div>
              {nav.map((g) => (
                <div key={g.title} className="mb-4">
                  <div className="mb-1 px-2 text-xs font-semibold text-muted">{g.title}</div>
                  <ul>
                    {g.items.map((i) => (
                      <li key={i.path}>
                        <a
                          href={i.path}
                          className={`block rounded-md px-2 py-1.5 text-sm ${i.path === currentPath ? 'bg-brand-soft font-medium text-brand-ink' : 'text-fg/80 hover:bg-bg-soft'}`}
                        >
                          {i.title}
                        </a>
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </nav>
            {themeToggle && (
              <div className="border-t border-line px-3 py-3">
                <ThemeToggle withLabel ui={ui} />
              </div>
            )}
          </div>
        </div>,
          document.body,
        )}

      <SearchDialog open={searchOpen} onClose={() => setSearchOpen(false)} ui={ui} locale={locale} />
    </div>
  )
}
