struct SubrectangleQueries {
    values: Vec<Vec<i32>>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { values: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        // for r in row1..=row2 {
        // for c in col1..=col2 {
        // self.values[r as usize][c as usize] = new_value;
        // }
        // }

        for r in self
            .values
            .iter_mut()
            .skip(row1 as usize)
            .take((row2 - row1) as usize + 1)
        {
            for val in r
                .iter_mut()
                .skip(col1 as usize)
                .take((col2 - col1) as usize + 1)
            {
                *val = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.values[row as usize][col as usize]
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subrectangle_queries() {
        let mut subrectangle_queries = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);

        assert_eq!(subrectangle_queries.get_value(0, 2), 1);

        subrectangle_queries.update_subrectangle(0, 0, 3, 2, 5);

        assert_eq!(
            subrectangle_queries.values,
            vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]]
        );

        assert_eq!(subrectangle_queries.get_value(0, 2), 5);
        assert_eq!(subrectangle_queries.get_value(3, 1), 5);

        subrectangle_queries.update_subrectangle(3, 0, 3, 2, 10);

        assert_eq!(
            subrectangle_queries.values,
            vec![
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![10, 10, 10]
            ]
        );

        assert_eq!(subrectangle_queries.get_value(3, 1), 10);
        assert_eq!(subrectangle_queries.get_value(0, 2), 5);
    }
}
