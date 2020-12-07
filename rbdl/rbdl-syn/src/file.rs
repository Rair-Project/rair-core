/*
 * structs.rs: RBDL struct implementation
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
use super::attrs::Attributes;
use super::enums::RBDLEnum;
use super::structs::RBDLStruct;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Ident, Result, Token};

/// RBDL source file implementation
#[derive(Debug)]
pub struct RBDLFile {
    pub items: Vec<RBDLItem>,
}

impl Parse for RBDLFile {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RBDLFile {
            items: {
                let mut items = Vec::new();
                while !input.is_empty() {
                    items.push(input.parse()?);
                }
                items
            },
        })
    }
}

#[derive(Debug)]
pub enum RBDLItem {
    Struct(RBDLStruct),
    Enum(RBDLEnum),
}

impl Parse for RBDLItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let ahead = input.fork();
        ahead.call::<Option<Attributes>>(Attributes::parse_outer)?;
        ahead.parse::<Ident>()?;
        ahead.parse::<Colon>()?;
        let lookahead = ahead.lookahead1();
        if lookahead.peek(Token!(struct)) {
            Ok(RBDLItem::Struct(input.parse()?))
        } else if lookahead.peek(Token!(enum)) {
            Ok(RBDLItem::Enum(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]

mod enums_test {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_file() {
        let file: RBDLFile = parse_str(
            "\
        #[align=128]
        DataStruct: struct {\
            A: char,\
            B: char\
        }
        #[align=128]
        DataEnum: enum {\
            #[static='a']\
            A: char,\
            #[static='b']\
            B: char\
        }",
        )
        .unwrap();

        assert_eq!(file.items.len(), 2);
        let item1 = file.items.first().unwrap();
        let item2 = file.items.last().unwrap();
        if let RBDLItem::Enum(_) = item1 {
            panic!("Expected Struct found enum for item1");
        }
        if let RBDLItem::Struct(_) = item2 {
            panic!("Expected Enum found struct for item2");
        }
    }

    #[test]
    fn test_error() {
        let err: Result<RBDLFile> = parse_str(
            "\
        Data: Struct {
            x: i32\
        }\
        ",
        );
        assert!(err.is_err());
    }
}
