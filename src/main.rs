// Import the clock math module
mod hand_placement;
use hand_placement::{what_time, place_hand};

fn identity_matrix(len: usize, one_var: i32) -> Vec<Vec<i32>> {
    let mut generated_matrix: Vec<Vec<i32>> = vec![vec![0; len]; len];

    for i in 0..len { 
        let col_position = len - 1 - i; 
        generated_matrix[i][col_position] = one_var;
    }
    generated_matrix
}


// Matrix multiplication
fn matmul(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();
    let mut result = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let mut sum = 0;
            for k in 0..p {
                sum += a[i][k] * b[k][j];
            }
            result[i][j] = sum;
        }
    }
    result
}


fn exchange_matrix(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    matmul(mat, mat)
}


// Quadrant 9-12 (top-left)
fn quad_9_to_12(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> Vec<Vec<i32>> {
    let mut matrix = identity_matrix(len, one_var);
    
    matrix[0][len-1] = row1_val;   
    matrix[len-1][0] = row16_val; 
    matrix
}


// Quadrant 12-3
fn quad_12_to_3(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> Vec<Vec<i32>> {
    let identity = identity_matrix(len, one_var);
    let mut matrix = exchange_matrix(&identity);
    
    matrix[0][0] = row1_val;       
    matrix[len-1][len-1] = row16_val; 
    matrix

}


// Quadrant 3-6 
fn quad_3_to_6(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> Vec<Vec<i32>> {
    let mut matrix = identity_matrix(len, one_var);
    
    
    matrix[0][len-1] = row1_val;  
    matrix[len-1][0] = row16_val;  
    matrix
}


// Quadrant 6-9 
fn quad_6_to_9(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> Vec<Vec<i32>> {
    let identity = identity_matrix(len, one_var);
    let mut matrix = exchange_matrix(&identity);
    
    
    matrix[0][0] = row1_val;       
    matrix[len-1][len-1] = row16_val; 
    matrix
}



fn concat_2x2_grid(q1: &Vec<Vec<i32>>, q2: &Vec<Vec<i32>>, 
                   q3: &Vec<Vec<i32>>, q4: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let quad_size = q1.len();
    let full_size = quad_size * 2;
    let mut clock: Vec<Vec<i32>> = vec![vec![0; full_size]; full_size];
    
    // Top half: [Q1 | Q2]
    for row in 0..quad_size {
        // Q1 - top left (9-12)
        for col in 0..quad_size {
            clock[row][col] = q1[row][col];
        }
        // Q2 - top right (12-3)
        for col in 0..quad_size {
            clock[row][col + quad_size] = q2[row][col];
        }
    }
    
    // Bottom half: [Q3 | Q4]
    for row in 0..quad_size {
        // Q3 - bottom left (6-9)
        for col in 0..quad_size {
            clock[row + quad_size][col] = q3[row][col];
        }
        // Q4 - bottom right (3-6)
        for col in 0..quad_size {
            clock[row + quad_size][col + quad_size] = q4[row][col];
        }
    }
    clock
}


fn strip_zeros(len: usize, grid: &Vec<Vec<i32>>) {
    let dot = "+"; 

    for i in 0..(2 * len) {
        for j in 0..(2 * len) {
            let entry = grid[i][j];

            match entry {
                72 => print!(" H "),  // Hour hand marker
                77 => print!(" M "),  // Minute hand marker
                3 | 6 | 9 | 12 => print!("{:2} ", entry),  // Clock numbers
                1 => print!("{} ", dot),  // Diagonal pattern
                _ => print!("  "),  // Empty space
            }
        }
        println!();




    }

}

fn main() {
    const QUAD_SIZE: usize = 16;
    const ONE_VAR: i32 = 1; 
    const MARKER_12: i32 = 12;
    const MARKER_3: i32 = 3;
    const MARKER_6: i32 = 6;
    const MARKER_9: i32 = 9;

    let current_time = what_time(); //gets OS time
    
    println!("Current Time: {}:{:02}:{:02}", 
             current_time.hour, 
             current_time.minute, 
             current_time.second);
    
    
    let quadrant_1 = quad_9_to_12(QUAD_SIZE, ONE_VAR, MARKER_12, MARKER_9);
    let quadrant_2 = quad_12_to_3(QUAD_SIZE, ONE_VAR, MARKER_12, MARKER_3);
    let quadrant_4 = quad_3_to_6(QUAD_SIZE, ONE_VAR, MARKER_3, MARKER_6);
    let quadrant_3 = quad_6_to_9(QUAD_SIZE, ONE_VAR, MARKER_9, MARKER_6);
    let mut full_clock = concat_2x2_grid(&quadrant_1, &quadrant_2, &quadrant_3, &quadrant_4);
    
    
    place_hand(&mut full_clock, &current_time, 'H');  // Hour hand
    place_hand(&mut full_clock, &current_time, 'M');  // Minute hand
    
    println!("\nAnalog Clock Face:");
    strip_zeros(QUAD_SIZE, &full_clock);
}

