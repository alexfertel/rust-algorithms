use std::iter::FromIterator;

pub fn naive(pattern: &str, text: &str) -> Vec<usize> {
    let pat_chars = Vec::from_iter(pattern.chars());
    let txt_chars = Vec::from_iter(text.chars());
    let pat_len = pat_chars.len();
    let txt_len = txt_chars.len();
    let mut res = vec![];

    for win_ptr in 0..(txt_len - pat_len + 1) {
        let mut win_len = 0;

        while win_len < pat_len {
            if txt_chars[win_ptr + win_len] != pat_chars[win_len] {
                break;
            }

            win_len += 1;
        }

        if win_len == pat_len {
            res.push(win_ptr);
        }
    }

    res
}

mod tests {
    #[allow(unused_imports)]
    use super::naive;

    #[test]
    fn naive_test_1() {
        // avg time complexity: O(N^2)
        // auxiliary space complexity: O(1)
        assert_eq!(vec![0, 9, 13], naive("AABA", "AABAACAADAABAAABAA"));
    }

    #[test]
    fn naive_best_case() {
        // The best case occurs when the first character in the pattern is not present at all.
        // best case time complexity: O(N)
        assert_eq!(Vec::<usize>::new(), naive("FAA", "AABCCAADDEE"));
    }

    #[test]
    fn naive_worst_case_1() {
        // The worst case can occur when all characters of the text and pattern are the same.
        // worst case time complexity: O(M * (N - M + 1)), M: length of pattern | N: length of text
        assert_eq!(
            (0..=13).collect::<Vec<usize>>(),
            naive("AAAAA", "AAAAAAAAAAAAAAAAAA")
        );
    }

    #[test]
    fn naive_wost_case_2() {
        // The worst case can also occur when all characters of the text except the last are the
        // same.
        assert_eq!(vec![13], naive("AAAAB", "AAAAAAAAAAAAAAAAAB"));
    }
}
