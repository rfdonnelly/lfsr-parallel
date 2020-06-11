use std::fmt;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Variable {
    InitialState,
    Data,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Term {
    variable: Variable,
    index: usize,
}

#[derive(Clone)]
pub struct Terms {
    terms: HashSet<Term>,
}

impl Term {
    fn new_data(index: usize) -> Self {
        Term {
            variable: Variable::Data,
            index,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable::InitialState => write!(f, "is"),
            Variable::Data => write!(f, "d"),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.variable, self.index)
    }
}

impl Terms {
    fn new() -> Self {
        Self {
            terms: HashSet::new(),
        }
    }

    fn with_initial_state(index: usize) -> Self {
        let mut terms = HashSet::new();

        terms.insert(
            Term {
                variable: Variable::InitialState,
                index,
            }
        );

        Self {
            terms,
        }
    }

    fn add_term(&mut self, term: Term) {
        if self.terms.contains(&term) {
            self.terms.remove(&term);
        } else {
            self.terms.insert(term);
        }
    }

    fn add_terms(&mut self, terms: &Terms) {
        for term in terms.iter() {
            self.add_term(*term)
        }
    }

    fn iter(&self) -> std::collections::hash_set::Iter<Term> {
        self.terms.iter()
    }
}

fn u64_to_vecbool(
    state_size: usize,
    polynomial: u64,
) -> Vec<bool> {
    (0..state_size).map(|i| {
        let mask = 1 << i;
        (polynomial & mask) > 0
    }).collect()
}

pub fn unroll_lfsr(
    data_size: usize,
    state_size: usize,
    polynomial: u64,
) -> Vec<Terms> {
    let mut state = vec![Terms::new(); state_size];

    let polynomial = u64_to_vecbool(state_size, polynomial);

    for data_bit_idx in (0..data_size).rev() {
        state.rotate_right(1);
        state[0].add_term(Term::new_data(data_bit_idx));
        for state_bit_idx in 1..state_size {
            if polynomial[state_bit_idx] {
                let (feedback, remainder) = state.split_at_mut(1);
                remainder[state_bit_idx - 1].add_terms(feedback.first().unwrap());
            }
        }
    }

    state
}

pub fn state_to_s(state: &[Terms]) -> String {
    state.iter().enumerate().map(|(i, terms)| {
        let mut terms_strings = terms.iter().map(ToString::to_string).collect::<Vec<String>>();
        terms_strings.sort();
        format!("c[{}] = {}", i, terms_strings.join(" ^ "))
    }).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod tests2 {
    use super::*;

    use indoc::indoc;

    /// Wrapper around string slice that makes debug output `{:?}` to print string same way as `{}`.
    /// Used in different `assert*!` macros in combination with `pretty_assertions` crate to make
    /// test failures to show nice diffs.
    #[derive(PartialEq, Eq)]
    #[doc(hidden)]
    pub struct PrettyString<'a>(pub &'a str);

    /// Make diff to display string as multi-line string
    impl<'a> fmt::Debug for PrettyString<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    // #[macro_export]
    macro_rules! assert_eq {
        ($left:expr, $right:expr) => {
            pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
        };
    }

    #[test]
    fn test_unroll() {
        let state = unroll_lfsr(8, 8, 0x7);
        let actual = state_to_s(&state);
        let expected = indoc!(
            "
            c[0] = d[0] ^ d[6] ^ d[7]
            c[1] = d[0] ^ d[1] ^ d[6]
            c[2] = d[0] ^ d[1] ^ d[2] ^ d[6]
            c[3] = d[1] ^ d[2] ^ d[3] ^ d[7]
            c[4] = d[2] ^ d[3] ^ d[4]
            c[5] = d[3] ^ d[4] ^ d[5]
            c[6] = d[4] ^ d[5] ^ d[6]
            c[7] = d[5] ^ d[6] ^ d[7]"
        );
        assert_eq!(expected, &actual);
    }
}

