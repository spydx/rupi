#![no_std]
#![no_main]

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rust-embedded/wg/master/assets/logo/ewg-logo-blue-white-on-transparent.png"
)]

mod bsp;
mod cpu;
mod panic_wait;
/*
static HELLO: &[u8] = b"RuPi says Hello!";

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let FSELECT = 0x3F20_0008;
    let GPIO_PIN21_SET = 0x3F20001c;
    let GPIO_PIN21_CLEAR = 0x3F20_0028;
    let SET_OUTPUT = 1 << 3;
    let TURN_ON = 1 << 21;
    let TURN_OFF = 1 << 21;

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    unsafe {
        core::ptr::write_volatile(FSELECT as *mut i32, SET_OUTPUT);

        loop {
            core::ptr::write_volatile(GPIO_PIN21_SET as *mut i32, TURN_ON);

            sleep(5);

            core::ptr::write_volatile(GPIO_PIN21_CLEAR as *mut i32, TURN_OFF);
        }
    };
}

fn sleep(sec: i32) -> () {
    let ms: i32 = sec * 10000;

    unsafe {
        for _ in 1..ms {
            asm!("nop")
        }
    }
}

// 3f20_0008 fsel2 1<<3 turn pin21 into an output
// 3f20_001c gpiol_set 1<<21 turn pin 21 on
// 3f20_0028 gpiol_clean 1<<21 turn pin 21 off
 */