#![allow(non_snake_case)]

use context_free::Terminal;

use crate::context_free::{Grammar, Symbol};
use std::collections::{HashMap, HashSet};

pub mod context_free;

pub fn compute_first_follow_nullable_sets(
    grammar: &Grammar,
) -> (
    HashMap<Symbol, HashSet<Symbol>>,
    HashMap<Symbol, HashSet<Symbol>>,
    HashSet<Symbol>,
) {
    // Initialize FIRST and FOLLOW to all empty sets. and nullable to signle Epsilon set.
    let (mut first_sets, mut follow_sets, mut nullable_set) = (
        HashMap::with_capacity(grammar.0.len()),
        HashMap::with_capacity(grammar.0.len()),
        [Symbol::Terminal(Terminal::Epsilon)]
            .iter()
            .cloned()
            .collect::<HashSet<_>>(),
    );

    for prod in &grammar.0 {
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
        // comput first sets and nullable set
        for prod in &grammar.0 {
            let mut i_nullable = true;

            for i in 0..prod.rhs.len() {
                if i_nullable {
                    // if Y1 ··· Yi−1 are all nullable
                    //   then FIRST[X] ← FIRST[X] ∪ FIRST[Yi]
                    let i_first_set = first_sets
                        .get(&prod.rhs[i])
                        .cloned()
                        .unwrap_or_else(HashSet::new);
                    let cur_first_set = first_sets.get_mut(&prod.start_symbol).unwrap();
                    if !sets_is_changed {
                        let old_len = cur_first_set.len();
                        cur_first_set.extend(i_first_set);
                        sets_is_changed = old_len != cur_first_set.len();
                    } else {
                        cur_first_set.extend(i_first_set);
                    }
                }

                if i_nullable && !nullable_set.contains(&prod.rhs[i]) {
                    i_nullable = false;
                }

                let mut j_nullable = true;
                for j in i + 1..prod.rhs.len() {
                    if j_nullable {
                        // if Yi+1 ··· Yj−1 are all nullable
                        //  then FOLLOW[Yi] = FOLLOW[Yi] ∪ FIRST[Yj]
                        let j_first_set = first_sets
                            .get(&prod.rhs[j])
                            .cloned()
                            .unwrap_or_else(HashSet::new);
                        let i_follow_set = follow_sets.get_mut(&prod.rhs[i]).unwrap();
                        if !sets_is_changed {
                            let old_len = i_follow_set.len();
                            i_follow_set.extend(j_first_set);
                            sets_is_changed = old_len != i_follow_set.len();
                        } else {
                            i_follow_set.extend(j_first_set);
                        }
                    }

                    if j_nullable && !nullable_set.contains(&prod.rhs[j]) {
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
                    let i_follow_set = follow_sets.get_mut(&prod.rhs[i]).unwrap();
                    if !sets_is_changed {
                        let old_len = i_follow_set.len();
                        i_follow_set.extend(cur_follow_set);
                        sets_is_changed = old_len != i_follow_set.len();
                    } else {
                        i_follow_set.extend(cur_follow_set);
                    }
                }
            }

            if i_nullable {
                // if all the Yi are nullable
                //  then nullable[X] ← true
                sets_is_changed = nullable_set.insert(prod.start_symbol.clone());
            }
        }
    }

    (first_sets, follow_sets, nullable_set)
}

#[cfg(test)]
mod tests {
    use crate::compute_first_follow_nullable_sets;
    use crate::context_free::{grammar_3_6, Symbol};
    use std::collections::{HashMap, HashSet};

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
        for (grammar, expected_first_sets, expected_follow_sets, expected_nullbale_set) in cases {
            let (expected_first_sets, expected_follow_sets, expected_nullbale_set) = (
                expected_first_sets.into_iter().collect::<HashMap<_, _>>(),
                expected_follow_sets.into_iter().collect::<HashMap<_, _>>(),
                expected_nullbale_set.into_iter().collect::<HashSet<_>>(),
            );

            let (actual_first_sets, actual_follow_sets, actual_nullbale_set) =
                compute_first_follow_nullable_sets(&grammar);
            assert!(assert_map(&actual_first_sets, &expected_first_sets));
            assert!(assert_map(&actual_follow_sets, &expected_follow_sets));
            assert!(assert_set(&actual_nullbale_set, &expected_nullbale_set));
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
