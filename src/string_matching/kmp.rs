pub fn knuth_morris_pratt(text: &str, pattern: &str) -> Vec<usize> {
    let mut t_i = 0;
    let mut p_i = 0;

    let precomputed_table = precompute_table(pattern);

    let text = text.as_bytes();
    let pattern = pattern.as_bytes();

    let mut matches = vec![];
    while t_i < text.len() {
        if pattern[p_i] == text[t_i] {
            t_i += 1;
            p_i += 1;
            if p_i == pattern.len() {
                matches.push(t_i - p_i);
                p_i = precomputed_table[p_i];
            }
        } else {
            p_i = precomputed_table[p_i];
            if p_i == 0 {
                t_i += 1;
                p_i += 1;
            }
        }
    }

    matches
}

fn precompute_table(pattern: &str) -> Vec<usize> {
    let pattern = pattern.as_bytes();

    let mut table = vec![0];
    let mut pos = 1;
    let mut cnd = 0;

    while pos < pattern.len() {
        if pattern[pos] == pattern[cnd] {
            table[pos] = table[cnd]
        } else {
            table[pos] = cnd;
            while pattern[pos] != pattern[cnd] {
                cnd = table[cnd];
            }
        }

        pos = pos + 1;
        cnd = cnd + 1;
    }

    table
}

#[cfg(test)]
mod test {
    use super::knuth_morris_pratt;

    #[test]
    fn each_letter_matches() {
        let index = knuth_morris_pratt("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = knuth_morris_pratt("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index = knuth_morris_pratt("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = knuth_morris_pratt("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = knuth_morris_pratt("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = knuth_morris_pratt("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = knuth_morris_pratt("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = knuth_morris_pratt("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = knuth_morris_pratt("", "abcdef");
        assert_eq!(index, vec![]);
    }
}
