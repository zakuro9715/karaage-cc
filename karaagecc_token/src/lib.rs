// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::iter::Peekable;

use karaage_utils::{derives::From, paste};
use karaagecc_source::Loc;

macro_rules! define_token_kind {
    (
        $(
            $( #[ $attr:meta ] )*
            $variant:ident $( ( $( $field:ty ),+ ) )? ,
        )+
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, From)]
        pub enum TokenKind {
            $(
                $( #[ $attr ] )*
                $variant $( ( $( $field ),+ ) )? ,
            )+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum TokenKindKey {
            $(
                $variant ,
            )+
        }

        impl TokenKind {
            pub fn key(&self) -> TokenKindKey {
                paste! {
                    match self {
                        $(
                            Self::$variant $( ( $( [<__ $field:lower>]  ),+ ) )?
                                => $crate::TokenKindKey::$variant,
                        )+
                    }
                }
            }
        }
    };
}

define_token_kind! {
    IntLiteral(i64),
    #[from(ignore)]
    Error(String),
}

impl TokenKind {
    pub fn into_token(self, loc: Loc) -> Token {
        Token { kind: self, loc }
    }
}

#[cfg(test)]
mod token_kind_tests {
    use crate::{TokenKind::*, *};
    #[test]
    fn test_into_token() {
        let kind = IntLiteral(42);
        let loc = Loc::head();
        let expected = Token {
            kind: kind.clone(),
            loc: loc.clone(),
        };
        assert_eq!(kind.into_token(loc), expected);
    }

    #[test]
    fn test_token_kind_from() {
        assert_eq!(TokenKind::from(42), IntLiteral(42));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self { kind, loc }
    }

    pub fn is(&self, kind: TokenKindKey) -> bool {
        self.kind.key() == kind
    }
}

#[macro_export]
macro_rules! token {
    ($v:expr, $loc:expr) => {
        TokenKind::from($v).into_token($loc)
    };
}

#[macro_export]
macro_rules! token_kind {
    ($name:ident) => {
        paste! { TokenKindKey::[< $name:camel>] }
    };
}

#[cfg(test)]
mod token_tests {
    use crate::*;
    use karaage_asserts::{assert_eq, *};
    fn_test_data_traits!(Token);

    #[test]
    fn test_new() {
        assert_eq!(
            Token::new(TokenKind::IntLiteral(42), Loc::head()),
            Token {
                kind: TokenKind::IntLiteral(42),
                loc: Loc::head(),
            }
        )
    }

    #[test]
    fn test_token_macro() {
        assert_eq!(
            token!(42, Loc::head()),
            Token::new(TokenKind::IntLiteral(42), Loc::head()),
        )
    }

    #[test]
    fn test_token_kind() {
        assert_eq!(token!(42, Loc::head()).kind.key(), token_kind!(int_literal));
        assert!(token!(42, Loc::head()).is(token_kind!(int_literal)));
    }
}

pub struct TokenReader<'a> {
    iter: Peekable<Box<dyn Iterator<Item = Token> + 'a>>,
}

impl<'a> Iterator for TokenReader<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

impl<'a> TokenReader<'a> {
    pub fn new(iter: impl Iterator<Item = Token> + 'a) -> Self {
        Self {
            iter: (Box::new(iter) as Box<dyn Iterator<Item = Token>>).peekable(),
        }
    }

    pub fn read(&mut self) -> Option<Token> {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.iter.peek()
    }
}

#[cfg(test)]
mod tokens_tests {
    use crate::*;
    use karaage_asserts::{assert_eq, *};

    #[test]
    fn test_new_and_iter() {
        let tokens = [
            token!(1, Loc::head()),
            token!(2, Loc::head()),
            token!(3, Loc::head()),
        ];
        assert_iter_eq!(TokenReader::new(tokens.clone().into_iter()), tokens);
    }
    #[test]
    fn test_peek() {
        let t1 = token!(1, Loc::head());
        let t2 = token!(2, Loc::head());
        let t3 = token!(3, Loc::head());
        let mut tokens = TokenReader::new([t1.clone(), t2.clone(), t3.clone()].into_iter());
        assert_eq!(tokens.peek(), Some(&t1));
        assert_eq!(tokens.next(), Some(t1));
        assert_eq!(tokens.peek(), Some(&t2));
        assert_eq!(tokens.next(), Some(t2));
        assert_eq!(tokens.peek(), Some(&t3));
        assert_eq!(tokens.next(), Some(t3));
        assert_eq!(tokens.peek(), None);
        assert_eq!(tokens.next(), None);
    }
}
