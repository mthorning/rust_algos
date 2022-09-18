pub fn binary_search(needle: i32, haystack: &[i32]) -> i32 {
    if haystack.len() == 1 {
        return match haystack[0] == needle {
            true => haystack[0],
            false => -1,
        };
    }

    let mid_idx = haystack.len() / 2;

    let value = haystack[mid_idx];
    return match needle == value {
        true => value,
        false if value > needle => binary_search(needle, &haystack[..mid_idx]),
        false if value < needle => binary_search(needle, &haystack[mid_idx..]),
        false => -1,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    fn get_haystack() -> Vec<i32> {
        let mut haystack = Vec::new();
        for number in 0..10 {
            haystack.push(number);
        }
        haystack
    }

    #[test]
    fn it_returns_the_found_value() {
        let mut rng = thread_rng();
        let needle = rng.gen_range(0..10);

        assert_eq!(binary_search(needle, &get_haystack().as_slice()), needle);
    }

    #[test]
    fn it_returns_negative_one_if_not_found() {
        assert_eq!(binary_search(11, &get_haystack().as_slice()), -1);
    }

    #[test]
    fn bottom_bound() {
        assert_eq!(binary_search(0, &get_haystack().as_slice()), 0);
    }

    #[test]
    fn top_bound() {
        assert_eq!(binary_search(9, &get_haystack().as_slice()), 9);
    }
}
