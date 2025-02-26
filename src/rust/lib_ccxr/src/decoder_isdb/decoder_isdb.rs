



fn isdb_set_global_time(ctx: *mut libc::c_void, timestamp: u64) -> i32 {
    if ctx.is_null() {
        return -1; // Error code for null context
    }
    unsafe {
        let ctx = &mut *(ctx as *mut ISDBSubContext);
        ctx.timestamp = timestamp;
    }
    0 // CCX_OK
}