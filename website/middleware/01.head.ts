import { defineMiddleware } from 'void'
import { DESCRIPTION, REPO_URL, SITE_ORIGIN } from '../lib/site.ts'
import { publicPath } from '../lib/nav.ts'
import { LOCALES, localeInfo, localeOf, stripLocale, withLocale } from '../lib/locales.ts'

declare module 'void' {
  interface CloudContextVariables {
    shared: { path: string }
  }
}

const THEME_BOOTSTRAP = `document.documentElement.setAttribute("data-theme","light")`

const EXTERNAL_LINKS = `(function(){
function mark(a){if(a.host&&a.host!==location.host&&/^https?:$/.test(a.protocol)){a.target="_blank";a.rel="noopener noreferrer"}}
document.addEventListener("click",function(e){var a=e.target&&e.target.closest&&e.target.closest("a[href]");if(a)mark(a)},true);
})();`

const LOCALE_LINKS = (rest: string) =>
  LOCALES.map((l) => `<link rel="alternate" hreflang="${l.lang}" href="${SITE_ORIGIN}${withLocale(rest, l.code)}">`).join('') +
  `<link rel="alternate" hreflang="x-default" href="${SITE_ORIGIN}${withLocale(rest, 'en')}">`

const AUTHOR = { '@type': 'Person', name: 'Erkam Yaman', url: `${SITE_ORIGIN}/about` }

const jsonLd = (data: unknown) =>
  `<script type="application/ld+json">${JSON.stringify(data).replace(/</g, '\\u003c')}</script>`

function structuredData(path: string, title: string, description: string): string {
  const rest = stripLocale(path)
  const url = `${SITE_ORIGIN}${path}`
  if (rest === '/about') {
    return jsonLd({
      '@context': 'https://schema.org',
      '@type': 'ProfilePage',
      url,
      name: title,
      description,
      mainEntity: {
        ...AUTHOR,
        alternateName: 'kæm',
        jobTitle: 'Software Engineer',
        image: 'https://github.com/erkamyaman.png?size=192',
        email: 'mailto:hello@rustanything.ai',
        homeLocation: { '@type': 'Place', name: 'Istanbul, Türkiye' },
        sameAs: [
          'https://erkamyaman.dev',
          'https://github.com/erkamyaman',
          'https://x.com/erkamyaman_ng',
          'https://bsky.app/profile/erkamyaman.dev',
          'https://www.linkedin.com/in/erkamyaman',
        ],
      },
    })
  }
  if (rest === '/' || rest === '') {
    return jsonLd({
      '@context': 'https://schema.org',
      '@graph': [
        {
          '@type': 'SoftwareApplication',
          name: 'Rust Anything',
          applicationCategory: 'DeveloperApplication',
          operatingSystem: 'macOS, Linux, Windows',
          description,
          url: SITE_ORIGIN,
          downloadUrl: 'https://crates.io/crates/rust-anything',
          softwareHelp: `${SITE_ORIGIN}/docs/introduction/quick-start`,
          codeRepository: REPO_URL,
          license: 'https://opensource.org/licenses/MIT',
          offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
          author: AUTHOR,
        },
        { '@type': 'WebSite', name: 'Rust Anything', url: SITE_ORIGIN },
      ],
    })
  }
  if (rest.startsWith('/docs/')) {
    return jsonLd({
      '@context': 'https://schema.org',
      '@graph': [
        {
          '@type': 'TechArticle',
          headline: title,
          description,
          url,
          author: AUTHOR,
          publisher: { '@type': 'Organization', name: 'Rust Anything', url: SITE_ORIGIN },
        },
        {
          '@type': 'BreadcrumbList',
          itemListElement: [
            { '@type': 'ListItem', position: 1, name: 'Rust Anything', item: `${SITE_ORIGIN}${withLocale('/', localeOf(path))}` },
            { '@type': 'ListItem', position: 2, name: 'Docs', item: `${SITE_ORIGIN}${withLocale('/docs/introduction/what-is-rust-anything', localeOf(path))}` },
            { '@type': 'ListItem', position: 3, name: title, item: url },
          ],
        },
      ],
    })
  }
  return ''
}

const escapeAttr = (s: string) =>
  s.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

const decode = (s: string) =>
  s
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&amp;/g, '&')

function ogImageFor(path: string): string {
  const p = stripLocale(path)
  return p.startsWith('/docs/') ? `/og${p}.png` : '/og/index.png'
}

export default defineMiddleware(async (c, next) => {
  const raw = c.isRewritten() ? (c.originalUrl()?.pathname ?? c.req.path) : c.req.path
  const path = publicPath(raw)
  const reqUrl = new URL(c.req.url)
  if (reqUrl.hostname.startsWith('www.')) {
    return new Response(null, {
      status: 301,
      headers: { Location: `https://${reqUrl.hostname.slice(4)}${path}${reqUrl.search}` },
    })
  }
  c.set('shared', { path })
  c.set('headDefaults', {
    script: [{ innerHTML: THEME_BOOTSTRAP }, { innerHTML: EXTERNAL_LINKS }],
    meta: [
      { property: 'og:image', content: `${SITE_ORIGIN}${ogImageFor(path)}` },
      { name: 'twitter:image', content: `${SITE_ORIGIN}${ogImageFor(path)}` },
    ],
  })
  await next()

  if (c.req.method !== 'GET' && c.req.method !== 'HEAD') return
  const res = c.res
  if (!(res.headers.get('content-type') ?? '').includes('text/html')) return
  const html = await res.text()
  const headEnd = html.indexOf('</head>')
  const rebuild = (body: string) => {
    const headers = new Headers(res.headers)
    headers.delete('content-length')
    c.res = new Response(body, { status: res.status, statusText: res.statusText, headers })
  }
  const info0 = localeInfo(localeOf(path))
  if (headEnd === -1 || html.includes('property="og:title"'))
    return rebuild(html.replace(/<html lang="en"/, `<html lang="${info0.lang}"${'rtl' in info0 && info0.rtl ? ' dir="rtl"' : ''}`))
  const head = html.slice(0, headEnd)
  const title =
    decode(head.match(/<title>([\s\S]*?)<\/title>/i)?.[1] ?? '').replace(/\s+·\s+Rust Anything\s*$/, '') ||
    'Rust Anything'
  const descMatch = head.match(/<meta[^>]*name="description"[^>]*content="([^"]*)"/i)
  const description = descMatch ? decode(descMatch[1]) : DESCRIPTION
  const tags = [
    descMatch ? '' : `<meta name="description" content="${escapeAttr(description)}">`,
    `<meta property="og:title" content="${escapeAttr(title)}">`,
    `<meta property="og:description" content="${escapeAttr(description)}">`,
    `<meta name="twitter:title" content="${escapeAttr(title)}">`,
    `<meta name="twitter:description" content="${escapeAttr(description)}">`,
    `<meta property="og:url" content="${SITE_ORIGIN}${path}">`,
    `<link rel="canonical" href="${SITE_ORIGIN}${path}">`,
    `<link rel="alternate" type="text/plain" title="llms.txt" href="${SITE_ORIGIN}/llms.txt">`,
    structuredData(path, title, description),
  ].join('')
  const info = localeInfo(localeOf(path))
  const htmlOpen = `<html lang="${info.lang}"${'rtl' in info && info.rtl ? ' dir="rtl"' : ''}`
  const alternates = stripLocale(path) === '/about' ? '' : LOCALE_LINKS(stripLocale(path))
  rebuild((html.slice(0, headEnd) + tags + alternates + html.slice(headEnd)).replace(/<html lang="en"/, htmlOpen))
})
