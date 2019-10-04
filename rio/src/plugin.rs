/**
 * plugin.rs: RIO interface for implementing new plugin.
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
use utils::*;

#[derive(PartialEq)]
pub struct RIOPluginMetadata {
    pub name: &'static str,
    pub desc: &'static str,
    pub author: &'static str,
    pub license: &'static str,
    pub version: &'static str,
}
pub struct RIOPluginDesc {
    pub name: String,
    pub perm: IoMode,
    pub raddr: u64, //padd is simulated physical address
    pub size: u64,
    pub plugin_operations: Box<dyn RIOPluginOperations>,
}

pub trait RIOPlugin {
    fn get_metadata(&self) -> &'static RIOPluginMetadata;
    fn open(&mut self, uri: &str, flags: IoMode) -> Result<RIOPluginDesc, IoError>;
    fn accept_uri(&self, uri: &str) -> bool;
}

pub trait RIOPluginOperations {
    fn read(&mut self, raddr: usize, buffer: &mut [u8]) -> Result<(), IoError>;
    fn write(&mut self, raddr: usize, buffer: &[u8]) -> Result<(), IoError>;
}
impl PartialEq for dyn RIOPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.get_metadata() == other.get_metadata()
    }
}

impl PartialEq for dyn RIOPluginOperations {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}