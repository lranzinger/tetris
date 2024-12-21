extern "C" {
    fn js_get_high_score() -> u32;
    fn js_save_high_score(score: u32) -> i32;
}

pub fn get_high_score() -> u32 {
    unsafe { js_get_high_score() }
}

pub fn update_high_score(score: u32) {
    unsafe {
        js_save_high_score(score);
    }
}
