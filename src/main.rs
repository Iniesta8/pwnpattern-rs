const MAX_LEN: usize = 16_348;

fn gen_pattern(len: usize) -> Option<String> {
    if len > MAX_LEN {
        eprintln!("Given length not supported");
        return None;
    }

    let mut pattern = String::new();
    for upper in "ABCDEFGHIJKLMNOPQRSTUVXYZ".chars() {
        for lower in "abcdefghijklmnopqrstuvwxyz".chars() {
            for digit in "0123456789".chars() {
                if pattern.len() < len {
                    pattern.push(upper);
                    pattern.push(lower);
                    pattern.push(digit);
                } else {
                    return Some(pattern[..len].to_string());
                }
            }
        }
    }
    Some(pattern)
}

fn find_pattern(pattern: String) -> Option<usize> {
    let mut needle = pattern.clone();

    // Typically, the function is called with a little-endian coded
    // hexadecimal number. Therefore, we decode the coding below and
    // reverse the search pattern.
    if pattern.starts_with("0x") {
        needle = pattern[2..].to_string();
        let mut needle_vec = hex::decode(needle).unwrap();
        needle_vec = needle_vec.iter().copied().rev().collect();
        needle = String::from_utf8(needle_vec).unwrap();
    }

    let mut haystack = String::new();
    for upper in "ABCDEFGHIJKLMNOPQRSTUVXYZ".chars() {
        for lower in "abcdefghijklmnopqrstuvwxyz".chars() {
            for digit in "0123456789".chars() {
                haystack.push(upper);
                haystack.push(lower);
                haystack.push(digit);
                match haystack.find(&needle[..]) {
                    Some(idx) => {
                        println!("Found at index {}", idx);
                        return Some(idx);
                    }
                    None => continue,
                }
            }
        }
    }
    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_pattern() {
        assert_eq!(gen_pattern(23), Some("Aa0Aa1Aa2Aa3Aa4Aa5Aa6Aa".to_string()));
        assert_ne!(gen_pattern(20), Some("Aa1Aa2Aa3Aa4Aa5Aa6Aa".to_string()));
        assert_eq!(gen_pattern(0), Some("".to_string()));
        assert_eq!(gen_pattern(MAX_LEN + 1), None);
    }

    #[test]
    fn test_find_pattern() {
        assert_eq!(find_pattern("Aa5".to_string()), Some(15));
        assert_eq!(find_pattern("0x42346642".to_string()), Some(942));
        assert_eq!(find_pattern("423642".to_string()), None);
    }
}
