#[path = "tests/matrix_operations_test.rs"] mod matrix_operations_tests;

/// builds a matrix with placeholder values
///  for the rows of left and columns of right
/// Description.
/// 
/// * `lhs` - first vector being multiplied
/// * `rhs` - second vector being multiplied
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
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


/// gets the transpose for a matrix
/// Description.
/// 
/// * `matrix` - vector of f64 vectors
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
pub fn get_transpose(matrix: Vec<Vec<f64>>)-> Vec<Vec<f64>>{
    let mut transpose: Vec<Vec<f64>> = vec![]; 

    for(row_pos, row) in matrix.iter().enumerate(){
        for(col_pos, col) in row.iter().enumerate(){
            if transpose.is_empty() || transpose.len() <= col_pos {
                let new_row_vector: Vec<f64> = vec![*col];
                transpose.push(new_row_vector);
            }else{
                transpose[col_pos].push(*col);
            }
        }
    }

    return transpose; 
} 

/// build an augmented matrix from
/// an x matrix and a y matrix
/// Description.
/// 
/// * `x_matrix` - vector of f64 vectors
/// * `y_matrix` - vector of f64 vectors
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
pub fn build_augmented_matrix(x_matrix: Vec<Vec<f64>>, y_matrix: Vec<Vec<f64>>)->Vec<Vec<f64>>{
    let mut augmented: Vec<Vec<f64>> = vec![];
    if x_matrix.len() == y_matrix.len(){
        for(pos, row) in x_matrix.iter().enumerate(){
            let mut row_vec: Vec<f64> = vec![];
            for (col_pos, col) in row.iter().enumerate(){
                row_vec.push(*col);
            }
            
            if y_matrix[pos].len() > 0 {
                row_vec.push(y_matrix[pos][0]);  
            }
            
            augmented.push(row_vec);
        }
    }

    return augmented;   
}

/// Performs matrix multiplication
/// Description.
/// 
/// * `lhs` - first vector being multiplied
/// * `rhs` - second vector being multiplied
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
pub fn matrix_multiplication(lhs: Vec<Vec<f64>>, rhs: Vec<Vec<f64>>)-> Vec<Vec<f64>>{
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

/// Finds row with largest value
/// Description.
/// finds the index of the row
/// with the largest value at 
// the column index
/// 
/// * `matrix` -  vector being multiplied
/// * `col_index` - column index finding the largest value
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn find_largest_row_by_col(matrix: Vec<Vec<f64>>, col_index: usize)->usize{
    println!{"col index {}", col_index};

    let mut largest = matrix[col_index][col_index];
    println!{"value at col index {} row index {} {}",col_index, col_index, largest};

    let mut largestIndex = col_index;
    let mut i = col_index; 

    if matrix.len() > 0 && matrix[0].len() >= col_index {
        while i < matrix.len() {
            println!("row {} value at col index {} {} ", i, col_index, matrix[i][col_index]);

            if matrix[i][col_index] > largest{
                println!("largest value is now {} and at row {}", matrix[i][col_index], i);
                largestIndex = i;
                largest = matrix[i][col_index];
            }
            i += 1; 
        }
    }
    
    println!("largest value is {}", largestIndex);
    return largestIndex;
}

/// Swap rows at indexes
/// Description.
/// 
/// * `matrix` -  vector being multiplied
/// * `from` - row index swapping from
/// * `to` - row index swapping to
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn swap_row(mut matrix: Vec<Vec<f64>>, from: usize, to: usize)->Vec<Vec<f64>>{
    if matrix.len() > from && matrix.len() > to{
        matrix.swap(from, to); 

        return matrix; 
    }

    return matrix;
}

/// Scale a row in a matrix
/// Description.
/// 
/// * `matrix` -  matrix scaling a row on
/// * `row_index` - row index of row to scale
/// * `scale` - value to scale on
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn scale_row(mut matrix: Vec<Vec<f64>>, row_index: usize, scale: f64)->Vec<Vec<f64>>{
    let mut i = 0; 
    while i < matrix[row_index].len(){
        if matrix[row_index][i] == 0.0{

            i += 1;
            continue;
        }

        matrix[row_index][i] = scale * matrix[row_index][i];
        i += 1; 
    }

    return matrix; 
}


/// Eliminate a row by subtracting each row by a scaled
/// source row
/// Description.
/// 
/// * `matrix` -  matrix scaling a row on
/// * `row_index` - row index of row to eliminate
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn eliminate_row(mut matrix: Vec<Vec<f64>>, row_index: usize)->Vec<Vec<f64>>{
    if row_index < matrix.len() {
        let mut i = row_index + 1; 
        while i < matrix.len(){
            let s = matrix[i][row_index];

            let mut j = row_index; 
            while j < matrix[i].len(){
                matrix[i][j] = matrix[i][j] - s * matrix[row_index][j];
                j += 1; 
            }
            
            matrix[i][row_index] = 0.0;
            i += 1; 
        }
    }

    return matrix; 
} 

/// Function to backsolve a matrix in row
/// echelon form
/// Description.
/// 
/// * `matrix` -  matrix scaling a row on
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn back_solve_matrix(mut matrix: Vec<Vec<f64>>)->Vec<Vec<f64>>{
    println!("total rows {}", matrix.len());
    if matrix.len() > 0 {
        let last_row_index = matrix.len() -1;
        let last_col = matrix[0].len() - 1;

        let mut i = last_row_index;
        while i > 0 {
            let mut j = i -1; 
            loop {
                let s = matrix[j][i];

                matrix[j][i] -= s * matrix[i][i];
                matrix[j][last_col] -= s * matrix[i][last_col];

                if j == 0 {
                    break;
                }
                
                j -= 1; 
            }
            
            i -= 1; 
        }
    }
    return matrix;
}

/// function to convert a matrix to
/// row echelon form
/// Description.
/// 
/// * `matrix` -  matrix scaling a row on
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
fn convert_matrix_to_row_echelon_form(matrix: Vec<Vec<f64>>)->Vec<Vec<f64>>{
    let mut updatedMatrix = matrix.clone(); 
    // Iterate through all rows
    let mut i = 0;
    while i < updatedMatrix.len(){
        //  Pivot
        let largestRow = find_largest_row_by_col(updatedMatrix.clone(), i);
        updatedMatrix = swap_row(updatedMatrix.clone(), largestRow, i);

        // scale if column of interest at position is not 1
        if updatedMatrix[i][i] != 1.0{
            let scale = 1.0/updatedMatrix[i][i];
            updatedMatrix = scale_row(updatedMatrix.clone(), i, scale);
        }

        updatedMatrix[i][i] = 1.0; 

        if i == 0 || (i > 0  && updatedMatrix[i][i] != 0.0){
            updatedMatrix = eliminate_row(updatedMatrix.clone(), i); 
        }

        i += 1;
    }
    return updatedMatrix; 
}

/// function to convert a matrix to
/// reduced row echelon form
/// Description.
/// 
/// * `matrix` -  matrix scaling a row on
/// 
/// Return.
/// * matrix Vec<Vec<f64>>
pub fn convert_matrix_to_reduced_row_echelon_form(matrix: Vec<Vec<f64>>)->Vec<Vec<f64>>{
    let updatedMatrix = convert_matrix_to_row_echelon_form(matrix);

    return back_solve_matrix(updatedMatrix);
}
