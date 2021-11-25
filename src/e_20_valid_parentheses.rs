//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']
//', determine if the input string is valid.
//
// An input string is valid if:
//
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
//
//
//
// Example 1:
//
//
//Input: s = "()"
//Output: true
//
//
// Example 2:
//
//
//Input: s = "()[]{}"
//Output: true
//
//
// Example 3:
//
//
//Input: s = "(]"
//Output: false
//
//
// Example 4:
//
//
//Input: s = "([)]"
//Output: false
//
//
// Example 5:
//
//
//Input: s = "{[]}"
//Output: true
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of parentheses only '()[]{}'.
//
// Related Topics String Stack 👍 9884 👎 385

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_valid("()".to_owned()));
    }

    #[test]
    fn test_2() {
        assert!(is_valid("()[]{}".to_owned()));
    }

    #[test]
    fn test_3() {
        assert!(!is_valid("(]".to_owned()));
    }

    #[test]
    fn test_4() {
        assert!(!is_valid("([)]".to_owned()));
    }

    #[test]
    fn test_5() {
        assert!(is_valid("{[]}".to_owned()));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn is_valid(s: String) -> bool {
    let mut lefts: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            ')' | ']' | '}' => {
                let equals = lefts.pop().map(|l| {
                    match c {
                        ')' => l == '(',
                        ']' => l == '[',
                        '}' => l == '{',
                        _ => false
                    }
                }).unwrap_or(false);
                if !equals {
                    return false;
                }
            },
            _ => lefts.push(c)
        }
    }

    lefts.is_empty()
}
//leetcode submit region end(Prohibit modification and deletion)
