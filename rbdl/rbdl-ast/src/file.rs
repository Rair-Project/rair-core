use super::{AstAttrValue, AstField, AstType, ErrorList, FullItem};
use rbdl_syn::RBDLFile;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use syn::{Error, Ident, Result};

lazy_static! {
    // The usize represents the number of type arguments should they ever exist.
    static ref DEFAULTTYPES : HashMap<String, usize> = {
        let mut h = HashMap::with_capacity(18);
        h.insert(String::from("char"), 0);
        h.insert(String::from("u8"), 0);
        h.insert(String::from("u16"), 0);
        h.insert(String::from("u32"), 0);
        h.insert(String::from("u64"), 0);
        h.insert(String::from("u128"), 0);
        h.insert(String::from("i8"), 0);
        h.insert(String::from("i16"), 0);
        h.insert(String::from("i32"), 0);
        h.insert(String::from("i64"), 0);
        h.insert(String::from("i128"), 0);
        h.insert(String::from("f32"), 0);
        h.insert(String::from("f64"), 0);
        h.insert(String::from("oct"), 0);
        h.insert(String::from("hex"), 0);
        h.insert(String::from("dec"), 0);
        h.insert(String::from("bin"), 0);
        h.insert(String::from("String"), 0);
        h.insert(String::from("Vec"), 1);
        h
    };
}
#[derive(Debug)]
pub struct AstFile {
    pub items: HashMap<Ident, AstItem>,
}

impl AstFile {
    pub fn check_fields_types(&self) -> Option<Error> {
        let mut err = ErrorList::new();
        for (ident, item) in &self.items {
            for field in &item.unwrap_ref().fields {
                let ty = &field.ty;
                err.append(self.contain_type(ty));
            }
            err.append(self.non_recursive(ident));
        }
        err.collapse()
    }
    /// Validate that we don't have any kind of recursively defined types
    /// We don't have any kinds of barriers like box or RC or Option
    fn non_recursive(&self, base_type: &Ident) -> ErrorList {
        let ty = AstType {
            ty: base_type.clone(),
            args: Vec::new(),
        };
        let mut visited = HashMap::new();
        self.non_recursive_internal(&ty, &mut visited)
    }
    /// What this function does is DFS traversal keeping track visited node as well as
    /// which of those visited nodes are in the current recursion stack.
    fn non_recursive_internal<'a, 'b: 'a>(&'b self, ty: &'a AstType, visited: &mut HashMap<&'a Ident, bool>) -> ErrorList {
        let mut err = ErrorList::new();
        // if type ident is visited return.
        // add type ident to visited.
        if let Some(recursive) = visited.get(&ty.ty) {
            if *recursive {
                err.push(ty.ty.span(), "Recursively defined type.");
            }
            return err;
        }
        visited.insert(&ty.ty, true);
        // for type in current type's fields:
        //      recurse over the type
        if let Some(item) = self.items.get(&ty.ty) {
            for field in &item.unwrap_ref().fields {
                err.append(self.non_recursive_internal(&field.ty, visited));
            }
        }
        // for type in current type's args:: &mut HashMap<&'a Ident, bool>
        //      recurse over the type
        for arg_type in &ty.args {
            err.append(self.non_recursive_internal(arg_type, visited));
        }
        visited.insert(&ty.ty, false);
        err
    }
    // Validate that all types used are valid types and with valid arguments if required.
    fn contain_type(&self, ty: &AstType) -> ErrorList {
        let mut err = ErrorList::new();
        if self.items.get(&ty.ty).is_some() {
            if !ty.args.is_empty() {
                err.push(ty.args[0].ty.span(), format!("type arguments are not allowed for this `{}`.", ty.ty));
            }
        } else if let Some(correct_count) = DEFAULTTYPES.get(&ty.ty.to_string()) {
            let len = ty.args.len();
            if len == *correct_count {
                for sub_type in &ty.args {
                    err.append(self.contain_type(sub_type))
                }
            } else {
                err.push(ty.ty.span(), format!("wrong number of type arguments: expected {}, found {}.", correct_count, len));
            }
        } else {
            err.push(ty.ty.span(), format!("cannot find type `{}`.", ty.ty));
        }
        err
    }
}
impl TryFrom<RBDLFile> for AstFile {
    type Error = Error;
    fn try_from(parse_tree: RBDLFile) -> Result<AstFile> {
        let mut set = HashSet::new();
        let mut err = ErrorList::new();
        let mut items = HashMap::with_capacity(parse_tree.items.len());
        for item in parse_tree.items {
            match FullItem::try_from(item) {
                Err(e) => err.push_err(e),
                Ok(full_item) => {
                    let (ident, ast_item) = full_item.unwrap();
                    if set.contains(&ident) {
                        err.push(ident.span(), format!("`{}` is already defined before.", &ident));
                    } else {
                        set.insert(ident.clone());
                        items.insert(ident, ast_item);
                    }
                }
            }
        }
        match err.collapse() {
            Some(e) => Err(e),
            None => Ok(AstFile { items }),
        }
    }
}

#[derive(Debug)]
pub enum AstItem {
    Struct(AstItemContent),
    Enum(AstItemContent),
}

impl AstItem {
    pub fn unwrap_ref(&self) -> &AstItemContent {
        match self {
            AstItem::Struct(s) => s,
            AstItem::Enum(e) => e,
        }
    }
}

#[derive(Debug)]
pub struct AstItemContent {
    pub attrs: HashMap<Ident, AstAttrValue>,
    pub fields: Vec<AstField>,
}

#[cfg(test)]
mod test_item {
    use super::*;
    use syn::parse_str;
    #[test]
    fn test_duplicate_attrs() {
        let parse_tree: RBDLFile = parse_str(
            "
            x: struct{
                #[a, a=x, a = y]
                x: A
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree);
        assert!(ast.is_err());
    }

    #[test]
    fn test_duplicate_item() {
        let parse_tree: RBDLFile = parse_str(
            "
            x: struct{
                x: A
            }
            x: struct {
                x: B
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree);
        assert!(ast.is_err());
    }
    #[test]
    fn test_no_duplicate() {
        let parse_tree: RBDLFile = parse_str(
            "
            x: struct{
                x: A
            }
            x: struct {
                x: B
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree);
        assert!(ast.is_err());
    }

    #[test]
    fn test_types() {
        let parse_tree: RBDLFile = parse_str(
            "
            x: struct{
                x: i8,
                y: i32
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_none());
    }

    #[test]
    fn test_self_recursion() {
        let parse_tree: RBDLFile = parse_str(
            "
            x: struct{
                x: i8,
                y: x
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_mutual_recursion() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: B
            }
            B: struct{
                x: i8,
                y: C
            }
            C: struct{
                x: i8,
                y: A
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_arg_recursion() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: Vec<B>
            }
            B: struct{
                x: i8,
                y: C
            }
            C: struct{
                x: i8,
                y: A
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_struct_field() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: B
            }
            B: struct{
                x: i8,
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_none());
    }

    #[test]
    fn test_invalid_type_arg() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: i16<i8>
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_invalid_type_arg2() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: vec
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_invalid_type_arg3() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: vec<i8, i16>
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_invalid_type_arg4() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: vec<i8>
            }
            B: struct {
                x: i8,
                y: A<i8>
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }

    #[test]
    fn test_invalid_type() {
        let parse_tree: RBDLFile = parse_str(
            "
            A: struct{
                x: i8,
                y: B
            }
        ",
        )
        .unwrap();
        let ast = AstFile::try_from(parse_tree).unwrap();
        assert!(ast.check_fields_types().is_some());
    }



}
