use chrono::{Local, Timelike};
use std::f64::consts::PI;

pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

pub fn what_time() -> Time {
    let now = Local::now();
    Time {
        hour: now.hour() % 12,
        minute: now.minute(),
        second: now.second(),
    }
}


/// 2x2 Matrix type for our rotation operations
type Mat2 = [[f64; 2]; 2];
type Vec2 = [f64; 2];


///     R_cw(θ) = | cos(θ)   sin(θ) |
///               | -sin(θ)  cos(θ) |

pub fn time_to_rotation_matrix(time: &Time, hand: char) -> Mat2 {
    let theta = match hand {
        'H' => {
            // Hour hand: 30 deg per hour + 0.5 deg per minute
            // θ = (hour + minute/60) × (2pi/12)
            (time.hour as f64 + time.minute as f64 / 60.0) * PI / 6.0
        }
        'M' => {
            // Minute hand: 6deg per minute
            // θ = minute × (2pi/60)
            time.minute as f64 * PI / 30.0
        }
        _ => 0.0,
    };

 
    [
        [theta.cos(), theta.sin()],
        [-theta.sin(), theta.cos()],
    ]
}

/// Matrix-Vec multiplication

fn mat_vec_mul(mat: &Mat2, vec: &Vec2) -> Vec2 {
    [
        mat[0][0] * vec[0] + mat[0][1] * vec[1],
        mat[1][0] * vec[0] + mat[1][1] * vec[1],
    ]
}


/// This serves as a mathematical sanity check.
fn determinant(mat: &Mat2) -> f64 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}


pub fn compute_hand_position(time: &Time, hand: char, grid_size: usize) -> (usize, usize) {
    let center = (grid_size / 2) as f64;
    
    
    let length = match hand {
        'H' => center * 0.55,  // Hour hand: 55% of radius
        'M' => center * 0.85,  // Minute hand: 85% of radius
        _ => center * 0.5,
    };

    // Reference vector: points to 12 o'clock
    // [0, 1]
    let reference: Vec2 = [0.0, 1.0];

    
    let rotation = time_to_rotation_matrix(time, hand);

    
    let det = determinant(&rotation);
    debug_assert!((det - 1.0).abs() < 1e-10, "Invalid rotation matrix: det = {}", det);

    
    let direction = mat_vec_mul(&rotation, &reference);

    // Scale by hand length 
    let scaled = [direction[0] * length, direction[1] * length];

    // Transform from Cartesian to grid coordinates:
    let row = center - scaled[1];  // Flip y-axis
    let col = center + scaled[0];

    // Clamp to valid grid indices
    let row = row.round().clamp(0.0, (grid_size - 1) as f64) as usize;
    let col = col.round().clamp(0.0, (grid_size - 1) as f64) as usize;

    (row, col)
}

/// Places a hand marker on the clock grid 
pub fn place_hand(grid: &mut Vec<Vec<i32>>, time: &Time, hand: char) {
    let grid_size = grid.len();
    let (row, col) = compute_hand_position(time, hand, grid_size);
    
    let marker = hand as i32;  // 'H' = 72, 'M' = 77
    grid[row][col] = marker;
}









// #[allow(dead_code)]
// fn angle_from_trace(mat: &Mat2) -> f64 {
//     let trace = mat[0][0] + mat[1][1];
//     (trace / 2.0).acos()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_rotation_determinant_is_one() {
//         let time = Time { hour: 10, minute: 43, second: 0 };
//         let r_hour = time_to_rotation_matrix(&time, 'H');
//         let r_min = time_to_rotation_matrix(&time, 'M');
        
//         assert!((determinant(&r_hour) - 1.0).abs() < 1e-10);
//         assert!((determinant(&r_min) - 1.0).abs() < 1e-10);
//     }

//     #[test]
//     fn test_12_oclock_points_up() {
//         let time = Time { hour: 0, minute: 0, second: 0 };
//         let (row, col) = compute_hand_position(&time, 'H', 32);
        
//         // At 12:00, hand should point to top center
//         assert!(row < 16, "Hour hand at 12:00 should be in top half");
//         assert!((col as i32 - 16).abs() <= 1, "Hour hand at 12:00 should be centered");
//     }

//     #[test]
//     fn test_3_oclock_points_right() {
//         let time = Time { hour: 3, minute: 0, second: 0 };
//         let (row, col) = compute_hand_position(&time, 'H', 32);
        
//         // At 3:00, hand should point to right center
//         assert!((row as i32 - 16).abs() <= 1, "Hour hand at 3:00 should be vertically centered");
//         assert!(col > 16, "Hour hand at 3:00 should be in right half");
//     }
// }
