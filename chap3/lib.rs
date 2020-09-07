#![allow(non_snake_case)]

use crate::context_free::{Grammar, Symbol};
use std::collections::{HashMap, HashSet};

pub mod context_free;

pub fn nullable_set(grammar: &Grammar) -> HashSet<Symbol> {
    let (mut nullable_set, mut ns_is_changed) = (HashSet::new(), true);

    while ns_is_changed {
        ns_is_changed = false;
        for production in &grammar.0 {
            if is_only_epsilon(&production.rhs)
                || (is_all_nonterminal(&production.rhs)
                    && is_all_in_nullable_set(&production.rhs, &nullable_set))
            {
                ns_is_changed = nullable_set.insert(production.start_symbol.clone());
            }
        }
    }

    #[inline]
    fn is_only_epsilon(ss: &Vec<Symbol>) -> bool {
        ss.len() == 1 && ss.first().unwrap().is_epsilon()
    }

    #[inline]
    fn is_all_nonterminal(ss: &Vec<Symbol>) -> bool {
        ss.iter().all(Symbol::is_nonterminal)
    }

    #[inline]
    fn is_all_in_nullable_set(ss: &Vec<Symbol>, nullable_set: &HashSet<Symbol>) -> bool {
        ss.iter().all(|s| nullable_set.contains(s))
    }

    nullable_set
}

pub fn first_sets(
    grammar: &Grammar,
    nullable_set: &HashSet<Symbol>,
) -> HashMap<Symbol, HashSet<Symbol>> {
    let (mut first_sets, mut fs_is_changed) = (HashMap::with_capacity(grammar.0.len()), true);
    // Initialize all nonterminal symbol to empty set.
    grammar.0.iter().for_each(|p| {
        first_sets.insert(p.start_symbol.clone(), HashSet::new());
    });

    while fs_is_changed {
        fs_is_changed = false;
        for production in &grammar.0 {
            for symbol in &production.rhs {
                if symbol.is_terminal() && !symbol.is_epsilon() {
                    fs_is_changed = first_sets
                        .get_mut(&production.start_symbol)
                        .unwrap()
                        .insert(symbol.clone());
                    break;
                }
                if symbol.is_nonterminal() {
                    let merge = first_sets.get(symbol).cloned().unwrap_or_else(HashSet::new);
                    let cur_first_set = first_sets.get_mut(&production.start_symbol).unwrap();

                    if !fs_is_changed {
                        let old_len = cur_first_set.len();
                        cur_first_set.extend(merge);
                        fs_is_changed = old_len != cur_first_set.len();
                    } else {
                        cur_first_set.extend(merge);
                    }

                    if !nullable_set.contains(symbol) {
                        break;
                    }
                }
            }
        }
    }

    first_sets
}

pub fn follow_sets(
    grammar: &Grammar,
    nullable_set: &HashSet<Symbol>,
    first_sets: &HashMap<Symbol, HashSet<Symbol>>,
) -> HashMap<Symbol, HashSet<Symbol>> {
    let (mut follow_sets, mut fs_is_changed) = (HashMap::with_capacity(grammar.0.len()), true);

    // Initialize all nonterminal symbol to empty set.
    grammar.0.iter().for_each(|p| {
        follow_sets.insert(p.start_symbol.clone(), HashSet::new());
    });

    while fs_is_changed {
        fs_is_changed = false;
        for production in &grammar.0 {
            for symbol in production.rhs.iter().rev() {
                if !(symbol.is_nonterminal() && nullable_set.contains(symbol)) {
                    break;
                }
                let start_symbol_follow_set = follow_sets
                    .get(&production.start_symbol)
                    .cloned()
                    .unwrap_or_else(HashSet::new);

                let cur_follow_set = follow_sets.get_mut(symbol).unwrap();
                if !fs_is_changed {
                    let old_len = cur_follow_set.len();
                    cur_follow_set.extend(start_symbol_follow_set);
                    fs_is_changed = old_len != cur_follow_set.len();
                } else {
                    cur_follow_set.extend(start_symbol_follow_set);
                }
            }

            for i in 0..production.rhs.len() {
                for j in (i + 1)..production.rhs.len() {
                    let j_symbol = &production.rhs[j];
                    if j_symbol.is_terminal() && !j_symbol.is_epsilon() {
                        fs_is_changed = follow_sets
                            .get_mut(&production.rhs[i])
                            .unwrap()
                            .insert(j_symbol.clone());
                        break;
                    }
                    if j_symbol.is_nonterminal() {
                        let j_first_set = first_sets
                            .get(j_symbol)
                            .cloned()
                            .unwrap_or_else(HashSet::new);

                        let i_follow_set = follow_sets.get_mut(&production.rhs[i]).unwrap();

                        if !fs_is_changed {
                            let old_len = i_follow_set.len();
                            i_follow_set.extend(j_first_set);
                            fs_is_changed = old_len != i_follow_set.len();
                        } else {
                            i_follow_set.extend(j_first_set);
                        }

                        if !nullable_set.contains(j_symbol) {
                            break;
                        }
                    }
                }
            }
        }
    }
    follow_sets
}

#[cfg(test)]
mod tests {
    use crate::context_free::{grammar_3_6, Symbol};
    use crate::{first_sets, follow_sets, nullable_set};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_nullable_set() {
        let cases = vec![(
            grammar_3_6(),
            vec![Symbol::new_nonterminal("X"), Symbol::new_nonterminal("Y")],
        )];
        for (grammar, expected) in cases {
            let expected = expected.into_iter().collect::<HashSet<_>>();
            assert_eq!(nullable_set(&grammar), expected);
        }
    }

    #[test]
    fn test_first_sets() {
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
        )];

        for (grammar, expected) in cases {
            let expected = expected.into_iter().collect::<HashMap<_, _>>();
            assert_eq!(first_sets(&grammar, &nullable_set(&grammar)), expected);
        }
    }

    #[test]
    fn test_follow_sets() {
        let cases = vec![(
            grammar_3_6(),
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
        )];

        for (grammar, expected) in cases {
            let expected = expected.into_iter().collect::<HashMap<_, _>>();
            let nullable_set = nullable_set(&grammar);

            assert_eq!(
                follow_sets(
                    &grammar,
                    &nullable_set,
                    &first_sets(&grammar, &nullable_set)
                ),
                expected
            );
        }
    }
}
