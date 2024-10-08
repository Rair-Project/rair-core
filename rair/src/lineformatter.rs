//! Autocompletion / hinting / colorzing input.

use alloc::borrow::Cow::{self, Owned};
use alloc::sync::Arc;
use parking_lot::Mutex;
use rair_cmd::ParseTree;
use rair_core::Commands;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::Context;
use rustyline_derive::{Helper, Validator};
use yansi::Paint;

#[derive(Helper, Validator)]
pub struct LineFormatter {
    hinter: HistoryHinter,
    commands: Arc<Mutex<Commands>>,
}

impl LineFormatter {
    pub fn new(commands: Arc<Mutex<Commands>>) -> Self {
        LineFormatter {
            hinter: HistoryHinter {},
            commands,
        }
    }
    fn tree_complete(&self, tree: ParseTree) -> (usize, Vec<Pair>) {
        match tree {
            // If we have a help then we just return
            // all the commands sharing same prefix ending with the help token
            ParseTree::Help(help) => {
                let mut ret = Vec::new();
                for suggestion in self.commands.lock().prefix(&help.command) {
                    let display = (*suggestion).to_owned();
                    let mut replacement = (*suggestion).to_owned();
                    replacement.push('?');
                    ret.push(Pair {
                        display,
                        replacement,
                    });
                }
                (0, ret)
            }
            // if it is command
            // first if we are taking arguments no autocomplate else autocomplete normally ;)
            ParseTree::Cmd(cmd) => {
                if !cmd.args.is_empty() {
                    return (0, Vec::new());
                }
                let mut ret = Vec::new();
                for suggestion in self.commands.lock().prefix(&cmd.command) {
                    let display = (*suggestion).to_owned();
                    let replacement = (*suggestion).to_owned();
                    ret.push(Pair {
                        display,
                        replacement,
                    });
                }
                (0, ret)
            }
            ParseTree::Comment | ParseTree::NewLine => (0, Vec::new()),
            ParseTree::HelpAll => unreachable!(),
        }
    }
}

impl Completer for LineFormatter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        // first figure which token are we completing
        // we will do so by starting at line[pos] and keep incrementing till:
        //  A- we get to see a white space
        //  B- we reach end of text.
        let mut p = pos;
        while p < line.len() {
            let c: Option<char> = line.chars().nth(p);
            if let Some(character) = c {
                if character.is_whitespace() {
                    break;
                }
            }
            p += 1;
        }
        // next we parse the line
        let t = ParseTree::construct(&line[0..p]);
        match t {
            Err(_) => Ok((0, Vec::new())),
            Ok(tree) => Ok(self.tree_complete(tree)),
        }
    }
}

impl Hinter for LineFormatter {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for LineFormatter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("{}", hint.primary().bold().italic().dim()))
    }
}
