import Logo from '../Logo'
import { REPO_URL } from '../../lib/site'
import { messages, withLocale } from '../../lib/i18n'

const AUTHOR_LINKS = [
  {
    label: 'GitHub',
    href: 'https://github.com/erkamyaman',
    icon: 'M12 .5a11.5 11.5 0 0 0-3.64 22.41c.58.1.79-.25.79-.56v-2c-3.2.7-3.87-1.37-3.87-1.37-.53-1.33-1.28-1.69-1.28-1.69-1.05-.71.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.56-.29-5.25-1.28-5.25-5.69 0-1.26.45-2.29 1.19-3.1-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.77 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.81 1.19 1.84 1.19 3.1 0 4.42-2.7 5.39-5.27 5.68.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 12 .5z',
  },
  {
    label: 'X',
    href: 'https://x.com/erkamyaman_ng',
    icon: 'M18.24 2.25h3.31l-7.23 8.26 8.5 11.24h-6.66l-5.21-6.82-5.97 6.82H1.67l7.73-8.84L1.25 2.25H8.08l4.71 6.23 5.45-6.23zm-1.16 17.52h1.83L7.08 4.13H5.12l11.96 15.64z',
  },
  {
    label: 'LinkedIn',
    href: 'https://www.linkedin.com/in/erkamyaman',
    icon: 'M20.45 20.45h-3.55v-5.57c0-1.33-.03-3.04-1.85-3.04-1.86 0-2.14 1.45-2.14 2.94v5.67H9.35V9h3.41v1.56h.05c.48-.9 1.64-1.85 3.37-1.85 3.6 0 4.27 2.37 4.27 5.46v6.28zM5.34 7.43a2.06 2.06 0 1 1 0-4.12 2.06 2.06 0 0 1 0 4.12zM7.12 20.45H3.56V9h3.56v11.45zM22.22 0H1.77C.79 0 0 .77 0 1.73v20.54C0 23.23.79 24 1.77 24h20.45c.98 0 1.78-.77 1.78-1.73V1.73C24 .77 23.2 0 22.22 0z',
  },
  {
    label: 'Bluesky',
    href: 'https://bsky.app/profile/erkamyaman.dev',
    icon: 'M5.2 3.2C7.95 5.27 10.9 9.46 12 11.73c1.1-2.27 4.05-6.46 6.8-8.53 1.98-1.49 5.2-2.65 5.2 1.03 0 .73-.42 6.17-.67 7.05-.85 3.06-3.97 3.84-6.74 3.37 4.84.82 6.07 3.55 3.41 6.28-5.05 5.18-7.26-1.3-7.83-2.96-.1-.3-.15-.45-.17-.33-.02-.12-.07.03-.17.33-.57 1.66-2.78 8.14-7.83 2.96-2.66-2.73-1.43-5.46 3.41-6.28-2.77.47-5.89-.31-6.74-3.37C.42 10.4 0 4.96 0 4.23 0 .55 3.22 1.71 5.2 3.2z',
  },
]

export default function Footer({ locale = 'en' }: { locale?: string }) {
  const ui = messages(locale).ui
  const L = (p: string) => withLocale(p, locale)
  const cols = [
    {
      title: ui.footerProduct,
      links: [
        [ui.quickStart, L('/docs/introduction/quick-start')],
        [ui.cliReference, L('/docs/reference/cli')],
        [ui.footerLanguages, L('/docs/languages/overview')],
        [ui.footerPorts, `${REPO_URL}/blob/main/docs/real-world.md`],
      ],
    },
    {
      title: ui.footerProject,
      links: [
        [ui.status, L('/docs/project/status')],
        [ui.footerChangelog, `${REPO_URL}/blob/main/CHANGELOG.md`],
        [ui.footerDecisions, `${REPO_URL}/blob/main/DECISIONS.md`],
        [ui.issues, `${REPO_URL}/issues`],
      ],
    },
    {
      title: ui.footerCommunity,
      links: [
        ['GitHub', REPO_URL],
        ['Discord', 'https://discord.gg/FXhUEA8eBR'],
        [ui.sponsor, 'https://github.com/sponsors/erkamyaman'],
        ['hello@rustanything.ai', 'mailto:hello@rustanything.ai'],
      ],
    },
  ]
  return (
    <footer className="border-t border-line bg-bg text-fg">
      <div className="mx-auto max-w-[1200px] px-5 pt-14 pb-8 sm:px-8 md:pt-20">
        <div className="grid gap-10 md:grid-cols-[minmax(0,1.4fr)_repeat(3,minmax(0,1fr))]">
          <div className="max-w-xs space-y-4">
            <span className="text-base">
              <Logo size={28} />
            </span>
            <p className="text-[15px] leading-relaxed text-muted">{ui.footerTagline}</p>
          </div>
          {cols.map((c) => (
            <div key={c.title}>
              <div className="mb-4 text-sm font-semibold">{c.title}</div>
              <ul className="space-y-3 text-[15px] text-muted">
                {c.links.map(([label, href]) => (
                  <li key={href}>
                    <a className="hover:text-fg" href={href}>
                      {label}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
        <div className="mt-14 flex flex-col gap-4 border-t border-line pt-6 text-sm text-muted md:flex-row md:items-center md:justify-between">
          <div className="flex flex-wrap items-center gap-x-5 gap-y-2">
            <a href="https://erkamyaman.dev" className="hover:text-fg">
              © 2026 Erkam Yaman
            </a>
            <a href={`${REPO_URL}#license`} className="hover:text-fg">
              {ui.footerLicense}
            </a>
          </div>
          <div className="flex flex-wrap items-center gap-x-4 gap-y-2">
            <a href="/about" className="inline-flex items-center gap-2 hover:text-fg">
              <img src="https://github.com/erkamyaman.png?size=64" width={20} height={20} alt="" className="rounded-full" loading="lazy" />
              {ui.madeBy} erKam Yaman
            </a>
            {AUTHOR_LINKS.map((l) => (
              <a key={l.label} href={l.href} aria-label={l.label} title={l.label} className="hover:text-fg">
                <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
                  <path d={l.icon} />
                </svg>
              </a>
            ))}
          </div>
        </div>
      </div>
    </footer>
  )
}
