

fn identity_matrix(len: usize, one_var: i32) -> Vec<Vec<i32>> {
    let mut generated_matrix: Vec<Vec<i32>> = vec![vec![0; len]; len];

    for i in 0..len { 
        let col_position = len - 1 - i; 
        generated_matrix[i][col_position] = one_var;
    }
    generated_matrix
}




// Matrix multiplication helper
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



fn 9_12(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> &Vec<Vec<i32>>{

    let mut 9_12_matrix : &Vec<Vec<i32>> = identity_matrix(len, one_var);

    9_12_matrix[1][16] = row1_val;
    9_12_matrix[16][1] = row16_val;

    // Future work, if Minutes and Hours have a landing zone here, place them now. 

}


fn 12_3(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> &Vec<Vec<i32>>{

    let mut 12_3_matrix : &Vec<Vec<i32>> = exchange_matrix(len, one_var);

    9_12_matrix[1][16] = row1_val;
    9_12_matrix[16][1] = row16_val;

    // Future work, if Minutes and Hours have a landing zone here, place them now. 

}



fn 3_6(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> &Vec<Vec<i32>>{

    let mut 3_6_matrix : &Vec<Vec<i32>> = identity_matrix(len, one_var);

    9_12_matrix[1][16] = row1_val;
    9_12_matrix[16][1] = row16_val;

    // Future work, if Minutes and Hours have a landing zone here, place them now. 

}


fn 6_9(len: usize, one_var: i32, row1_val: i32, row16_val: i32) -> &Vec<Vec<i32>>{

    let mut 6_9_matrix : &Vec<Vec<i32>> = exchange_matrix(len, one_var);

    9_12_matrix[1][16] = row1_val;
    9_12_matrix[16][1] = row16_val;

    // Future work, if Minutes and Hours have a landing zone here, place them now. 

}






fn main() {
    const QUAD_SIZE: usize = 16;
    const ONE_VAR: i32 = 1; 

    // let quadrant_1 = identity_matrix(QUAD_SIZE, ONE_VAR);
    // let quadrant_2 = exchange_matrix(&quadrant_1);
    // let quadrant_4 = identity_matrix(QUAD_SIZE, ONE_VAR);
    // let quadrant_3 = exchange_matrix(&quadrant_1);

    // let full_clock = concat_2x2_grid(&quadrant_1, &quadrant_2, &quadrant_3, &quadrant_4);

    println!("Full Clock Face ({}x{}):", full_clock.len(), full_clock[0].len());
    println!("[Q1 | Q2]");
    println!("[Q3 | Q4]\n");

    for row in full_clock.iter() {
        for &num in row.iter() {
            print!("{:2} ", num);
        }
        println!();
    }
}

