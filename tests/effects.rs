mod common;

use std::path::Path;

use rust_anything::app::{self, RecordOptions, ReplayOptions};
use rust_anything::effects::EffectsConfig;

const NOTIFY: &str = r#"import json, os, smtplib, sys, urllib.request
from email.message import EmailMessage

name = sys.argv[1] if len(sys.argv) > 1 else "nobody"
times = int(os.environ.get("MAILS", "1"))
req = urllib.request.Request(
    os.environ["WEBHOOK_URL"] + "/signup",
    data=json.dumps({"user": name}).encode(),
    headers={"Content-Type": "application/json"},
    method="POST",
)
urllib.request.urlopen(req).read()
for _ in range(times):
    m = EmailMessage()
    m["From"] = "app@example.test"
    m["To"] = name + "@example.test"
    m["Subject"] = "welcome"
    m.set_content("hi " + name)
    with smtplib.SMTP(os.environ["SMTP_HOST"], int(os.environ["SMTP_PORT"]), local_hostname="localhost") as s:
        s.send_message(m)
print("signed up", name)
"#;

fn record(repo: &Path, python: &str, effects: Option<EffectsConfig>) -> app::RecordOutcome {
    app::record(&RecordOptions {
        repo,
        run: &[python.to_string(), "notify.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("notify"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects,
    })
    .unwrap()
}

fn replay(repo: &Path, build: &str) -> app::ReplayOutcome {
    app::replay(&ReplayOptions {
        repo,
        build: Some(build),
        timeout_secs: None,
        port: None,
        case: None,
    })
    .unwrap()
}

#[test]
fn a_second_email_fails_replay_even_when_the_output_matches() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = std::env::temp_dir().join(format!("rust-anything-effects-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("notify.py"), NOTIFY).unwrap();
    std::fs::write(
        repo.join("twice.py"),
        NOTIFY.replace("\"MAILS\", \"1\"", "\"MAILS\", \"2\""),
    )
    .unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"alice\", \"args\": [\"alice\"]}\n{\"name\": \"bob\", \"args\": [\"bob\"]}\n",
    )
    .unwrap();

    let effects = EffectsConfig::from_args(
        &["smtp".into()],
        &["WEBHOOK_URL=http-url".into()],
        None,
        None,
    )
    .unwrap();
    let outcome = record(&repo, python, effects);
    assert!(outcome.cases >= 2, "{outcome:?}");
    assert!(outcome.unstable.is_empty(), "{outcome:?}");

    let recorded = std::fs::read_to_string(app::cases_dir(&repo).join("alice.effects")).unwrap();
    assert!(
        recorded.starts_with("http POST http://sink/signup\ncontent-type: application/json\n"),
        "{recorded}"
    );
    assert!(recorded.contains("{\"user\": \"alice\"}"), "{recorded}");
    assert_eq!(
        recorded
            .matches("smtp from <app@example.test> to <alice@example.test>")
            .count(),
        1,
        "{recorded}"
    );

    let same = replay(&repo, &format!("{python} notify.py"));
    assert_eq!(same.passed, same.total, "{:?}", same.failures);

    let twice = replay(&repo, &format!("{python} twice.py"));
    assert_eq!(twice.passed, 0, "{twice:?}");
    for failure in &twice.failures {
        let fields: Vec<&str> = failure.problems.iter().map(|p| p.field.as_str()).collect();
        assert_eq!(fields, ["effects"], "{failure:?}");
    }

    std::fs::remove_dir_all(&repo).unwrap();
}

const SERVER: &str = r#"import json, os, smtplib
from email.message import EmailMessage
from http.server import BaseHTTPRequestHandler, HTTPServer

MAILS = int(os.environ.get("MAILS", "1"))

class H(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def reply(self, code, body):
        data = json.dumps(body).encode()
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        self.reply(200, {"ok": True})

    def do_POST(self):
        n = int(self.headers.get("Content-Length", "0"))
        user = json.loads(self.rfile.read(n) or b"{}").get("user", "nobody")
        for _ in range(MAILS):
            m = EmailMessage()
            m["From"] = "app@example.test"
            m["To"] = user + "@example.test"
            m["Subject"] = "welcome"
            m.set_content("hi " + user)
            with smtplib.SMTP(os.environ["SMTP_HOST"], int(os.environ["SMTP_PORT"]), local_hostname="localhost") as s:
                s.send_message(m)
        self.reply(201, {"user": user})

HTTPServer(("127.0.0.1", int(os.environ["PORT"])), H).serve_forever()
"#;

#[test]
fn a_server_that_mails_twice_fails_only_on_that_request() {
    let Some(python) = common::python() else {
        return;
    };
    let repo =
        std::env::temp_dir().join(format!("rust-anything-effects-http-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("server.py"), SERVER).unwrap();
    std::fs::write(
        repo.join("twice.py"),
        SERVER.replace("\"MAILS\", \"1\"", "\"MAILS\", \"2\""),
    )
    .unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"health\", \"method\": \"GET\", \"path\": \"/health\"}\n\
         {\"name\": \"signup\", \"method\": \"POST\", \"path\": \"/signup\", \"headers\": {\"Content-Type\": \"application/json\"}, \"body\": \"{\\\"user\\\": \\\"carol\\\"}\"}\n",
    )
    .unwrap();
    let outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "server.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("server"),
        traps: false,
        timeout_secs: None,
        http: true,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects: EffectsConfig::from_args(&["smtp".into()], &[], None, None).unwrap(),
    })
    .unwrap();
    assert_eq!(outcome.cases, 2, "{outcome:?}");
    let http_dir = app::cases_dir(&repo).join("http");
    assert_eq!(
        std::fs::read_to_string(http_dir.join("health.effects")).unwrap(),
        ""
    );
    let mail = std::fs::read_to_string(http_dir.join("signup.effects")).unwrap();
    assert!(
        mail.starts_with("smtp from <app@example.test> to <carol@example.test>\n"),
        "{mail}"
    );

    let same = replay(&repo, &format!("{python} server.py"));
    assert_eq!(same.passed, same.total, "{:?}", same.failures);
    let twice = replay(&repo, &format!("{python} twice.py"));
    assert_eq!((twice.passed, twice.total), (1, 2), "{twice:?}");
    assert_eq!(twice.failures[0].case, "signup");
    assert_eq!(twice.failures[0].problems[0].field, "effects");

    std::fs::remove_dir_all(&repo).unwrap();
}

const DB_APP: &str = r#"import os, sqlite3, sys
db = sqlite3.connect("app.db")
name = sys.argv[1] if len(sys.argv) > 1 else "nobody"
for _ in range(int(os.environ.get("ROWS", "1"))):
    db.execute("insert into users(name) values (?)", (name,))
db.commit()
print("added", name)
"#;

#[test]
fn rows_written_differently_fail_replay_through_the_db_dump() {
    let Some(python) = common::python() else {
        return;
    };
    let repo =
        std::env::temp_dir().join(format!("rust-anything-effects-db-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("add.py"), DB_APP).unwrap();
    std::fs::write(
        repo.join("add_twice.py"),
        DB_APP.replace("\"ROWS\", \"1\"", "\"ROWS\", \"2\""),
    )
    .unwrap();
    std::fs::write(
        repo.join("reset.py"),
        "import sqlite3\ndb = sqlite3.connect('app.db')\ndb.execute('drop table if exists users')\ndb.execute('create table users(id integer primary key, name text)')\ndb.commit()\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("dump.py"),
        "import sqlite3\nfor row in sqlite3.connect('app.db').execute('select id, name from users order by id'):\n    print(*row)\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"dave\", \"args\": [\"dave\"]}\n",
    )
    .unwrap();
    let effects = EffectsConfig::from_args(
        &[],
        &[],
        Some(&format!("{python} reset.py")),
        Some(&format!("{python} dump.py")),
    )
    .unwrap();
    let outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "add.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("add"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects,
    })
    .unwrap();
    assert!(outcome.unstable.is_empty(), "{outcome:?}");
    assert_eq!(
        std::fs::read_to_string(app::cases_dir(&repo).join("dave.effects")).unwrap(),
        "db\n1 dave\n"
    );
    let same = replay(&repo, &format!("{python} add.py"));
    assert_eq!(same.passed, same.total, "{:?}", same.failures);
    let twice = replay(&repo, &format!("{python} add_twice.py"));
    let dave = twice.failures.iter().find(|f| f.case == "dave").unwrap();
    assert_eq!(dave.problems[0].field, "effects", "{dave:?}");

    std::fs::remove_dir_all(&repo).unwrap();
}

const MAILER_A: &str = r#"import os, smtplib, sys
from email.message import EmailMessage

name = sys.argv[1] if len(sys.argv) > 1 else "nobody"
m = EmailMessage()
m["From"] = "App <app@example.test>"
m["To"] = f"{name}@example.test"
m["Subject"] = f"Hello — {name}"
m.set_content(f"hi {name} from café", cte="quoted-printable")
m.add_alternative(f"<p>hi {name}</p>", subtype="html", cte="7bit")
m.add_attachment(b"%PDF-1.4", maintype="application", subtype="pdf", filename="doc.pdf")
with smtplib.SMTP(os.environ["SMTP_HOST"], int(os.environ["SMTP_PORT"]), local_hostname="localhost") as s:
    s.send_message(m)
print("sent", name)
"#;

const MAILER_B: &str = r#"import base64, os, smtplib, sys

name = sys.argv[1] if len(sys.argv) > 1 else "nobody"
plain_b64 = base64.b64encode(f"hi {name} from café".encode("utf-8")).decode("ascii")
subj_b64 = base64.b64encode(f"Hello — {name}".encode("utf-8")).decode("ascii")
raw = (
    f"To: {name}@example.test\r\n"
    f"Subject: =?UTF-8?B?{subj_b64}?=\r\n"
    "From: App <app@example.test>\r\n"
    "Message-ID: <lettre-1@rust.local>\r\n"
    "MIME-Version: 1.0\r\n"
    "Content-Type: multipart/mixed; boundary=rust_outer\r\n"
    "\r\n"
    "--rust_outer\r\n"
    "Content-Type: multipart/alternative; boundary=rust_alt\r\n"
    "\r\n"
    "--rust_alt\r\n"
    "Content-Type: text/plain; charset=utf-8\r\n"
    "Content-Transfer-Encoding: base64\r\n"
    "\r\n"
    f"{plain_b64}\r\n"
    "--rust_alt\r\n"
    "Content-Type: text/html; charset=utf-8\r\n"
    "Content-Transfer-Encoding: quoted-printable\r\n"
    "\r\n"
    f"<p>hi {name}</p>\r\n"
    "--rust_alt--\r\n"
    "--rust_outer\r\n"
    "Content-Type: application/pdf; name=\"doc.pdf\"\r\n"
    "Content-Disposition: attachment; filename=\"doc.pdf\"\r\n"
    "Content-Transfer-Encoding: base64\r\n"
    "\r\n"
    "JVBERi0xLjQ=\r\n"
    "--rust_outer--\r\n"
)
with smtplib.SMTP(os.environ["SMTP_HOST"], int(os.environ["SMTP_PORT"]), local_hostname="localhost") as s:
    s.sendmail("app@example.test", [f"{name}@example.test"], raw.encode("utf-8"))
print("sent", name)
"#;

#[test]
fn same_mail_with_different_mime_framing_matches_unless_smtp_raw() {
    let Some(python) = common::python() else {
        return;
    };
    let repo =
        std::env::temp_dir().join(format!("rust-anything-effects-mime-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("mailer_a.py"), MAILER_A).unwrap();
    std::fs::write(repo.join("mailer_b.py"), MAILER_B).unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"alice\", \"args\": [\"alice\"]}\n",
    )
    .unwrap();

    // Default `smtp`: parsed parts view — different MIME framing still matches.
    let effects = EffectsConfig::from_args(&["smtp".into()], &[], None, None).unwrap();
    let outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "mailer_a.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("mailer"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects,
    })
    .unwrap();
    assert!(outcome.unstable.is_empty(), "{outcome:?}");
    let replayed = replay(&repo, &format!("{python} mailer_b.py"));
    assert_eq!(replayed.passed, replayed.total, "{:?}", replayed.failures);

    // `smtp-raw`: raw MIME bytes — different MIME framing fails on `effects`.
    let raw_effects = EffectsConfig::from_args(&["smtp-raw".into()], &[], None, None).unwrap();
    let raw_outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "mailer_a.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("mailer"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects: raw_effects,
    })
    .unwrap();
    assert!(raw_outcome.unstable.is_empty(), "{raw_outcome:?}");
    let raw_replayed = replay(&repo, &format!("{python} mailer_b.py"));
    assert_eq!(raw_replayed.passed, 0, "{raw_replayed:?}");
    assert_eq!(
        raw_replayed.failures[0].problems[0].field, "effects",
        "{:?}",
        raw_replayed.failures
    );

    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn db_reset_and_dump_run_where_the_case_runs() {
    let Some(python) = common::python() else {
        return;
    };
    let repo =
        std::env::temp_dir().join(format!("rust-anything-effects-cwd-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("scripts")).unwrap();
    std::fs::write(
        repo.join("notes.py"),
        "import sys\nopen('notes.txt', 'a', newline='').write(sys.argv[1] + '\\n')\nprint('added')\n",
    )
    .unwrap();
    // The repository's own notes.txt must be left alone by the reset and unseen by the dump.
    std::fs::write(repo.join("notes.txt"), "repo copy\n").unwrap();
    std::fs::write(
        repo.join("reset.py"),
        "import os\nif os.path.exists('notes.txt'):\n    os.remove('notes.txt')\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("scripts").join("dump.py"),
        "import os, sys\nsys.stdout.buffer.write(open('notes.txt', 'rb').read() if os.path.exists('notes.txt') else b'(no notes.txt)\\n')\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"add\", \"args\": [\"hello\"]}\n",
    )
    .unwrap();
    // Scripts named by a relative path still come from the repository.
    let effects = EffectsConfig::from_args(
        &[],
        &[],
        Some(&format!("{python} reset.py")),
        Some(&format!("{python} scripts/dump.py")),
    )
    .unwrap();
    let outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "notes.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("notes"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects,
    })
    .unwrap();
    assert!(outcome.unstable.is_empty(), "{outcome:?}");
    assert_eq!(
        std::fs::read_to_string(app::cases_dir(&repo).join("add.effects")).unwrap(),
        "db\nhello\n"
    );
    assert_eq!(
        std::fs::read_to_string(repo.join("notes.txt")).unwrap(),
        "repo copy\n"
    );
    let same = replay(&repo, &format!("{python} notes.py"));
    assert_eq!(same.passed, same.total, "{:?}", same.failures);

    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn http_effect_responses_are_saved_and_replayed() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = std::env::temp_dir().join(format!(
        "rust-anything-http-effect-responses-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("fixtures")).unwrap();
    std::fs::write(
        repo.join("fixtures/charge.json"),
        r#"{"id":"ch_42","status":"paid"}"#,
    )
    .unwrap();
    std::fs::write(
        repo.join("pay.py"),
        r#"import json, os, sys, urllib.error, urllib.request
base = os.environ["PAY_API"]
mode = sys.argv[1]
if mode == "charge":
    req = urllib.request.Request(f"{base}/v1/charges", data=b'{"amount":10}', method="POST")
    with urllib.request.urlopen(req) as r:
        data = json.loads(r.read().decode())
        print(f"charged:{r.status}:{data['id']}")
else:
    req = urllib.request.Request(f"{base}/hooks/dead", data=b"{}", method="POST")
    try:
        urllib.request.urlopen(req)
        print("hook:ok")
    except urllib.error.HTTPError as e:
        print(f"hook:err:{e.code}")
"#,
    )
    .unwrap();
    std::fs::write(
        repo.join("inputs.jsonl"),
        "{\"name\": \"charge\", \"args\": [\"charge\"]}\n{\"name\": \"gone\", \"args\": [\"gone\"]}\n",
    )
    .unwrap();

    let effects = EffectsConfig::from_args_with_responses(
        &[],
        &["PAY_API=http-url".into()],
        &[
            "POST /v1/charges=201 @fixtures/charge.json".into(),
            "POST /hooks/*=410".into(),
        ],
        &repo,
        None,
        None,
    )
    .unwrap();
    let outcome = app::record(&RecordOptions {
        repo: &repo,
        run: &[python.to_string(), "pay.py".to_string()],
        inputs: Some(&repo.join("inputs.jsonl")),
        normalize: &[],
        name: Some("pay"),
        traps: false,
        timeout_secs: None,
        http: false,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects,
    })
    .unwrap();
    assert!(outcome.unstable.is_empty(), "{outcome:?}");
    assert_eq!(
        std::fs::read_to_string(app::cases_dir(&repo).join("charge.stdout"))
            .unwrap()
            .trim(),
        "charged:201:ch_42"
    );
    assert_eq!(
        std::fs::read_to_string(app::cases_dir(&repo).join("gone.stdout"))
            .unwrap()
            .trim(),
        "hook:err:410"
    );

    // Even if the fixture file is deleted after recording, replay uses the saved response config.
    std::fs::remove_file(repo.join("fixtures/charge.json")).unwrap();
    let replayed = replay(&repo, &format!("{python} pay.py"));
    assert_eq!(replayed.passed, replayed.total, "{:?}", replayed.failures);

    std::fs::remove_dir_all(&repo).unwrap();
}
