//! Calc - An easy-to-use stateful scientific calculator focusing on intuitive user experience
//! Copyright (C) 2027  Andras Vinter
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.

//! AST produced by the parser and consumed by the evaluator.
//! Percent is represented as a tagged node so the evaluator can
//! apply its context-dependent semantics on the right-hand side of
//! a binary operator.

use crate::engine::decimal::Decimal;
use crate::engine::item::{BinOp, BinaryFunc, ConstKind, UnaryFunc};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Numeric literal, exact as the user wrote it.
    Num(Decimal),
    /// π or 𝑒.
    Const(ConstKind),
    /// Unary negation.
    Neg(Box<Node>),
    /// Binary arithmetic operator ('+', '-', '*', '/', '^').
    Bin(BinOp, Box<Node>, Box<Node>),
    /// Modulo (the `%` character when it acts as a binary operator).
    Mod(Box<Node>, Box<Node>),
    /// Postfix factorial `x!`.
    Factorial(Box<Node>),
    /// Postfix percent `x%`. Final semantics depend on context —
    /// see eval.rs for details.
    Percent(Box<Node>),
    /// Single-argument function application (sin, cos, sqrt, log, …).
    UnaryFn(UnaryFunc, Box<Node>),
    /// Two-argument function application (log(base,x), root(x,n)).
    BinaryFn(BinaryFunc, Box<Node>, Box<Node>),
    /// log with an integer base baked into the function name
    /// (log2, log6, log10, …).
    LogN(Decimal, Box<Node>),
}
