import { defineHandler } from 'void'
import pages from '@void/md/pages'
import { publicPath } from '../lib/nav.ts'
import { LOCALES } from '../lib/locales.ts'

export const GET = defineHandler((c) => {
  const origin = new URL(c.req.url).origin
  const homes = LOCALES.filter((l) => l.code !== 'en').map((l) => `/${l.code}`)
  const paths = ['/', '/about', ...homes, ...pages.map((p) => publicPath(p.path)).filter((p) => p !== '/')]
  const body =
    '<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n' +
    paths.map((p) => `  <url><loc>${origin}${p}</loc></url>`).join('\n') +
    '\n</urlset>\n'
  return c.body(body, 200, { 'Content-Type': 'application/xml; charset=utf-8' })
})
