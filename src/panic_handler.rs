use core::panic::PanicInfo;

// See: https://doc.rust-lang.org/std/panic/struct.PanicInfo.html#method.location
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    print!("[!] KERNEL PANIC\n");

    if let Some(location) = info.location() {
        print!("[!] PANIC OCCURED IN FILE '{}' AT LINE {}\n",
            location.file(),
            location.line(),
        );
    };

    if let Some(message) = info.message() {
        print!("[!] PANIC MESSAGE: {}\n",
            message
        );
    };

    loop{
        unsafe{
            core::arch::asm!("hlt");
        }
    }
}
