import { defineHandler } from 'void'

const FIRST_DAY = '2026-11-01'
const DAY = 86_400_000

const day = (t: number) => new Date(t).toISOString().slice(0, 10)

async function cratesTotal(): Promise<number> {
  const res = await fetch('https://crates.io/api/v1/crates/rust-anything', {
    headers: { 'User-Agent': 'rustanything.ai downloads badge (https://rustanything.ai)' },
  })
  if (!res.ok) throw new Error(`crates.io ${res.status}`)
  const body = (await res.json()) as { crate: { downloads: number } }
  return body.crate.downloads
}

async function npmTotal(): Promise<number> {
  let total = 0
  const end = Date.now()
  for (let start = Date.parse(FIRST_DAY); start <= end; start += 500 * DAY) {
    const until = Math.min(start + 499 * DAY, end)
    const res = await fetch(`https://api.npmjs.org/downloads/point/${day(start)}:${day(until)}/rust-anything`)
    if (!res.ok) throw new Error(`npm ${res.status}`)
    total += ((await res.json()) as { downloads: number }).downloads
  }
  return total
}

export function compact(n: number): string {
  if (n < 1000) return String(n)
  if (n < 1_000_000) return `${(n / 1000).toFixed(1).replace(/\.0$/, '')}k`
  return `${(n / 1_000_000).toFixed(1).replace(/\.0$/, '')}M`
}

export const GET = defineHandler(async (c) => {
  try {
    const [crates, npm] = await Promise.all([cratesTotal(), npmTotal()])
    c.header('Cache-Control', 'public, max-age=3600')
    return c.json({
      schemaVersion: 1,
      label: 'downloads',
      message: compact(crates + npm),
      labelColor: '111111',
      color: 'b34a1a',
    })
  } catch {
    c.header('Cache-Control', 'no-store')
    return c.json({ schemaVersion: 1, label: 'downloads', message: 'unavailable', isError: true }, 502)
  }
})
