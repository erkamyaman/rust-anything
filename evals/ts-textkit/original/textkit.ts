export function slugify(input: string): string {
  const slug = input
    .normalize("NFKD")
    .replace(/[̀-ͯ]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  if (slug.length === 0) {
    throw new Error(`slugify: nothing left of "${input}"`);
  }
  return slug;
}

export function truncate(input: string, max: number): string {
  if (!Number.isInteger(max) || max < 1) {
    throw new RangeError(`truncate: max must be a positive integer, got ${max}`);
  }
  if (input.length <= max) {
    return input;
  }
  let cut = input.slice(0, max - 1);
  const last = cut.charCodeAt(cut.length - 1);
  if (last >= 0xd800 && last <= 0xdbff) {
    cut = cut.slice(0, -1);
  }
  return cut.trimEnd() + "…";
}

const UNITS = ["B", "KB", "MB", "GB", "TB"];

export function formatBytes(bytes: number, decimals = 1): string {
  if (!Number.isFinite(bytes) || bytes < 0) {
    throw new RangeError(`formatBytes: expected a non-negative number, got ${bytes}`);
  }
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < UNITS.length - 1) {
    value /= 1024;
    unit++;
  }
  if (unit === 0) {
    return `${bytes} B`;
  }
  return `${value.toFixed(decimals)} ${UNITS[unit]}`;
}

export function parseDuration(text: string): number {
  const m = /^\s*(?:(\d+)h)?\s*(?:(\d+)m)?\s*(?:(\d+)s)?\s*$/.exec(text);
  if (!m || (m[1] === undefined && m[2] === undefined && m[3] === undefined)) {
    throw new SyntaxError(`parseDuration: cannot read "${text}"`);
  }
  const [h, min, s] = [m[1], m[2], m[3]].map((part) => (part === undefined ? 0 : Number(part)));
  if (min >= 60 || s >= 60) {
    throw new RangeError(`parseDuration: minutes and seconds must be under 60 in "${text}"`);
  }
  return h * 3600 + min * 60 + s;
}
