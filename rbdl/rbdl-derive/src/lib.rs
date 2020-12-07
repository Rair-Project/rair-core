/*
 * rbdl-derive: rair binary descriptor language derive macros.
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
extern crate proc_macro;
extern crate rbdl_ast;
extern crate rbdl_syn;
extern crate syn;
// extern crate quote;
use proc_macro::TokenStream;
use rbdl_ast::AstFile;
use rbdl_syn::*;
use std::convert::TryFrom;
use std::fs::File as FsFile;
use std::io::prelude::*;
use std::path::PathBuf;
use syn::parse_macro_input;
use syn::LitStr;

#[proc_macro]
pub fn rbdl_include(input: TokenStream) -> TokenStream {
    let definition_file = parse_macro_input!(input as LitStr);
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push(definition_file.value());
    let mut file = match FsFile::open(&path) {
        Ok(file) => file,
        Err(e) => panic!(format!("{}: {}", &definition_file.value(), e)),
    };
    let mut definitions = String::new();
    if let Err(e) = file.read_to_string(&mut definitions) {
        panic!(format!("{}: {}", &definition_file.value(), e));
    }
    let stream: proc_macro::TokenStream = definitions.parse().unwrap();
    return rbdl_inline(stream);
}
/*macro_rules! ast_input {
    ($tokenstream:ident as $ty:ty) => {
        match <$ty>::try_from($tokenstream) {
            Ok(data) => data,
            Err(err) => {
                return err.to_compile_error().into();
            }
        }
    };
    ($tokenstream:ident) => {
        parse_macro_input!($tokenstream as _)
    };
}*/
#[proc_macro]
pub fn rbdl_inline(input: TokenStream) -> TokenStream {
    let parse_tree = parse_macro_input!(input as RBDLFile);
    //let ast = ast_input!(parse_tree as AstFile);
    let ast = match AstFile::try_from(parse_tree) {
        Ok(ast) => ast,
        Err(e) => return e.to_compile_error().into(),
    };
    if let Some(e) = ast.check_fields_types() {
        return e.to_compile_error().into();
    }
    println!("No Error{:#?}", ast);
    TokenStream::new()
}
