use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Variable {
    InitialState,
    Data,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Term {
    variable: Variable,
    index: usize,
}

#[derive(Clone)]
struct Terms {
    terms: HashSet<Term>,
}

pub struct Lfsr {
    data_size: usize,
    state_size: usize,
    polynomial: u64,
    state: Vec<Terms>,
    include_initial_state: bool,
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
            Variable::InitialState => write!(f, "i"),
            Variable::Data => write!(f, "d"),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.variable, self.index)
    }
}

impl fmt::Display for Lfsr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "module parallel_lfsr(")?;
        writeln!(f, "    // data in")?;
        writeln!(f, "    input  logic [{}:0] d;", self.data_size - 1)?;
        if self.include_initial_state {
            writeln!(f, "    // initial state")?;
            writeln!(f, "    input  logic [{}:0] i;", self.state_size - 1)?;
        }
        writeln!(f, "    // final state")?;
        writeln!(f, "    output logic [{}:0] c;", self.state_size - 1)?;
        writeln!(f, ");")?;
        for (i, terms) in self.state.iter().enumerate() {
            let mut terms = terms.iter().collect::<Vec<&Term>>();
            terms.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let terms = terms
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>();
            writeln!(f, "    c[{}] = {};", i, terms.join(" ^ "))?;
        }
        writeln!(f, "endmodule")
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

        terms.insert(Term {
            variable: Variable::InitialState,
            index,
        });

        Self { terms }
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

fn u64_to_vecbool(state_size: usize, polynomial: u64) -> Vec<bool> {
    (0..state_size)
        .map(|i| {
            let mask = 1 << i;
            (polynomial & mask) > 0
        })
        .collect()
}

impl Lfsr {
    pub fn new(
        data_size: usize,
        state_size: usize,
        polynomial: u64,
        include_initial_state: bool,
    ) -> Self {
        let state = unroll_lfsr(data_size, state_size, polynomial, include_initial_state);

        Self {
            data_size,
            state_size,
            polynomial,
            state,
            include_initial_state,
        }
    }
}

fn unroll_lfsr(
    data_size: usize,
    state_size: usize,
    polynomial: u64,
    include_initial_state: bool,
) -> Vec<Terms> {
    let mut state = match include_initial_state {
        true => (0..state_size).map(Terms::with_initial_state).collect(),
        false => vec![Terms::new(); state_size],
    };

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

#[cfg(test)]
mod tests {
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
        let lfsr = Lfsr::new(8, 8, 0x7, true);
        let actual = lfsr.to_string();
        let expected = indoc!(
            "
            module parallel_lfsr(
                // data in
                input  logic [7:0] d;
                // initial state
                input  logic [7:0] i;
                // final state
                output logic [7:0] c;
            );
                c[0] = d[7] ^ d[6] ^ d[0] ^ i[7] ^ i[6] ^ i[0];
                c[1] = d[6] ^ d[1] ^ d[0] ^ i[6] ^ i[1] ^ i[0];
                c[2] = d[6] ^ d[2] ^ d[1] ^ d[0] ^ i[6] ^ i[2] ^ i[1] ^ i[0];
                c[3] = d[7] ^ d[3] ^ d[2] ^ d[1] ^ i[7] ^ i[3] ^ i[2] ^ i[1];
                c[4] = d[4] ^ d[3] ^ d[2] ^ i[4] ^ i[3] ^ i[2];
                c[5] = d[5] ^ d[4] ^ d[3] ^ i[5] ^ i[4] ^ i[3];
                c[6] = d[6] ^ d[5] ^ d[4] ^ i[6] ^ i[5] ^ i[4];
                c[7] = d[7] ^ d[6] ^ d[5] ^ i[7] ^ i[6] ^ i[5];
            endmodule
        "
        );
        assert_eq!(expected, &actual);
    }

    #[test]
    fn test_unroll_large() {
        let lfsr = Lfsr::new(56, 8, 0x7, false);
        let actual = lfsr.to_string();
        let expected = indoc!("
            module parallel_lfsr(
                // data in
                input  logic [55:0] d;
                // final state
                output logic [7:0] c;
            );
                c[0] = d[54] ^ d[53] ^ d[52] ^ d[50] ^ d[49] ^ d[48] ^ d[45] ^ d[43] ^ d[40] ^ d[39] ^ d[35] ^ d[34] ^ d[31] ^ d[30] ^ d[28] ^ d[23] ^ d[21] ^ d[19] ^ d[18] ^ d[16] ^ d[14] ^ d[12] ^ d[8] ^ d[7] ^ d[6] ^ d[0];
                c[1] = d[55] ^ d[52] ^ d[51] ^ d[48] ^ d[46] ^ d[45] ^ d[44] ^ d[43] ^ d[41] ^ d[39] ^ d[36] ^ d[34] ^ d[32] ^ d[30] ^ d[29] ^ d[28] ^ d[24] ^ d[23] ^ d[22] ^ d[21] ^ d[20] ^ d[18] ^ d[17] ^ d[16] ^ d[15] ^ d[14] ^ d[13] ^ d[12] ^ d[9] ^ d[6] ^ d[1] ^ d[0];
                c[2] = d[54] ^ d[50] ^ d[48] ^ d[47] ^ d[46] ^ d[44] ^ d[43] ^ d[42] ^ d[39] ^ d[37] ^ d[34] ^ d[33] ^ d[29] ^ d[28] ^ d[25] ^ d[24] ^ d[22] ^ d[17] ^ d[15] ^ d[13] ^ d[12] ^ d[10] ^ d[8] ^ d[6] ^ d[2] ^ d[1] ^ d[0];
                c[3] = d[55] ^ d[51] ^ d[49] ^ d[48] ^ d[47] ^ d[45] ^ d[44] ^ d[43] ^ d[40] ^ d[38] ^ d[35] ^ d[34] ^ d[30] ^ d[29] ^ d[26] ^ d[25] ^ d[23] ^ d[18] ^ d[16] ^ d[14] ^ d[13] ^ d[11] ^ d[9] ^ d[7] ^ d[3] ^ d[2] ^ d[1];
                c[4] = d[52] ^ d[50] ^ d[49] ^ d[48] ^ d[46] ^ d[45] ^ d[44] ^ d[41] ^ d[39] ^ d[36] ^ d[35] ^ d[31] ^ d[30] ^ d[27] ^ d[26] ^ d[24] ^ d[19] ^ d[17] ^ d[15] ^ d[14] ^ d[12] ^ d[10] ^ d[8] ^ d[4] ^ d[3] ^ d[2];
                c[5] = d[53] ^ d[51] ^ d[50] ^ d[49] ^ d[47] ^ d[46] ^ d[45] ^ d[42] ^ d[40] ^ d[37] ^ d[36] ^ d[32] ^ d[31] ^ d[28] ^ d[27] ^ d[25] ^ d[20] ^ d[18] ^ d[16] ^ d[15] ^ d[13] ^ d[11] ^ d[9] ^ d[5] ^ d[4] ^ d[3];
                c[6] = d[54] ^ d[52] ^ d[51] ^ d[50] ^ d[48] ^ d[47] ^ d[46] ^ d[43] ^ d[41] ^ d[38] ^ d[37] ^ d[33] ^ d[32] ^ d[29] ^ d[28] ^ d[26] ^ d[21] ^ d[19] ^ d[17] ^ d[16] ^ d[14] ^ d[12] ^ d[10] ^ d[6] ^ d[5] ^ d[4];
                c[7] = d[55] ^ d[53] ^ d[52] ^ d[51] ^ d[49] ^ d[48] ^ d[47] ^ d[44] ^ d[42] ^ d[39] ^ d[38] ^ d[34] ^ d[33] ^ d[30] ^ d[29] ^ d[27] ^ d[22] ^ d[20] ^ d[18] ^ d[17] ^ d[15] ^ d[13] ^ d[11] ^ d[7] ^ d[6] ^ d[5];
            endmodule
        ");
        assert_eq!(expected, &actual);
    }
}
