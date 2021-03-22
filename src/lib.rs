use std::{cmp::Ordering, fmt::Debug, ops::Add, usize};

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

fn selection_sort<T: Copy + Ord>(list: Vec<T>) -> Vec<T> {
    let mut list = list;
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

fn factorial(n: u8) -> u32 {
    match n.cmp(&1) {
        Ordering::Equal | Ordering::Less => 1,
        _ => n as u32 * factorial(n-1)
    }
}

fn land_split(w: u32, h: u32) -> u32 {
    if w.min(h) == 0 || w == h {
        w.max(h)
    } else if w > h {
        land_split(w%h, h)
    } else {
        land_split(w, h%w)
    }
}

fn rec_sum<T: Clone + Add<Output = T>>(list: &[T]) -> T {
    match list.len() {
        1 => list[0].clone(),
        _ => list[0].clone() + rec_sum(&list[1..])
    }
}

fn rec_len<T>(list: &[T]) -> usize {
    match list {
        [] => 0,
        _ => 1 + rec_len(&list[1..])
    }
}

fn rec_max<T: Ord + Clone + Copy>(list: &[T]) -> T {
    match list.len() {
        1 => return list[0].clone(),
        2 => return list[0].max(list[1]).clone(),
        s @ _ => return rec_max(&list[..s/2]).max(rec_max(&list[s/2..])),
    }
}

fn rec_binary_search<T: Ord + Clone + Debug>(list: &[T], item: T) -> Result<usize, &'static str> {
    match list.len() {
        0 => return Err("Empty"),
        1 if list[0] == item => Ok(0),
        1 if list[0] != item => Err("Guess not found"),
        s @ _ => {
            let left = rec_binary_search(&list[..s/2], item.clone());
            let right = rec_binary_search(&list[s/2..], item.clone());

            // If left, we don't need to increment
            if right.is_ok() {
                // if right, we need to increment depending on where we split
                Ok(right.unwrap()+ s/2)
            } else {
                left
            }
        }
    }
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
            selection_sort(vec![2,3,4,1,7,4,5]),
            vec![1,2,3,4,4,5,7]
        )
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120)
    }

    #[test]
    fn test_land_split() {
        assert_eq!(land_split(1680, 640), 80)
    }

    #[test]
    fn test_rec_sum() {
        assert_eq!(rec_sum(&vec![1,2,3]), 6);
    }

    #[test]
    fn test_rec_len() {
        assert_eq!(rec_len(&vec![1,2,3,4]), 4)
    }

    #[test]
    fn test_rec_max() {
        assert_eq!(rec_max(&vec![1,5,4,2,3]), 5)
    }

    #[test]
    fn test_rec_binary_search() {
        let data = vec![1, 3, 5, 7, 9];
        let tests = vec![
            (3, Ok(1)),
            (-1, Err("Guess not found")),
            (2, Err("Guess not found")),
            (9, Ok(4)),
        ];
        for test in tests.iter() {
            assert_eq!(rec_binary_search(&data, test.0), test.1);    
        }
    }
}
