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


fn reference_vec_12(which_quadrant : i32, _origin_ : vec!<vec<i32>>) {

	let origin : vec!<vec<i32>> = _origin_;
	let ref_vec : vec!<vec<i32>> = vec!((idk, 1), idk); //need to find true matrix position of the 12

	//180 total minutes in each quadrant. So for quadrant 2, your hour is either 12, 1, or 2. 

	if Time::hour == 12 {
		let which_quadrant : i32 = 0; //special case to rewrite the 12 to 0, this is only needed in this quadrant since the successor of the 12 position needs to be 1 (not 13)
	}

	let minutes_from_ref : i32 = ( which_quadrant - 0) * 60 + Time::minute // minutes within quadrant, for example: 1:24 is 60+24 = 84 minutes

	let ratio : f64 = minutes_from_ref/60;

	angle_from_ref : f64 = ratio*90;


}

fn reference_vec_12(which_quadrant : i32) {

	
}

fn reference_vec_12(which_quadrant : i32) {

	
}

fn reference_vec_12(which_quadrant : i32) {

	
}




