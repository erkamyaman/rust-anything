// node ts.mjs <module> <held-out jsonl>
// Counts, per exported function, the held-out calls that entered a function exported by a
// Rust native addon (process.dlopen) or a Rust WebAssembly instance the workspace loaded.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const [modulePath, heldoutPath] = process.argv.slice(2);
const self = fileURLToPath(import.meta.url);
const MARKERS = ["rust_begin_unwind", "__rust_alloc", "/rustc/", "rust_panic", "__wbindgen"];
let entered = 0;
const natives = [];

const fromWorkspace = () =>
  new Error().stack
    .split("\n")
    .slice(1)
    .some((frame) => !frame.includes(self) && !frame.includes("node:") && /\/|file:/.test(frame));
const rustMarkers = (bytes) => {
  const text = Buffer.from(bytes).toString("latin1");
  return MARKERS.filter((m) => text.includes(m));
};

function wrapFn(f) {
  const w = function (...args) {
    entered++;
    return new.target ? Reflect.construct(f, args, new.target) : f.apply(this, args);
  };
  Object.defineProperty(w, "name", { value: f.name });
  w.prototype = f.prototype;
  return w;
}

const dlopen = process.dlopen;
process.dlopen = function (module, filename, ...rest) {
  const result = dlopen.call(this, module, filename, ...rest);
  const rust = rustMarkers(fs.readFileSync(filename));
  natives.push({ kind: "node-addon", file: filename, rust });
  const exp = module.exports;
  if (rust.length && exp && (typeof exp === "object" || typeof exp === "function")) {
    for (const key of Object.keys(exp)) {
      if (typeof exp[key] === "function") exp[key] = wrapFn(exp[key]);
    }
  }
  return result;
};

const moduleBytes = new WeakMap();
const OrigModule = WebAssembly.Module;
function HookedModule(bytes) {
  const mod = new OrigModule(bytes);
  moduleBytes.set(mod, rustMarkers(bytes));
  return mod;
}
Object.setPrototypeOf(HookedModule, OrigModule);
HookedModule.prototype = OrigModule.prototype;
WebAssembly.Module = HookedModule;

const OrigInstance = WebAssembly.Instance;
const wrapInstance = (instance, mod, rust) => {
  if (!fromWorkspace()) return instance;
  const markers = rust ?? moduleBytes.get(mod) ?? [];
  natives.push({ kind: "wasm", rust: markers });
  if (!markers.length) return instance;
  const exports = {};
  for (const [key, value] of Object.entries(instance.exports)) {
    exports[key] = typeof value === "function" ? wrapFn(value) : value;
  }
  return { exports };
};
function HookedInstance(mod, imports) {
  return wrapInstance(new OrigInstance(mod, imports), mod);
}
HookedInstance.prototype = OrigInstance.prototype;
WebAssembly.Instance = HookedInstance;

const origInstantiate = WebAssembly.instantiate;
WebAssembly.instantiate = async function (source, imports) {
  const result = await origInstantiate.call(WebAssembly, source, imports);
  if (result instanceof OrigInstance) return wrapInstance(result, source);
  return { module: result.module, instance: wrapInstance(result.instance, result.module, rustMarkers(source)) };
};
WebAssembly.instantiateStreaming = async function (source, imports) {
  return WebAssembly.instantiate(await (await source).arrayBuffer(), imports);
};

const mod = await import(pathToFileURL(path.resolve(modulePath)).href);
const functions = {};
for (const line of fs.readFileSync(heldoutPath, "utf8").split("\n")) {
  if (!line.trim()) continue;
  const { fn, args } = JSON.parse(line);
  const row = (functions[fn] ??= { calls: 0, native: 0 });
  row.calls++;
  const before = entered;
  try {
    mod[fn](...args);
  } catch {}
  if (entered > before) row.native++;
}
process.stdout.write(JSON.stringify({ functions, natives }) + "\n");
