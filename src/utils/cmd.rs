/*
 * cmd.rs: commands for handling another commands.
 * Copyright (C) 2020  gogo
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

use crate::core::*;
use crate::helper::*;
use std::fs::File;
use std::path::Path;

#[derive(Default)]
pub struct ExecuteScript {}

impl ExecuteScript {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Cmd for ExecuteScript {
    fn run(&mut self, core: &mut Core, args: &[String]) {
        if args.len() != 1 {
            expect(core, args.len() as u64, 1);
            return;
        }
        let path = Path::new(&args[0]);
        let display = path.display();

        let _script_file = match File::open(&path) {
            Err(why) => return error_msg(core, &format!("Failed to load file {}.", &display), &why.to_string()),
            Ok(script_file) => script_file,
        };
    }
    fn help(&self, core: &mut Core) {
        help(core, &"executeScript", &"es", vec![("[file].rr", "execute each command coded in [file] script.")]);
    }
}

#[cfg(test)]
mod test_cmd {
    use super::*;
    use crate::writer::*;
    use rair_env::Environment as Env;
    #[test]
    fn test_help() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.help("es");
    }

    #[test]
    fn test_cmd_help() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        core.run("es", &["no utils/unit-tests.rr".to_string()]);
    }
}
