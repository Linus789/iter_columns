use std::collections::HashMap;
use iter_columns_derive::IterColumns;
#[cfg(not(feature = "no_array"))]
use iter_columns_derive::IterColumnsArray;

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
}

pub trait ColExtIntoIterVec<T>: Iterator<Item = Vec<T>> {
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

impl<T, I: Iterator<Item = Vec<T>>> ColExtIntoIterVec<T> for I {}

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
        let column = self
            .arr
            .iter_mut()
            .map(move |row| {
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
}

// into_iter().columns() for arrays
#[cfg(not(feature = "no_array"))]
pub struct ColumnsIntoIterArray<T, const N: usize> {
    arr: Vec<HashMap<usize, T>>,
    len: usize,
    index: usize,
}

#[cfg(not(feature = "no_array"))]
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
}

#[cfg(not(feature = "no_array"))]
pub trait ColExtIntoIterArray<T, const N: usize>: Iterator<Item = [T; N]> {
    fn columns(self) -> ColumnsIntoIterArray<T, N>
    where
        Self: Sized,
    {
        let arr: Vec<HashMap<usize, T>> = self.map(|x| std::array::IntoIter::new(x).enumerate().collect()).collect();
        let len = N;

        ColumnsIntoIterArray { arr, len, index: 0 }
    }
}

#[cfg(not(feature = "no_array"))]
impl<T, I: Iterator<Item = [T; N]>, const N: usize> ColExtIntoIterArray<T, N> for I {}

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed1<'a, T>(&'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed2<'a, T>(&'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed3<'a, T>(&'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed4<'a, T>(&'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed5<'a, T>(&'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed6<'a, T>(&'a &'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed7<'a, T>(&'a &'a &'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed8<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed9<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsVecBorrowed10<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a &'a &'a Vec<T>);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed1<'a, T>(&'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed2<'a, T>(&'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed3<'a, T>(&'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed4<'a, T>(&'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed5<'a, T>(&'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed6<'a, T>(&'a &'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed7<'a, T>(&'a &'a &'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed8<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed9<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[derive(IterColumns)]
struct IterColumnsSliceBorrowed10<'a, T>(&'a &'a &'a &'a &'a &'a &'a &'a &'a &'a [T]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed1<'a, T, const N: usize>(&'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed2<'a, T, const N: usize>(&'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed3<'a, T, const N: usize>(&'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed4<'a, T, const N: usize>(&'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed5<'a, T, const N: usize>(&'a &'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed6<'a, T, const N: usize>(&'a &'a &'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed7<'a, T, const N: usize>(&'a &'a &'a &'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed8<'a, T, const N: usize>(&'a &'a &'a &'a &'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed9<'a, T, const N: usize>(&'a &'a &'a &'a &'a &'a &'a &'a &'a [T; N]);

#[allow(dead_code)]
#[cfg(not(feature = "no_array"))]
#[derive(IterColumnsArray)]
struct IterColumnsArrayBorrowed10<'a, T, const N: usize>(&'a &'a &'a &'a &'a &'a &'a &'a &'a &'a [T; N]);
