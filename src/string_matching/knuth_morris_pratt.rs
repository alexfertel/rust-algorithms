fn precompute_table(pattern: &str) -> Vec<usize> {
    let p = pattern.as_bytes();

    let mut pi = vec![0; pattern.len()];
    let mut k = 0;

    for q in 1..p.len() {
        while k > 0 && p[k] != p[q] {
            k = pi[k];
        }

        if p[k] == p[q] {
            k = k + 1;
        }

        pi[q] = k;
    }

    pi
}

pub fn knuth_morris_pratt(text: &str, pattern: &str) -> Vec<usize> {
    if text.is_empty() || pattern.is_empty() {
        return vec![];
    }

    let t = text.as_bytes();
    let p = pattern.as_bytes();

    let pi = precompute_table(pattern);
    let mut matches = vec![];

    let mut q = 0;
    for i in 1..=t.len() {
        while q > 0 && p[q] != t[i - 1] {
            q = pi[q - 1];
        }

        if p[q] == t[i - 1] {
            q = q + 1;
        }

        if q == p.len() {
            matches.push(i - p.len());
            q = pi[q - 1];
        }
    }

    matches
}

#[cfg(test)]
mod test {
    use super::{knuth_morris_pratt, precompute_table};

    #[test]
    fn builds_pi_correctly() {
        let pi = precompute_table("ababaca");
        assert_eq!(pi, vec![0, 0, 1, 2, 3, 0, 1]);
    }

    #[test]
    fn each_letter_matches() {
        let pi = precompute_table("aaa");
        assert_eq!(pi, vec![0, 1, 2]);

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
