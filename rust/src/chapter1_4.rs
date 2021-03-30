use std::{cmp::Ordering, fmt::Debug, ops::Add, usize};
use rand::{Rng, thread_rng};

pub fn binary_search(l: &[i32], item: &i32) -> Result<i32, &'static str> {
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

pub fn selection_sort<T: Copy + Ord>(list: Vec<T>) -> Vec<T> {
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

pub fn factorial(n: u8) -> u32 {
    match n.cmp(&1) {
        Ordering::Equal | Ordering::Less => 1,
        _ => n as u32 * factorial(n-1)
    }
}

pub fn land_split(w: u32, h: u32) -> u32 {
    if w.min(h) == 0 || w == h {
        w.max(h)
    } else if w > h {
        land_split(w%h, h)
    } else {
        land_split(w, h%w)
    }
}

pub fn rec_sum<T: Clone + Add<Output = T>>(list: &[T]) -> T {
    match list.len() {
        1 => list[0].clone(),
        _ => list[0].clone() + rec_sum(&list[1..])
    }
}

pub fn rec_len<T>(list: &[T]) -> usize {
    match list {
        [] => 0,
        _ => 1 + rec_len(&list[1..])
    }
}

pub fn rec_max<T: Ord + Clone>(list: &[T]) -> T {
    match list.len() {
        1 => return list[0].clone(),
        s @ _ => rec_max(&list[..s/2]).max(rec_max(&list[s/2..])),
    }
}

pub fn rec_binary_search<T: Ord + Clone + Debug>(list: &[T], item: T) -> Result<usize, &'static str> {
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

pub fn quicksort<T: Ord + Clone + Debug>(list: &Vec<T>) -> Vec<T> {
    match list.len() {
        0 => vec![],
        1 => list.to_vec(),
        2 if list[0] >= list[1] => vec![list[1].clone(), list[0].clone()], 
        2 if list[0] < list[1] => list.to_vec(), 
        s @ _ => {
            let pivot_idx = thread_rng().gen_range(0..s);
            let pivot = list[pivot_idx].clone();
            let mut to_sort = list.clone();
            to_sort.remove(pivot_idx);
            let smaller = to_sort.iter().filter(|&i| i <= &pivot).cloned().collect::<Vec<_>>();
            let higher = to_sort.iter().filter(|&i| i > &pivot).cloned().collect::<Vec<_>>();
            [quicksort(&smaller).to_vec(), quicksort(&higher).to_vec()].join(&pivot)
        }
    }
}
