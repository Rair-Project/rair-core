/*
 * core.rs: Linking all rair parts together into 1 module.
 * Copyright (C) 2019  Oddcoder
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::commands::Commands;
use crate::helper::*;
use crate::io::*;
use crate::loc::*;
use crate::utils::command_line_utils;
use crate::utils::register_utils;
use crate::writer::Writer;
use parking_lot::{Mutex, RwLock};
use rair_env::*;
use rair_io::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use std::mem;
use std::sync::Arc;
use yansi::Paint;
#[derive(Serialize, Deserialize)]
pub struct Core {
    pub mode: AddrMode,
    pub io: RIO,
    loc: u64,
    // Every time you add some new serde(skip) variable
    // make sure that this variable is well initialized
    // in the projects commands.
    #[serde(skip)]
    pub stdout: Writer,
    #[serde(skip)]
    pub stderr: Writer,
    #[serde(skip)]
    commands: Arc<Mutex<Commands>>,
    #[serde(skip)]
    pub env: Arc<RwLock<Environment<Core>>>,
}

impl Default for Core {
    fn default() -> Self {
        Core {
            mode: AddrMode::Phy,
            stdout: Writer::new_write(Box::new(io::stdout())),
            stderr: Writer::new_write(Box::new(io::stderr())),
            io: RIO::new(),
            loc: 0,
            commands: Default::default(),
            env: Default::default(),
        }
    }
}
fn set_global_color(_: &str, value: bool, _: &Environment<Core>, _: &mut Core) -> bool {
    if value {
        Paint::enable();
    } else {
        Paint::disable();
    }
    true
}
impl Core {
    pub(crate) fn load_commands(&mut self) {
        register_io(self);
        register_loc(self);
        register_utils(self);
        command_line_utils(self);
    }
    /// Returns list of all available commands in [Core].
    pub fn commands(&mut self) -> Arc<Mutex<Commands>> {
        self.commands.clone()
    }
    pub fn set_commands(&mut self, commands: Arc<Mutex<Commands>>) {
        self.commands = commands
    }
    fn init_colors(&mut self, enable: bool) {
        let locked_env = self.env.clone();
        let mut env = locked_env.write();
        env.add_bool_with_cb("color.enable", enable, "Enable/Disable color theme globally", self, set_global_color).unwrap();
        env.add_color("color.1", (0x58, 0x68, 0x75), "").unwrap();
        env.add_color("color.2", (0xb5, 0x89, 0x00), "").unwrap();
        env.add_color("color.3", (0xcb, 0x4b, 0x16), "").unwrap();
        env.add_color("color.4", (0xdc, 0x32, 0x2f), "").unwrap();
        env.add_color("color.5", (0xd3, 0x36, 0x82), "").unwrap();
        env.add_color("color.6", (0x6c, 0x71, 0xc4), "").unwrap();
        env.add_color("color.7", (0x26, 0x8b, 0xd2), "").unwrap();
        env.add_color("color.8", (0x2a, 0xa1, 0x98), "").unwrap();
        env.add_color("color.9", (0x85, 0x99, 0x00), "").unwrap();
    }
    fn new_settings(color: bool) -> Self {
        let mut core: Core = Default::default();
        core.init_colors(color);
        core.load_commands();
        core
    }
    pub fn new() -> Self {
        Core::new_settings(true)
    }
    pub fn new_no_colors() -> Self {
        Core::new_settings(false)
    }
    pub fn set_loc(&mut self, loc: u64) {
        self.loc = loc;
    }

    pub fn get_loc(&self) -> u64 {
        self.loc
    }

    pub fn add_command(&mut self, long: &'static str, short: &'static str, funcs: MRc<dyn Cmd + Sync + Send>) {
        if !long.is_empty() && !self.commands.lock().add_command(long, funcs.clone()) {
            let msg = format!("Command {} already existed.", Paint::default(long).bold());
            error_msg(self, "Cannot add this command.", &msg);
        }

        if !short.is_empty() && !self.commands.lock().add_command(short, funcs) {
            let msg = format!("Command {} already existed.", Paint::default(short).bold());
            error_msg(self, "Cannot add this command.", &msg);
        }
    }
    fn command_not_found(&mut self, command: &str) {
        let msg = format!("Command {} is not found.", Paint::default(command).bold());
        error_msg(self, "Execution failed", &msg);
        let commands = self.commands.lock();
        let similar = commands.suggest(&command.to_string(), 2);
        let mut s = similar.iter();
        if let Some(suggestion) = s.next() {
            let (r, g, b) = self.env.read().get_color("color.6").unwrap();
            write!(self.stderr, "Similar command: {}", Paint::rgb(r, g, b, suggestion)).unwrap();
            for suggestion in s {
                write!(self.stderr, ", {}", Paint::rgb(r, g, b, suggestion)).unwrap();
            }
            writeln!(self.stderr, ".").unwrap();
        }
    }

    pub fn run(&mut self, command: &str, args: &[String]) {
        let cmds = self.commands.clone();
        let cmds_ref = cmds.lock();
        let cmd = cmds_ref.find(&command.to_string());
        if let Some(cmd) = cmd {
            cmd.lock().run(self, args);
        } else {
            drop(cmds_ref);
            self.command_not_found(command)
        }
    }

    pub fn run_at(&mut self, command: &str, args: &[String], at: u64) {
        let old_loc = mem::replace(&mut self.loc, at);
        self.run(command, args);
        self.loc = old_loc;
    }

    pub fn help(&mut self, command: &str) {
        let cmds = self.commands.clone();
        let cmds_ref = cmds.lock();
        let cmd = cmds_ref.find(&command.to_string());
        if let Some(cmd) = cmd {
            cmd.as_ref().lock().help(self);
        } else {
            drop(cmds_ref);
            self.command_not_found(command);
        }
    }
}

#[cfg(test)]
mod test_core {
    use super::*;
    use crate::utils::Quit;
    use parking_lot::Mutex;
    use std::sync::Arc;
    #[test]
    fn test_loc() {
        let mut core = Core::new_no_colors();
        core.set_loc(0x500);
        assert_eq!(core.get_loc(), 0x500);
    }
    #[test]
    fn test_add_command() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.add_command("a_non_existing_command", "a", Arc::new(Mutex::new(Quit::new())));
        assert_eq!(core.stderr.utf8_string().unwrap(), "");
        assert_eq!(core.stdout.utf8_string().unwrap(), "");
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.add_command("test_command", "s", Arc::new(Mutex::new(Quit::new())));
        assert_eq!(core.stderr.utf8_string().unwrap(), "Error: Cannot add this command.\nCommand s already existed.\n");
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.add_command("seek", "test_stuff", Arc::new(Mutex::new(Quit::new())));
        assert_eq!(core.stderr.utf8_string().unwrap(), "Error: Cannot add this command.\nCommand seek already existed.\n");
    }
    #[test]
    fn test_help() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.help("seeker");
        assert_eq!(core.stdout.utf8_string().unwrap(), "");
        assert_eq!(core.stderr.utf8_string().unwrap(), "Error: Execution failed\nCommand seeker is not found.\nSimilar command: seek.\n");
    }
    #[test]
    fn test_run_at() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.run_at("mep", &[], 0x500);
        assert_eq!(core.stdout.utf8_string().unwrap(), "");
        assert_eq!(
            core.stderr.utf8_string().unwrap(),
            "Error: Execution failed\nCommand mep is not found.\nSimilar command: map, maps, m, e, er, eh, es.\n"
        );
    }
}
