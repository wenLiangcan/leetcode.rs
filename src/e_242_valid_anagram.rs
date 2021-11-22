//Given two strings s and t, return true if t is an anagram of s, and false
//otherwise.
//
//
// Example 1:
// Input: s = "anagram", t = "nagaram"
//Output: true
// Example 2:
// Input: s = "rat", t = "car"
//Output: false
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 5 * 10⁴
// s and t consist of lowercase English letters.
//
//
//
// Follow up: What if the inputs contain Unicode characters? How would you
//adapt your solution to such a case?
// Related Topics Hash Table String Sorting 👍 3596 👎 189

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_anagram("anagram".to_owned(), "nagaram".to_owned()));
    }

    #[test]
    fn test_false() {
        assert!(!is_anagram("rat".to_owned(), "car".to_owned()));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn is_anagram(s: String, t: String) -> bool {
    s.len() == t.len() && {
        let mut s = s.chars().into_iter().collect::<Vec<char>>();
        s.sort();
        let mut t = t.chars().into_iter().collect::<Vec<char>>();
        t.sort();
        t == s
    }
}
//leetcode submit region end(Prohibit modification and deletion)
