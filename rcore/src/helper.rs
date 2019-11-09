/*
 * helper.rs: Helper functions for implementing external or internal commands.
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

use core::*;
use std::fmt;
use std::fmt::Display;
use std::io::Write;
use std::num;
use yansi::Paint;

pub fn str_to_num(n: &str) -> Result<u64, num::ParseIntError> {
    if n.len() >= 2 {
        match &*n[0..2].to_lowercase() {
            "0b" => return u64::from_str_radix(&n[2..], 2),
            "0x" => return u64::from_str_radix(&n[2..], 16),
            _ => (),
        }
    }
    if n.chars().nth(0).unwrap() == '0' {
        return u64::from_str_radix(&n[1..], 8);
    }
    return u64::from_str_radix(n, 10);
}

pub fn expect(core: &mut Core, args_len: u64, expect: u64) {
    let (r, g, b) = core.color_palette[3];
    let error = Paint::rgb(r, g, b, "Arguments Error").bold();
    let expected = Paint::rgb(r, g, b, format!("{}", expect));
    let found = Paint::rgb(r, g, b, format!("{}", args_len));
    writeln!(core.stderr, "{}: Expected {} argument(s), found {}.", error, expected, found).unwrap();
}

pub fn error_msg(core: &mut Core, title: &str, msg: &str) {
    let (r, g, b) = core.color_palette[3];
    writeln!(core.stderr, "{}: {}", Paint::rgb(r, g, b, "Error").bold(), Paint::rgb(r, g, b, title)).unwrap();
    writeln!(core.stderr, "{}", msg).unwrap();
}

pub fn help(core: &mut Core, long: &str, short: &str, usage: Vec<(&str, &str)>) {
    let (r1, g1, b1) = core.color_palette[5];
    let (r2, g2, b2) = core.color_palette[6];
    writeln!(core.stdout, "Commands: [{} | {}]\n", Paint::rgb(r1, g1, b1, long), Paint::rgb(r1, g1, b1, short)).unwrap();
    writeln!(core.stdout, "Usage:").unwrap();
    for (args, description) in usage {
        writeln!(core.stdout, "{} {}\t{}", Paint::rgb(r1, g1, b1, short), Paint::rgb(r2, g2, b2, args), description,).unwrap()
    }
}

pub struct CmdFunctions {
    pub run: fn(&mut Core, &Vec<String>),
    pub help: fn(&mut Core),
}

pub enum AddrMode {
    Vir,
    Phy,
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrMode::Phy => write!(f, "Phy"),
            AddrMode::Vir => write!(f, "Vir"),
        }
    }
}

#[cfg(test)]
mod test_helper {
    use super::*;
    use std::fmt::Write;
    use writer::Writer;
    #[test]
    fn test_str_to_num() {
        assert_eq!(str_to_num("12345").unwrap(), 12345);
        assert_eq!(str_to_num("012345").unwrap(), 0o12345);
        assert_eq!(str_to_num("0b101001").unwrap(), 0b101001);
        assert_eq!(str_to_num("0x12345").unwrap(), 0x12345);
        assert_eq!(str_to_num("0X1F2f345").unwrap(), 0x1f2f345);
        assert!(str_to_num("0x12345123451234512").is_err());
    }

    #[test]
    fn test_except() {
        let mut core = Core::new();
        core.stderr = Writer::new_buf();
        Paint::disable();
        expect(&mut core, 5, 7);
        assert_eq!(core.stderr.utf8_string().unwrap(), "Arguments Error: Expected 7 argument(s), found 5.\n");
    }

    #[test]
    fn test_error_msg() {
        let mut core = Core::new();
        core.stderr = Writer::new_buf();
        Paint::disable();
        error_msg(&mut core, "Error Title", "Something might have failed.");
        assert_eq!(core.stderr.utf8_string().unwrap(), "Error: Error Title\nSomething might have failed.\n");
    }
    #[test]
    fn test_help() {
        let mut core = Core::new();
        core.stdout = Writer::new_buf();
        Paint::disable();
        help(&mut core, "Test", "t", vec![("t1", "test 1"), ("t2", "test 2")]);
        assert_eq!(core.stdout.utf8_string().unwrap(), "Commands: [Test | t]\n\nUsage:\nt t1\ttest 1\nt t2\ttest 2\n");
    }
    #[test]
    fn test_addr_mode() {
        let mut s = String::new();
        write!(s, "{} {}", AddrMode::Phy, AddrMode::Vir).unwrap();
        assert_eq!(s, "Phy Vir");
    }
}