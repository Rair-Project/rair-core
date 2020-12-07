/*
 * vec.rs: Parse tree representation for vectors present in RBDL Attribute value.
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
use syn::token::{Bracket, Comma};
use syn::{bracketed, Result};
/// Vector is defined as `[V1, V2, V3,..., Vn]` where `Vi` can be of type vector as well.

#[derive(Debug)]
pub struct RBDLVec<T> {
    pub bracket_token: Bracket,
    pub content: Punctuated<T, Comma>,
}

impl<T, U> Into<Vec<U>> for RBDLVec<T>
where
    U: From<T>,
{
    fn into(self) -> Vec<U> {
        self.content.into_iter().map(|t| t.into()).collect()
    }
}

impl<T> Parse for RBDLVec<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let bracket = bracketed!(content in input);
        Ok(RBDLVec {
            bracket_token: bracket,
            content: content.parse_terminated(T::parse)?,
        })
    }
}

#[cfg(test)]
mod test_rbdl_syn_vec {
    use super::*;
    use syn::{parse_str, LitInt};
    #[test]
    fn resolve_normal_vectors() {
        let parsed: RBDLVec<LitInt> = parse_str("[0, 1, 2,3, 4,5,  6]").unwrap();
        assert_eq!(parsed.content.len(), 7);
        for (i, int) in parsed.content.iter().enumerate() {
            assert_eq!(i, int.base10_parse().unwrap());
        }
    }
    #[test]
    fn resolve_nested_vectors() {
        let parsed: RBDLVec<RBDLVec<LitInt>> = parse_str("[[0, 1, 2], [3, 4, 5]]").unwrap();
        assert_eq!(parsed.content.len(), 2);
        for (i, v) in parsed.content.iter().enumerate() {
            assert_eq!(v.content.len(), 3);
            for (j, int) in v.content.iter().enumerate() {
                assert_eq!(i * 3 + j, int.base10_parse().unwrap());
            }
        }
    }
}
