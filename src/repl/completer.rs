use crate::help::{get_command_names, get_player_resource_completions, get_query_completions};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct OwcliCompleter {
    static_paths: Vec<String>,
    player_resources: Vec<String>,
    commands: Vec<String>,
}

impl OwcliCompleter {
    pub fn new() -> Self {
        Self {
            static_paths: get_query_completions()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            player_resources: get_player_resource_completions()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            // Dynamic from Clap introspection - stays in sync with cli.rs
            commands: get_command_names(),
        }
    }
}

impl Completer for OwcliCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line_to_cursor = &line[..pos];
        let words: Vec<&str> = line_to_cursor.split_whitespace().collect();

        // If empty or just starting, complete with paths or keywords
        if words.is_empty() || (words.len() == 1 && !line_to_cursor.ends_with(' ')) {
            let prefix = words.first().unwrap_or(&"");
            let mut completions = Vec::new();

            // Add static paths
            for path in &self.static_paths {
                if path.starts_with(prefix) {
                    completions.push(Pair {
                        display: path.clone(),
                        replacement: path.clone(),
                    });
                }
            }

            // Add special keywords
            for kw in &["help", "exit", "quit", "command", "tiles"] {
                if kw.starts_with(prefix) {
                    completions.push(Pair {
                        display: kw.to_string(),
                        replacement: kw.to_string(),
                    });
                }
            }

            let start = line_to_cursor.len() - prefix.len();
            return Ok((start, completions));
        }

        // After "command", complete with action names
        if words.len() >= 1 && (words[0] == "command" || words[0] == "cmd") {
            if words.len() == 1 || (words.len() == 2 && !line_to_cursor.ends_with(' ')) {
                let prefix = if words.len() == 2 { words[1] } else { "" };
                let completions: Vec<Pair> = self
                    .commands
                    .iter()
                    .filter(|cmd| cmd.starts_with(prefix))
                    .map(|cmd| Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    })
                    .collect();

                let start = if words.len() == 2 {
                    line_to_cursor.len() - prefix.len()
                } else {
                    line_to_cursor.len()
                };
                return Ok((start, completions));
            }
        }

        // Complete player resources after player/<index>/
        let last_word = words.last().unwrap_or(&"");
        if last_word.starts_with("player/") {
            let parts: Vec<&str> = last_word.split('/').collect();
            if parts.len() == 3 {
                // player/0/un... -> complete with resources
                let prefix = parts[2];
                let completions: Vec<Pair> = self
                    .player_resources
                    .iter()
                    .filter(|r| r.starts_with(prefix))
                    .map(|r| Pair {
                        display: r.clone(),
                        replacement: format!("player/{}/{}", parts[1], r),
                    })
                    .collect();

                let start = line_to_cursor.len() - last_word.len();
                return Ok((start, completions));
            } else if parts.len() == 2 && !last_word.ends_with('/') {
                // player/0 -> suggest adding /
                let completions = vec![Pair {
                    display: format!("{}/", last_word),
                    replacement: format!("{}/", last_word),
                }];
                let start = line_to_cursor.len() - last_word.len();
                return Ok((start, completions));
            }
        }

        Ok((pos, Vec::new()))
    }
}

impl Hinter for OwcliCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for OwcliCompleter {}
impl Validator for OwcliCompleter {}
impl Helper for OwcliCompleter {}
