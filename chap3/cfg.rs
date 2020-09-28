#![allow(non_snake_case)]

use crate::extend;
use std::collections::{HashMap, HashSet};
use std::{fmt, ops::Deref, ops::DerefMut};

pub struct Grammar(Vec<Production>);

impl Grammar {
    pub fn new(prods: Vec<Production>) -> Self {
        Self(prods)
    }
    pub fn get_prods(&self, x: &Symbol) -> Vec<&Production> {
        if x.is_nonterminal() {
            self.0
                .iter()
                .filter(|prod| &prod.start_symbol == x)
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Deref for Grammar {
    type Target = Vec<Production>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grammar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
        Symbol::Terminal(Terminal::Epsilon) == *self
    }

    pub fn is_nonterminal(&self) -> bool {
        if let Symbol::Nonterminal(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_terminal(&self) -> bool {
        if let Symbol::Terminal(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

pub type SymbolSets = HashMap<Symbol, HashSet<Symbol>>;

pub fn compute_first_follow_nullable_sets(
    grammar: &Grammar,
) -> (SymbolSets, SymbolSets, HashSet<Symbol>) {
    // Initialize FIRST and FOLLOW to all empty sets. and nullable to signle Epsilon set.
    let (mut first_sets, mut follow_sets, mut nullable_set) = (
        HashMap::with_capacity(grammar.len()),
        HashMap::with_capacity(grammar.len()),
        [Symbol::Terminal(Terminal::Epsilon)]
            .iter()
            .cloned()
            .collect::<HashSet<_>>(),
    );

    for prod in grammar.iter() {
        first_sets.insert(prod.start_symbol.clone(), HashSet::new());
        follow_sets.insert(prod.start_symbol.clone(), HashSet::new());

        for symbol in &prod.rhs {
            if symbol.is_terminal() {
                if !first_sets.contains_key(symbol) {
                    // Init first_sets for each terminal symbol.
                    first_sets.insert(symbol.clone(), [symbol.clone()].iter().cloned().collect());
                }

                if !follow_sets.contains_key(symbol) {
                    follow_sets.insert(symbol.clone(), HashSet::new());
                }
            }
        }
    }

    let mut sets_is_changed = true;
    while sets_is_changed {
        sets_is_changed = false;
        // comput first sets and nullable set
        for prod in grammar.iter() {
            let mut i_nullable = true;

            for (i, i_symbol) in prod.rhs.iter().enumerate() {
                if i_nullable {
                    // if Y1 ··· Yi−1 are all nullable
                    //   then FIRST[X] ← FIRST[X] ∪ FIRST[Yi]
                    let i_first_set = first_sets
                        .get(i_symbol)
                        .cloned()
                        .unwrap_or_else(HashSet::new);
                    let cur_first_set = first_sets.get_mut(&prod.start_symbol).unwrap();
                    extend!(cur_first_set, i_first_set, sets_is_changed);
                }

                if i_nullable && !nullable_set.contains(i_symbol) {
                    i_nullable = false;
                }

                let mut j_nullable = true;
                for j_symbol in prod.rhs.iter().skip(i + 1) {
                    if j_nullable {
                        // if Yi+1 ··· Yj−1 are all nullable
                        //  then FOLLOW[Yi] = FOLLOW[Yi] ∪ FIRST[Yj]
                        let j_first_set = first_sets
                            .get(j_symbol)
                            .cloned()
                            .unwrap_or_else(HashSet::new);
                        let i_follow_set = follow_sets.get_mut(i_symbol).unwrap();
                        extend!(i_follow_set, j_first_set, sets_is_changed);
                    }

                    if j_nullable && !nullable_set.contains(j_symbol) {
                        j_nullable = false;
                    }
                }

                if j_nullable {
                    // if Yi+1 ··· Yk are all nullable
                    //  then FOLLOW[Yi] = FOLLOW[Yi] ∪ FOLLOW[X]
                    let cur_follow_set = first_sets
                        .get(&prod.start_symbol)
                        .cloned()
                        .unwrap_or_else(HashSet::new);
                    let i_follow_set = follow_sets.get_mut(i_symbol).unwrap();
                    extend!(i_follow_set, cur_follow_set, sets_is_changed);
                }
            }

            if i_nullable {
                // if all the Yi are nullable
                //  then nullable[X] ← true
                sets_is_changed = nullable_set.insert(prod.start_symbol.clone()) || sets_is_changed;
            }
        }
    }

    (first_sets, follow_sets, nullable_set)
}

pub struct FirstSSetOp<'a> {
    first_sets: &'a SymbolSets,
    nullable_set: &'a HashSet<Symbol>,
}

impl<'a> FirstSSetOp<'a> {
    pub fn new(first_sets: &'a SymbolSets, nullable_set: &'a HashSet<Symbol>) -> Self {
        Self {
            first_sets,
            nullable_set,
        }
    }

    pub fn compute(&self, symbols: impl IntoIterator<Item = &'a Symbol>) -> HashSet<Symbol> {
        let mut symbols = symbols.into_iter();
        if let Some(first_symbol) = symbols.next() {
            let mut first_s_set = self
                .first_sets
                .get(first_symbol)
                .cloned()
                .unwrap_or_else(HashSet::new);
            for symbol in symbols {
                if !self.nullable_set.contains(symbol) {
                    break;
                }
                first_s_set.extend(
                    self.first_sets
                        .get(symbol)
                        .cloned()
                        .unwrap_or_else(HashSet::new),
                );
            }
            first_s_set
        } else {
            HashSet::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::{compute_first_follow_nullable_sets, Symbol};
    use crate::grammar_3_6;
    use std::collections::{HashMap, HashSet};

    use super::FirstSSetOp;

    #[test]
    fn test_compute_first_follow_nullable_sets() {
        let cases = vec![(
            grammar_3_6(),
            vec![
                (
                    Symbol::new_nonterminal("X"),
                    vec![
                        Symbol::new_other_terminal("a"),
                        Symbol::new_other_terminal("c"),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>(),
                ),
                (
                    Symbol::new_nonterminal("Y"),
                    vec![Symbol::new_other_terminal("c")]
                        .into_iter()
                        .collect::<HashSet<_>>(),
                ),
                (
                    Symbol::new_nonterminal("Z"),
                    vec![
                        Symbol::new_other_terminal("a"),
                        Symbol::new_other_terminal("c"),
                        Symbol::new_other_terminal("d"),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>(),
                ),
            ],
            vec![
                (
                    Symbol::new_nonterminal("X"),
                    vec![
                        Symbol::new_other_terminal("a"),
                        Symbol::new_other_terminal("c"),
                        Symbol::new_other_terminal("d"),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>(),
                ),
                (
                    Symbol::new_nonterminal("Y"),
                    vec![
                        Symbol::new_other_terminal("a"),
                        Symbol::new_other_terminal("c"),
                        Symbol::new_other_terminal("d"),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>(),
                ),
                (Symbol::new_nonterminal("Z"), HashSet::new()),
            ],
            vec![Symbol::new_nonterminal("X"), Symbol::new_nonterminal("Y")],
        )];
        for (grammar, expected_first_sets, expected_follow_sets, expected_nullable_set) in cases {
            let (expected_first_sets, expected_follow_sets, expected_nullable_set) = (
                expected_first_sets.into_iter().collect::<HashMap<_, _>>(),
                expected_follow_sets.into_iter().collect::<HashMap<_, _>>(),
                expected_nullable_set.into_iter().collect::<HashSet<_>>(),
            );

            let (actual_first_sets, actual_follow_sets, actual_nullable_set) =
                compute_first_follow_nullable_sets(&grammar);
            assert!(assert_map(&actual_first_sets, &expected_first_sets));
            assert!(assert_map(&actual_follow_sets, &expected_follow_sets));
            assert!(assert_set(&actual_nullable_set, &expected_nullable_set));
        }
    }

    #[test]
    fn test_compute_first_s_set() {
        let cases = vec![
            (
                grammar_3_6(),
                vec![
                    Symbol::new_nonterminal("X"),
                    Symbol::new_nonterminal("Y"),
                    Symbol::new_nonterminal("Z"),
                ],
                vec![
                    Symbol::new_other_terminal("a"),
                    Symbol::new_other_terminal("c"),
                ],
            ),
            (
                grammar_3_6(),
                vec![Symbol::new_other_terminal("c")],
                vec![Symbol::new_other_terminal("c")],
            ),
        ];

        for (grammar, symbols, expected_first_s_set) in cases {
            let (first_sets, _, nullable_set) = compute_first_follow_nullable_sets(&grammar);
            let expected_first_s_set = expected_first_s_set.into_iter().collect::<HashSet<_>>();
            let first_s_set_op = FirstSSetOp::new(&first_sets, &nullable_set);
            assert!(assert_set(
                &first_s_set_op.compute(&symbols),
                &expected_first_s_set
            ));
        }
    }

    fn assert_map(
        actual: &HashMap<Symbol, HashSet<Symbol>>,
        expected: &HashMap<Symbol, HashSet<Symbol>>,
    ) -> bool {
        for (k, v) in expected {
            match actual.get(&k) {
                Some(set) => {
                    if !assert_set(set, v) {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    fn assert_set(actual: &HashSet<Symbol>, expected: &HashSet<Symbol>) -> bool {
        expected.difference(&actual).count() == 0
    }
}
