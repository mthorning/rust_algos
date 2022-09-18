#![allow(dead_code)]

fn bubble_sort(input: &mut Vec<i32>){
    for i in 0..input.len() {
        for j in 0..input.len() - i - 1 {
            if input[j] > input[j + 1] {
                let temp = input[j + 1];
                input[j + 1] = input[j];
                input[j] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    fn get_randoms() -> Vec<i32> {
        let mut rng = thread_rng();
        let mut randoms = Vec::new();
        for _number in 0..10 {
            randoms.push(rng.gen_range(0..10))
        }
        randoms
    }

    #[test]
    fn it_works() {
        let mut input = get_randoms();

        let mut expected = input.clone();
        expected.sort();

        bubble_sort(&mut input);

        assert_eq!(input, expected);
    }
}
