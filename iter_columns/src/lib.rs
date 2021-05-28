mod iter;
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_into_vec() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(test_data.into_iter().columns().collect::<Vec<_>>(), [[1, 4], [2, 5], [
            3, 6
        ]]);
    }

    #[test]
    fn test_into_vec_options() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        assert_eq!(test_data.into_iter().columns_options().collect::<Vec<_>>(), [
            [Some(1), Some(4)],
            [Some(2), Some(5)],
            [None, Some(6)]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    #[allow(array_into_iter)]
    fn test_into_array() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(test_data.into_iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_1() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(test_data.iter().columns().collect::<Vec<_>>(), [[&1, &4], [&2, &5], [
            &3, &6
        ]]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_2() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_3() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_4() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_5() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_6() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_7() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_8() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_9() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[cfg(not(feature = "no_array"))]
    fn test_array_borrowed_10() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_1() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(test_data.iter().columns().collect::<Vec<_>>(), [[&1, &4], [&2, &5], [
            &3, &6
        ]]);
    }

    #[test]
    fn test_vec_borrowed_2() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_3() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_4() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_5() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_6() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_7() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_8() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_9() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_10() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_1() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        assert_eq!(test_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_2() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_3() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_4() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_5() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_6() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_7() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_8() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_9() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_borrowed_options_10() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_1() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.into_iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_2() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_3() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_4() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_5() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_6() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_7() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_8() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_9() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_10() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_1() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.into_iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_2() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_3() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_4() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_5() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_6() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_7() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_8() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_9() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_borrowed_options_10() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        let borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }
}
