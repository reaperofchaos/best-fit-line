
#[cfg(test)]
mod line_output_tests {
    use crate::linefunctions;

    #[test]
    fn test_calculate_slope() {
        let x1 = 1.0;
        let x2 = 2.0;
        let y1 = 1.0;
        let y2 = 1.0;

        let expected = (y2 - y1) / (x2 - x1);
        
        assert_eq!(linefunctions::calculate_slope(x1, y1, x2, y2), expected);
    }
    #[test]
    fn test_calculate_y_intercept() {
        let x1 = 1.0;
        let y1 = 1.0;
        let slope = 0.5;
        let expected =  y1 - slope*x1;
        
        assert_eq!(linefunctions::calculate_y_intercept(x1, y1, slope), expected);
    }
}