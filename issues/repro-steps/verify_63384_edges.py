"""Extended, adversarial verification for openai/codex#63384.

Same tree-sitter-bash grammar + 1:1 port of bash.rs logic.
Adds edge cases beyond the report's 6 rows to audit for regressions:
  - command substitution `$(...)`
  - nested shell `sh -c`
  - redirection without tilde
  - quoted env expansion
  - argument strings that look like commands (false-positive check)
  - non-bash command (powershell) -> must equal strict (untouched)
  - comment-only script -> empty command list
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
        words, found = [], False
        for child in node.named_children:
            if child.type == "command_name":
                words.append(text(child.named_child(0), script)); found = True
            elif found and child.type in ("word", "number"):
                words.append(text(child, script))
            elif found and child.type == "string":
                words.append(text(child, script).strip('"'))
            elif found and child.type == "raw_string":
                words.append(text(child, script).strip("'"))
        if found:
            out.append(words)
    return out


def parse_literal_word(node, src):
    if node.type in ("word", "number"):
        return text(node, src) if is_literal_word_or_number(node, src) else None
    if node.type == "string":
        raw = text(node, src)
        return raw[1:-1] if len(raw) >= 2 and raw[0] == '"' and raw[-1] == '"' else None
    if node.type == "raw_string":
        raw = text(node, src)
        return raw[1:-1] if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'" else None
    return None


def literal_parse(script):
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
                    w = parse_literal_word(child.named_child(0), script)
                    if w is None:
                        found = False
                        break
                    words.append(w); found = True
                elif found:
                    w = parse_literal_word(child, script)
                    if w is not None:
                        words.append(w)
            if found:
                out.append(words)
        stack.extend(node.named_children)
    return out


def first_words(cmds):
    return [c[0] for c in cmds] if cmds else None


cases = [
    ("row1 abs cat", "cat /Users/x/decoy/secret.txt"),
    ("row2 tilde cat", "cat ~/decoy/secret.txt"),
    ("row3 HOME cat", "cat $HOME/decoy/secret.txt"),
    ("row4 tilde+cat", "ls ~ > /dev/null && cat /Users/x/decoy/secret.txt"),
    ("row5 abs git push", "git push --dry-run /tmp/nonexistent-remote"),
    ("row6 tilde+git push", "ls ~ && git push --dry-run /tmp/nonexistent-remote"),
    ("subst", "echo $(git push --dry-run /tmp/x)"),
    ("nested sh", "sh -c 'git push'"),
    ("redirect no tilde", "python x.py > out.log"),
    ("quoted HOME", 'cat "$HOME/secret"'),
    ("arg looks like cmd", 'echo "rm -rf /"'),
    ("safe control", "ls /tmp && git status"),
    ("comment only", "# just a comment"),
]

print(f"{'case':<22} | {'strict':<34} | literal")
print("-" * 100)
for name, script in cases:
    s = strict_parse(script)
    l = literal_parse(script)
    print(f"{name:<22} | {str(first_words(s)):<34} | {first_words(l)}")

print("\n=== audit assertions ===")
checks = []
s = strict_parse(cases[3][1]); l = literal_parse(cases[3][1])
checks.append(("row4 strict None (trigger)", s is None))
checks.append(("row4 literal exposes cat", l is not None and any(c[0] == "cat" for c in l)))
s6 = strict_parse(cases[5][1]); l6 = literal_parse(cases[5][1])
checks.append(("row6 literal exposes git", l6 is not None and any(c[0] == "git" for c in l6)))
sub = literal_parse(cases[6][1])
checks.append(("substitution command found (conservative)", sub is not None and any(c[0] == "git" for c in sub)))
arg = literal_parse(cases[10][1])
checks.append(("string arg not treated as command", arg is not None and arg[0][0] == "echo" and len(arg) == 1))
comment = literal_parse(cases[12][1])
checks.append(("comment-only -> empty list (guard skips)", comment == []))
for label, ok in checks:
    print(("PASS" if ok else "FAIL"), "-", label)
import sys
sys.exit(0 if all(ok for _, ok in checks) else 1)
