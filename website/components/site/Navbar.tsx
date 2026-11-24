import { useEffect, useRef, useState } from 'react'
import { createPortal } from 'react-dom'
import Logo from '../Logo'
import ThemeToggle from './ThemeToggle'
import SearchDialog, { SearchButton } from './SearchDialog'
import { CloseIcon, DiscordIcon, GitHubIcon, HeartIcon, MenuIcon } from './icons'
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
        <Flag l={current} />
        <span className="hidden xl:inline">{current.name}</span>
        <svg className="hidden opacity-60 sm:block" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" aria-hidden>
          <path d="M6 9l6 6 6-6" />
        </svg>
      </summary>
      <div className="absolute end-0 top-11 z-[70] w-[24rem] max-sm:fixed max-sm:inset-x-3 max-sm:top-[4.25rem] max-sm:w-auto rounded-xl border border-line bg-bg p-2 shadow-2xl">
        <div className="px-2 pb-1.5 pt-1 text-xs font-semibold uppercase tracking-wide text-muted">{label}</div>
        <ul className="thin-scrollbar grid max-h-[min(30rem,75vh)] grid-cols-2 gap-0.5 overflow-y-auto">
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
                  <Flag l={l} />
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
  themeToggle = true,
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
        <Logo size={30} />
      </a>
      <div className="ml-auto flex items-center gap-1 sm:gap-2">
        <nav className="mr-2 hidden items-center gap-1 md:flex" aria-label="Main">
          {LINKS.map((l) => (
            <a
              key={l.href}
              href={l.href}
              className={`rounded-md px-3 py-1.5 text-sm ${active(l.match) ? 'font-semibold text-fg' : 'text-muted hover:text-fg'}`}
              aria-current={active(l.match) ? 'page' : undefined}
            >
              {l.title}
            </a>
          ))}
        </nav>
        <div className="hidden sm:block">
          <SearchButton ui={ui} onOpen={() => setSearchOpen(true)} />
        </div>
        <div className="sm:hidden">
          <SearchButton ui={ui} compact onOpen={() => setSearchOpen(true)} />
        </div>
        <LanguageMenu currentPath={currentPath} locale={locale} label={ui.language} />
        <a
          href={REPO_URL}
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg"
          aria-label={ui.github}
        >
          <GitHubIcon className="h-[22px] w-[22px]" />
        </a>
        <a
          href="https://discord.gg/FXhUEA8eBR"
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-fg/80 hover:text-fg"
          aria-label="Discord"
        >
          <DiscordIcon className="h-[22px] w-[22px]" />
        </a>
        <a
          href="https://github.com/sponsors/erkamyaman"
          className="inline-flex h-9 w-9 items-center justify-center rounded-md text-[#db61a2] hover:text-[#e879b4]"
          aria-label={ui.sponsor}
          title={ui.sponsor}
        >
          <HeartIcon className="h-[20px] w-[20px]" />
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
        <div data-theme={themeToggle ? undefined : "dark"} className="fixed inset-0 z-[65] lg:hidden" role="dialog" aria-modal="true" aria-label={ui.menu}>
          <div className="absolute inset-0 bg-black/50" onClick={() => setMenuOpen(false)} />
          <div className="absolute inset-y-0 right-0 flex w-[min(20rem,88vw)] flex-col border-l border-line bg-bg text-fg">
            <div className="flex h-16 items-center justify-between border-b border-line px-4">
              <span className="font-display font-bold"><Logo size={26} /></span>
              <button
                type="button"
                className="inline-flex h-9 w-9 items-center justify-center rounded-md text-muted hover:text-fg"
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

function Flag({ l }: { l: (typeof LOCALES)[number] }) {
  const country = 'country' in l ? l.country : undefined
  if (!country) {
    return (
      <svg className="h-[15px] w-5 shrink-0 text-muted" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden>
        <circle cx="12" cy="12" r="9" />
        <path d="M3 12h18M12 3c2.5 2.7 3.8 5.7 3.8 9s-1.3 6.3-3.8 9c-2.5-2.7-3.8-5.7-3.8-9S9.5 5.7 12 3z" />
      </svg>
    )
  }
  return (
    <img
      src={`/flags/${country}.svg`}
      width={20}
      height={15}
      alt=""
      loading="lazy"
      decoding="async"
      className="h-[15px] w-5 shrink-0 rounded-[2px] object-cover ring-1 ring-black/10"
    />
  )
}
