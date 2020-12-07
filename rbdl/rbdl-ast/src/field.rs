/*
 * field.rs: AST Field implementation.
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

use super::{AstType, Table};
use rbdl_syn::{RBDLField, RBDLValue};
use std::collections::HashMap;
use std::convert::TryFrom;
use syn::{Error, Ident, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, Result};
#[derive(Debug)]
pub struct AstField {
    pub ident: Ident,
    pub attrs: HashMap<Ident, AstAttrValue>,
    pub ty: AstType,
}
impl TryFrom<RBDLField> for AstField {
    type Error = Error;

    fn try_from(parse_tree: RBDLField) -> Result<AstField> {
        Ok(AstField {
            ident: parse_tree.ident,
            ty: parse_tree.ty.into(),
            attrs: {
                if let Some(attrs) = parse_tree.attrs {
                    Table::try_from(attrs)?.unwrap()
                } else {
                    HashMap::new()
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum AstAttrValue {
    Str(LitStr),
    ByteStr(LitByteStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Ident(Ident),
    Vec(Vec<AstAttrValue>),
    None,
}

impl From<RBDLValue> for AstAttrValue {
    fn from(parse_tree: RBDLValue) -> AstAttrValue {
        match parse_tree {
            RBDLValue::Str(s) => AstAttrValue::Str(s),
            RBDLValue::ByteStr(s) => AstAttrValue::ByteStr(s),
            RBDLValue::Byte(b) => AstAttrValue::Byte(b),
            RBDLValue::Char(c) => AstAttrValue::Char(c),
            RBDLValue::Int(i) => AstAttrValue::Int(i),
            RBDLValue::Float(f) => AstAttrValue::Float(f),
            RBDLValue::Bool(b) => AstAttrValue::Bool(b),
            RBDLValue::Ident(i) => AstAttrValue::Ident(i),
            RBDLValue::Vec(v) => AstAttrValue::Vec(v.into()),
        }
    }
}

#[cfg(test)]
mod test_field {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_simple_field() {
        let parse_tree: RBDLField = parse_str("x: A").unwrap();
        let ast = AstField::try_from(parse_tree).unwrap();
        assert_eq!(ast.ident, "x");
        assert!(ast.attrs.is_empty());
        let ty = ast.ty;
        assert_eq!(ty.ty, "A");
        assert_eq!(ty.args.len(), 0);
    }
    #[test]
    fn test_meta_field() {
        let parse_tree: RBDLField = parse_str("x: Vec<u8>").unwrap();
        let ast = AstField::try_from(parse_tree).unwrap();
        assert_eq!(ast.ident, "x");
        assert!(ast.attrs.is_empty());
        let ty = ast.ty;
        assert_eq!(ty.ty, "Vec");
        assert_eq!(ty.args.len(), 1);
    }
    #[test]
    fn test_field_attributes() {
        let parse_tree: RBDLField = parse_str("#[a, b]x: u8").unwrap();
        let ast = AstField::try_from(parse_tree).unwrap();
        assert_eq!(ast.ident, "x");
        assert_eq!(ast.attrs.len(), 2);
        let ty = ast.ty;
        assert_eq!(ty.ty, "u8");
        assert_eq!(ty.args.len(), 0);
    }
    #[test]
    fn test_duplicate_attribute() {
        let parse_tree: RBDLField = parse_str("#[a, a=x, a = y]x: A").unwrap();
        let ast = AstField::try_from(parse_tree);
        assert!(ast.is_err());
    }
}
