#[path = "tests/matrix_operations_test.rs"] mod matrix_operations_tests;

fn initialize_result_matrix(lhs: Vec<Vec<f64>>, rhs: Vec<Vec<f64>>)-> Vec<Vec<f64>>{
let mut result_vec: Vec<Vec<f64>> = vec![];
    let mut a = 0;
    while a < lhs.len(){
        result_vec.push(vec![0.0]);
        a += 1; 
    }

    if lhs.len() > 0 && rhs.len() == lhs[0].len() {
        println!("initiaizing columns");
        // initialize a result vector
        let mut n = 0;
        while n < result_vec.len(){
            if rhs.len() > 0 {

                let mut result_col_vec: Vec<f64> = vec![];
                let mut i = 0; 
                while i < rhs[0].len(){
                    result_col_vec.push(0.0);
                    i += 1; 
                }
                result_vec[n] = result_col_vec;
            }

            n += 1; 
        }
    }

    return result_vec; 
}
/// Performs matrix multiplication
fn matrix_multiplication(lhs: Vec<Vec<f64>>, rhs: Vec<Vec<f64>>)-> Vec<Vec<f64>>{
    // columns of left equal to rows on right
    let mut result_vec = initialize_result_matrix(lhs.clone(), rhs.clone()); 
        
        //populate the result vector with product
    let mut i = 0;
    while i < result_vec.len(){
        let mut j = 0;
        while j < result_vec[0].len() {
            let mut k = 0; 
            while k < lhs[0].len() {
                result_vec[i][j] += lhs[i][k] * rhs[k][j];
                k += 1; 
            }
            j += 1; 
        }
        i +=1; 
    }

    return result_vec; 
}
