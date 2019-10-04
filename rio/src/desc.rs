/**
 * desc.rs: file descriptor data structure and needed tools to operate on single file.
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
 **/
use plugin::*;
use utils::*;

pub struct RIODesc {
    pub name: String,
    pub perm: IoMode,
    hndl: u64,
    pub paddr: u64, //padd is simulated physical address
    raddr: u64,     // raddr is the IO descriptor address, general rule of interaction paddr is high level lie, while raddr is the real thing.
    pub size: u64,
    plugin_operations: Box<dyn RIOPluginOperations>,
}
impl PartialEq for RIODesc {
    fn eq(&self, other: &RIODesc) -> bool {
        return self.hndl == other.hndl;
    }
}
impl RIODesc {
    pub fn open(plugin: &mut Box<dyn RIOPlugin>, uri: &str, flags: IoMode, hndl: u64) -> Result<RIODesc, IoError> {
        let plugin_desc = plugin.open(uri, flags)?;
        let desc = RIODesc {
            hndl: hndl,
            name: plugin_desc.name,
            perm: plugin_desc.perm,
            paddr: 0,
            size: plugin_desc.size,
            plugin_operations: plugin_desc.plugin_operations,
            raddr: plugin_desc.raddr,
        };
        return Ok(desc);
    }
    pub fn get_hndl(&self) -> u64 {
        return self.hndl;
    }
    pub fn has_paddr(&self, paddr: u64) -> bool {
        return paddr >= self.paddr && paddr < self.paddr + self.size as u64;
    }
    pub fn read(&mut self, paddr: usize, buffer: &mut [u8]) -> Result<(), IoError> {
        return self.plugin_operations.read(paddr - self.paddr as usize + self.raddr as usize as usize, buffer);
    }
    pub fn write(&mut self, paddr: usize, buffer: &[u8]) -> Result<(), IoError> {
        return self.plugin_operations.write(paddr - self.paddr as usize + self.raddr as usize, buffer);
    }
}

#[cfg(test)]
mod default_plugin_tests {
    use super::*;
    use defaultplugin;
    use std::io;
    use std::path::Path;
    use test_aids::*;
    fn test_desc_read_cb(path: &Path) {
        let mut plugin = defaultplugin::plugin();
        let mut desc = RIODesc::open(&mut plugin, &path.to_string_lossy(), IoMode::READ, 0).unwrap();
        desc.paddr = 0x40000;
        let mut buffer: &mut [u8] = &mut [0; 8];
        // read at the begining
        desc.read(desc.paddr as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0d]);
        // read at the middle
        desc.read((desc.paddr + 0x10) as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0xdb, 0x3d, 0x18, 0x55, 0x6d, 0xc2, 0x2f, 0xf1]);
        // read at the end
        desc.read((desc.paddr + 97) as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0x41, 0xc1, 0x02, 0xc3, 0xc5, 0x88, 0x4d, 0xd5]);
    }
    #[test]
    fn test_desc_read() {
        operate_on_file(&test_desc_read_cb, DATA)
    }
    fn test_desc_has_paddr_cb(path: &Path) {
        let mut plugin = defaultplugin::plugin();
        let mut desc = RIODesc::open(&mut plugin, &path.to_string_lossy(), IoMode::READ, 0).unwrap();
        desc.paddr = 0x40000;
        assert_eq!(desc.has_paddr(0x40000), true);
        assert_eq!(desc.has_paddr(0x5), false);
        assert_eq!(desc.has_paddr(0x40000 + DATA.len() as u64), false);
        assert_eq!(desc.has_paddr(0x40000 + DATA.len() as u64 - 1), true);
    }
    #[test]
    fn test_desc_has_paddr() {
        operate_on_file(&test_desc_has_paddr_cb, DATA);
    }
    fn test_desc_read_errors_cb(path: &Path) {
        let mut plugin = defaultplugin::plugin();
        let mut desc = RIODesc::open(&mut plugin, &path.to_string_lossy(), IoMode::READ, 0).unwrap();
        desc.paddr = 0x40000;
        let mut buffer: &mut [u8] = &mut [0; 8];
        // read past the end
        let mut e = desc.read((desc.paddr + desc.size) as usize, &mut buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };
        // read at the middle past the the end
        e = desc.read((desc.paddr + desc.size - 5) as usize, &mut buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };

        // read at the start past the end
        let mut v: Vec<u8> = vec![0; (desc.size + 8) as usize];
        buffer = &mut v;
        e = desc.read(desc.paddr as usize, &mut buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };
    }
    #[test]
    fn test_desc_read_errors() {
        operate_on_file(&test_desc_read_errors_cb, DATA);
    }

    fn test_desc_write_cb(path: &Path) {
        let mut plugin = defaultplugin::plugin();
        let mut desc = RIODesc::open(&mut plugin, &path.to_string_lossy(), IoMode::READ | IoMode::WRITE, 0).unwrap();
        let mut buffer: &mut [u8] = &mut [0; 8];
        desc.paddr = 0x40000;
        // write at the begining
        desc.write(desc.paddr as usize, &buffer).unwrap();
        desc.read(desc.paddr as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // write at the middle
        desc.write((desc.paddr + 0x10) as usize, &buffer).unwrap();
        desc.read((desc.paddr + 0x10) as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // write at the end
        desc.write((desc.paddr + 97) as usize, &buffer).unwrap();
        desc.read((desc.paddr + 97) as usize, &mut buffer).unwrap();
        assert_eq!(buffer, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_desc_write() {
        operate_on_file(&test_desc_write_cb, DATA);
    }

    fn test_write_errors_cb(path: &Path) {
        let mut plugin = defaultplugin::plugin();
        let mut desc = RIODesc::open(&mut plugin, &path.to_string_lossy(), IoMode::READ | IoMode::WRITE, 0).unwrap();
        let mut buffer: &[u8] = &[0; 8];
        desc.paddr = 0x40000;
        // write past the end
        let mut e = desc.write((desc.paddr + desc.size) as usize, &buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };
        // middle at the middle past the the end
        e = desc.write((desc.paddr + desc.size - 5) as usize, &buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };
        // read at the start past the end
        let v: Vec<u8> = vec![0; (desc.size + 8) as usize];
        buffer = &v;
        e = desc.write(desc.paddr as usize, &mut buffer);
        match e {
            Err(IoError::Parse(io_err)) => assert_eq!(io_err.kind(), io::ErrorKind::UnexpectedEof),
            _ => assert!(true, "UnexpectedEof Error should have been generated"),
        };
    }
    #[test]
    fn test_write_errors() {
        operate_on_file(&test_write_errors_cb, DATA);
    }
}