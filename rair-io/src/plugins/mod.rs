/*
 * plugins: List of built-in RIO plugins.
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
use crate::io::RIO;
pub mod base64;
pub mod defaultplugin;
pub mod dummy;
pub mod ihex;
pub mod malloc;
pub mod srec;
pub(crate) fn load_plugins(io: &mut RIO) {
    io.load_plugin(defaultplugin::plugin());
    io.load_plugin(ihex::plugin());
    io.load_plugin(malloc::plugin());
    io.load_plugin(base64::plugin());
    io.load_plugin(srec::plugin());
}
