use std::fs;
use inputlibrary::TemperatureLine;
use crate::line_functions;
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
///
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
        }
        for(rpos, _r) in t.readings.iter().enumerate(){
            if tpos < data.len() -1 {
                let start = data[tpos].time_step;
                let end = data[tpos + 1].time_step;
                let slope = line_functions::calculate_slope(end as f64, data[tpos + 1].readings[rpos] as f64, start as f64, data[tpos].readings[rpos] as f64);
                let yintercept = line_functions::calculate_y_intercept(start as f64, t.readings[rpos] as f64, slope);
                let line = format_output(start, end, yintercept, slope, "interpolation");
                if lines.is_empty() || lines.len() -1 < rpos{
                    let core_line: Vec<String> = vec![line];
                    lines.push(core_line)
                }else{
                    lines[rpos].push(line);
                }
            }
        }
    }
    
    let least_squares_values = least_squares(&data);
    
    for (core, least_square_value) in least_squares_values.iter().enumerate(){
        lines[core].push(least_square_value.to_string())
    }

    return lines;
}


/// converts the temperature line vector into 
/// a vector of formatted string vectors that
/// contain the calculated least squares 
/// approximation functions as formatted 
/// strings
/// 
/// Description.
/// 
/// * `data` - vector of formatted strings
/// 
/// Return.
/// * a vector of string vectors
pub fn least_squares(data: &Vec<TemperatureLine>)->Vec<String>{
    let mut lines: Vec<String> = vec![];

    let x = get_x_matrix(&data);
    let all_y = get_y_matrix(&data);
    let x_t = get_x_transpose_matrix(&data);
    
    if all_y.len() > 0 {
        let cores = all_y.len(); 
        
        let mut core = 0;
        while core < cores{
            let y: Vec<Vec<f64>> = all_y[core].iter().map(|val| vec!(*val as f64)).collect();
            let x_t_x = matrix_operations::matrix_multiplication(x_t.clone(), x.clone());
            let x_t_y = matrix_operations::matrix_multiplication(x_t.clone(), y);
            let augmented: Vec<Vec<f64>> = matrix_operations::build_augmented_matrix(x_t_x.clone(), x_t_y.clone());
            let reduced_row_echelon: Vec<Vec<f64>> = matrix_operations::convert_matrix_to_reduced_row_echelon_form(augmented);
            if reduced_row_echelon.len() > 0 && x.len() > 0 && x[0].len()>0{
                if reduced_row_echelon[0].len()>2 {
                    let start = x[0][1];
                    let end = x[x.len() - 1][1]; 
                    let y_intercept = reduced_row_echelon[0][2];
                    let slope = reduced_row_echelon[1][2];
                    let line = format_output(start as u64, end as u64, y_intercept, slope, "least-squares");

                    lines.push(line);
                }
            }

            core += 1; 
        }        
    }

    return lines; 
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
/// into linear piecewise and
/// least squares data and
/// stores the output in files for each
/// column. It also writes the output to the
/// console.
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
    for (core, lines) in piece_wise_values.iter().enumerate(){
        println!("Core {}", core);
        for(_line_no, line) in lines.iter().enumerate(){
            println!("{}", line); 
        }

    }
    create_files(base_name, piece_wise_values);
}