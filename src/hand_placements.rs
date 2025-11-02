use Chrono::{Local, TimeLike};

pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

pub fn what_time() -> Time {
    let now = Local::now();
    Time {
        hour: now.hour() % 12,  // Convert 24-hour to 12-hour
        minute: now.minute(),
        second: now.second(),
    }
}


fn which_quadrant() -> i32{

	if (Time::hour == 12 || Time::hour == 1 || Time::hour == 2) {
		// Quadrant = 2
		let hour_quadrant : i32 = 2;
	}

	else if (Time::hour == 3 || Time::hour == 4 || Time::hour == 5) {
		 // Quadrant = 4
		 let hour_quadrant : i32 = 4;
	}

	else if (Time::hour == 6 || Time::hour == 7 || Time::hour == 8) {
		// Quadrant = 3
		let hour_quadrant : i32 = 3;
	}

	else if (Time::hour == 9 || Time::hour == 10 || Time::hour == 11) {
		// Quadrant = 1
		let hour_quadrant : i32 = 1;
	}

}


fn reference_vec_12(which_quadrant : i32) {


}

fn reference_vec_12(which_quadrant : i32) {

	
}

fn reference_vec_12(which_quadrant : i32) {

	
}

fn reference_vec_12(which_quadrant : i32) {

	
}




