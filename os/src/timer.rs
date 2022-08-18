use riscv::register::time;
use crate::config::CLOCK_FREQ;
pub fn get_time() -> usize {
    time::read()
}

const TICKS_PER_SEC: usize = 100;
use crate::sbi::set_timer;
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}