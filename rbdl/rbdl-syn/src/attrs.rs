/*
 * attrs.rs: RBDL Attributes implementation
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
use super::value::*;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma, Eq, Pound};
use syn::{bracketed, Ident, Result, Token};

/// Attributes are defined as `#[key1, key2=value2, ..]`
#[derive(Debug)]
pub struct Attributes {
    pub pound_token: Pound,
    pub bracket_token: Bracket,
    pub attrs: Punctuated<Attribute, Comma>,
}

impl Attributes {
    pub fn parse_outer(input: ParseStream) -> Result<Option<Self>> {
        if input.peek(Token!(#)) {
            Attributes::parse(input).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let pound = input.parse()?;
        let bracket = bracketed!(content in input);
        Ok(Attributes {
            pound_token: pound,
            bracket_token: bracket,
            attrs: content.parse_terminated(Attribute::parse)?,
        })
    }
}

#[derive(Debug)]
pub enum Attribute {
    Valued(ValuedAttribute),
    Unvalued(UnvaluedAttribute),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Token!(=)) {
            input.parse().map(Attribute::Valued)
        } else {
            input.parse().map(Attribute::Unvalued)
        }
    }
}

#[derive(Debug)]
pub struct UnvaluedAttribute {
    pub ident: Ident,
}

impl Parse for UnvaluedAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(UnvaluedAttribute { ident: input.parse()? })
    }
}

#[derive(Debug)]
pub struct ValuedAttribute {
    pub ident: Ident,
    pub equal_token: Eq,
    pub value: RBDLValue,
}

impl Parse for ValuedAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ValuedAttribute {
            ident: input.call(Ident::parse_any)?,
            equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test_rbdl_syn_attrs {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_attribute_unvalued() {
        let attrs: Attributes = parse_str("#[key]").unwrap();
        assert_eq!(attrs.attrs.len(), 1);
        let attr = attrs.attrs.first().unwrap();
        if let Attribute::Unvalued(uv) = attr {
            assert_eq!(uv.ident, "key");
        } else {
            panic!("Expected unvalued Attribute");
        }
    }

    #[test]
    fn test_attribute_valued() {
        let attrs: Attributes = parse_str("#[key=value]").unwrap();
        assert_eq!(attrs.attrs.len(), 1);
        let attr = attrs.attrs.first().unwrap();
        if let Attribute::Valued(v) = attr {
            assert_eq!(v.ident, "key");
            match &v.value {
                RBDLValue::Ident(i) => assert_eq!(i, "value"),
                _ => panic!("Expected Ident value"),
            }
        } else {
            panic!("Expected valued Attribute");
        }
    }

    #[test]
    fn test_attributes_multi() {
        let attrs: Attributes = parse_str("#[key=value, key2, key3=[\"a\", \"b\", \"c\"]]").unwrap();
        assert_eq!(attrs.attrs.len(), 3);
        let mut iter = attrs.attrs.iter();
        let attr = iter.next().unwrap();
        if let Attribute::Valued(v) = attr {
            assert_eq!(v.ident, "key");
            match &v.value {
                RBDLValue::Ident(i) => assert_eq!(i, "value"),
                _ => panic!("Expected Ident value"),
            }
        } else {
            panic!("Expected valued Attribute");
        }
        let attr = iter.next().unwrap();
        if let Attribute::Unvalued(uv) = attr {
            assert_eq!(uv.ident, "key2");
        } else {
            panic!("Expected unvalued Attribute");
        }
        let attr = iter.next().unwrap();
        if let Attribute::Valued(v) = attr {
            assert_eq!(v.ident, "key3");
            match &v.value {
                RBDLValue::Vec(_) => (),
                _ => panic!("Expected Vec value"),
            }
        }
        assert!(iter.next().is_none());
    }
}
