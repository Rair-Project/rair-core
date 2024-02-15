/*
 * fields.rs: RBDL fields impelementation (used in structs and enums)
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

use super::attrs::*;
use super::types::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Colon, Comma};
use syn::{braced, Ident, Result};

/// Fields list for enums and structs `{d1: T1, ..}`
#[derive(Debug)]
pub struct RBDLFields {
    pub brace_token: Brace,
    pub named: Punctuated<RBDLField, Comma>,
}

impl Parse for RBDLFields {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let brace = braced!(content in input);
        Ok(RBDLFields {
            brace_token: brace,
            named: content.parse_terminated(RBDLField::parse)?,
        })
    }
}

#[derive(Debug)]
pub struct RBDLField {
    pub attrs: Option<Attributes>,
    pub ident: Ident,
    pub colon_token: Colon,
    pub ty: RBDLType,
}

impl Parse for RBDLField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RBDLField {
            attrs: input.call(Attributes::parse_outer)?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test_fields {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_fields() {
        let rbdl_fields = "\
        {\
            #[size=6, encoding=\"ascii\"]\
            x: oct,\
            #[count = x]\
            y: vec<u8>,\
            z: i64,
        }";
        let fields: RBDLFields = parse_str(rbdl_fields).unwrap();
        assert_eq!(fields.named.len(), 3);
        let mut iter = fields.named.iter();
        let field1 = iter.next().unwrap();
        assert_eq!(field1.ident, "x");
        assert_eq!(field1.ty.ident, "oct");
        assert!(field1.ty.arguments.is_none());
        assert!(field1.attrs.is_some());

        let field2 = iter.next().unwrap();
        assert_eq!(field2.ident, "y");
        assert_eq!(field2.ty.ident, "vec");
        assert!(field2.ty.arguments.is_some());
        let ty_args = field2.ty.arguments.as_ref().unwrap();
        assert_eq!(ty_args.args.len(), 1);
        assert_eq!(ty_args.args.first().unwrap().ident, "u8");
        assert!(field2.attrs.is_some());
        let field3 = iter.next().unwrap();
        assert_eq!(field3.ident, "z");
        assert_eq!(field3.ty.ident, "i64");
        assert!(field3.ty.arguments.is_none());
        assert!(field3.attrs.is_none());

        assert!(iter.next().is_none());
    }
}
