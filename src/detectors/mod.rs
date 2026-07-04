use crate::analyzer::DetectedStack;

pub fn detect_stacks(files: &[String]) -> Vec<DetectedStack> {
    let mut stacks = Vec::new();

    detect_exact(files, &mut stacks, "Rust", &["Cargo.toml"]);
    detect_exact(files, &mut stacks, "Node.js", &["package.json"]);
    detect_exact(
        files,
        &mut stacks,
        "Python",
        &["pyproject.toml", "requirements.txt"],
    );
    detect_exact(files, &mut stacks, "Go", &["go.mod"]);
    detect_exact(files, &mut stacks, "Docker", &["Dockerfile"]);
    detect_prefix(files, &mut stacks, "GitHub Actions", ".github/workflows/");

    if has_all(files, &["AGENTS.md", "CLAUDE.md", "GEMINI.md"]) {
        stacks.push(DetectedStack {
            name: "Multi-harness AI agents".to_string(),
            evidence: vec![
                "AGENTS.md".to_string(),
                "CLAUDE.md".to_string(),
                "GEMINI.md".to_string(),
            ],
        });
    }

    stacks
}

fn detect_exact(
    files: &[String],
    stacks: &mut Vec<DetectedStack>,
    name: &str,
    candidates: &[&str],
) {
    let evidence: Vec<String> = candidates
        .iter()
        .filter(|candidate| files.iter().any(|file| file == **candidate))
        .map(|candidate| (*candidate).to_string())
        .collect();

    if !evidence.is_empty() {
        stacks.push(DetectedStack {
            name: name.to_string(),
            evidence,
        });
    }
}

fn detect_prefix(files: &[String], stacks: &mut Vec<DetectedStack>, name: &str, prefix: &str) {
    let evidence: Vec<String> = files
        .iter()
        .filter(|file| file.starts_with(prefix))
        .take(5)
        .cloned()
        .collect();

    if !evidence.is_empty() {
        stacks.push(DetectedStack {
            name: name.to_string(),
            evidence,
        });
    }
}

fn has_all(files: &[String], candidates: &[&str]) -> bool {
    candidates
        .iter()
        .all(|candidate| files.iter().any(|file| file == candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_stack_markers() {
        let files = vec![
            "Cargo.toml".to_string(),
            "Dockerfile".to_string(),
            ".github/workflows/ci.yml".to_string(),
            "AGENTS.md".to_string(),
            "CLAUDE.md".to_string(),
            "GEMINI.md".to_string(),
        ];

        let stacks = detect_stacks(&files);
        let names: Vec<&str> = stacks.iter().map(|stack| stack.name.as_str()).collect();

        assert!(names.contains(&"Rust"));
        assert!(names.contains(&"Docker"));
        assert!(names.contains(&"GitHub Actions"));
        assert!(names.contains(&"Multi-harness AI agents"));
    }
}
