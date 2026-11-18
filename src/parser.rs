use nom::{bytes::complete::{tag, take_while1}, character::complete::{alphanumeric1, space0, digit1}, sequence::{pair, tuple}, branch::alt};

use crate::syntax::{SyntaxNode, Span};
