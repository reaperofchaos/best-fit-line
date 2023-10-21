use std::fs;
use inputlibrary::TemperatureLine;
use crate::linefunctions;
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
pub fn format_output(start: u64, end: u64, yintercept: f32, slope: f32, method: &str)->String{
    format!("{start} <= x <=\t{end} ; y = \t {yintercept:.4} +\t {slope:.4} x ; {method}")
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
                let slope = linefunctions::calculate_slope(end as f32, data[tpos + 1].readings[rpos] as f32, start as f32, data[tpos].readings[rpos] as f32);
                let yintercept = linefunctions::calculate_y_intercept(start as f32, t.readings[rpos] as f32, slope);
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