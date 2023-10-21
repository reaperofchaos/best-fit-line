#[path = "tests/line_functions_tests.rs"] mod line_functions_tests;

/// calculates slope with two provided points
/// 
/// Description.
/// 
/// * `x1` - first point's x coordinate
/// * `y1` - first point's y coordinate
/// * `x2` - second point's x coordinate
/// * `y2` - second point's y coordinate
/// 
/// Return.
/// * slope of line containing both points
pub fn calculate_slope(x1: f32, y1: f32, x2: f32, y2: f32) -> f32{
    if x2 - x1 != 0.0{
        return (y2 - y1)/(x2 - x1);
    }

    return 0.0;
}

/// calculates the y intercept with an
/// a point's x coordinate and y coordinate and a 
/// slope
/// 
/// Description.
/// 
/// * `x1` - point's x coordinate
/// * `y1` - point's y coordinate
/// * `slope` - slope of line
/// 
/// Return.
/// * y intercept of line with the point and slope
pub fn calculate_y_intercept(x1: f32, y1: f32, m: f32 )->f32{
    return y1 - m*x1;
}