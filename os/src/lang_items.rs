// os/src/lang_items.rs
use crate::sbi::shutdown;
use core::panic::PanicInfo;
use crate::println;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        unsafe{
            println!(
                "Panicked at {}:{} {}",
                location.file(),
                location.line(),
                _info.message().unwrap()
            );
        }
        
    } else {
        unsafe{
            println!("Panicked: {}", _info.message().unwrap());
        }
        
    }
    shutdown()
}