use std::{cmp::Ordering, usize};

fn binary_search(l: &[i32], item: &i32) -> Result<i32, &'static str> {
    let mut low: i32 = 0;
    let mut high: i32 = (l.len() as i32) -1;
    let mut mid;
    let mut guess: i32;

    while low <= high {

        mid = (low + high)/2;
        guess = *l.get(mid as usize).unwrap();

        match guess.cmp(item) {
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Ok(mid as i32),
            Ordering::Less => low = mid + 1,
        }
    }
    Err("Guess not found")
}

fn selection_sort<T: Copy + Ord>(list: &mut Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::new();

    for _ in 0..list.len() {
        let mut lowest: usize = 0;
        for (i, item) in list.iter().enumerate() {
            if *item < list[lowest] {
                lowest = i
            }
        }
        sorted.push(list.remove(lowest));
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let data = vec![1, 3, 5, 7, 9];
        let tests = vec![
            (3, Ok(1)),
            (-1, Err("Guess not found")),
            (2, Err("Guess not found")),
            (9, Ok(4)),
        ];
        for test in tests.iter() {
            assert_eq!(binary_search(&data, &test.0), test.1);    
        }
        
    }

    #[test]
    fn test_selection_sort() {
        assert_eq!(
            selection_sort(vec![2,3,4,1,7,4,5].as_mut()),
            vec![1,2,3,4,4,5,7]
        )
    }
}
