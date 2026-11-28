# python3 py.py <module dir> <module name> <held-out jsonl>
# Counts, per function, the held-out calls that entered a function of a Rust extension
# module (ExtensionFileLoader) or a Rust library loaded through ctypes.
import ctypes
import importlib.machinery
import json
import sys

MARKERS = [b"rust_begin_unwind", b"__rust_alloc", b"/rustc/", b"rust_panic"]
entered = [0]
natives = []


def rust_markers(path):
    try:
        with open(path, "rb") as f:
            data = f.read()
    except OSError:
        return []
    return [m.decode() for m in MARKERS if m in data]


def counted(f):
    def wrapper(*args, **kwargs):
        entered[0] += 1
        return f(*args, **kwargs)

    wrapper.__name__ = getattr(f, "__name__", "wrapper")
    wrapper.__doc__ = getattr(f, "__doc__", None)
    return wrapper


orig_exec = importlib.machinery.ExtensionFileLoader.exec_module


def exec_module(self, module):
    orig_exec(self, module)
    rust = rust_markers(self.path)
    natives.append({"kind": "extension", "file": self.path, "rust": rust})
    if not rust:
        return
    for name, value in list(vars(module).items()):
        if type(value).__name__ == "builtin_function_or_method":
            setattr(module, name, counted(value))
        elif isinstance(value, type):
            for attr, member in list(vars(value).items()):
                if type(member).__name__ in ("method_descriptor", "builtin_function_or_method"):
                    try:
                        setattr(value, attr, counted(member))
                    except (TypeError, AttributeError):
                        pass


importlib.machinery.ExtensionFileLoader.exec_module = exec_module

orig_cdll_init = ctypes.CDLL.__init__


def cdll_init(self, name, *args, **kwargs):
    orig_cdll_init(self, name, *args, **kwargs)
    rust = rust_markers(str(name)) if name else []
    natives.append({"kind": "ctypes", "file": str(name), "rust": rust})
    if not rust:
        return
    base = self._FuncPtr

    class CountedFuncPtr(base):
        _flags_ = base._flags_
        _restype_ = base._restype_

        def __call__(fp, *a, **k):
            entered[0] += 1
            return base.__call__(fp, *a, **k)

    self._FuncPtr = CountedFuncPtr


ctypes.CDLL.__init__ = cdll_init


def main():
    module_dir, module_name, heldout = sys.argv[1:4]
    sys.path.insert(0, module_dir)
    mod = __import__(module_name)
    functions = {}
    with open(heldout, encoding="utf-8") as f:
        lines = [l for l in f.read().split("\n") if l.strip()]
    for line in lines:
        call = json.loads(line)
        row = functions.setdefault(call["fn"], {"calls": 0, "native": 0})
        row["calls"] += 1
        before = entered[0]
        try:
            getattr(mod, call["fn"])(*call["args"])
        except Exception:
            pass
        if entered[0] > before:
            row["native"] += 1
    sys.stdout.write(json.dumps({"functions": functions, "natives": natives}) + "\n")


main()
