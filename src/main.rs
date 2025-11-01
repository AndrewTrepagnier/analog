


fn identity_quad(len: usize, one_var: i32) -> Vec<Vec<i32>> {
    let mut generated_matrix: Vec<Vec<i32>> = vec![vec![0; len]; len];

    for i in 0..len { 
        let col_position = len - 1 - i; 
        generated_matrix[i][col_position] = one_var;
    }
    generated_matrix
}






// Function to create a full clock with 4 quadrants arranged as:
// [Q1 | Q2]
// [Q3 | Q4]

fn create_clock(quad_size: usize, one_var: i32) -> Vec<Vec<i32>> {
    

    let mut quads: Vec<Vec<Vec<i32>>> = Vec::new();
    
    for _q in 0..4 {
        let quad = identity_quad(quad_size, one_var);
        quads.push(quad);
    }
    


    // Now concatenate them into a clock layout
    let full_size = quad_size * 2;
    let mut clock: Vec<Vec<i32>> = vec![vec![0; full_size]; full_size];
    
    // Top half: [Q1 | Q2]
    for row in 0..quad_size {
        // Q1
        for col in 0..quad_size {
            clock[row][col] = quads[0][row][col];
        }
        // Q2 
        for col in 0..quad_size {
            clock[row][col + quad_size] = quads[1][row][col];
        }
    }

    
    // Bottom half: [Q3 | Q4]
    for row in 0..quad_size {
        // Q3 
        for col in 0..quad_size {
            clock[row + quad_size][col] = quads[2][row][col];
        }
        // Q4
        for col in 0..quad_size {
            clock[row + quad_size][col + quad_size] = quads[3][row][col];
        }
    }
    
    clock
}





fn main() {
    const QUAD_SIZE: usize = 16;
    const ONE_VAR: i32 = 1; 

    
    let full_clock = create_clock(QUAD_SIZE, ONE_VAR);


    println!("Full Clock Face ({}x{}):", full_clock.len(), full_clock[0].len());
    println!("[Q1 | Q2]");
    println!("[Q3 | Q4]\n");
    
    for row in full_clock.iter() {
        for &num in row.iter() {
            print!("{} ", num);
        }
        println!();
    }
}
