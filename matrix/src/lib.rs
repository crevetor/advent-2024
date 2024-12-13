use anyhow::{bail, Result};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    pub contents: Vec<Vec<T>>,
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.contents {
            for col in row {
                write!(f, "{col}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Clone> FromIterator<Vec<T>> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Matrix {
            contents: iter.into_iter().collect(),
        }
    }
}

impl<T: Clone + PartialEq> Matrix<T> {
    pub fn new(c: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            contents: c.clone(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<T> {
        if y >= self.contents.len() || x >= self.contents[0].len() {
            bail!("index out of bounds");
        }

        Ok(self.contents[y][x].clone())
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T> {
        if y >= self.contents.len() || x >= self.contents[0].len() {
            bail!("index out of bounds");
        }

        Ok(&mut self.contents[y][x])
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.contents[y][x] = val;
    }

    pub fn num_rows(&self) -> usize {
        self.contents.len()
    }

    pub fn num_cols(&self) -> usize {
        self.contents[0].len()
    }

    pub fn row(&self, idx: usize) -> Result<Vec<T>> {
        if idx >= self.contents.len() {
            bail!("Index out of bounds");
        }

        Ok(self.contents[idx].clone())
    }

    pub fn col(&self, idx: usize) -> Result<Vec<T>> {
        if idx >= self.contents[0].len() {
            bail!("Index out of bounds");
        }

        Ok(self.contents.iter().map(|x| x[idx].clone()).collect())
    }

    pub fn diag(&self, start: [usize; 2], dir: [isize; 2]) -> Result<Vec<T>> {
        if start[0] >= self.num_cols() || start[1] >= self.num_rows() {
            bail!("Start index out of bounds");
        }

        let xs: Vec<usize> = if dir[0] == -1 {
            (0..=start[0]).rev().collect()
        } else {
            (start[0]..self.num_cols()).collect()
        };
        let ys: Vec<usize> = if dir[1] == -1 {
            (0..=start[1]).rev().collect()
        } else {
            (start[1]..self.num_rows()).collect()
        };

        Ok(xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| self.contents[*y][*x].clone())
            .collect())
    }

    pub fn insert_row(&mut self, idx: usize, content: Vec<T>) {
        self.contents.insert(idx, content.clone());
    }

    pub fn insert_col(&mut self, idx: usize, content: Vec<T>) {
        for (i, val) in content.iter().enumerate() {
            self.contents[i].insert(idx, val.clone());
        }
    }

    pub fn cols(&self) -> impl DoubleEndedIterator<Item = Vec<T>> + '_ {
        (0..self.contents[0].len()).map(|x| {
            self.contents
                .iter()
                .map(|row| row[x].clone())
                .collect::<Vec<T>>()
        })
    }

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = Vec<T>> + '_ {
        self.contents.iter().cloned()
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<([usize; 2], T)> {
        let mut ret = Vec::new();
        if x > 0 {
            ret.push(([x - 1, y], self.get(x - 1, y).unwrap()));
        }
        if x < self.num_cols() - 1 {
            ret.push(([x + 1, y], self.get(x + 1, y).unwrap()));
        }
        if y > 0 {
            ret.push(([x, y - 1], self.get(x, y - 1).unwrap()));
        }
        if y < self.num_rows() - 1 {
            ret.push(([x, y + 1], self.get(x, y + 1).unwrap()));
        }
        ret
    }

    pub fn get_sub_matrix(&self, x: usize, y: usize, size: [usize; 2]) -> Result<Matrix<T>> {
        if size[0] % 2 == 0 || size[1] % 2 == 0 {
            bail!("Matrix sizes must not be even numbers.");
        }

        if x < size[0] / 2
            || x > self.num_cols() - 1 - (size[0] / 2)
            || y < size[1] / 2
            || y > self.num_rows() - 1 - (size[1] / 2)
        {
            bail!(
                "A matrix of {} by {} cannot fit at ({x}, {y})",
                size[0],
                size[1]
            );
        }

        Ok(Matrix::from_iter(
            (y - (size[1] / 2)..=y + (size[1] / 2))
                .map(|y| self.row(y).unwrap()[x - (size[0] / 2)..=x + (size[0] / 2)].to_vec()),
        ))
    }

    pub fn get_neighbors_wraparound(&self, x: i32, y: i32) -> Vec<([i32; 2], T)> {
        let coords = [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]];
        let mut ret = Vec::new();
        if x == -10 {
            println!("");
        }
        for coord in coords {
            let mut inside_coord = coord;
            if coord[0] <= -i32::try_from(self.num_cols()).unwrap() {
                inside_coord[0] = i32::try_from(self.num_cols()).unwrap() - 1
                    + (coord[0] % (i32::try_from(self.num_cols()).unwrap()));
            } else if coord[0] < 0 {
                inside_coord[0] = i32::try_from(self.num_cols()).unwrap() + coord[0];
            } else if coord[0] >= self.num_cols().try_into().unwrap() {
                inside_coord[0] = (coord[0]) % (i32::try_from(self.num_cols()).unwrap());
            }

            if coord[1] <= -i32::try_from(self.num_rows()).unwrap() {
                inside_coord[1] = (i32::try_from(self.num_rows()).unwrap()) - 1
                    + (coord[1] % (i32::try_from(self.num_rows()).unwrap()));
            } else if coord[1] < 0 {
                inside_coord[1] = i32::try_from(self.num_cols()).unwrap() + coord[1];
            } else if coord[1] >= self.num_rows().try_into().unwrap() {
                inside_coord[1] = (coord[1]) % (i32::try_from(self.num_rows()).unwrap());
            }
            ret.push((
                coord,
                self.get(
                    inside_coord[0].try_into().unwrap(),
                    inside_coord[1].try_into().unwrap(),
                )
                .unwrap(),
            ));
        }
        ret
    }

    pub fn find(&self, needle: T) -> Option<[usize; 2]> {
        for (y, row) in self.rows().enumerate() {
            if let Some(x) = row.iter().position(|x| x == &needle) {
                return Some([x, y]);
            }
        }
        None
    }

    pub fn find_all(&self, needle: T) -> Result<Vec<[usize; 2]>> {
        let mut ret = Vec::new();
        for y in 0..self.num_rows() {
            for x in 0..self.num_cols() {
                if self.get(x, y)? == needle {
                    ret.push([x, y]);
                }
            }
        }
        Ok(ret)
    }
}
