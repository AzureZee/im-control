use core::ptr::null_mut as ptr_null_mut;
use windows_sys::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows_sys::Win32::UI::WindowsAndMessaging::{SendMessageA, WM_IME_CONTROL};
use windows_sys::Win32::{
    Foundation::HWND,
    UI::{
        Input::KeyboardAndMouse::GetKeyboardLayout,
        WindowsAndMessaging::{
            GetForegroundWindow, GetWindowThreadProcessId, PostMessageA, WM_INPUTLANGCHANGEREQUEST,
        },
    },
};

fn main() {
    let mut args = std::env::args().skip(1);
    let arg = args.next();
    if let Some(arg) = arg {
        match arg.as_str() {
            "get-im" => {
                println!("{}", get_input_method());
            }
            "set-im" => {
                if let Some(id) = args.next() {
                    if let Ok(id) = id.parse::<isize>() {
                        set_input_method(id);
                    }
                } else {
                    show_help();
                }
            }
            "get-status" => {
                println!("{}", if get_ime_status() { "on" } else { "off" });
            }
            "set-status" => {
                if let Some(status) = args.next() {
                    let status = match status.as_str() {
                        "on" => true,
                        "off" => false,
                        _ => {
                            show_help();
                        }
                    };
                    set_ime_status(status);
                } else {
                    show_help();
                }
            }
            _ => {
                show_help();
            }
        }
    } else {
        show_help();
    }
}

fn get_input_method() -> usize {
    let hwnd = get_ime_window();
    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, ptr_null_mut()) };
    let hkl = unsafe { GetKeyboardLayout(thread_id) } as usize;
    // 低16位为语言标识符
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayout
    let lo = hkl & 0xFFFF;
    let hi = (hkl >> 16) & 0xFFFF;
    if lo == hi { lo } else { hkl }
}

fn set_input_method(lang_id: isize) {
    let _ = unsafe {
        PostMessageA(
            get_foreground_window(),
            WM_INPUTLANGCHANGEREQUEST,
            0,
            lang_id,
        )
    };
}
type ImeStatus = bool;
fn get_window_ime_status(hwnd: HWND) -> ImeStatus {
    unsafe {
        SendMessageA(
            hwnd,
            WM_IME_CONTROL,
            5, // IMC_GETOPENSTATUS
            0,
        ) != 0
    }
}
fn get_ime_status() -> bool {
    get_window_ime_status(get_ime_window())
}
fn set_window_ime_status(hwnd: HWND, status: ImeStatus) {
    unsafe {
        SendMessageA(
            hwnd,
            WM_IME_CONTROL,
            6, // IMC_GETOPENSTATUS
            if status { 1 } else { 0 },
        )
    };
}
fn set_ime_status(status: bool) {
    set_window_ime_status(get_ime_window(), status);
}
fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

fn get_ime_window() -> HWND {
    unsafe { ImmGetDefaultIMEWnd(get_foreground_window()) }
}

fn show_help() -> ! {
    eprintln!(
"Usage:
    imc get-im             output klid
    imc set-im <klid>
    imc get-status         output im status on/off
    imc set-status on/off");
    std::process::exit(0)
}
