#![allow(non_snake_case)]

use std::fmt;

pub struct Grammar(pub Vec<Production>);

pub struct Production {
    pub start_symbol: Symbol,
    pub rhs: Vec<Symbol>,
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ", self.start_symbol)?;
        for s in self.rhs.iter() {
            write!(f, "{} ", s)?;
        }
        Ok(())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Symbol {
    Nonterminal(String),
    Terminal(Terminal),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(name) => write!(f, "{}", name),
            Symbol::Nonterminal(nonterminal) => write!(f, "{}", nonterminal),
        }
    }
}

impl Symbol {
    pub fn new_other_terminal(name: impl ToString) -> Self {
        Symbol::Terminal(Terminal::Other(name.to_string()))
    }

    pub fn new_epsilon() -> Self {
        Symbol::Terminal(Terminal::Epsilon)
    }

    pub fn new_nonterminal(name: impl ToString) -> Self {
        Symbol::Nonterminal(name.to_string())
    }

    pub fn is_epsilon(&self) -> bool {
        if let Symbol::Terminal(Terminal::Epsilon) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Terminal {
    Other(String),
    Epsilon,
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::Other(name) => write!(f, "{}", name),
            Terminal::Epsilon => write!(f, "ε"),
        }
    }
}

/// 返回文法 3-6
/// Z -> d          Y -> ε      X -> Y
/// Z -> X Y Z      Y -> c       X -> a
pub fn grammar_3_6() -> Grammar {
    let (X, Y, Z) = (
        Symbol::new_nonterminal("X"),
        Symbol::new_nonterminal("Y"),
        Symbol::new_nonterminal("Z"),
    );
    let (a, c, d) = (
        Symbol::new_other_terminal("a"),
        Symbol::new_other_terminal("c"),
        Symbol::new_other_terminal("d"),
    );
    let epsilon = Symbol::new_epsilon();
    let mut grammar_inner = Vec::new();
    grammar_inner.push(Production {
        start_symbol: Z.clone(),
        rhs: vec![d.clone()],
    });
    grammar_inner.push(Production {
        start_symbol: Z.clone(),
        rhs: vec![X.clone(), Y.clone(), Z.clone()],
    });
    grammar_inner.push(Production {
        start_symbol: Y.clone(),
        rhs: vec![epsilon.clone()],
    });
    grammar_inner.push(Production {
        start_symbol: Y.clone(),
        rhs: vec![c.clone()],
    });
    grammar_inner.push(Production {
        start_symbol: X.clone(),
        rhs: vec![Y.clone()],
    });
    grammar_inner.push(Production {
        start_symbol: X.clone(),
        rhs: vec![a.clone()],
    });
    Grammar(grammar_inner)
}
