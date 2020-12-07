/*
 * value.rs: RBDL Attributes value implementation
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

use super::vec::*;
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{Ident, Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, Result};

// Possible Values to be used inside an attribute
#[derive(Debug)]
pub enum RBDLValue {
    Str(LitStr),
    ByteStr(LitByteStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Ident(Ident),
    Vec(RBDLVec<RBDLValue>),
}

impl RBDLValue {
    fn parse_vec(input: ParseStream) -> Result<Self> {
        let vec = input.parse::<RBDLVec<RBDLValue>>()?;
        Ok(RBDLValue::Vec(vec))
    }

    fn parse_lit(input: ParseStream) -> Result<Self> {
        let head = input.fork();
        if input.peek(Lit) {
            match input.parse::<Lit>()? {
                Lit::Str(s) => Ok(RBDLValue::Str(s)),
                Lit::ByteStr(bs) => Ok(RBDLValue::ByteStr(bs)),
                Lit::Byte(b) => Ok(RBDLValue::Byte(b)),
                Lit::Char(c) => Ok(RBDLValue::Char(c)),
                Lit::Int(i) => Ok(RBDLValue::Int(i)),
                Lit::Float(f) => Ok(RBDLValue::Float(f)),
                Lit::Bool(b) => Ok(RBDLValue::Bool(b)),
                _ => Err(head.error("Unknown literal type")),
            }
        } else if input.peek(Ident) {
            Ok(RBDLValue::Ident(input.parse::<Ident>()?))
        } else {
            Err(head.error("Expected Idenitifier or Literal."))
        }
    }
}
impl Parse for RBDLValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Bracket) {
            RBDLValue::parse_vec(input)
        } else {
            RBDLValue::parse_lit(input)
        }
    }
}

#[cfg(test)]
mod test_rbdl_syn_value {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_value_str() {
        let val: RBDLValue = parse_str("\"Hello world\"").unwrap();
        if let RBDLValue::Str(s) = val {
            assert_eq!(s.value(), "Hello world");
        } else {
            panic!("Failed to parse string literal");
        }
    }
    #[test]
    fn test_value_byte_str() {
        let val: RBDLValue = parse_str("b\"Hello world\"").unwrap();
        if let RBDLValue::ByteStr(s) = val {
            assert_eq!(s.value(), b"Hello world");
        } else {
            panic!("Failed to parse byte string literal");
        }
    }

    #[test]
    fn test_value_byte() {
        let val: RBDLValue = parse_str("b'H'").unwrap();
        if let RBDLValue::Byte(s) = val {
            assert_eq!(s.value(), b'H');
        } else {
            panic!("Failed to parse byte");
        }
    }
    #[test]
    fn test_value_char() {
        let val: RBDLValue = parse_str("'H'").unwrap();
        if let RBDLValue::Char(s) = val {
            assert_eq!(s.value(), 'H');
        } else {
            panic!("Failed to parse char");
        }
    }

    #[test]
    fn test_value_int() {
        let val: RBDLValue = parse_str("1234").unwrap();
        if let RBDLValue::Int(s) = val {
            assert_eq!(1234, s.base10_parse().unwrap());
        } else {
            panic!("Failed to parse Int type");
        }
    }
    #[test]
    fn test_value_float() {
        let val: RBDLValue = parse_str("1234.4321").unwrap();
        if let RBDLValue::Float(s) = val {
            assert_eq!(1234.4321, s.base10_parse().unwrap());
        } else {
            panic!("Failed to parse Float type");
        }
    }
    #[test]
    fn test_value_bool() {
        let val: RBDLValue = parse_str("false").unwrap();
        if let RBDLValue::Bool(s) = val {
            assert_eq!(false, s.value);
        } else {
            panic!("Failed to parse Bool type");
        }
    }
    #[test]
    fn test_value_ident() {
        let val: RBDLValue = parse_str("bla").unwrap();
        if let RBDLValue::Ident(s) = val {
            assert_eq!(s, "bla");
        } else {
            panic!("Failed to parse Bool type");
        }
    }
    #[test]
    fn test_value_vec() {
        let val: RBDLValue = parse_str("[0, 1, 2 , 3, 4]").unwrap();
        if let RBDLValue::Vec(vec) = val {
            assert_eq!(vec.content.len(), 5);
            for (i, int_val) in vec.content.iter().enumerate() {
                if let RBDLValue::Int(int) = int_val {
                    assert_eq!(i, int.base10_parse().unwrap());
                } else {
                    panic!("Failed to parse Vec type");
                }
            }
        } else {
            panic!("Failed to parse Vec type");
        }
    }
    #[test]
    fn test_error() {
        assert!(parse_str::<RBDLValue>("+").is_err());
    }
}
