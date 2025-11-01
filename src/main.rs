

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



fn concat_2x2_grid(
    q1: &Vec<Vec<i32>>,
    q2: &Vec<Vec<i32>>,
    q3: &Vec<Vec<i32>>,
    q4: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let rows = q1.len();
    let cols = q1[0].len();
    let mut result = Vec::with_capacity(rows * 2);

    // Top half
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols * 2);
        row.extend(&q1[i]);
        row.extend(&q2[i]);
        result.push(row);
    }
    // Bottom half
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols * 2);
        row.extend(&q3[i]);
        row.extend(&q4[i]);
        result.push(row);
    }

    result
}


fn main() {
    const QUAD_SIZE: usize = 16;
    const ONE_VAR: i32 = 1; 

    let quadrant_1 = identity_matrix(QUAD_SIZE, ONE_VAR);
    let quadrant_2 = exchange_matrix(&quadrant_1);
    let quadrant_4 = identity_matrix(QUAD_SIZE, ONE_VAR);
    let quadrant_3 = exchange_matrix(&quadrant_1);

    let full_clock = concat_2x2_grid(&quadrant_1, &quadrant_2, &quadrant_3, &quadrant_4);

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

