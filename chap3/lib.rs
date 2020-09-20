#![allow(non_snake_case)]

use crate::cfg::{Grammar, Production, Symbol};

pub mod cfg;
pub mod lalr_1;

#[macro_export]
macro_rules! extend {
    ($collection: ident, $iter_expr: expr, $is_changed_var: ident) => {
        if $is_changed_var {
            $collection.extend($iter_expr);
        } else {
            let old_len = $collection.len();
            $collection.extend($iter_expr);
            $is_changed_var = old_len != $collection.len() || $is_changed_var;
        }
    };
}

/// grammar_3_6() returns grammar 3-6 (p33)
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

    // Z -> d
    grammar_inner.push(Production {
        start_symbol: Z.clone(),
        rhs: vec![d.clone()],
    });

    // Z -> X Y Z
    grammar_inner.push(Production {
        start_symbol: Z.clone(),
        rhs: vec![X.clone(), Y.clone(), Z.clone()],
    });

    // Y -> ε
    grammar_inner.push(Production {
        start_symbol: Y.clone(),
        rhs: vec![epsilon.clone()],
    });

    // Y -> c
    grammar_inner.push(Production {
        start_symbol: Y.clone(),
        rhs: vec![c.clone()],
    });

    // X -> Y
    grammar_inner.push(Production {
        start_symbol: X.clone(),
        rhs: vec![Y.clone()],
    });

    // X -> a
    grammar_inner.push(Production {
        start_symbol: X.clone(),
        rhs: vec![a.clone()],
    });
    Grammar::new(grammar_inner)
}

/// grammar_3_10() returns grammar 3-10 (p64)
/// 0  S' -> S          3  E -> V
/// 1  S -> V = E       4  V -> x
/// 2  S -> E           5  V -> * E
pub fn grammar_3_10() -> Grammar {
    let (S_, S, E, V) = (
        Symbol::new_nonterminal("S'"),
        Symbol::new_nonterminal("S"),
        Symbol::new_nonterminal("E"),
        Symbol::new_nonterminal("V"),
    );
    let (eq, deref, x) = (
        Symbol::new_other_terminal("="),
        Symbol::new_other_terminal("*"),
        Symbol::new_other_terminal("x"),
    );
    let mut grammar_inner = Vec::new();
    // 0  S' -> S
    grammar_inner.push(Production {
        start_symbol: S_.clone(),
        rhs: vec![S.clone()],
    });

    // 1  S -> V = E
    grammar_inner.push(Production {
        start_symbol: S.clone(),
        rhs: vec![V.clone(), eq.clone(), E.clone()],
    });

    // 2  S -> E
    grammar_inner.push(Production {
        start_symbol: S,
        rhs: vec![E.clone()],
    });

    // 3  E -> V
    grammar_inner.push(Production {
        start_symbol: E.clone(),
        rhs: vec![V.clone()],
    });

    // 4  V -> x
    grammar_inner.push(Production {
        start_symbol: V.clone(),
        rhs: vec![x],
    });

    // 5  V -> * E
    grammar_inner.push(Production {
        start_symbol: V,
        rhs: vec![deref, E],
    });

    Grammar::new(grammar_inner)
}

#[cfg(test)]
mod tests {
    use crate::grammar_3_10;

    #[test]
    fn test() {
        for p in grammar_3_10().iter() {
            println!("{}", p);
        }
    }
}
