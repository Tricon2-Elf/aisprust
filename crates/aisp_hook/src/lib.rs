use std::ffi::c_void;


// #[link(name = "aisp_hook_cpp", kind = "dylib")] 
unsafe extern "C" {
    // #[link_name = "CustomDllMain@12"]
    pub fn CustomDllMain(module: *mut c_void, call_reason: u32, reserved: *mut c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(module: *mut c_void, call_reason: u32, reserved: *mut c_void) -> u32 {

    return unsafe { CustomDllMain(module, call_reason, reserved)};
}
