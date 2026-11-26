import CODE from '../content/home-code.json'

const MAP = CODE as Record<string, string>

export default function Hl({ code }: { code: string }) {
  const html = MAP[code]
  return html ? <span dangerouslySetInnerHTML={{ __html: html }} /> : <>{code}</>
}
