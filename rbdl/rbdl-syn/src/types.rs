/*
 * types.rs: RBDL types parser impelementation.
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

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{Ident, Result, Token};

/// Represents both normal types as `u8` or types with templates as `Vec<u8>`
#[derive(Debug)]
pub struct RBDLType {
    pub ident: Ident,
    pub arguments: Option<RBDLTypeList>,
}
impl Parse for RBDLType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RBDLType {
            ident: input.parse()?,
            arguments: input.call(RBDLTypeList::parse_outer)?,
        })
    }
}

#[derive(Debug)]
pub struct RBDLTypeList {
    pub lt_token: Lt,
    pub args: Punctuated<RBDLType, Comma>,
    pub gt_token: Gt,
}
impl RBDLTypeList {
    fn parse_outer(input: ParseStream) -> Result<Option<Self>> {
        if input.peek(Token!(<)) {
            RBDLTypeList::parse(input).map(Some)
        } else {
            Ok(None)
        }
    }
}
impl Parse for RBDLTypeList {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RBDLTypeList {
            lt_token: input.parse()?,
            args: {
                let mut args = Punctuated::new();
                loop {
                    if input.peek(Token![>]) {
                        break;
                    }
                    let value = input.parse()?;
                    args.push_value(value);
                    if input.peek(Token![>]) {
                        break;
                    }
                    let punct = input.parse()?;
                    args.push_punct(punct);
                }
                args
            },
            gt_token: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test_type {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_single_type() {
        let ty: RBDLType = parse_str("my_type").unwrap();
        assert_eq!(ty.ident, "my_type");
        assert!(ty.arguments.is_none());
    }
    #[test]
    fn test_template_type() {
        let ty: RBDLType = parse_str("my_type<abc, xyz>").unwrap();
        assert_eq!(ty.ident, "my_type");
        assert!(ty.arguments.is_some());
        let args = ty.arguments.as_ref().unwrap();
        assert_eq!(args.args.len(), 2);
        assert_eq!(args.args.first().unwrap().ident, "abc");
        assert_eq!(args.args.last().unwrap().ident, "xyz");
    }
}
