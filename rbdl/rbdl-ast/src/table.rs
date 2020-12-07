/*
 * table.rs: Thin wrappr around HashMap<Ident, AstAttrValue>.
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
use super::AstAttrValue;
use super::ErrorList;
use rbdl_syn::{Attribute, Attributes};
use std::collections::HashMap;
use std::convert::TryFrom;
use syn::{Error, Ident, Result};

pub struct Table(HashMap<Ident, AstAttrValue>);

impl Table {
    pub fn unwrap(self) -> HashMap<Ident, AstAttrValue> {
        self.0
    }
}
impl TryFrom<Attributes> for Table {
    type Error = Error;

    fn try_from(parse_tree: Attributes) -> Result<Table> {
        let mut tbl = HashMap::new();
        let mut errs = ErrorList::new();
        for attr in parse_tree.attrs {
            let (k, v): (Ident, AstAttrValue) = match attr {
                Attribute::Valued(v) => (v.ident, v.value.into()),
                Attribute::Unvalued(u) => (u.ident, AstAttrValue::None),
            };
            if tbl.contains_key(&k) {
                errs.push(k.span(), format!("Attribute `{}` has been already set before", k));
            } else {
                tbl.insert(k, v);
            }
        }
        match errs.collapse() {
            Some(e) => Err(e),
            None => Ok(Table(tbl)),
        }
    }
}

#[cfg(test)]
mod test_table {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_unvalued() {
        let parse_tree: Attributes = parse_str("#[a, b, c]").unwrap();
        let a: Ident = parse_str("a").unwrap();
        let b: Ident = parse_str("b").unwrap();
        let c: Ident = parse_str("c").unwrap();
        let d: Ident = parse_str("d").unwrap();
        let ast = Table::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ast.get(&a).unwrap(), &AstAttrValue::None);
        assert_eq!(ast.get(&b).unwrap(), &AstAttrValue::None);
        assert_eq!(ast.get(&c).unwrap(), &AstAttrValue::None);
        assert!(ast.get(&d).is_none());
    }
    #[test]
    fn test_valued() {
        let parse_tree: Attributes = parse_str("#[a=a, b=b, c=c]").unwrap();
        let a: Ident = parse_str("a").unwrap();
        let b: Ident = parse_str("b").unwrap();
        let c: Ident = parse_str("c").unwrap();
        let d: Ident = parse_str("d").unwrap();
        let ast = Table::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ast.get(&a).unwrap(), &AstAttrValue::Ident(a.clone()));
        assert_eq!(ast.get(&b).unwrap(), &AstAttrValue::Ident(b.clone()));
        assert_eq!(ast.get(&c).unwrap(), &AstAttrValue::Ident(c.clone()));
        assert!(ast.get(&d).is_none());
    }

    #[test]
    fn test_mixed() {
        let parse_tree: Attributes = parse_str("#[a, b=b, c]").unwrap();
        let a: Ident = parse_str("a").unwrap();
        let b: Ident = parse_str("b").unwrap();
        let c: Ident = parse_str("c").unwrap();
        let d: Ident = parse_str("d").unwrap();
        let ast = Table::try_from(parse_tree).unwrap().unwrap();
        assert_eq!(ast.get(&a).unwrap(), &AstAttrValue::None);
        assert_eq!(ast.get(&b).unwrap(), &AstAttrValue::Ident(b.clone()));
        assert_eq!(ast.get(&c).unwrap(), &AstAttrValue::None);
        assert!(ast.get(&d).is_none());
    }

    #[test]
    fn test_duplicate() {
        let parse_tree: Attributes = parse_str("#[a, a=x, a = y]").unwrap();
        let ast = Table::try_from(parse_tree);
        assert!(ast.is_err());
    }
}
