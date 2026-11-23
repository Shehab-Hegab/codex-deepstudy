//! Faithful extraction of the parser functions in
//! `openai/codex` @ 06d76ee: `codex-rs/shell-command/src/bash.rs`.
//! Used to verify issue #63384 without building the whole 130-crate workspace.

use tree_sitter::{Node, Parser, Tree};
use tree_sitter_bash::LANGUAGE as BASH;

pub fn try_parse_shell(script: &str) -> Option<Tree> {
    let lang = BASH.into();
    let mut parser = Parser::new();
    parser.set_language(&lang).expect("load bash grammar");
    parser.parse(script, None)
}

/// Exact port of bash.rs:257 `is_literal_word_or_number`.
fn is_literal_word_or_number(node: Node<'_>, src: &str) -> bool {
    if !matches!(node.kind(), "word" | "number") {
        return false;
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor).next().is_none()
        && node.utf8_text(src.as_bytes()).is_ok_and(|word| {
            !word.starts_with('=')
                && !word.contains(['{', '}', '*', '?', '[', ']', '\\', '~', '^', '#', '$', '`'])
        })
}

/// Exact port of bash.rs:29 `try_parse_word_only_commands_sequence`.
pub fn try_parse_word_only_commands_sequence(tree: &Tree, src: &str) -> Option<Vec<Vec<String>>> {
    if tree.root_node().has_error() {
        return None;
    }
    const ALLOWED_KINDS: &[&str] = &[
        "program", "list", "pipeline", "command", "command_name", "word", "string",
        "string_content", "raw_string", "number", "concatenation",
    ];
    const ALLOWED_PUNCT_TOKENS: &[&str] = &["&&", "||", ";", "|", "\"", "'"];

    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut stack = vec![root];
    let mut command_nodes = Vec::new();
    while let Some(node) = stack.pop() {
        let kind = node.kind();
        if node.is_named() {
            if !ALLOWED_KINDS.contains(&kind) {
                return None;
            }
            if matches!(kind, "word" | "number") && !is_literal_word_or_number(node, src) {
                return None;
            }
            if kind == "command" {
                command_nodes.push(node);
            }
        } else {
            if kind.chars().any(|c| "&;|".contains(c)) && !ALLOWED_PUNCT_TOKENS.contains(&kind) {
                return None;
            }
            if !(ALLOWED_PUNCT_TOKENS.contains(&kind) || kind.trim().is_empty()) {
                return None;
            }
        }
        for child in node.children(&mut cursor) {
            stack.push(child);
        }
    }
    command_nodes.sort_by_key(Node::start_byte);
    let mut commands = Vec::new();
    for node in command_nodes {
        commands.push(parse_plain_command_from_node(node, src)?);
    }
    Some(commands)
}

fn parse_double_quoted_string(node: Node, src: &str) -> Option<String> {
    if node.kind() != "string" {
        return None;
    }
    let mut cursor = node.walk();
    for part in node.named_children(&mut cursor) {
        if part.kind() != "string_content" {
            return None;
        }
    }
    let raw = node.utf8_text(src.as_bytes()).ok()?;
    let stripped = raw.strip_prefix('"')?.strip_suffix('"')?;
    if stripped.as_bytes().windows(2).any(|pair| {
        pair[0] == b'\\' && matches!(pair[1], b'$' | b'`' | b'"' | b'\\' | b'\n')
    }) {
        return None;
    }
    Some(stripped.to_owned())
}

fn parse_raw_string(node: Node<'_>, src: &str) -> Option<String> {
    if node.kind() != "raw_string" {
        return None;
    }
    let raw = node.utf8_text(src.as_bytes()).ok()?;
    raw.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')).map(str::to_owned)
}

fn parse_plain_command_from_node(cmd: Node<'_>, src: &str) -> Option<Vec<String>> {
    if cmd.kind() != "command" {
        return None;
    }
    let mut words = Vec::new();
    let mut cursor = cmd.walk();
    for child in cmd.named_children(&mut cursor) {
        match child.kind() {
            "command_name" => {
                let word_node = child.named_child(0)?;
                if word_node.kind() != "word" {
                    return None;
                }
                words.push(word_node.utf8_text(src.as_bytes()).ok()?.to_owned());
            }
            "word" | "number" => words.push(child.utf8_text(src.as_bytes()).ok()?.to_owned()),
            "string" => words.push(parse_double_quoted_string(child, src)?),
            "raw_string" => words.push(parse_raw_string(child, src)?),
            _ => return None,
        }
    }
    Some(words)
}

fn parse_literal_shell_word(node: Node<'_>, src: &str) -> Option<String> {
    match node.kind() {
        "word" | "number" if is_literal_word_or_number(node, src) => {
            Some(node.utf8_text(src.as_bytes()).ok()?.to_owned())
        }
        "string" => parse_double_quoted_string(node, src),
        "raw_string" => parse_raw_string(node, src),
        _ => None,
    }
}

/// Exact port of bash.rs:221 `parse_literal_command_from_node`.
fn parse_literal_command_from_node(cmd: Node<'_>, src: &str) -> Option<Vec<String>> {
    if cmd.kind() != "command" {
        return None;
    }
    let mut words = Vec::new();
    let mut found_command_name = false;
    let mut cursor = cmd.walk();
    for child in cmd.named_children(&mut cursor) {
        if child.kind() == "command_name" {
            words.push(parse_literal_shell_word(child.named_child(0)?, src)?);
            found_command_name = true;
        } else if found_command_name && let Some(word) = parse_literal_shell_word(child, src) {
            words.push(word);
        }
    }
    found_command_name.then_some(words)
}

/// Exact port of bash.rs:133 `parse_shell_lc_literal_commands` (script part).
pub fn literal_parse(script: &str) -> Option<Vec<Vec<String>>> {
    let tree = try_parse_shell(script)?;
    let root = tree.root_node();
    if root.has_error() {
        return None;
    }
    let mut commands = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.kind() == "command"
            && let Some(command) = parse_literal_command_from_node(node, script)
        {
            commands.push(command);
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    Some(commands)
}

pub fn strict_parse(script: &str) -> Option<Vec<Vec<String>>> {
    let tree = try_parse_shell(script)?;
    try_parse_word_only_commands_sequence(&tree, script)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW4: &str = "ls ~ > /dev/null && cat /Users/x/decoy/secret.txt";
    const ROW6: &str = "ls ~ && git push --dry-run /tmp/nonexistent-remote";
    const ROW1: &str = "cat /Users/x/decoy/secret.txt";
    const ROW5: &str = "git push --dry-run /tmp/nonexistent-remote";

    #[test]
    fn rows_1_and_5_are_checked_today() {
        assert_eq!(strict_parse(ROW1), Some(vec![vec!["cat".into(), "/Users/x/decoy/secret.txt".into()]]));
        assert_eq!(strict_parse(ROW5), Some(vec![vec!["git".into(), "push".into(), "--dry-run".into(), "/tmp/nonexistent-remote".into()]]));
    }

    #[test]
    fn rows_4_and_6_strict_parse_returns_none_the_bypass_trigger() {
        assert_eq!(strict_parse(ROW4), None);
        assert_eq!(strict_parse(ROW6), None);
    }

    #[test]
    fn proposed_fallback_recovers_forbidden_commands_in_rows_4_and_6() {
        let l4 = literal_parse(ROW4).expect("literal parse must succeed");
        assert!(l4.iter().any(|c| c.first().map(String::as_str) == Some("cat")), "{l4:?}");
        let l6 = literal_parse(ROW6).expect("literal parse must succeed");
        assert!(l6.iter().any(|c| c.first().map(String::as_str) == Some("git")), "{l6:?}");
    }
}
