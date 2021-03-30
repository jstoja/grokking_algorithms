mod chapter1_4;
mod chapter5_;
use chapter1_4::*;

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

    #[test]
    fn test_quicksort() {
        assert_eq!(
            quicksort(&vec![2,3,4,1,7,4,5]),
            vec![1,2,3,4,4,5,7]
        )
    }
}