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
use super::fields::RBDLFields;
use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, Struct};
use syn::{Ident, Result};

/// RBDL struct implementation
#[derive(Debug)]
pub struct RBDLStruct {
    pub attrs: Option<Attributes>,
    pub ident: Ident,
    pub colon_token: Colon,
    pub struct_token: Struct,
    pub fields: RBDLFields,
}

impl Parse for RBDLStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RBDLStruct {
            attrs: input.call(Attributes::parse_outer)?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            struct_token: input.parse()?,
            fields: input.parse()?,
        })
    }
}

#[cfg(test)]

mod enums_test {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_enums() {
        let en: RBDLStruct = parse_str(
            "\
        #[align=128]
        Data: struct {\
            A: char,\
            B: char\
        }",
        )
        .unwrap();
        assert_eq!(en.attrs.as_ref().unwrap().attrs.len(), 1);
    }
}
