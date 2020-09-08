/*
 * utils: Utility commands.
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
mod cmd;
mod env;
mod project;
mod quit;

pub use self::cmd::*;
use self::env::*;
use self::project::*;
pub use self::quit::Quit;
use crate::core::Core;
use parking_lot::Mutex;
use std::sync::Arc;

pub fn register_utils(core: &mut Core) {
    core.add_command("quit", "q", Arc::new(Mutex::new(Quit::new())));
    core.add_command("save", "", Arc::new(Mutex::new(Save::new())));
    core.add_command("load", "", Arc::new(Mutex::new(Load::new())));
    core.add_command("environment", "e", Arc::new(Mutex::new(Environment::new())));
    core.add_command("environmentReset", "er", Arc::new(Mutex::new(EnvironmentReset::new())));
    let eh = Arc::new(Mutex::new(EnvironmentHelp::new(core)));
    core.add_command("environmentHelp", "eh", eh);
}

pub fn command_line_utils(core: &mut Core) {
    core.add_command("executeScript", "es", Arc::new(Mutex::new(ExecuteScript::new())));
}
