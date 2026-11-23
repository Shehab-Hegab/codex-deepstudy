"""Independent cross-check of openai/codex#63384 using the same tree-sitter-bash
grammar and the exact logic from codex-rs/shell-command/src/bash.rs.

Mirrors:
  try_parse_word_only_commands_sequence()  (bash.rs:29)
  is_literal_word_or_number()              (bash.rs:257)
  parse_literal_command_from_node()        (bash.rs:~220)
"""
from tree_sitter import Language, Parser
import tree_sitter_bash

BASH = Language(tree_sitter_bash.language())
parser = Parser(BASH)

ALLOWED_KINDS = {
    "program", "list", "pipeline", "command", "command_name",
    "word", "string", "string_content", "raw_string", "number", "concatenation",
}
ALLOWED_PUNCT = {"&&", "||", ";", "|", '"', "'"}
BAD_CHARS = set("{}*?[]\\~^#$`")


def text(node, src):
    return src[node.start_byte:node.end_byte]


def is_literal_word_or_number(node, src):
    if node.type not in ("word", "number"):
        return False
    if node.named_child_count != 0:
        return False
    w = text(node, src)
    return not w.startswith("=") and not any(c in BAD_CHARS for c in w)


def strict_parse(script):
    """-> list[list[str]] or None. Exact port of the strict path."""
    tree = parser.parse(script.encode())
    root = tree.root_node
    if root.has_error:
        return None
    stack, command_nodes = [root], []
    while stack:
        node = stack.pop()
        kind = node.type
        if node.is_named:
            if kind not in ALLOWED_KINDS:
                return None
            if kind in ("word", "number") and not is_literal_word_or_number(node, script):
                return None
            if kind == "command":
                command_nodes.append(node)
        else:
            if any(c in "&;|" for c in kind) and kind not in ALLOWED_PUNCT:
                return None
            if not (kind in ALLOWED_PUNCT or not kind.strip()):
                return None
        stack.extend(node.children)
    command_nodes.sort(key=lambda n: n.start_byte)
    out = []
    for node in command_nodes:
        words = []
        found_name = False
        for child in node.named_children:
            if child.type == "command_name":
                words.append(text(child.named_child(0), script))
                found_name = True
            elif found_name and child.type in ("word", "number"):
                words.append(text(child, script))
            elif found_name and child.type == "string":
                words.append(text(child, script).strip('"'))
            elif found_name and child.type == "raw_string":
                words.append(text(child, script).strip("'"))
        if found_name:
            out.append(words)
    return out


def literal_parse(script):
    """-> list[list[str]] or None. Port of parse_shell_lc_literal_commands()."""
    tree = parser.parse(script.encode())
    root = tree.root_node
    if root.has_error:
        return None
    out, stack = [], [root]
    while stack:
        node = stack.pop()
        if node.type == "command":
            words, found = [], False
            for child in node.named_children:
                if child.type == "command_name":
                    cn = child.named_child(0)
                    if cn is not None and is_literal_word_or_number(cn, script):
                        words.append(text(cn, script)); found = True
                elif found:
                    if is_literal_word_or_number(child, script):
                        words.append(text(child, script))
                    elif child.type == "raw_string":
                        words.append(text(child, script).strip("'"))
            if found:
                out.append(words)
        stack.extend(node.named_children)
    return out


rows = [
    ("1", "cat /Users/x/decoy/secret.txt"),
    ("2", "cat ~/decoy/secret.txt"),
    ("3", "cat $HOME/decoy/secret.txt"),
    ("4", "ls ~ > /dev/null && cat /Users/x/decoy/secret.txt"),
    ("5", "git push --dry-run /tmp/nonexistent-remote"),
    ("6", "ls ~ && git push --dry-run /tmp/nonexistent-remote"),
]

print("ROW | STRICT PARSE (what policy sees today) | LITERAL PARSE (proposed fallback)")
print("-" * 100)
for n, script in rows:
    print(f"{n:>3} | {str(strict_parse(script)):<37} | {literal_parse(script)}")
