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
    fn test_into_vec_backwards() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut iter = test_data.into_iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [1, 4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [3, 6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [2, 5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
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
    fn test_into_vec_options_backwards() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let mut iter = test_data.into_iter().columns_options();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [Some(1), Some(4)]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [None, Some(6)]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [Some(2), Some(5)]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    #[allow(unknown_lints)]
    #[allow(array_into_iter)]
    fn test_into_array_old() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(test_data.into_iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    #[allow(unknown_lints)]
    #[allow(array_into_iter)]
    fn test_into_array_old_backwards() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let mut iter = test_data.into_iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    #[cfg(feature = "array_into_iter")]
    fn test_into_array_new() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(std::array::IntoIter::new(test_data).columns().collect::<Vec<_>>(), [
            [1, 4],
            [2, 5],
            [3, 6]
        ]);
    }

    #[test]
    #[cfg(feature = "array_into_iter")]
    fn test_into_array_new_backwards() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let mut iter = std::array::IntoIter::new(test_data).columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [1, 4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [3, 6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [2, 5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_array_borrowed_1() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(test_data.iter().columns().collect::<Vec<_>>(), [[&1, &4], [&2, &5], [
            &3, &6
        ]]);
    }

    #[test]
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
    fn test_array_borrowed_backwards() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let mut iter = test_data.iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
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
    fn test_vec_borrowed_backwards() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut iter = test_data.iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
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
    fn test_vec_borrowed_options_backwards() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let mut iter = test_data.iter().columns_options();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [Some(&1), Some(&4)]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [None, Some(&6)]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [Some(&2), Some(&5)]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
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
    fn test_slice_borrowed_backwards() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let mut iter = borrowed_data.into_iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
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
    fn test_slice_borrowed_options_backwards() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let mut iter = borrowed_data.into_iter().columns_options();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [Some(&1), Some(&4)]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [None, Some(&6)]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [Some(&2), Some(&5)]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    // Mutable
    #[test]
    fn test_array_mut_borrowed_1() {
        let mut test_data = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(test_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_array_mut_borrowed_2() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let mut borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_array_mut_borrowed_3() {
        let test_data = [[1, 2, 3], [4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let mut borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_array_mut_borrowed_backwards() {
        let mut test_data = [[1, 2, 3], [4, 5, 6]];
        let mut iter = test_data.iter_mut().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_vec_mut_borrowed_1() {
        let mut test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(test_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_2() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_3() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let mut borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_backwards() {
        let mut test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut iter = test_data.iter_mut().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_vec_mut_borrowed_options_1() {
        let mut test_data = vec![vec![1, 2], vec![4, 5, 6]];
        assert_eq!(test_data.iter_mut().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_options_2() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let mut borrowed_data = vec![&test_data[0], &test_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_options_3() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![&test_data[0], &test_data[1]];
        let mut borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_vec_mut_borrowed_options_backwards() {
        let mut test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let mut iter = test_data.iter_mut().columns_options();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [Some(&1), Some(&4)]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [None, Some(&6)]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [Some(&2), Some(&5)]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_slice_mut_borrowed_1() {
        let mut row_1 = vec![1, 2, 3];
        let mut row_2 = vec![4, 5, 6];
        let borrowed_data = vec![row_1.as_mut_slice(), row_2.as_mut_slice()];
        assert_eq!(borrowed_data.into_iter().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_2() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_3() {
        let test_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let mut borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns().collect::<Vec<_>>(), [
            [&1, &4],
            [&2, &5],
            [&3, &6]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_backwards() {
        let mut row_1 = vec![1, 2, 3];
        let mut row_2 = vec![4, 5, 6];
        let borrowed_data = vec![row_1.as_mut_slice(), row_2.as_mut_slice()];
        let mut iter = borrowed_data.into_iter().columns();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [&1, &4]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [&3, &6]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [&2, &5]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_slice_mut_borrowed_options_1() {
        let mut row_1 = vec![1, 2];
        let mut row_2 = vec![4, 5, 6];
        let borrowed_data = vec![row_1.as_mut_slice(), row_2.as_mut_slice()];
        assert_eq!(borrowed_data.into_iter().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_options_2() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let mut borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        assert_eq!(borrowed_data.iter_mut().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_options_3() {
        let test_data = vec![vec![1, 2], vec![4, 5, 6]];
        let borrowed_data = vec![test_data[0].as_slice(), test_data[1].as_slice()];
        let mut borrowed_data = vec![&borrowed_data[0], &borrowed_data[1]];
        assert_eq!(borrowed_data.iter_mut().columns_options().collect::<Vec<_>>(), [
            [Some(&1), Some(&4)],
            [Some(&2), Some(&5)],
            [None, Some(&6)]
        ]);
    }

    #[test]
    fn test_slice_mut_borrowed_options_backwards() {
        let mut row_1 = vec![1, 2];
        let mut row_2 = vec![4, 5, 6];
        let borrowed_data = vec![row_1.as_mut_slice(), row_2.as_mut_slice()];
        let mut iter = borrowed_data.into_iter().columns_options();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap(), [Some(&1), Some(&4)]);
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next_back().unwrap(), [None, Some(&6)]);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), [Some(&2), Some(&5)]);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next_back(), None);
    }
}
