use argp::FromArgs;
use std::ffi::{CString, c_void};
use std::path::PathBuf;
use std::{env, mem};

use tokio::{net::UdpSocket, signal};

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Environment::SetCurrentDirectoryA;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{MEM_COMMIT, PAGE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{
    CREATE_SUSPENDED, CreateProcessA, CreateRemoteThread, DETACHED_PROCESS, GetExitCodeThread,
    INFINITE, PROCESS_INFORMATION, ResumeThread, STARTUPINFOA, TerminateProcess,
    WaitForSingleObject,
};
use windows::core::{PSTR, s};

#[derive(FromArgs, Debug)]
#[argp(description = "top level args")]
struct TopLevelArgs {
    #[argp(positional)]
    path: String,
}

#[tokio::main]
async fn main() {
    let args: TopLevelArgs = argp::parse_args_or_exit(argp::DEFAULT);
    println!("{:#?}", args);

    let mut directory_path = PathBuf::from(&args.path);

    if let Some(ext) = directory_path.as_path().extension() {
        if ext == "exe" {
            directory_path.pop();
        }
    }

    println!("Setting dir to [{:?}]", directory_path);

    {
        let mut dir_path_str = String::from(
            directory_path
                .to_str()
                .expect("failed to conert string to string"),
        );
        unsafe {
            SetCurrentDirectoryA(PSTR::from_raw(dir_path_str.as_mut_ptr()));
        }
    }

    let dll_path = match env::var("DLL_FILE") {
        Ok(str) => str,
        Err(_) => {
            "/home/txt/Documents/Code/AiSpace2/target/i686-pc-windows-msvc/debug/aispace_hook.dll"
                .into()
        }
    };

    let (process_handle, thread_handle) =
        match create_suspend_inject(".\\ai sp@ce.exe ./data", &dll_path) {
            Some(pi) => (pi.hProcess.0 as usize, pi.hThread.0 as usize),
            None => panic!("Failed to spawn process!"),
        };

    let process_task = tokio::task::spawn_blocking(move || unsafe {
        WaitForSingleObject(HANDLE(process_handle as *mut c_void), INFINITE);
        CloseHandle(HANDLE(thread_handle as *mut c_void));
        CloseHandle(HANDLE(process_handle as *mut c_void));
    });

    let udp_task = tokio::spawn(async {
        if let Err(e) = udp_listener().await {
            eprintln!("UDP error: {:?}", e);
        }
    });

    // wait for either ctrl + c or wait for process to exit
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C received. Exiting...");
            unsafe {
                TerminateProcess(HANDLE(process_handle as *mut c_void), 0);

                CloseHandle(HANDLE(thread_handle as *mut c_void));
                CloseHandle(HANDLE(process_handle as *mut c_void));
            }
        },
        _ = process_task => {
            println!("Child process exited. Exiting...");
        }
    }
}

async fn udp_listener() -> Result<(), std::io::Error> {
    let listen_ip = "0.0.0.0";
    let listen_port = 9999;
    let listen_str = format!("{}:{}", listen_ip, listen_port);

    println!("Listening for UDP on [{}]", &listen_str);
    let socket = UdpSocket::bind(&listen_str).await?;

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        print!("{}", msg);
    }

    Ok(())
}

fn create_suspend_inject(command_line: &str, dll_path: &str) -> Option<PROCESS_INFORMATION> {
    let load_library_address = unsafe {
        GetProcAddress(
            GetModuleHandleA(s!("kernel32.dll"))
                .expect("kernel32.dll must always be loaded on windows. you are not on windows"),
            s!("LoadLibraryA"),
        )
    }
    .expect("Failed to get LoadLibraryA address");

    println!("LoadLibraryA: {:p}", load_library_address);

    let dll_path_c = CString::new(dll_path.to_string()).expect("failed to do CString for dll path");

    let mut startup_info = STARTUPINFOA::default();
    startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

    let mut process_info = PROCESS_INFORMATION::default();

    match unsafe {
        CreateProcessA(
            None,
            Some(PSTR::from_raw(command_line.to_string().as_mut_ptr())), // command line args
            None,                                                        // process attributes
            None,                                                        // thread attributes
            false,                                                       // inherit handles
            CREATE_SUSPENDED | DETACHED_PROCESS,                         // creation flags
            None,                                                        // environment
            None,                                                        // current directory
            &mut startup_info,
            &mut process_info,
        )
    } {
        Ok(_) => {
            let string_address = unsafe {
                VirtualAllocEx(
                    process_info.hProcess,
                    None,
                    dll_path_c.count_bytes() + 1,
                    MEM_COMMIT,
                    PAGE_READWRITE,
                )
            };

            let mut write_count: usize = 0;
            let _ = unsafe {
                WriteProcessMemory(
                    process_info.hProcess,
                    string_address,
                    dll_path_c.as_c_str().as_ptr() as *const c_void,
                    dll_path_c.count_bytes() + 1,
                    Some(&mut write_count),
                )
            };

            let dll_thread = match unsafe {
                CreateRemoteThread(
                    process_info.hProcess,
                    None,
                    0,
                    Some(mem::transmute(load_library_address)),
                    Some(string_address),
                    0,
                    None,
                )
            } {
                Ok(handle) => handle,
                Err(_) => panic!("Failed to create remote thread"),
            };

            // wait untill it finished executing
            let _ = unsafe { WaitForSingleObject(dll_thread, INFINITE) };

            // got return value
            let mut exit_code = 0;
            let _ = unsafe { GetExitCodeThread(dll_thread, &mut exit_code) };

            println!("Got exit code {}", exit_code);

            // resume the main process
            unsafe {
                ResumeThread(process_info.hThread);
            }

            Some(process_info)
        }
        Err(_) => {
            panic!("Error creating process");
            None
        }
    }
}
