//! Commands to save/load projects.

use crate::core::Core;
use crate::helper::{error_msg, expect};
use crate::Cmd;
use core::mem;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Default)]
pub struct Save;

impl Cmd for Save {
    fn run(&mut self, core: &mut Core, args: &[String]) {
        if args.len() != 1 {
            expect(core, args.len() as u64, 1);
            return;
        }
        let data = match serde_cbor::to_vec(&core) {
            Ok(data) => data,
            Err(e) => return error_msg(core, "Failed to serialize project", &e.to_string()),
        };
        let mut file = match File::create(&args[0]) {
            Ok(file) => file,
            Err(e) => return error_msg(core, "Failed to open file", &e.to_string()),
        };
        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
        compressor.write_all(&data).unwrap();
        let compressed_data = compressor.finish().unwrap();
        if let Err(e) = file.write_all(&compressed_data) {
            error_msg(core, "Failed to save project", &e.to_string());
        }
    }
    fn commands(&self) -> &'static [&'static str] {
        &["save"]
    }
    fn help_messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("[file_path]", "Save project into given path.")]
    }
}

#[derive(Default)]
pub struct Load;

impl Cmd for Load {
    fn run(&mut self, core: &mut Core, args: &[String]) {
        if args.len() != 1 {
            expect(core, args.len() as u64, 1);
            return;
        }
        let compressed_data = match fs::read(&args[0]) {
            Ok(data) => data,
            Err(e) => return error_msg(core, "Failed to load project", &e.to_string()),
        };
        let mut data = Vec::new();
        let mut decompressor = ZlibDecoder::new(data);
        if let Err(e) = decompressor.write_all(&compressed_data) {
            return error_msg(core, "Failed to decompress project", &e.to_string());
        }
        data = match decompressor.finish() {
            Ok(data) => data,
            Err(e) => return error_msg(core, "Failed to decompress project", &e.to_string()),
        };
        let mut deserializer = serde_cbor::Deserializer::from_slice(&data);

        let mut core2: Core = match Core::deserialize(&mut deserializer) {
            Ok(core) => core,
            Err(e) => return error_msg(core, "Failed to load project", &e.to_string()),
        };
        mem::swap(&mut core.stdout, &mut core2.stdout);
        mem::swap(&mut core.stderr, &mut core2.stderr);
        mem::swap(&mut core.env, &mut core2.env);
        core2.set_commands(core.commands());
        *core = core2;
    }
    fn commands(&self) -> &'static [&'static str] {
        &["load"]
    }

    fn help_messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("[file_path]", "load project from given path.")]
    }
}

#[cfg(test)]

mod test_project {
    use super::*;
    use crate::{writer::*, CmdOps};
    use rair_io::*;
    use std::fs;
    #[test]
    fn test_project_help() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        let load = Load;
        let save = Save;
        load.help(&mut core);
        save.help(&mut core);
        assert_eq!(
            core.stdout.utf8_string().unwrap(),
            "Command: [load]\n\
             Usage:\n\
             load [file_path]\tload project from given path.\n\
             Command: [save]\n\
             Usage:\n\
             save [file_path]\tSave project into given path.\n"
        );
        assert_eq!(core.stderr.utf8_string().unwrap(), "");
    }
    #[test]
    fn test_project() {
        let mut core = Core::new_no_colors();
        core.stderr = Writer::new_buf();
        core.stdout = Writer::new_buf();
        let mut load = Load;
        let mut save = Save;
        core.io
            .open("malloc://0x500", IoMode::READ | IoMode::WRITE)
            .unwrap();
        core.io
            .open_at("malloc://0x1337", IoMode::READ | IoMode::WRITE, 0x31000)
            .unwrap();
        core.io.map(0x31000, 0xfff31000, 0x337).unwrap();
        save.run(&mut core, &["rair_project".to_owned()]);
        core.io.close_all();
        load.run(&mut core, &["rair_project".to_owned()]);
        core.run("files", &[]);
        core.run("maps", &[]);
        assert_eq!(
            core.stdout.utf8_string().unwrap(),
            "Handle\tStart address\tsize\t\tPermissions\tURI\n\
             0\t0x00000000\t0x00000500\tWRITE | READ\tmalloc://0x500\n\
             1\t0x00031000\t0x00001337\tWRITE | READ\tmalloc://0x1337\n\
             Virtual Address     Physical Address    Size\n\
             0xfff31000          0x31000             0x337\n"
        );
        assert_eq!(core.stderr.utf8_string().unwrap(), "");
        fs::remove_file("rair_project").unwrap();
    }
}
