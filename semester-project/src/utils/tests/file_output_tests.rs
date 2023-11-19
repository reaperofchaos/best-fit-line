
#[cfg(test)]
mod file_output_tests {
    use crate::file_output;

    #[test]
    fn test_format_string() {
        let start = 0;
        let end = 30;
        let yintercept = 1.0;
        let slope = 0.5;
        let method = "interpolation";
        let expected = format!("{start} <= x <=\t{end} ; y = \t {yintercept:.4} +\t {slope:.4} x ; {method}");
        
        assert_eq!(file_output::format_output(start, end, yintercept, slope, method), expected);
    }
}