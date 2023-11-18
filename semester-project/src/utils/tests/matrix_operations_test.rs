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

    #[test]
    fn test_find_largest_row_by_col() {
        let test_row_1: Vec<f64> = vec![0.0, 1.0, 3.0];
        let test_row_2: Vec<f64> = vec![1.0, 2.0, 2.0];
        let test_row_3: Vec<f64> = vec![1.0, 3.0, 2.0];
        let test: Vec<Vec<f64>> = vec![test_row_1, test_row_2, test_row_3];

        assert_eq!(matrix_operations::find_largest_row_by_col(test.clone(), 0), 1);
        assert_eq!(matrix_operations::find_largest_row_by_col(test.clone(), 1), 2);
        assert_eq!(matrix_operations::find_largest_row_by_col(test.clone(), 2), 2);
    }

    #[test]
    fn test_row_swap() {
        let test_row_1: Vec<f64> = vec![0.0, 1.0, 3.0];
        let test_row_2: Vec<f64> = vec![1.0, 2.0, 2.0];
        let test_row_3: Vec<f64> = vec![1.0, 3.0, 2.0];
        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone(), test_row_3.clone()];
        let expected: Vec<Vec<f64>> = vec![test_row_3, test_row_2, test_row_1];

        assert_eq!(matrix_operations::swap_row(test, 0, 2), expected);
    }

        #[test]
    fn test_scale_row() {
        let test_row_1: Vec<f64> = vec![0.0, 1.0, 3.0];
        let test_row_2: Vec<f64> = vec![1.0, 2.0, 2.0];
        let test_row_3: Vec<f64> = vec![1.0, 3.0, 2.0];
        let scaled_test_row_3: Vec<f64> = vec![3.0, 9.0, 6.0];

        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone(), test_row_3.clone()];
        let expected: Vec<Vec<f64>> = vec![test_row_1, test_row_2, scaled_test_row_3];

        assert_eq!(matrix_operations::scale_row(test, 2, 3.0), expected);
    }

    #[test]
    fn test_eliminate_row() {
        let test_row_1: Vec<f64> = vec![1.0, 7.0, 55.0];
        let test_row_2: Vec<f64> = vec![11.0, 55.0, 385.0];
        let eliminated_row_2: Vec<f64> = vec![0.0, -22.0, -220.0];

        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone()];
        let expected: Vec<Vec<f64>> = vec![test_row_1, eliminated_row_2];

        assert_eq!(matrix_operations::eliminate_row(test, 0), expected);
    }

    #[test]
    fn test_back_solve_matrix() {
        let test_row_1: Vec<f64> = vec![1.0, 7.0, 55.0];
        let test_row_2: Vec<f64> = vec![0.0, 1.0, 10.0];
        let back_solve_row_1: Vec<f64> = vec![1.0, 0.0, -15.0];

        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone()];
        let expected: Vec<Vec<f64>> = vec![back_solve_row_1, test_row_2];

        assert_eq!(matrix_operations::back_solve_matrix(test), expected);
    }

    #[test]
    fn test_convert_matrix_to_row_echelon_form() {    
        let test_row_1: Vec<f64> = vec![11.0, 55.0, 385.0];
        let test_row_2: Vec<f64> = vec![55.0, 385.0, 3025.0];
        let solve_row_1: Vec<f64> = vec![1.0, 7.0, 55.0];
        let solve_row_2: Vec<f64> = vec![0.0, 1.0, 10.0];

        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone()];
        let expected: Vec<Vec<f64>> = vec![solve_row_1, solve_row_2];

        assert_eq!(matrix_operations::convert_matrix_to_row_echelon_form(test), expected);
    }

    #[test]
    fn test_convert_matrix_to_reduced_row_echelon_form() {    
        let test_row_1: Vec<f64> = vec![11.0, 55.0, 385.0];
        let test_row_2: Vec<f64> = vec![55.0, 385.0, 3025.0];
        let solve_row_1: Vec<f64> = vec![1.0, 0.0, -15.0];
        let solve_row_2: Vec<f64> = vec![0.0, 1.0, 10.0];

        let test: Vec<Vec<f64>> = vec![test_row_1.clone(), test_row_2.clone()];
        let expected: Vec<Vec<f64>> = vec![solve_row_1, solve_row_2];

        assert_eq!(matrix_operations::convert_matrix_to_reduced_row_echelon_form(test), expected);
    }

}
