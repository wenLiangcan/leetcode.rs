//Implement the myAtoi(string s) function, which converts a string to a 32-bit
//signed integer (similar to C/C++'s atoi function).
//
// The algorithm for myAtoi(string s) is as follows:
//
//
// Read in and ignore any leading whitespace.
// Check if the next character (if not already at the end of the string) is '-'
//or '+'. Read this character in if it is either. This determines if the final
//result is negative or positive respectively. Assume the result is positive if
//neither is present.
// Read in next the characters until the next non-digit character or the end of
//the input is reached. The rest of the string is ignored.
// Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If
//no digits were read, then the integer is 0. Change the sign as necessary (from
//step 2).
// If the integer is out of the 32-bit signed integer range [-2³¹, 2³¹ - 1],
//then clamp the integer so that it remains in the range. Specifically, integers
//less than -2³¹ should be clamped to -2³¹, and integers greater than 2³¹ - 1 should
//be clamped to 2³¹ - 1.
// Return the integer as the final result.
//
//
// Note:
//
//
// Only the space character ' ' is considered a whitespace character.
// Do not ignore any characters other than the leading whitespace or the rest
//of the string after the digits.
//
//
//
// Example 1:
//
//
//Input: s = "42"
//Output: 42
//Explanation: The underlined characters are what is read in, the caret is the
//current reader position.
//Step 1: "42" (no characters read because there is no leading whitespace)
//         ^
//Step 2: "42" (no characters read because there is neither a '-' nor '+')
//         ^
//Step 3: "42" ("42" is read in)
//           ^
//The parsed integer is 42.
//Since 42 is in the range [-2³¹, 2³¹ - 1], the final result is 42.
//
//
// Example 2:
//
//
//Input: s = "   -42"
//Output: -42
//Explanation:
//Step 1: "   -42" (leading whitespace is read and ignored)
//            ^
//Step 2: "   -42" ('-' is read, so the result should be negative)
//             ^
//Step 3: "   -42" ("42" is read in)
//               ^
//The parsed integer is -42.
//Since -42 is in the range [-2³¹, 2³¹ - 1], the final result is -42.
//
//
// Example 3:
//
//
//Input: s = "4193 with words"
//Output: 4193
//Explanation:
//Step 1: "4193 with words" (no characters read because there is no leading
//whitespace)
//         ^
//Step 2: "4193 with words" (no characters read because there is neither a '-'
//nor '+')
//         ^
//Step 3: "4193 with words" ("4193" is read in; reading stops because the next
//character is a non-digit)
//             ^
//The parsed integer is 4193.
//Since 4193 is in the range [-2³¹, 2³¹ - 1], the final result is 4193.
//
//
// Example 4:
//
//
//Input: s = "words and 987"
//Output: 0
//Explanation:
//Step 1: "words and 987" (no characters read because there is no leading
//whitespace)
//         ^
//Step 2: "words and 987" (no characters read because there is neither a '-'
//nor '+')
//         ^
//Step 3: "words and 987" (reading stops immediately because there is a non-
//digit 'w')
//         ^
//The parsed integer is 0 because no digits were read.
//Since 0 is in the range [-2³¹, 2³¹ - 1], the final result is 0.
//
//
// Example 5:
//
//
//Input: s = "-91283472332"
//Output: -2147483648
//Explanation:
//Step 1: "-91283472332" (no characters read because there is no leading
//whitespace)
//         ^
//Step 2: "-91283472332" ('-' is read, so the result should be negative)
//          ^
//Step 3: "-91283472332" ("91283472332" is read in)
//                     ^
//The parsed integer is -91283472332.
//Since -91283472332 is less than the lower bound of the range [-2³¹, 2³¹ - 1],
//the final result is clamped to -2³¹ = -2147483648.
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 200
// s consists of English letters (lower-case and upper-case), digits (0-9), ' ',
// '+', '-', and '.'.
//
// Related Topics String 👍 785 👎 2153

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), (String, String)>;

    #[test]
    fn test_parse_char_c() -> TestResult {
        let p_c = p_char('c');
        assert_eq!(p_c("cab".to_owned())?, ('c', String::from("ab")));
        Ok(())
    }

    #[test]
    fn test_and_then() -> TestResult {
        let p_a = p_char('a');
        let p_b = p_char('b');
        let a_then_b = and_then(once(p_a), once(p_b));
        assert_eq!(a_then_b("abc".to_owned())?, (('a', 'b'), String::from("c")));
        Ok(())
    }

    #[test]
    fn test_or_else() -> TestResult {
        let p_a = p_char('a');
        let p_b = p_char('b');
        let a_or_b = or_else(p_a, p_b);
        assert_eq!(a_or_b("bad".to_owned())?, ('b', String::from("ad")));
        Ok(())
    }

    #[test]
    fn test_any() -> TestResult {
        let p_a = p_char('a');
        let p_b = p_char('b');
        let p_c = p_char('c');
        let abc = any(vec![p_a, p_b, p_c]);
        assert_eq!(abc("crtret".to_owned())?, ('c', String::from("rtret")));
        Ok(())
    }

    #[test]
    fn test_any_of_chars() -> TestResult {
        let number = any_of_chars('0' ..= '9');
        assert_eq!(number("5abc".to_owned())?, ('5', String::from("abc")));
        Ok(())
    }

    #[test]
    fn test_map() -> TestResult {
        let number: Parser<u32> = map(|c| c.to_digit(10).unwrap(),
                                      any_of_chars('0' ..= '9'));
        assert_eq!(number("7abc".to_owned())?, (7, String::from("abc")));
        Ok(())
    }

    #[test]
    fn test_sequence() -> TestResult {
        let p_a = p_char('a');
        let p_b = p_char('b');
        let p_c = p_char('c');
        let abc = sequence(vec![p_a, p_b, p_c]);
        assert_eq!(abc("abcdefg".to_owned())?, (vec!['a', 'b', 'c'], String::from("defg")));
        Ok(())
    }

    #[test]
    fn test_many() -> TestResult {
        let number = any_of_chars('0' ..= '9');
        let numbers = many(number);
        assert_eq!(numbers("123sasds".to_owned())?, (vec!['1', '2', '3'], String::from("sasds")));
        Ok(())
    }

    #[test]
    fn test_many_empty() -> TestResult {
        let number = any_of_chars('0' ..= '9');
        let numbers = many(number);
        assert_eq!(numbers("abcd".to_owned())?, (vec![], String::from("abcd")));
        Ok(())
    }

    #[test]
    fn test_atoi() {
        assert_eq!(my_atoi("42".to_owned()), 42);
    }

    #[test]
    fn test_atoi_leading_spaces_negative_number() {
        assert_eq!(my_atoi("   -42".to_owned()), -42);
    }

    #[test]
    fn test_atoi_non_numeric_trialing() {
        assert_eq!(my_atoi("4193 with words".to_owned()), 4193);
    }

    #[test]
    fn test_atoi_non_numeric_leading() {
        assert_eq!(my_atoi("words and 987".to_owned()), 0);
    }

    #[test]
    fn test_atoi_out_of_i32_lower_bound() {
        assert_eq!(my_atoi("-91283472332".to_owned()), -2147483648);
    }

    #[test]
    fn test_atoi_out_of_i32_upper_bound() {
        assert_eq!(my_atoi("20000000000000000000".to_owned()), i32::MAX);
    }
}

fn id<T>(v: T) -> Parser<T>
    where
        T: Copy + 'static
{
    Box::new(move |s: String| {
        Ok((v, s))
    })
}

fn map<T1, T2>(f: impl Fn(T1) -> T2 + 'static, p: Parser<T1>) -> Parser<T2>
    where
        T1: 'static,
        T2: 'static,
{
    Box::new(move |s: String| {
        match p(s) {
            Ok((r, remain)) => Ok((f(r), remain)),
            Err(err) => Err(err)
        }
    })
}

fn sequence<T>(ps: Vec<Parser<T>>) -> ParserOnce<Vec<T>>
    where
        T: 'static
{
    Box::new(move |s: String| {
        ps.into_iter().fold(Ok((vec![], s)), |acc, p| {
            match acc {
                Ok((mut v, remain)) => match p(remain) {
                    Ok((t, remain)) => {
                        v.push(t);
                        Ok((v, remain))
                    },
                    Err(err) => Err(err)
                },
                err => err
            }
        })
    })
}

//leetcode submit region begin(Prohibit modification and deletion)
type ErrMsg = String;
type ParseResult<T> = Result<(T, String), (ErrMsg, String)>;
type Parser<T> = Box<dyn Fn(String) -> ParseResult<T>>;
type ParserOnce<T> = Box<dyn FnOnce(String) -> ParseResult<T>>;

fn p_char(c: char) -> Parser<char> {
    Box::new(move |s: String| {
        if s.starts_with(c) {
            let remain: String = s[1..].to_string();
            Ok((c, remain))
        } else {
            Err((format!("not found '{}'", c), s))
        }
    })
}

fn and_then<T1, T2>(p1: ParserOnce<T1>, p2: ParserOnce<T2>) -> ParserOnce<(T1, T2)>
where
    T1: 'static,
    T2: 'static,
{
    Box::new(move |s: String| {
        match p1(s) {
            Ok((r1, remain)) => match p2(remain) {
                Ok((r2, remain)) => Ok(((r1, r2), remain)),
                Err(msg) => Err(msg)
            }
            Err(msg) => Err(msg)
        }
    })
}

fn or_else<T>(p1: Parser<T>, p2: Parser<T>) -> Parser<T>
where
    T: 'static
{
    Box::new(move |s: String| {
        match p1(s) {
            r1 @ Ok(_) => r1,
            Err((_, s)) => p2(s)
        }
    })
}

fn any<T>(ps: Vec<Parser<T>>) -> Parser<T>
where
    T: 'static
{
    ps.into_iter().reduce(or_else).unwrap()
    // let mut p = ps.remove(0);
    // for q in ps.into_iter() {
    //     p = or_else(p, q);
    // }
    // p
}

fn any_of_chars(chars: impl Iterator<Item=char>) -> Parser<char> {
    any(chars.map(p_char).collect())
}

fn maybe<T>(p: Parser<T>) -> Parser<Option<T>>
where
    T: 'static
{
    Box::new(move |s: String| {
        match p(s) {
            Ok((r, remain)) => Ok((Some(r), remain)),
            Err((_, s)) => Ok((None, s))
        }
    })
}

fn many<T>(p: Parser<T>) -> ParserOnce<Vec<T>>
where
    T: 'static
{
    Box::new(|s: String| {
        match p(s) {
            Err((_, s)) => Ok((vec![], s)),
            Ok((first, remain)) => {
                let (mut subsequences, remain2) = many(p)(remain).unwrap();
                subsequences.insert(0, first);
                Ok((subsequences, remain2))
            }
        }
    })
}

fn once<T>(p: Parser<T>) -> ParserOnce<T>
where
    T: 'static
{
    Box::new(move |s| p(s))
}


pub fn my_atoi(s: String) -> i32 {
    let space = p_char(' ');
    let spaces = many(space);
    let sign_opt = maybe(or_else(p_char('-'), p_char('+')));
    let digits = many(any_of_chars('0' ..= '9'));
    let int = and_then(spaces, and_then(once(sign_opt), digits));
    match int(s) {
        Ok(((_, (sign, mut numbers)), _)) => {
            if numbers.len() > 0 {
                let is_negative = match sign {
                    Some('-') => true,
                    _ => false
                };
                if is_negative {
                    numbers.insert(0, '-');
                }
                let number_str: String = numbers.into_iter().collect();
                match number_str.parse::<i32>() {
                    Ok(n) => n,
                    _ => if is_negative { i32::MIN } else { i32::MAX }
                }
            } else {
                0
            }
        },
        _ => 0
    }
}
//leetcode submit region end(Prohibit modification and deletion)

// fn flatten<T1, T2, FT1toT2>(p: Parser<FT1toT2>) -> Box<dyn FnOnce(Parser<T1>) -> Parser<T2>>
// where
//     T1: 'static,
//     T2: 'static,
//     FT1toT2: Fn(T1) -> T2 + 'static,
// {
//     Box::new(|p1: Parser<T1>| {
//         map(|(t1, t1_to_t2)| t1_to_t2(t1), and_then(p1, p))
//     })
// }
//
// fn lift2<T1, T2, T3, F>(f: F, p_t1: Parser<T1>, p_t2: Parser<T2>) -> Parser<T3>
// where
//     T1: 'static,
//     T2: 'static,
//     T3: 'static,
//     F: Fn(T1) -> (Box<dyn Fn(T2) -> T3>) + Copy + 'static,
// {
//     let p_f = id(f);
//     let p_t2_to_t3 = flatten(p_f)(p_t1);
//     flatten(p_t2_to_t3)(p_t2)
// }
