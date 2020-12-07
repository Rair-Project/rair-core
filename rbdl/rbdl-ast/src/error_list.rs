/*
 * error_list.rs: API for reporting more than one error in rustc.
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
use proc_macro2::Span;
use std::fmt::Display;
use syn::Error;

pub struct ErrorList {
    list: Vec<Error>,
}
impl ErrorList {
    pub fn new() -> ErrorList {
        ErrorList { list: Vec::new() }
    }
    pub fn push<T>(&mut self, span: Span, message: T)
    where
        T: Display,
    {
        let err = Error::new(span, message);
        self.list.push(err);
    }
    pub fn push_err(&mut self, e: Error) {
        self.list.push(e);
    }
    pub fn append(&mut self, mut list: ErrorList) {
        self.list.append(&mut list.list);
    }
    pub fn collapse(mut self) -> Option<Error> {
        match self.list.len() {
            0 => None,
            1 => Some(self.list.pop().unwrap()),
            _ => {
                let mut err = self.list.pop().unwrap();
                for mut e in self.list.into_iter().rev() {
                    e.combine(err);
                    err = e;
                }
                Some(err)
            }
        }
    }
}
impl From<Error> for ErrorList {
    fn from(err: Error) -> ErrorList {
        ErrorList { list: vec![err] }
    }
}
