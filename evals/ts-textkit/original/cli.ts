import { createInterface } from "node:readline";
import { formatBytes, parseDuration, slugify, truncate } from "./textkit.ts";

const FUNCTIONS: Record<string, (...args: any[]) => unknown> = {
  slugify,
  truncate,
  formatBytes,
  parseDuration,
};

const lines = createInterface({ input: process.stdin, crlfDelay: Infinity });
for await (const line of lines) {
  if (line.trim() === "") {
    continue;
  }
  const call = JSON.parse(line) as { fn: string; args: unknown[] };
  let out: unknown;
  try {
    const f = FUNCTIONS[call.fn];
    if (!f) {
      throw new TypeError(`unknown function ${call.fn}`);
    }
    out = { ok: f(...call.args) };
  } catch (e) {
    const err = e as Error;
    out = { error: `${err.name}: ${err.message}` };
  }
  process.stdout.write(JSON.stringify(out) + "\n");
}
