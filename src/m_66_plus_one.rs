//You are given a large integer represented as an integer array digits, where
//each digits[i] is the iᵗʰ digit of the integer. The digits are ordered from most
//significant to least significant in left-to-right order. The large integer does
//not contain any leading 0's.
//
// Increment the large integer by one and return the resulting array of digits.
//
//
//
// Example 1:
//
//
//Input: digits = [1,2,3]
//Output: [1,2,4]
//Explanation: The array represents the integer 123.
//Incrementing by one gives 123 + 1 = 124.
//Thus, the result should be [1,2,4].
//
//
// Example 2:
//
//
//Input: digits = [4,3,2,1]
//Output: [4,3,2,2]
//Explanation: The array represents the integer 4321.
//Incrementing by one gives 4321 + 1 = 4322.
//Thus, the result should be [4,3,2,2].
//
//
// Example 3:
//
//
//Input: digits = [0]
//Output: [1]
//Explanation: The array represents the integer 0.
//Incrementing by one gives 0 + 1 = 1.
//Thus, the result should be [1].
//
//
// Example 4:
//
//
//Input: digits = [9]
//Output: [1,0]
//Explanation: The array represents the integer 9.
//Incrementing by one gives 9 + 1 = 10.
//Thus, the result should be [1,0].
//
//
//
// Constraints:
//
//
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits does not contain any leading 0's.
//
// Related Topics Array Math 👍 3066 👎 3711


#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn test_case_123() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_case_4321() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_case_0() {
        assert_eq!(plus_one(vec![0]), vec![1]);
    }

    #[test]
    fn test_case_9() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn test_case_999() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0])
    }

    #[test]
    fn test_case_789() {
        assert_eq!(plus_one(vec![7, 8, 9]), vec![7, 9, 0])
    }

    #[test]
    fn test_case_9876543210() {
        assert_eq!(plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1])
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    plus_one_optimized(digits)
}

fn plus_one_version_1(digits: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut carry = 1;
    for i in digits.into_iter().rev() {
        if carry == 0 {
            result.push(i)
        } else {
            let added = i + carry;
            let sum = added % 10;
            carry = (added - sum) / 10;
            result.push(sum);
        }
    }
    if carry > 0 {
        result.push(carry);
    }
    result.reverse();
    result
}

fn plus_one_optimized(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut carry = 1;
    let mut i = digits.len();
    loop {
        if i == 0 || carry == 0 {
            break;
        }
        i -= 1;
        let added = carry + digits[i];
        let sum = added % 10;
        carry = (added - sum) / 10;
        digits[i] = sum;
    }
    if carry > 0 {
        digits.insert(0, carry);
    }
    digits
}
//leetcode submit region end(Prohibit modification and deletion)
