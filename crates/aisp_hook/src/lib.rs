// use std::ffi::c_void;
//
// use windows::Win32::Foundation::{GetLastError, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
// use windows::Win32::System::Console::AllocConsole;
// use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleW};
// use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
//
// // use retour::static_detour;
//
// // static_detour! {
// //     static PresentHook:  unsafe extern "system" fn(*mut c_void, u32, u32) -> HRESULT;
// // }
//
// #[unsafe(no_mangle)]
// #[allow(non_snake_case)]
// pub extern "system" fn DllMain(module: HMODULE, call_reason: u32, _reserved: *mut c_void) -> u32 {
//     if call_reason == DLL_PROCESS_ATTACH {
//         unsafe {
//             DisableThreadLibraryCalls(module);
//         }
//
//         unsafe {
//             AllocConsole();
//         }
//
//         println!("Test AllocConsole");
//     }
//
//     return true.into();
// }
