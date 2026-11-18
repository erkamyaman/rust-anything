"""json-server benchmark: Node (lib/bin.js) against the lifted Rust binary, same requests.
usage: python3 bench.py <json-server checkout> <rust binary> <out.json>"""
import json, os, re, shutil, socket, statistics, subprocess, sys, tempfile, threading, time

repo, rust_bin, out_path = sys.argv[1:4]
NODE = [shutil.which("node"), os.path.join(repo, "lib/bin.js")]
RUST = [rust_bin]
SMALL = os.path.join(repo, "rust/fixtures/db.json")

def big_db(path):
    posts = [{"id": str(i), "title": f"post number {i}", "views": (i * 7919) % 10007, "published": i % 3 == 0,
              "userId": str(i % 50), "author": {"name": f"author {i % 97}"}} for i in range(10000)]
    comments = [{"id": str(i), "postId": str(i % 10000), "text": f"comment {i}"} for i in range(20000)]
    users = [{"id": str(i), "name": f"user {i}"} for i in range(50)]
    json.dump({"posts": posts, "comments": comments, "users": users}, open(path, "w"))

def get(port, path, timeout=0.5):
    s = socket.create_connection(("127.0.0.1", port), timeout=timeout)
    s.sendall(f"GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".encode())
    data = b""
    while True:
        c = s.recv(65536)
        if not c: break
        data += c
    s.close()
    return data

def rss_kb(pid):
    out = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return int(out) if out else 0

class Server:
    def __init__(self, cmd, db_src, port):
        self.dir = tempfile.mkdtemp(prefix="jsbench-")
        shutil.copy(db_src, os.path.join(self.dir, "db.json"))
        os.makedirs(os.path.join(self.dir, "public"))
        shutil.copy(os.path.join(repo, "public/test.html"), os.path.join(self.dir, "public/test.html"))
        self.port = port
        t0 = time.perf_counter()
        self.p = subprocess.Popen(cmd + ["db.json", "-p", str(port)], cwd=self.dir, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        while True:
            try:
                if get(port, "/posts").startswith(b"HTTP/1.1 200"):
                    break
            except OSError:
                time.sleep(0.001)
        self.startup_ms = (time.perf_counter() - t0) * 1000
    def stop(self):
        self.p.terminate(); self.p.wait(); shutil.rmtree(self.dir, ignore_errors=True)

def ab(port, path, n, c, post=None):
    cmd = ["ab", "-q", "-k", "-n", str(n), "-c", str(c)]
    if post:
        cmd += ["-p", post, "-T", "application/json"]
    out = subprocess.run(cmd + [f"http://127.0.0.1:{port}{path}"], capture_output=True, text=True).stdout
    rps = float(re.search(r"Requests per second:\s+([\d.]+)", out).group(1))
    failed = int(re.search(r"Failed requests:\s+(\d+)", out).group(1))
    non2xx = re.search(r"Non-2xx responses:\s+(\d+)", out)
    p50 = re.search(r"\s50%\s+(\d+)", out)
    p99 = re.search(r"\s99%\s+(\d+)", out)
    return {"rps": rps, "failed": failed, "non2xx": int(non2xx.group(1)) if non2xx else 0,
            "p50_ms": int(p50.group(1)) if p50 else None, "p99_ms": int(p99.group(1)) if p99 else None}

def peak_during(pid, fn):
    peak = [0]; stop = [False]
    def watch():
        while not stop[0]:
            peak[0] = max(peak[0], rss_kb(pid)); time.sleep(0.05)
    t = threading.Thread(target=watch); t.start()
    try:
        r = fn()
    finally:
        stop[0] = True; t.join()
    return r, peak[0]

results = {"node": subprocess.run(["node", "--version"], capture_output=True, text=True).stdout.strip()}
tmp = tempfile.mkdtemp()
BIG = os.path.join(tmp, "big.json"); big_db(BIG)
body = os.path.join(tmp, "post.json"); open(body, "w").write('{"text": "a new comment", "postId": "1"}')

for name, db in [("small", SMALL), ("big", BIG)]:
    for impl, cmd in [("node", NODE), ("rust", RUST)]:
        starts, idle = [], []
        for i in range(15):
            s = Server(cmd, db, 4600 + i)
            starts.append(s.startup_ms); time.sleep(0.3); idle.append(rss_kb(s.p.pid)); s.stop()
        results[f"{name}/{impl}/startup_ms"] = statistics.median(starts)
        results[f"{name}/{impl}/idle_rss_mb"] = statistics.median(idle) / 1024
        print(name, impl, "startup", round(statistics.median(starts), 1), "ms idle", round(statistics.median(idle) / 1024, 1), "MB", flush=True)

ENDPOINTS = {
    "small": ["/posts", "/posts/1", "/posts?views:gt=10&_sort=-views&_page=1&_per_page=2", "/posts?_embed=comments", "/", "/test.html"],
    "big": ["/posts/5000", "/posts?_page=3&_per_page=10", "/posts?title:contains=number%2099&_sort=-views", "/posts?_embed=comments&_page=1&_per_page=5", "/posts"],
}
for name, db in [("small", SMALL), ("big", BIG)]:
    for impl, cmd in [("node", NODE), ("rust", RUST)]:
        s = Server(cmd, db, 4700 if impl == "node" else 4701)
        peak_all = 0
        for path in ENDPOINTS[name]:
            heavy = name == "big" and ("_embed" in path or path == "/posts")
            n = 40 if heavy else 20000
            ab(s.port, path, 10 if heavy else 2000, 10)
            for c in ((1, 10) if heavy else (1, 50)):
                r, peak = peak_during(s.p.pid, lambda: ab(s.port, path, n, c))
                peak_all = max(peak_all, peak)
                results[f"{name}/{impl}/GET {path}/c{c}"] = r
                print(name, impl, path, "c", c, r, flush=True)
                json.dump(results, open(out_path, "w"), indent=2)
        r, peak = peak_during(s.p.pid, lambda: ab(s.port, "/comments", 3000, 10, post=body))
        peak_all = max(peak_all, peak)
        results[f"{name}/{impl}/POST /comments/c10"] = r
        print(name, impl, "POST", r, flush=True)
        results[f"{name}/{impl}/peak_rss_mb"] = peak_all / 1024
        s.stop()
json.dump(results, open(out_path, "w"), indent=2)
