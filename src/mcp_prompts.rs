//! MCP prompts: the skill's playbooks as ready-to-send messages, for agents that load prompts
//! rather than skills.

use serde_json::{Value, json};

const SKILL: &str = include_str!("../skills/rust-anything/SKILL.md");
const WHOLE_APP: &str = include_str!("../skills/rust-anything/whole-app.md");

/// The `prompts/list` result.
pub fn list() -> Value {
    json!({ "prompts": [
        {
            "name": "port_unit",
            "title": "Port one unit to Rust",
            "description": "Port one unit of a repository to Rust: analyze, bridge, record, port, build, diff, shadow, gate.",
            "arguments": [
                { "name": "path", "description": "Repository root.", "required": true },
                { "name": "unit", "description": "Unit id from analyze; the best first port from `start_here` when omitted.", "required": false },
                { "name": "language", "description": "The unit's language (typescript, python, go, ruby, java, kotlin, c, php, csharp or powershell): adds how its constructs map to Rust and the traps that make cases differ.", "required": false },
            ],
        },
        {
            "name": "port_whole_app",
            "title": "Port a whole app to Rust",
            "description": "Rewrite a whole app (CLI, server or library) as a pure Rust app: grill the decisions, record the app, front door, port every unit, lift.",
            "arguments": [
                { "name": "path", "description": "Repository root.", "required": true },
            ],
        },
    ] })
}

/// The `prompts/get` result, or the JSON-RPC invalid-params message.
pub fn get(name: &str, arguments: &Value) -> Result<Value, String> {
    let arg = |key: &str| arguments[key].as_str().filter(|s| !s.trim().is_empty());
    let path = arg("path").ok_or_else(|| format!("prompt `{name}` needs the `path` argument"))?;
    let (description, text) = match name {
        "port_unit" => {
            let target = match arg("unit") {
                Some(unit) => format!(
                    "Port the unit `{unit}` of the repository at {path} to Rust with rust-anything."
                ),
                None => format!(
                    "Port one unit of the repository at {path} to Rust with rust-anything: run \
                     analyze on it first and take the best first port from `start_here`."
                ),
            };
            let language = match arg("language") {
                Some(lang) => format!("\n\n{}", language_file(lang)?.trim()),
                None => String::new(),
            };
            (
                "Port one unit to Rust",
                format!(
                    "{target} Call the port done only when gate passes.\n\nFollow this guide:\n\n{}{language}",
                    body(SKILL)
                ),
            )
        }
        "port_whole_app" => (
            "Port a whole app to Rust",
            format!(
                "Rewrite the app at {path} as a pure Rust app with rust-anything, keeping the \
                 original only as reference.\n\nFollow this guide:\n\n{}\n\nEach unit goes through \
                 the per-unit loop:\n\n{}",
                body(WHOLE_APP),
                body(SKILL)
            ),
        ),
        _ => return Err(format!("unknown prompt `{name}`")),
    };
    let rules = crate::rules::section(std::path::Path::new(path));
    let text = if rules.is_empty() {
        text
    } else {
        format!("{text}\n\n{}", rules.trim_end())
    };
    Ok(json!({
        "description": description,
        "messages": [{ "role": "user", "content": { "type": "text", "text": text } }],
    }))
}

/// `languages/<name>.md` from the skill, or the names there are.
fn language_file(name: &str) -> Result<&'static str, String> {
    let languages = crate::setup::SKILL_FILES.iter().filter_map(|(file, text)| {
        Some((file.strip_prefix("languages/")?.strip_suffix(".md")?, *text))
    });
    let mut names = Vec::new();
    for (lang, text) in languages {
        if lang == name {
            return Ok(text);
        }
        names.push(lang);
    }
    Err(format!(
        "`language` is one of {}, not `{name}`",
        names.join(", ")
    ))
}

fn body(markdown: &str) -> &str {
    let rest = markdown
        .strip_prefix("---\r\n")
        .or_else(|| markdown.strip_prefix("---\n"));
    let stripped = rest.and_then(|r| {
        r.split_once("\r\n---\r\n")
            .or_else(|| r.split_once("\n---\n"))
            .or_else(|| r.split_once("\r\n---\n"))
            .or_else(|| r.split_once("\n---\r\n"))
    });
    stripped.map_or(markdown, |(_, body)| body).trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_unit_adds_the_language_file_it_is_given() {
        let dir = crate::app::temp_dir("prompt-language").unwrap();
        let text = |args: Value| {
            get("port_unit", &args).unwrap()["messages"][0]["content"]["text"]
                .as_str()
                .unwrap()
                .to_string()
        };
        let python = include_str!("../skills/rust-anything/languages/python.md");
        assert!(!text(json!({ "path": dir })).contains(python.trim()));
        assert!(text(json!({ "path": dir, "language": "python" })).contains(python.trim()));
        let err = get("port_unit", &json!({ "path": dir, "language": "cobol" })).unwrap_err();
        assert!(err.contains("typescript"), "{err}");
    }

    #[test]
    fn both_prompts_carry_the_project_rules() {
        let dir = crate::app::temp_dir("prompt-rules").unwrap();
        let text = |name: &str| {
            get(name, &json!({ "path": dir })).unwrap()["messages"][0]["content"]["text"]
                .as_str()
                .unwrap()
                .to_string()
        };
        assert!(!text("port_unit").contains("## Project rules"));
        crate::rules::add(&dir, &["Keep `\\r\\n` line endings in output.".into()]).unwrap();
        for name in ["port_unit", "port_whole_app"] {
            let text = text(name);
            assert!(
                text.ends_with("- Keep `\\r\\n` line endings in output."),
                "{name}: {text}"
            );
            assert!(text.contains("## Project rules"), "{name}");
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn strips_lf_front_matter() {
        assert_eq!(body("---\nname: x\n---\n\n# Title\n"), "# Title");
    }

    #[test]
    fn strips_crlf_front_matter() {
        assert_eq!(body("---\r\nname: x\r\n---\r\n\r\n# Title\r\n"), "# Title");
    }

    #[test]
    fn leaves_text_without_front_matter_unchanged() {
        assert_eq!(body("# Title\n"), "# Title");
        assert_eq!(body("plain markdown"), "plain markdown");
    }

    #[test]
    fn leaves_opening_delimiter_without_closing_delimiter_unchanged() {
        assert_eq!(body("---\nname: x\n"), "---\nname: x");
        assert_eq!(body("---\r\nname: x\r\n"), "---\r\nname: x");
    }
}
