use std::collections::HashMap;
use std::iter::FusedIterator;
use std::marker::PhantomData;

// into_iter().columns() for Vecs
pub struct ColumnsIntoIterVec<T> {
    arr: Vec<HashMap<usize, T>>,
    len: usize,
    index: usize,
}

impl<T> Iterator for ColumnsIntoIterVec<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.index;
        let column = self.arr.iter_mut().filter_map(move |row| row.remove(&index)).collect();

        self.index += 1;
        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_len = self.len - self.index;
        (remaining_len, Some(remaining_len))
    }
}

impl<T> DoubleEndedIterator for ColumnsIntoIterVec<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.len - 1;
        let column = self.arr.iter_mut().filter_map(move |row| row.remove(&index)).collect();

        self.len -= 1;
        Some(column)
    }
}

impl<T> ExactSizeIterator for ColumnsIntoIterVec<T> {}
impl<T> FusedIterator for ColumnsIntoIterVec<T> {}

pub struct ColumnsOptionsIntoIterVec<T> {
    arr: Vec<HashMap<usize, T>>,
    len: usize,
    index: usize,
}

impl<T> Iterator for ColumnsOptionsIntoIterVec<T> {
    type Item = Vec<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.index;
        let column = self.arr.iter_mut().map(move |row| row.remove(&index)).collect();

        self.index += 1;
        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_len = self.len - self.index;
        (remaining_len, Some(remaining_len))
    }
}

impl<T> DoubleEndedIterator for ColumnsOptionsIntoIterVec<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.len - 1;
        let column = self.arr.iter_mut().map(move |row| row.remove(&index)).collect();

        self.len -= 1;
        Some(column)
    }
}

impl<T> ExactSizeIterator for ColumnsOptionsIntoIterVec<T> {}
impl<T> FusedIterator for ColumnsOptionsIntoIterVec<T> {}

pub trait ColumnsIntoIterVecTrait<T>: Iterator<Item = Vec<T>> {
    fn columns(self) -> ColumnsIntoIterVec<T>
    where
        Self: Sized,
    {
        let arr: Vec<HashMap<usize, T>> = self.map(|x| x.into_iter().enumerate().collect()).collect();
        let len = arr.iter().map(|row| row.len()).max().unwrap_or(0);

        ColumnsIntoIterVec { arr, len, index: 0 }
    }

    fn columns_options(self) -> ColumnsOptionsIntoIterVec<T>
    where
        Self: Sized,
    {
        let arr: Vec<HashMap<usize, T>> = self.map(|x| x.into_iter().enumerate().collect()).collect();
        let len = arr.iter().map(|row| row.len()).max().unwrap_or(0);

        ColumnsOptionsIntoIterVec { arr, len, index: 0 }
    }
}

impl<T, I: Iterator<Item = Vec<T>>> ColumnsIntoIterVecTrait<T> for I {}

// into_iter().columns() for arrays
#[cfg(feature = "array_into_iter")]
pub struct ColumnsIntoIterArray<T, const N: usize> {
    arr: Vec<HashMap<usize, T>>,
    len: usize,
    index: usize,
}

#[cfg(feature = "array_into_iter")]
impl<T, const N: usize> Iterator for ColumnsIntoIterArray<T, N> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.index;
        let column = self
            .arr
            .iter_mut()
            .filter_map(move |row| {
                if let Some(cell) = row.remove(&index) {
                    Some(cell)
                } else {
                    None
                }
            })
            .collect();

        self.index += 1;
        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_len = self.len - self.index;
        (remaining_len, Some(remaining_len))
    }
}

#[cfg(feature = "array_into_iter")]
impl<T, const N: usize> DoubleEndedIterator for ColumnsIntoIterArray<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.len - 1;
        let column = self
            .arr
            .iter_mut()
            .filter_map(move |row| {
                if let Some(cell) = row.remove(&index) {
                    Some(cell)
                } else {
                    None
                }
            })
            .collect();

        self.len -= 1;
        Some(column)
    }
}

#[cfg(feature = "array_into_iter")]
impl<T, const N: usize> ExactSizeIterator for ColumnsIntoIterArray<T, N> {}
#[cfg(feature = "array_into_iter")]
impl<T, const N: usize> FusedIterator for ColumnsIntoIterArray<T, N> {}

#[cfg(feature = "array_into_iter")]
pub trait ColumnsIntoIterArrayTrait<T, const N: usize>: Iterator<Item = [T; N]> {
    fn columns(self) -> ColumnsIntoIterArray<T, N>
    where
        Self: Sized,
    {
        let arr: Vec<HashMap<usize, T>> = self
            .map(|x| std::array::IntoIter::new(x).enumerate().collect())
            .collect();
        let len = N;

        ColumnsIntoIterArray { arr, len, index: 0 }
    }
}

#[cfg(feature = "array_into_iter")]
impl<T, I: Iterator<Item = [T; N]>, const N: usize> ColumnsIntoIterArrayTrait<T, N> for I {}

// Borrowed Vecs, slices, arrays
pub trait SliceHelper<T> {
    fn helper_get(&self, index: usize) -> Option<&T>;
    fn helper_len(&self) -> usize;
}

impl<T, C: AsRef<[T]> + ?Sized> SliceHelper<T> for C {
    fn helper_get(&self, index: usize) -> Option<&T> {
        self.as_ref().get(index)
    }

    fn helper_len(&self) -> usize {
        self.as_ref().len()
    }
}

pub struct Columns<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> {
    arr: Vec<&'a C>,
    len: usize,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> Iterator for Columns<'a, T, C> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.index;
        let column = self.arr.iter().filter_map(move |row| row.helper_get(index)).collect();

        self.index += 1;
        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_len = self.len - self.index;
        (remaining_len, Some(remaining_len))
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> DoubleEndedIterator for Columns<'a, T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.len - 1;
        let column = self.arr.iter().filter_map(move |row| row.helper_get(index)).collect();

        self.len -= 1;
        Some(column)
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> ExactSizeIterator for Columns<'a, T, C> {}
impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> FusedIterator for Columns<'a, T, C> {}

pub struct ColumnsOptions<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> {
    arr: Vec<&'a C>,
    len: usize,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> Iterator for ColumnsOptions<'a, T, C> {
    type Item = Vec<Option<&'a T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.index;
        let column = self.arr.iter().map(move |row| row.helper_get(index)).collect();

        self.index += 1;
        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_len = self.len - self.index;
        (remaining_len, Some(remaining_len))
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> DoubleEndedIterator for ColumnsOptions<'a, T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let index = self.len - 1;
        let column = self.arr.iter().map(move |row| row.helper_get(index)).collect();

        self.len -= 1;
        Some(column)
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> ExactSizeIterator for ColumnsOptions<'a, T, C> {}
impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a> FusedIterator for ColumnsOptions<'a, T, C> {}

pub trait ColumnsTrait<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a>: Iterator<Item = &'a C> {
    fn columns(self) -> Columns<'a, T, C>
    where
        Self: Sized,
    {
        let arr: Vec<_> = self.collect();
        let len = arr.iter().map(|row| row.helper_len()).max().unwrap_or(0);

        Columns {
            arr,
            len,
            index: 0,
            _phantom: PhantomData,
        }
    }

    fn columns_options(self) -> ColumnsOptions<'a, T, C>
    where
        Self: Sized,
    {
        let arr: Vec<_> = self.collect();
        let len = arr.iter().map(|row| row.helper_len()).max().unwrap_or(0);

        ColumnsOptions {
            arr,
            len,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a, I: Iterator<Item = &'a C>> ColumnsTrait<'a, T, C> for I {}

pub trait ColumnsMutTrait<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a>: Iterator<Item = &'a mut C> {
    fn columns(self) -> Columns<'a, T, C>
    where
        Self: Sized,
    {
        let arr: Vec<_> = self.map(|x| &*x).collect();
        let len = arr.iter().map(|row| row.helper_len()).max().unwrap_or(0);

        Columns {
            arr,
            len,
            index: 0,
            _phantom: PhantomData,
        }
    }

    fn columns_options(self) -> ColumnsOptions<'a, T, C>
    where
        Self: Sized,
    {
        let arr: Vec<_> = self.map(|x| &*x).collect();
        let len = arr.iter().map(|row| row.helper_len()).max().unwrap_or(0);

        ColumnsOptions {
            arr,
            len,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, C: SliceHelper<T> + ?Sized + 'a, I: Iterator<Item = &'a mut C>> ColumnsMutTrait<'a, T, C> for I {}
