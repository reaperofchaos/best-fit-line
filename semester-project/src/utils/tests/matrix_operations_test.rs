#[cfg(test)]
mod matrix_operation_tests {
    use crate::matrix_operations;

    #[test]
    fn test_matrix_multiplication() {
        let test_row_1: Vec<f64> = vec![1.0, 1.0];
        let test_2_row_1: Vec<f64> = vec![1.0];
        let test_2_row_2: Vec<f64> = vec![1.0];

        let test_1: Vec<Vec<f64>> = vec![test_row_1];
        let test_2: Vec<Vec<f64>> = vec![test_2_row_1, test_2_row_2];

        let expected_row: Vec<f64> = vec![2.0];
        let expected: Vec<Vec<f64>> = vec![expected_row];

        assert_eq!(matrix_operations::matrix_multiplication(test_1, test_2), expected);
    }
    #[test]
    fn test_matrix_multiplication_2_x_2() {
        let test_row_1: Vec<f64> = vec![1.0, 1.0];
        let test_2_row_1: Vec<f64> = vec![1.0, 1.0];

        let lhs: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_1.clone()];
        let rhs: Vec<Vec<f64>> = vec![test_2_row_1.clone(), test_2_row_1.clone()];

        let expected_row: Vec<f64> = vec![2.0, 2.0];
        let expected: Vec<Vec<f64>> = vec![expected_row.clone(), expected_row.clone()];

        assert_eq!(matrix_operations::matrix_multiplication(lhs, rhs), expected);
    }

}
