#[no_mangle]
pub extern "C" fn my_gpu_add(args: *const i64, num_args: usize, _state: *const std::ffi::c_void) -> i64 {
    unsafe {
        let args_slice = std::slice::from_raw_parts(args, num_args);
        if num_args < 2 { return 0; }
        args_slice[0] + args_slice[1] + 100
    }
}
