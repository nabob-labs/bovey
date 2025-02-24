/// Return the remaining compute units the program may consume
#[inline]
pub fn bov_remaining_compute_units() -> u64 {
    #[cfg(target_os = "bovey")]
    unsafe {
        crate::syscalls::bov_remaining_compute_units()
    }

    #[cfg(not(target_os = "bovey"))]
    {
        crate::program_stubs::bov_remaining_compute_units()
    }
}
