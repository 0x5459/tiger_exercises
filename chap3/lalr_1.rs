use crate::{cfg, extend};
use std::{collections::HashSet, fmt};

#[derive(Clone, Debug)]
pub struct Item {
    prod: cfg::Production,
    dot_pos: usize,
    lockaheads: HashSet<cfg::Symbol>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.prod.start_symbol)?;
        for (i, s) in self.prod.rhs.iter().enumerate() {
            if i == self.dot_pos {
                write!(f, ".")?;
            }
            write!(f, "{} ", s)?;
        }
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
        self.prod.rhs.get(self.dot_pos + 1)
    }

    pub fn next(&mut self) -> Option<&cfg::Symbol> {
        self.dot_pos += 1;
        self.peek()
    }

    pub fn rest_rhs(&self) {
        
    }
}

pub type ClosureFn<'a> = Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a>;

pub fn closure<'a, I: Iterator<Item = &'a cfg::Grammar>>(
    grammar: &'a cfg::Grammar,
    first_s_set_fn: cfg::ComputeFirstSSetFn<'a, I>,
) -> ClosureFn<'a>
where
    I: Iterator<Item = &'a cfg::Grammar>,
{
    Box::new(move |mut items: Vec<Item>| {
        let mut is_changed = true;
        while is_changed {
            is_changed = false;
            let mut i = 0;
            while i < items.len() {
                if let Some(next_symbol) = items[i].peek() {
                    let lockaheads = first_s_set_fn(items[i].rest_rhs());
                    let merge = grammar
                        .get_prods(next_symbol)
                        .into_iter()
                        .cloned()
                        .flat_map(|prod| {
                            lockaheads
                                .into_iter()
                                .map(|lah| Item::new(prod, [lah].into_iter().cloned().collect()))
                        });
                    extend!(items, merge, is_changed);
                }
                i += 1;
            }
        }
        items
    })
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
    use crate::grammar_3_10;
    use crate::lalr_1::{closure, Item};
    use std::collections::HashMap;

    #[test]
    fn test_closure() {
        let grammar_3_10 = grammar_3_10();

        let closure = closure(&grammar_3_10, HashMap::new());
        let c = closure(vec![Item::new(grammar_3_10.first().unwrap().clone())]);
        for item in c {
            println!("{}", item);
        }
    }
}
