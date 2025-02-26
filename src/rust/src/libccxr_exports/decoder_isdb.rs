#[no_mangle]
pub extern "C" fn ccxr_isdb_set_global_time(ctx: *mut libc::c_void, timestamp: u64) -> i32 {
    pub fn isdb_set_global_time(ctx: *mut libc::c_void, timestamp: u64) -> i32;
}
