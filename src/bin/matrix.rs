#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i64>>,
}

impl Matrix {
    // コンストラクタ
    fn new(rows: usize, cols: usize, fill: i64) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![vec![fill; cols]; rows],
        }
    }

    // 行列の乗算
    fn multiply(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.cols != other.rows {
            return Err("Number of columns of the first matrix must equal the number of rows of the second.".to_string());
        }

        let mut result = Matrix::new(self.rows, other.cols, 0);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        Ok(result)
    }

    // 行列の累乗
    fn power(&self, exponent: u64) -> Result<Matrix, String> {
        if self.rows != self.cols {
            return Err("Matrix must be square for exponentiation.".to_string());
        }

        // 単位行列
        let mut result = Matrix::identity(self.rows);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.multiply(&base)?;
            }
            base = base.multiply(&base)?;
            exp /= 2;
        }

        Ok(result)
    }

    // 単位行列の生成
    fn identity(size: usize) -> Matrix {
        let mut result = Matrix::new(size, size, 0);
        for i in 0..size {
            result.data[i][i] = 1;
        }

        result
    }

    // 行列の表示
    fn display(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }
}

fn multiple_matrix(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    // 行列Aの行数と列数
    let a_rows = a.len();
    let a_cols = a[0].len();

    // 行列Bの行数と列数
    let b_rows = b.len();
    let b_cols = b[0].len();

    // 行列Aの列数と行列Bの行数が一致している必要がある
    assert_eq!(a_cols, b_rows, "行列Aの列数と行列Bの行数が一致していません");

    // 結果行列の初期化
    let mut ret = vec![vec![0; b_cols]; a_rows];

    // 行列の掛け算
    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                ret[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_matrix() {
        // Test case 1
        let a1 = vec![vec![1, 2], vec![3, 4]];
        let b1 = vec![vec![5, 6], vec![7, 8]];
        let expected_result1 = vec![vec![19, 22], vec![43, 50]];
        let result1 = multiple_matrix(&a1, &b1);
        assert_eq!(result1, expected_result1);

        // Test case 2
        let a2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let b2 = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        let expected_result2 = vec![vec![30, 24, 18], vec![84, 69, 54], vec![138, 114, 90]];
        let result2 = multiple_matrix(&a2, &b2);
        assert_eq!(result2, expected_result2);
    }
}
