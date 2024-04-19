#[derive(Default)]
pub struct DeterministicFiniteAutomata {
    current_state: usize,
    transition_matrix: Vec<Vec<usize>>, // Matrix of transitions for '0' and '1'
    accept_states: Vec<bool>,           // Indicates if a state is an accepting state
}

impl DeterministicFiniteAutomata {
    pub fn new() -> Self {
        let transition_matrix = vec![
            vec![1, 4], // From State 0, '0' leads to State 1, '1' to State 4
            vec![2, 0], // From State 1, '0' leads to State 2, '1' to State 0
            vec![3, 1], // From State 2, '0' leads to State 3, '1' to State 1
            vec![4, 2], // From State 3, '0' leads to State 4, '1' to State 2
            vec![0, 3], // From State 4, '0' leads to State 0, '1' to State 3
        ];
        let accept_states = vec![true, false, true, false, true]; // Accepting states: 0, 2, 4
        DeterministicFiniteAutomata {
            current_state: 0,
            transition_matrix,
            accept_states,
        }
    }

    pub fn process(&mut self, input: &str) -> bool {
        for c in input.chars() {
            if let Some(index) = c.to_digit(10) {
                let index = index as usize;
                // Update the current state of the DFA using the transition matrix
                // The new state is determined by the current state and the input digit
                self.current_state = self.transition_matrix[self.current_state][index];
            }
        }
        // Return whether the final state after processing the input string is an accepting state
        self.accept_states[self.current_state]
    }

    pub fn reset(&mut self) {
        self.current_state = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::string::finite_automaton::DeterministicFiniteAutomata;

    #[test]
    fn test_even_zeros() {
        let mut dfa = DeterministicFiniteAutomata::new();
        assert!(dfa.process("00000"));
        dfa.reset();
        assert!(dfa.process("01011"));
        dfa.reset();
        assert!(dfa.process("10011"));
        dfa.reset();
    }

    #[test]
    fn test_odd_zeros() {
        let mut dfa = DeterministicFiniteAutomata::new();
        assert!(!dfa.process("0"));
        dfa.reset();
        assert!(!dfa.process("000"));
        dfa.reset();
        assert!(!dfa.process("01001"));
        dfa.reset();
    }

    #[test]
    fn test_empty_input() {
        let mut dfa = DeterministicFiniteAutomata::new();
        assert!(dfa.process("")); // No moves, starts and remains at zero step
        dfa.reset();
    }
}
