// Records what Node answers for the JS behavior json-server-js reproduces, into
// tests/node-oracle.json. Run from the json-server checkout: node rust/json-server-js/oracle.mjs
import { writeFileSync } from 'node:fs'
import querystring from 'node:querystring'
import inflection from 'inflection'
import { getProperty, setProperty } from 'dot-prop'

let seed = 42
const rand = (n) => {
  seed = (seed * 1103515245 + 12345) % 2147483648
  return seed % n
}
const pick = (xs) => xs[rand(xs.length)]

const valid = [
  '{"a":1,"b":[true,false,null],"c":{"d":"e"}}',
  '[1,2.5,-3e2,"x\\n\\u00e9",{"k":[]}]',
  '{"title":"👍🏽 thumbs","n":-0.5e-3}',
  '"just a string"',
  '{"aaaaaaaaaaaaaaaaaaaaaaaa":{"bbbbbbbbbbbbbbbbbbbb":[1,2,3]}}',
  '\r\n{\n  "a": 1,\r\n  "b": "日本語"\n}\n',
]
const noise = ['', '"', '\\', ',', ':', '{', '}', '[', ']', 'x', '0', '-', '.', 'e', '+', '\t', '\n', '\r', ' ', 'é', '👍', 't', 'n', 'u', '\u0001']
const jsonCases = []
for (let i = 0; i < 4000; i++) {
  let s = pick(valid)
  const edits = 1 + rand(3)
  for (let e = 0; e < edits; e++) {
    const at = rand(s.length + 1)
    const kind = rand(3)
    if (kind === 0) s = s.slice(0, at) + pick(noise) + s.slice(at)
    else if (kind === 1) s = s.slice(0, at) + s.slice(at + 1)
    else s = s.slice(0, at) + pick(noise) + s.slice(at + 1)
  }
  if (s.isWellFormed()) jsonCases.push(s)
}
const json = jsonCases.map((s) => {
  try {
    return [s, { ok: JSON.stringify(JSON.parse(s), null, 2) }]
  } catch (e) {
    return [s, { err: e.message }]
  }
})

const numberInputs = ['', ' ', '12', ' 12 ', '1e3', '1E+3', '.5', '5.', '-.5', '+1', '0x1F', '0X1f', '-0x1', '0b101', '0o17', '0o8', 'Infinity', '-Infinity', '+Infinity', 'infinity', 'NaN', '1_000', '1,5', '١٢', ' 1 ', '﻿7', '1e400', '-0', '0.1e-7', '12abc', 'e5', '1e', '--1', '0x', '00012', '9007199254740993', ' 1 ']
const numbers = numberInputs.map((s) => [s, String(Number(s))])

const words = ['a', 'A', 'b', 'B', 'z', 'Z', 'é', 'e', 'E', 'f', 'ß', 'ss', 'æ', 'ae', 'ø', 'o', 'å', 'ä', 'Ä', 'ö', 'ü', 'ı', 'i', 'İ', 'I', '1', '10', '2', '-', '_', ' ', '', 'apple', 'Apple', 'APPLE', 'apples', 'zebra', 'Zebra', 'naïve', 'naive', 'co-op', 'coop', 'co op', 'résumé', 'resume', '日本', '日本語', 'にほん', 'カタカナ', '한국', 'Ελλάδα', 'Россия', 'العربية', '👍', '👍🏽', 'a b', 'a-b', 'a_b', 'a.b', 'A title', 'a title', 'another title', 'Ünïcödé', 'numeric id', 'no id at all', '#', '@', '~', '́e', 'é', ' ', 'x y']
const collate = []
for (let i = 0; i < 3000; i++) {
  const a = pick(words) + (rand(4) === 0 ? pick(words) : '')
  const b = pick(words) + (rand(4) === 0 ? pick(words) : '')
  collate.push([a, b, a.localeCompare(b)])
}

const nouns = ['post', 'posts', 'comment', 'comments', 'person', 'people', 'status', 'statuses', 'child', 'children', 'man', 'men', 'woman', 'women', 'mouse', 'mice', 'index', 'indices', 'indexes', 'matrix', 'matrices', 'quiz', 'quizzes', 'ox', 'oxen', 'bus', 'buses', 'alias', 'aliases', 'octopus', 'octopi', 'virus', 'viruses', 'analysis', 'analyses', 'thesis', 'theses', 'datum', 'data', 'medium', 'media', 'movie', 'movies', 'series', 'news', 'sheep', 'fish', 'equipment', 'information', 'rice', 'money', 'species', 'tooth', 'teeth', 'foot', 'feet', 'goose', 'geese', 'half', 'halves', 'wife', 'wives', 'knife', 'knives', 'leaf', 'leaves', 'hive', 'hives', 'archive', 'archives', 'drive', 'drives', 'database', 'databases', 'shoe', 'shoes', 'tomato', 'tomatoes', 'hero', 'heroes', 'photo', 'photos', 'box', 'boxes', 'church', 'churches', 'wish', 'wishes', 'kiss', 'kisses', 'city', 'cities', 'day', 'days', 'toy', 'toys', 'Post', 'POSTS', 'People', 'userId', 'users', 'user', 'todos', 'todo', 'tags', 'tag', 'profile', 'profiles', 'address', 'addresses', 'axis', 'axes', 'crisis', 'crises', 'testis', 'tests', 'test', 'summons', 'bonus', 'bonuses', 'campus', 'campuses', 'canvas', 'canvases', 'ſtatus', 'ſheep', 'K', 'ks', 'x', 'xs', '', 's', 'ss', 'us', '10', 'a_b', 'some-items', 'Ünïcödés', '日本語', 'π', 'foo_bars', 'curves', 'curve', 'lives', 'life', 'moves', 'move', 'cookie', 'cookies', 'zombie', 'zombies', 'goods', 'jeans', 'police', 'cows', 'kine']
const inflect = nouns.map((w) => [w, inflection.singularize(w), inflection.pluralize(w)])

const qsParts = ['a', 'b', '_embed', 'x y', 'é', '%E9', '%C3%A9', '%zz', '%', '+', '%2B', '=', '', '1', 'a%20b', '%F0%9F%91%8D', '%E0%A4', '_dependent', 'title:contains', 'views_gt', 'a.b', 'a[0]', '%5B']
const queries = []
for (let i = 0; i < 600; i++) {
  const n = rand(5)
  const parts = []
  for (let k = 0; k < n; k++) {
    parts.push(rand(5) === 0 ? pick(qsParts) : pick(qsParts) + '=' + pick(qsParts) + (rand(3) === 0 ? pick(qsParts) : ''))
  }
  queries.push(parts.join(rand(6) === 0 ? '&&' : '&'))
}
const search = queries.map((q) => {
  const p = new URLSearchParams(q)
  return [q, [...p.entries()], p.toString(), Object.entries(querystring.parse(q))]
})

const pathParts = ['a', 'b', '0', '1', '01', '[0]', '[1]', '[x]', '[]', '.', '\\', '\\.', '__proto__', 'constructor', 'é', '1000001', '[1000001]', 'length', 'lt', '[', ']', '..']
const paths = []
for (let i = 0; i < 1500; i++) {
  let p = ''
  const n = 1 + rand(4)
  for (let k = 0; k < n; k++) p += (k > 0 && rand(2) === 0 ? '.' : '') + pick(pathParts)
  paths.push(p)
}
const sample = { a: { b: [10, { lt: 'x' }], 0: 'zero' }, b: 'héllo', 1: [1, 2], length: 3 }
const dotprop = paths.map((p) => {
  let set
  try {
    set = { ok: JSON.stringify(setProperty({}, p + '.eq', 1)) }
  } catch (e) {
    set = { err: e.message }
  }
  let get
  try {
    const v = getProperty(sample, p)
    get = { ok: v === undefined ? '<undefined>' : JSON.stringify(v) }
  } catch (e) {
    get = { err: e.message }
  }
  return [p, set, get]
})

writeFileSync(
  new URL('./tests/node-oracle.json', import.meta.url),
  JSON.stringify({ node: process.version, icu: process.versions.icu, json, numbers, collate, inflect, search, dotprop }) + '\n',
)
console.log({ json: json.length, numbers: numbers.length, collate: collate.length, inflect: inflect.length, search: search.length, dotprop: dotprop.length })
