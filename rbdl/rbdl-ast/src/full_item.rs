/*
 * full_item.rs: Thin wrappr around AstItem.
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
use super::ErrorList;
use super::{AstField, AstItem, AstItemContent, Table};
use rbdl_syn::{RBDLEnum, RBDLFields, RBDLItem, RBDLStruct};
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use syn::{Error, Ident, Result};

pub struct FullItem(Ident, AstItem);

impl FullItem {
    pub fn unwrap(self) -> (Ident, AstItem) {
        (self.0, self.1)
    }
}
fn prep_fields(rbdl_fields: RBDLFields) -> (Vec<AstField>, ErrorList) {
    let mut set = HashSet::new();
    let mut fields = Vec::new();
    let mut err = ErrorList::new();
    for field in rbdl_fields.named {
        if set.contains(&field.ident) {
            err.push(field.ident.span(), format!("Field `{}` is already declared before", &field.ident));
        } else {
            set.insert(field.ident.clone());
        }
        match AstField::try_from(field) {
            Ok(f) => fields.push(f),
            Err(e) => err.push_err(e),
        }
    }
    (fields, err)
}

impl TryFrom<RBDLStruct> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLStruct) -> Result<FullItem> {
        let ident = parse_tree.ident;
        let (fields, mut err) = prep_fields(parse_tree.fields);
        let mut attrs = HashMap::new();
        if let Some(rbdl_attrs) = parse_tree.attrs {
            match Table::try_from(rbdl_attrs) {
                Ok(table) => attrs = table.unwrap(),
                Err(e) => err.push_err(e),
            }
        }
        match err.collapse() {
            Some(e) => Err(e),
            None => Ok(FullItem(ident, AstItem::Struct(AstItemContent { fields, attrs }))),
        }
    }
}
impl TryFrom<RBDLEnum> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLEnum) -> Result<FullItem> {
        let ident = parse_tree.ident;
        let (fields, mut err) = prep_fields(parse_tree.fields);
        let mut attrs = HashMap::new();
        if let Some(rbdl_attrs) = parse_tree.attrs {
            match Table::try_from(rbdl_attrs) {
                Ok(table) => attrs = table.unwrap(),
                Err(e) => err.push_err(e),
            }
        }
        match err.collapse() {
            Some(e) => Err(e),
            None => Ok(FullItem(ident, AstItem::Enum(AstItemContent { fields, attrs }))),
        }
    }
}

impl TryFrom<RBDLItem> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLItem) -> Result<FullItem> {
        match parse_tree {
            RBDLItem::Struct(s) => s.try_into(),
            RBDLItem::Enum(e) => e.try_into(),
        }
    }
}

#[cfg(test)]
mod test_item {
    use super::*;
    use crate::AstAttrValue;
    use syn::parse_str;
    #[test]
    fn test_unwrap() {
        let parse_tree: RBDLItem = parse_str("x: struct {a: T}").unwrap();
        let (ident, _) = FullItem::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ident, "x");
    }
    #[test]
    fn test_struct_duplicate_field_attributes() {
        let parse_tree: RBDLItem = parse_str(
            "
        x: struct{
            #[a, a=x, a = y]
            x: A
        }
        ",
        )
        .unwrap();
        let ast = FullItem::try_from(parse_tree);
        assert!(ast.is_err());
    }
    #[test]
    fn test_struct_duplicate_fields() {
        let parse_tree: RBDLItem = parse_str(
            "
        x: struct{
            #[a]
            x: A,
            x: B
        }
        ",
        )
        .unwrap();
        let ast = FullItem::try_from(parse_tree);
        assert!(ast.is_err());
    }
    #[test]
    fn test_struct_passing_attribute() {
        let parse_tree: RBDLItem = parse_str(
            "
        #[a]
        x: struct{
            x: B
        }
        ",
        )
        .unwrap();
        let a: Ident = parse_str("a").unwrap();
        let (ident, item) = FullItem::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ident, "x");
        if let AstItem::Struct(s) = item {
            assert_eq!(*s.attrs.get(&a).unwrap(), AstAttrValue::None);
        } else {
            panic!("Expected Struct!");
        }
    }
    #[test]
    fn test_struct_failing_attribute() {
        let parse_tree: RBDLItem = parse_str(
            "
        #[a, a]
        x: struct{
            x: B
        }
        ",
        )
        .unwrap();
        assert!(FullItem::try_from(parse_tree).is_err());
    }

    #[test]
    fn test_enum_duplicate_field_attributes() {
        let parse_tree: RBDLItem = parse_str(
            "\
        x: enum{ \
            #[a, a=x, a = y]
            x: A\
        }\
        ",
        )
        .unwrap();
        let ast = FullItem::try_from(parse_tree);
        assert!(ast.is_err());
    }
    #[test]
    fn test_enum_duplicate_fields() {
        let parse_tree: RBDLItem = parse_str(
            "
        x: enum{
            #[a]
            x: A,
            x: B
        }
        ",
        )
        .unwrap();
        let ast = FullItem::try_from(parse_tree);
        assert!(ast.is_err());
    }
    #[test]
    fn test_enum_passing_attribute() {
        let parse_tree: RBDLItem = parse_str(
            "
        #[a]
        x: enum{
            x: B
        }
        ",
        )
        .unwrap();
        let a: Ident = parse_str("a").unwrap();
        let (ident, item) = FullItem::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ident, "x");
        if let AstItem::Enum(e) = item {
            assert_eq!(*e.attrs.get(&a).unwrap(), AstAttrValue::None);
        } else {
            panic!("Expected Enum!");
        }
    }
    #[test]
    fn test_enum_failing_attribute() {
        let parse_tree: RBDLItem = parse_str(
            "
        #[a, a]
        x: enum{
            x: B
        }
        ",
        )
        .unwrap();
        assert!(FullItem::try_from(parse_tree).is_err());
    }
}
