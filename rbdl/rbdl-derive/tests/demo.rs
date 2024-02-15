/*
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

use rbdl::*;
/*
rbdl_inline!(
    FileType: enum {
        #[static='0']
        Regular: char,
        #[static='1']
        HardLink: char,
        #[static='2']
        SymbolicLink: char,
        #[static='3']
        CharacterDevice: char,
        #[static='4']
        BlockDevice: char,
        #[static='5']
        Directory: char,
        #[static='6']
        FIFONode: char,
    }

    #[padding=512]
    TarHeader: struct {
        #[size=100, encoding="ascii"]
        file_name: String,
        #[size=8, encoding="ascii"]
        mode: oct,
        #[size=8, encoding="ascii"]
        uid: oct,
        #[size=8, encoding="ascii"]
        gid: oct,
        #[size=12, encoding="ascii"]
        size: oct,
        #[size=12, encoding="ascii"]
        mtime: oct,
        #[size=6, encoding="ascii"]
        checksum: oct,
        #[static=[0, 32], hidden]
        checksum_deimiter: Vec<u8>,
        typeflag: FileType,
        #[size=100, encoding="ascii", delimiter=0]
        linkname: String,
        #[static="ustar", delimiter=0, encoding="ascii", hidden]
        magic: String,
        #[size=2]
        version: String,
        #[size=32, encoding="ascii", delimiter=0]
        user: String,
        #[size=32, encoding="ascii", delimiter=0]
        group: String,
        #[size=8, encoding="ascii"]
        devmajor: oct,
        #[size=8, encoding="ascii"]
        devminor: oct,
        #[size=155, encoding="ascii", delimiter=0]
        prefix: String,
    }
);

*/
rbdl_inline!(
    /*X: struct {
        #[a=x]
        a: i8,
        b: i8,
        c: Vec<Y>,
    }*/
    Y: struct {
        a: i16,
        b: Vec<i8>,
    }
    A: struct{
        x: i16,
        y: Y<i8>
    }
);

fn main() {

}
