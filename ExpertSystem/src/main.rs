use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Gender {
    Male,
    Female,
    X,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Fact {
    Person { name: &'static str, gender: Gender },
    Parent { parent: &'static str, child: &'static str },
    Father { father: &'static str, child: &'static str },
    Mother { mother: &'static str, child: &'static str },
    Sibling { a: &'static str, b: &'static str },
    Uncle { uncle: &'static str, nibling: &'static str },
    Cousin { a: &'static str, b: &'static str },
    Nibling { nibling: &'static str, relative: &'static str }, // Nephew or Niece
    Grandson { grandson: &'static str, grandparent: &'static str },
}

#[derive(Default)]
struct KnowledgeBase {
    facts: HashSet<Fact>,
}

impl KnowledgeBase {
    fn new() -> Self {
        Self::default()
    }

    fn add_fact(&mut self, fact: Fact) -> bool {
        self.facts.insert(fact)
    }

    fn has_person(&self, name: &str, gender: Gender) -> bool {
        self.facts.iter().any(|f| {
            if let Fact::Person { name: n, gender: g } = f {
                *n == name && *g == gender
            } else {
                false
            }
        })
    }
}


fn snapshot(kb: &KnowledgeBase) -> Vec<Fact> {
    kb.facts.iter().cloned().collect()
}

fn iter_parents<'a>(facts: &'a [Fact]) -> impl Iterator<Item = (&'static str, &'static str)> + 'a {
    facts.iter().filter_map(|f| {
        if let Fact::Parent { parent, child } = f {
            Some((*parent, *child))
        } else {
            None
        }
    })
}

fn iter_siblings<'a>(facts: &'a [Fact]) -> impl Iterator<Item = (&'static str, &'static str)> + 'a {
    facts.iter().filter_map(|f| {
        if let Fact::Sibling { a, b } = f {
            Some((*a, *b))
        } else {
            None
        }
    })
}

fn infer_father_mother(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    for (parent, child) in iter_parents(&facts) {
        if kb.has_person(parent, Gender::Male) {
            changed |= kb.add_fact(Fact::Father { father: parent, child });
        }
        if kb.has_person(parent, Gender::Female) {
            changed |= kb.add_fact(Fact::Mother { mother: parent, child });
        }
    }

    changed
}

fn infer_sibling(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    let parents: Vec<(&str, &str)> = iter_parents(&facts).collect();

    for (p1, c1) in &parents {
        for (p2, c2) in &parents {
            if p1 == p2 && c1 != c2 {
                changed |= kb.add_fact(Fact::Sibling { a: c1, b: c2 });
                changed |= kb.add_fact(Fact::Sibling { a: c2, b: c1 });
            }
        }
    }
    changed
}

fn infer_uncle(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    let parents: Vec<(&str, &str)> = iter_parents(&facts).collect();

    for (s, p) in iter_siblings(&facts) {
        for (parent, child) in &parents {
            if *parent == p && kb.has_person(s, Gender::Male) {
                changed |= kb.add_fact(Fact::Uncle {
                    uncle: s,
                    nibling: *child,
                });
            }
        }
    }
    changed
}

fn infer_cousin(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    let parents: Vec<(&str, &str)> = iter_parents(&facts).collect();

    for (p1, p2) in iter_siblings(&facts) {
        for (par1, c1) in &parents {
            if *par1 == p1 {
                for (par2, c2) in &parents {
                    if *par2 == p2 && c1 != c2 {
                        changed |= kb.add_fact(Fact::Cousin { a: c1, b: c2 });
                        changed |= kb.add_fact(Fact::Cousin { a: c2, b: c1 });
                    }
                }
            }
        }
    }
    changed
}

fn infer_nibling(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    let parents: Vec<(&str, &str)> = iter_parents(&facts).collect();

    for (p, s) in iter_siblings(&facts) {
        for (parent, child) in &parents {
            if *parent == s {
                changed |= kb.add_fact(Fact::Nibling {
                    nibling: *child,
                    relative: p,
                });
            }
        }
    }
    changed
}

fn infer_grandson(kb: &mut KnowledgeBase) -> bool {
    let facts = snapshot(kb);
    let mut changed = false;

    let parents: Vec<(&str, &str)> = iter_parents(&facts).collect();

    for (a, b) in &parents {
        for (bb, c) in &parents {
            if b == bb && kb.has_person(c, Gender::Male) {
                changed |= kb.add_fact(Fact::Grandson {
                    grandson: c,
                    grandparent: a,
                });
            }
        }
    }
    changed
}

fn infer_all(kb: &mut KnowledgeBase) {
    loop {
        let mut changed = false;
        changed |= infer_father_mother(kb);
        changed |= infer_sibling(kb);
        changed |= infer_uncle(kb);
        changed |= infer_cousin(kb);
        changed |= infer_nibling(kb);
        changed |= infer_grandson(kb);
        if !changed {
            break;
        }
    }
}

fn sons_of(kb: &KnowledgeBase, parent_name: &str) -> Vec<&'static str> {
    kb.facts
        .iter()
        .filter_map(|f| {
            if let Fact::Father { father, child } = f {
                if *father == parent_name && kb.has_person(child, Gender::Male) {
                    Some(*child)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn grandsons_of(kb: &KnowledgeBase, grandparent_name: &str) -> Vec<&'static str> {
    kb.facts
        .iter()
        .filter_map(|f| {
            if let Fact::Grandson {
                grandson,
                grandparent,
            } = f
            {
                if *grandparent == grandparent_name {
                    Some(*grandson)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let mut kb = KnowledgeBase::new();

    kb.add_fact(Fact::Person {
        name: "lars",
        gender: Gender::Male,
    });
    kb.add_fact(Fact::Person {
        name: "lotte",
        gender: Gender::Female,
    });
    kb.add_fact(Fact::Person {
        name: "ellis",
        gender: Gender::Female,
    });
    kb.add_fact(Fact::Person {
        name: "erik",
        gender: Gender::Male,
    });
    kb.add_fact(Fact::Person {
        name: "arie",
        gender: Gender::Male,
    });
    kb.add_fact(Fact::Person {
        name: "ariejan",
        gender: Gender::Male,
    });

    kb.add_fact(Fact::Parent {
        parent: "arie",
        child: "ellis",
    });
    kb.add_fact(Fact::Parent {
        parent: "arie",
        child: "ariejan",
    });

    kb.add_fact(Fact::Parent {
        parent: "ellis",
        child: "lars",
    });
    kb.add_fact(Fact::Parent {
        parent: "ellis",
        child: "lotte",
    });
    kb.add_fact(Fact::Parent {
        parent: "erik",
        child: "lars",
    });
    kb.add_fact(Fact::Parent {
        parent: "erik",
        child: "lotte",
    });

    infer_all(&mut kb);

    println!("Sons of erik: {:?}", sons_of(&kb, "erik"));
    println!("Sons of arie: {:?}", sons_of(&kb, "arie"));
    println!("Grandsons of arie: {:?}", grandsons_of(&kb, "arie"));
}
