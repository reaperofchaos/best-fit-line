use inputlibrary::TemperatureLine;
#[path = "utils/line_functions.rs"] mod linefunctions;
#[path = "utils/file_output.rs"] mod fileoutput;
#[path = "utils/matrix_operations.rs"] mod matrix_operations;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name: &str = &args[1];
    let base_name: &str = &file_name[0..file_name.len() - 4];

    let readings: Vec<TemperatureLine> = match inputlibrary::read_temperature_file(file_name){
        Ok(readings) => readings,
        Err(_e) => return,
    };

    fileoutput::write_output(readings, base_name);
}
