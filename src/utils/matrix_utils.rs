use itertools::Itertools;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    // expressed in terms of column size
    width: usize,
}

impl<T> Matrix<T>
where
    T: Clone + PartialEq,
{
    pub fn new(data: Vec<T>, width: usize) -> Self {
        Matrix { data, width }
    }

    pub fn get_ele(&self, row: usize, col: usize) -> Option<&T> {
        if row > (self.width / (col + 1)) || col >= self.width {
            None
        } else {
            *&self.data.get(self.derive_index(row, col))
        }
    }

    pub fn get_ele_with_idx(&self, idx: usize) -> T {
        self.data[idx].clone()
    }

    pub fn get_indicies_of_ele(&self, ele: T) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_i, x)| **x == ele)
            .map(|(i, _x)| i)
            .collect()
    }

    pub fn derive_index(&self, row: usize, col: usize) -> usize {
        row * &self.width + col
    }

    pub fn derive_row_col(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    // abbreviated for get up down left right for neighbours given row and col
    pub fn get_udlr_neighbours_2d_vec(&self, row: usize, col: usize) -> Vec<(T, usize)> {
        let idx = *&self.derive_index(row, col);

        let left_offset = if idx % self.width == 0 {
            // exclude if at left side of arr
            -1_isize
        } else {
            -1 + (idx as isize)
        };
        let right_offset = if (idx + 1) % self.width == 0 {
            -1_isize
        } else {
            (idx + 1) as isize
        };
        let offsets: Vec<isize> = vec![
            -1 * (self.width as isize) + idx as isize,
            left_offset,
            right_offset,
            (idx + self.width) as isize,
        ];
        offsets
            .iter()
            .filter(|&&x| x >= 0 || x < self.data.len() as isize)
            .filter_map(|&x| {
                let safe_idx = x as usize;
                if let Some(ele) = self.data.get(safe_idx) {
                    return Some((ele.clone(), safe_idx));
                }
                None
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod utils_test {
    use super::Matrix;

    #[test]
    pub fn test_accessing_ele_happy_case() {
        // expected layout of grid is
        // [ 2 , 1 ]
        // [ 3 , 4 ]
        let grid_data = vec![2, 1, 3, 4];
        let mat = Matrix::new(grid_data, 2);

        assert_eq!(mat.get_ele(0, 0).unwrap(), &2);
        assert_eq!(mat.get_ele(0, 1).unwrap(), &1);
        assert_eq!(mat.get_ele(1, 0).unwrap(), &3);
        assert_eq!(mat.get_ele(1, 1).unwrap(), &4);
    }

    #[test]
    pub fn test_accessing_ele_oob() {
        // expected layout of grid is
        // [ 2 , 1 ]
        // [ 3 , 4 ]
        let grid_data = vec![2, 1, 3, 4];
        let mat = Matrix::new(grid_data, 2);

        assert_eq!(mat.get_ele(0, 2), None);
        assert_eq!(mat.get_ele(0, 3), None);
        assert_eq!(mat.get_ele(2, 0), None);
        assert_eq!(mat.get_ele(10, 0), None);
    }
    #[test]
    pub fn test_derive_index() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6];
        let mat = Matrix::new(grid_data, 3);

        assert_eq!(mat.derive_index(0, 2), 2);
        assert_eq!(mat.derive_index(0, 0), 0);
        // this is OOB but eh
        assert_eq!(mat.derive_index(2, 1), 7);
    }

    #[test]
    pub fn test_udlr_neighbours() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        // [ 8 , 9 , 7 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6, 8, 9, 7];
        let mat = Matrix::new(grid_data, 3);

        let results = mat.get_udlr_neighbours_2d_vec(1, 1);
        let expected = vec![(1, 1), (3, 3), (6, 5), (9, 7)];
        assert_eq!(results, expected);
    }

    #[test]
    pub fn test_udlr_neighbours_with_left_oob() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        // [ 8 , 9 , 7 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6, 8, 9, 7];
        let mat = Matrix::new(grid_data, 3);

        let results = mat.get_udlr_neighbours_2d_vec(1, 0);
        let expected = vec![(2, 0), (4, 4), (8, 6)];
        assert_eq!(results, expected);
    }

    #[test]
    pub fn test_udlr_neighbours_with_right_oob() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        // [ 8 , 9 , 7 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6, 8, 9, 7];
        let mat = Matrix::new(grid_data, 3);

        let results = mat.get_udlr_neighbours_2d_vec(1, 2);
        let expected = vec![(5, 2), (4, 4), (7, 8)];
        assert_eq!(results, expected);
    }

    #[test]
    pub fn test_udlr_neighbours_with_up_right_oob() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        // [ 8 , 9 , 7 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6, 8, 9, 7];
        let mat = Matrix::new(grid_data, 3);

        let results = mat.get_udlr_neighbours_2d_vec(0, 2);
        let expected = vec![(1, 1), (6, 5)];
        assert_eq!(results, expected);
    }

    #[test]
    pub fn test_udlr_neighbours_with_down_left_oob() {
        // expected layout of grid is
        // [ 2 , 1 , 5 ]
        // [ 3 , 4 , 6 ]
        // [ 8 , 9 , 7 ]
        let grid_data = vec![2, 1, 5, 3, 4, 6, 8, 9, 7];
        let mat = Matrix::new(grid_data, 3);

        let results = mat.get_udlr_neighbours_2d_vec(2, 0);
        let expected = vec![(3, 3), (9, 7)];
        assert_eq!(results, expected);
    }
}
