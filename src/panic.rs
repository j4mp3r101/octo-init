use crate::new_asm::debug::print;
use crate::stage3::REBOOT;
use crate::stage4::stage4;

#[inline(always)]
pub fn panic() -> ! {
    print("PANIC! rebooting");

    stage4(REBOOT)
}
