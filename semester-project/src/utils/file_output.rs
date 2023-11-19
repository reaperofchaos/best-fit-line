use std::fs;
use inputlibrary::TemperatureLine;
use crate::linefunctions;
use crate::matrix_operations;
#[path = "tests/file_output_tests.rs"] mod file_output_tests;

/// Creates a formatted line string using a start
/// interval number, an end interval number,
/// a y intercept float and a slope float
/// 
/// Description.
/// 
/// * `start` - start time
/// * `end` - end time
/// * yintercept - float
/// * slope - float
/// 
/// Return.
/// * void
pub fn format_output(start: u64, end: u64, yintercept: f64, slope: f64, method: &str)->String{
    format!("{start} <= x <=\t{end} ; y = \t {yintercept:.4} +\t {slope:.4} x ; {method}")
}


/// Creates an x matrix from the temperature reading
/// time intervals
/// Description.
/// 
/// * `data` - vector of temperature readings
/// 
/// Return.
/// * Vec<Vec<f64>
pub fn get_x_matrix(data: &Vec<TemperatureLine>)->Vec<Vec<f64>>{
    return data.iter().map(|t| vec!(1.0, t.time_step as f64)).collect();
}

/// Creates a y matrix from the temperature readings
/// measurements
/// 
/// Description.
/// 
/// * `data` - vector of temperature readings
/// 
/// Return.
/// * Vec<Vec<f64>
pub fn get_y_matrix(data: &Vec<TemperatureLine>)->Vec<Vec<f64>>{
    let mut y_values: Vec<Vec<f64>> = vec![]; 

    let readings: Vec<Vec<f64>> = data.iter().map(|f| f.readings.clone()).collect();

    for(_rpos, r) in readings.iter().enumerate(){
        for (col_pos, col) in r.iter().enumerate(){
            if y_values.is_empty() || y_values.len() <= col_pos {
                let core_y_vector: Vec<f64> = vec![*col];
                y_values.push(core_y_vector);
            }else{
                y_values[col_pos].push(*col);
            }
        }
    }

    return y_values; 
}

/// Creates an x transpose matrix from the temperature readings
/// measurements
/// 
/// Description.
/// 
/// * `data` - vector of temperature readings
/// 
/// Return.
/// * Vec<Vec<f64>
pub fn get_x_transpose_matrix(data: &Vec<TemperatureLine>)->Vec<Vec<f64>>{
    let x = get_x_matrix(&data); 

    return matrix_operations::get_transpose(x);
}



/// converts the temperature line vector into 
/// a vector of formatted string vectors that
/// contain the calculated piecewise functions
/// as formatted strings
/// 
/// Description.
/// 
/// * `data` - vector of formatted strings
/// 
/// Return.
/// * a vector of string vectors
pub fn format_readings(data: Vec<TemperatureLine>)->Vec<Vec<String>>{
    let mut lines: Vec<Vec<String>> = vec![];

    for (tpos, t) in data.iter().enumerate() {
        if tpos < data.len() - 1{
            println!("Core {tpos}");
        }
        for(rpos, _r) in t.readings.iter().enumerate(){
            if tpos < data.len() -1 {
                let start = data[tpos].time_step;
                let end = data[tpos + 1].time_step;
                let slope = linefunctions::calculate_slope(end as f64, data[tpos + 1].readings[rpos] as f64, start as f64, data[tpos].readings[rpos] as f64);
                let yintercept = linefunctions::calculate_y_intercept(start as f64, t.readings[rpos] as f64, slope);
                let line = format_output(start, end, yintercept, slope, "interpolation");
                println!("{line}");
                if lines.is_empty() || lines.len() -1 < rpos{
                    let core_line: Vec<String> = vec![line];
                    lines.push(core_line)
                }else{
                    lines[rpos].push(line);
                }
            }
        }
        
    }

    lines.push(vec!(least_squares(&data)));
    return lines;
}


pub fn least_squares(data: &Vec<TemperatureLine>)->String{
    let x = get_x_matrix(&data);
    let y = get_y_matrix(&data);
    let x_t = get_x_transpose_matrix(&data);
    println!("x, {:?}", x);
    println!("y, {:?}", y);
    println!("xt, {:?}", x_t);

    let x_t_x = matrix_operations::matrix_multiplication(x_t.clone(), x.clone());
    let x_t_y = matrix_operations::matrix_multiplication(x_t.clone(), y.clone());
    println!("xtx, {:?}", x_t_x);
    println!("xty, {:?}", x_t_y);
    let augmented: Vec<Vec<f64>> = matrix_operations::build_augmented_matrix(x_t_x.clone(), x_t_y.clone());
    let reduced_row_echeolon: Vec<Vec<f64>> = matrix_operations::convert_matrix_to_reduced_row_echelon_form(augmented);
    
    if reduced_row_echeolon.len() > 0 && x.len() > 0 && x[0].len()>0{
        if reduced_row_echeolon[0].len()>1 {
            let start = x[0][0];
            let end = x[x.len() - 1][0]; 
            let y_intercept = reduced_row_echeolon[reduced_row_echeolon.len()-1][0];
            let slope = reduced_row_echeolon[reduced_row_echeolon.len()-1][1];
            let line = format_output(start as u64, end as u64, y_intercept, slope, "least-squares");
            return line;
        }

    }
    return "".to_string(); 

}
/// Creates a text file using a vector
/// of strings and a string to name the file
/// 
/// Description.
/// 
/// * `name` - file name
/// * `data` - vector of formatted strings
/// 
/// Return.
/// * void
pub fn create_file(name: &str, data: &[String]){
    let mut file_contents: String = "".to_owned();
    for line in data.iter(){
        let with_line_breaks = format!("{line}\n");
        file_contents.push_str(&with_line_breaks);
    }

    let _ = fs::write(name, file_contents);
}

/// Creates files from a vector
/// of line vectors and a base name
/// 
/// Description.
/// 
/// * `base_name` - original file name
/// * `lines` - array of formatted strings to write to a file
/// 
/// Return.
/// * void
pub fn create_files(base_name: &str, lines: Vec<Vec<String>>){
    for (lpos, l) in lines.iter().enumerate() {
        let core_count = if lpos < 10 { format!("0{lpos}")} else{format!("{lpos}")};
        let file_name = format!("{base_name}-core-{core_count}.txt");
        create_file(&file_name, l);
    }
}

/// Formats the temperature input
/// into linear piecewise data and
/// stores the output in files for each
/// column
/// 
/// Description.
/// 
/// * `data` - vector of TemperatureLine objects
/// * `base_name` - original file name
/// 
/// Return.
/// * void
pub fn write_output(data: Vec<TemperatureLine>, base_name: &str){
    let piece_wise_values = format_readings(data);
    create_files(base_name, piece_wise_values);
}