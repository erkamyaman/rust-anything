import { writeFileSync } from 'node:fs'
import { codeToHtml } from '@pierre/highlights'
import { githubDark } from '@pierre/highlights/themes'

const BASH = [
  'cargo install rust-anything',
  'npm install -g rust-anything',
  'rust-anything setup',
  'rust-anything profile . -- node bench.mjs',
  'rust-anything bridge . src/fuzzy.ts --batch fuzzyScore:text',
  'rust-anything record rust/fuzzy-rs',
  'rust-anything build rust/fuzzy-rs',
  'rust-anything diff rust/fuzzy-rs',
  'rust-anything bench rust/fuzzy-rs',
  'rust-anything gate rust/fuzzy-rs',
  'rust-anything bridge . src/words.ts',
]
const JSON_LINES = ['{"fn":"wordCount","args":["a\\u00a0b"],"ok":1}']

const dec = new TextDecoder()
const inner = (code, lang) => {
  const html = dec.decode(codeToHtml(code, { lang, theme: githubDark }))
  const m = /<code>([\s\S]*)<\/code>/.exec(html)
  return (m ? m[1] : code).replace(/^<span class="line">([\s\S]*)<\/span>$/, '$1')
}
const out = {}
for (const l of BASH) out[l] = inner(l, 'bash')
for (const l of JSON_LINES) out[l] = inner(l, 'json')
writeFileSync(new URL('../content/home-code.json', import.meta.url), JSON.stringify(out, null, 2) + '\n')
console.log(Object.keys(out).length, 'snippets')
