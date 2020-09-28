use crate::{cfg, extend};
use std::{collections::HashSet, fmt, hash};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    prod: cfg::Production,
    dot_pos: usize,
    lockaheads: HashSet<cfg::Symbol>,
}

impl hash::Hash for Item {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.dot_pos.hash(state);
        self.prod.hash(state);
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.prod.start_symbol)?;
        for (i, s) in self.prod.rhs.iter().enumerate() {
            if i == self.dot_pos {
                write!(f, ".")?;
            }
            write!(f, "{}", s)?;
        }
        write!(f, "    [")?;

        let mut lockaheads_iter = self.lockaheads.iter();
        if let Some(first) = lockaheads_iter.next() {
            write!(f, "{}", first)?;
            for lockahead in lockaheads_iter {
                write!(f, ", {}", lockahead)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl Item {
    pub fn new(prod: cfg::Production, lockaheads: HashSet<cfg::Symbol>) -> Self {
        Self {
            prod,
            dot_pos: 0,
            lockaheads,
        }
    }

    pub fn peek(&self) -> Option<&cfg::Symbol> {
        self.prod.rhs.get(self.dot_pos)
    }

    pub fn next(&mut self) -> Option<&cfg::Symbol> {
        self.dot_pos += 1;
        self.peek()
    }

    pub fn rest_rhs(&self) -> impl IntoIterator<Item = &cfg::Symbol> {
        self.prod.rhs.iter().skip(self.dot_pos)
    }
}

struct ClosureOp<'a> {
    grammar: &'a cfg::Grammar,
    first_s_set_op: &'a cfg::FirstSSetOp<'a>,
}

impl<'a> ClosureOp<'a> {
    pub fn new(grammar: &'a cfg::Grammar, first_s_set_op: &'a cfg::FirstSSetOp<'a>) -> Self {
        Self {
            grammar,
            first_s_set_op,
        }
    }

    pub fn compute(&self, mut items: HashSet<Item>) -> HashSet<Item> {
        let mut is_changed = true;
        while is_changed {
            is_changed = false;
            let mut merge_items = HashSet::new();
            for item in &items {
                if let Some(next_symbol) = item.peek() {
                    let lockaheads = self.first_s_set_op.compute(item.rest_rhs());
                    let merge = self
                        .grammar
                        .get_prods(next_symbol)
                        .into_iter()
                        .cloned()
                        .map(|prod| Item::new(prod.clone(), lockaheads.clone()));
                    merge_items.extend(merge);
                }
            }
            extend!(items, merge_items, is_changed);
        }
        items
    }
}

struct GotoOp<'a> {
    closure_op: &'a ClosureOp<'a>,
}

impl<'a> GotoOp<'a> {
    pub fn compute(&self, items: HashSet<Item>) {
        self.closure_op.compute(
            items
                .into_iter()
                .map(|mut item| {
                    item.next();
                    item
                })
                .collect(),
        );
    }
}

pub fn goto(closure_fn: impl Fn(Vec<Item>) -> Vec<Item>) -> impl Fn(&Vec<Item>) -> Vec<Item> {
    move |items: &Vec<Item>| -> Vec<Item> {
        let mut s = Vec::new();
        for item in items {
            let mut item = item.clone();
            item.next();
            s.push(item);
        }
        closure_fn(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::lalr_1::{ClosureOp, Item};
    use crate::{cfg, grammar_3_10};
    use std::collections::HashSet;

    #[test]
    fn test_closure() {
        let grammar_3_10 = grammar_3_10();
        let (first_sets, _, nullable_set) = cfg::compute_first_follow_nullable_sets(&grammar_3_10);
        let first_s_set_op = cfg::FirstSSetOp::new(&first_sets, &nullable_set);
        let closure_op = ClosureOp::new(&grammar_3_10, &first_s_set_op);

        let init_items: HashSet<_> = [Item::new(
            grammar_3_10.first().unwrap().clone(),
            HashSet::new(),
        )]
        .iter()
        .cloned()
        .collect();
        let closure = closure_op.compute(init_items);
        for item in closure {
            println!("{}", item);
        }
    }
}
