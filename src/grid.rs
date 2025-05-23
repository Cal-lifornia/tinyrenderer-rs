use crate::Vec3;

pub struct Grid<T, const W: usize, const H: usize> {
    array: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Grid<T, W, H> {
    pub fn get(&self, v: &Vec3<usize>) -> &T {
        &self.array[v.y()][v.x()]
    }

    /// How many elements are in the grid?
    pub fn size(&self) -> usize {
        W * H
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn width(&self) -> usize {
        W
    }

    /// Recompute each item in the grid using the provided setter function.
    /// The setter function takes the item's location in the grid and outputs
    /// the desired value of that item.
    pub fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Vec3<usize>) -> Option<T>,
        T: Send,
    {
        use rayon::prelude::*;
        self.array.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                if let Some(val) = setter(Vec3::new(x, y, 1)) {
                    *item = val;
                }
            }
        })
    }
}

impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [[Default::default(); W]; H],
        }
    }
}
