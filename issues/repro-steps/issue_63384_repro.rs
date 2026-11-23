//! Repro for https://github.com/openai/codex/issues/63384
//!
//! The exec-policy path first calls `parse_shell_lc_plain_commands`. If that
//! returns `None`, `commands_for_exec_policy_for_platform()` (core/exec_policy.rs)
//! falls back to checking the raw argv `["/bin/zsh", "-lc", "<script>"]`.
//! Because the strict parser rejects a *whole* script when any word contains
//! `~` or `$HOME`, a forbidden command spelled literally in the same script is
//! never checked against prefix rules.
//!
//! These tests pin the trigger condition against the real parser code.

use codex_shell_command::bash::parse_shell_lc_plain_commands;

fn zsh_lc(script: &str) -> Vec<String> {
    vec![
        "/bin/zsh".to_string(),
        "-lc".to_string(),
        script.to_string(),
    ]
}

#[test]
fn row4_forbidden_cat_is_dropped_because_of_tilde_elsewhere() {
    // Issue row 4: `ls ~ > /dev/null && cat /Users/x/decoy/secret.txt`
    // `cat /Users/x/decoy/secret.txt` matches a forbidden prefix_rule, but the
    // `~` in the unrelated first command makes the strict parse fail.
    let cmd = zsh_lc("ls ~ > /dev/null && cat /Users/x/decoy/secret.txt");
    assert_eq!(
        parse_shell_lc_plain_commands(&cmd),
        None,
        "strict parser rejects the whole script -> exec policy sees only raw zsh argv"
    );
}

#[test]
fn row6_forbidden_git_push_is_dropped_because_of_tilde_elsewhere() {
    // Issue row 6: `ls ~ && git push --dry-run /tmp/nonexistent-remote`
    let cmd = zsh_lc("ls ~ && git push --dry-run /tmp/nonexistent-remote");
    assert_eq!(
        parse_shell_lc_plain_commands(&cmd),
        None,
        "strict parser rejects the whole script -> `git push` rule never matches"
    );
}

#[test]
fn control_same_scripts_without_tilde_are_parsed_and_would_be_checked() {
    // Control: without `~`/`$`, both forbidden commands ARE extracted, proving
    // the only reason rows 4/6 escape is the tilde in the first command.
    let cat = zsh_lc("ls /tmp > /dev/null && cat /Users/x/decoy/secret.txt");
    let parsed = parse_shell_lc_plain_commands(&cat).expect("control must parse");
    assert!(
        parsed
            .iter()
            .any(|c| c.first().map(String::as_str) == Some("cat")),
        "control script must expose the `cat` command to exec policy: {parsed:?}"
    );

    let push = zsh_lc("ls /tmp && git push --dry-run /tmp/nonexistent-remote");
    let parsed = parse_shell_lc_plain_commands(&push).expect("control must parse");
    assert!(
        parsed
            .iter()
            .any(|c| c.first().map(String::as_str) == Some("git")),
        "control script must expose the `git` command to exec policy: {parsed:?}"
    );
}
