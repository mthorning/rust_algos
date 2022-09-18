#![allow(dead_code)]

fn quick_sort(to_sort: &mut Vec<i32>, lo: i32, hi: i32) {
    if lo >= hi {
        return;
    }
    let pivot_idx = partition(to_sort, lo, hi);

    quick_sort(to_sort, lo, pivot_idx -1);
    quick_sort(to_sort, pivot_idx + 1, hi);
}

fn partition(to_sort: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
    let pivot = to_sort[hi as usize];
    let mut pivot_idx = lo - 1;
 
    for i in lo..hi {
        if to_sort[i as usize] <= pivot {
            pivot_idx += 1;
            let temp = to_sort[i as usize];
            to_sort[i as usize] = to_sort[pivot_idx as usize];
            to_sort[pivot_idx as usize] = temp;
        }
    }

    pivot_idx += 1;
        
    to_sort[hi as usize] = to_sort[pivot_idx as usize];
    to_sort[pivot_idx as usize] = pivot;

    pivot_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_works() {
        let mut to_sort: Vec<i32> = vec![7, 2, 64, 38, 8, 1, 12, 6, 128, 3];
        
        let mut sorted = to_sort.clone();
        sorted.sort();

        let hi = (to_sort.len() - 1) as i32;

        quick_sort(&mut to_sort, 0, hi);
        assert_eq!(sorted, to_sort);
    }
}
