/*
 * types.rs: Comparable Type representation in for rbdl.
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

use rbdl_syn::RBDLType;
use syn::Ident;

// AST representation for RBDL Types.
#[derive(Debug, PartialEq)]
pub struct AstType {
    pub ty: Ident,
    pub args: Vec<AstType>,
}
impl From<RBDLType> for AstType {
    fn from(parse_tree: RBDLType) -> AstType {
        AstType {
            ty: parse_tree.ident,
            args: match parse_tree.arguments {
                None => Vec::new(),
                Some(types) => types.args.into_iter().map(|t| t.into()).collect(),
            },
        }
    }
}

#[cfg(test)]

mod test_types {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_type() {
        let ty: RBDLType = parse_str("bla").unwrap();
        let ast = AstType::from(ty);
        assert_eq!(ast.ty, "bla");
        assert_eq!(ast.args.len(), 0);
    }
    #[test]
    fn test_type_with_args() {
        let ty: RBDLType = parse_str("bla<abc, def<xyz>>").unwrap();
        let ast = AstType::from(ty);
        assert_eq!(ast.ty, "bla");
        assert_eq!(ast.args.len(), 2);
        assert_eq!(ast.args[0].ty, "abc");
        assert_eq!(ast.args[0].args.len(), 0);
        assert_eq!(ast.args[1].ty, "def");
        assert_eq!(ast.args[1].args.len(), 1);
        assert_eq!(ast.args[1].args[0].ty, "xyz");
        assert_eq!(ast.args[1].args[0].args.len(), 0);
    }
}
