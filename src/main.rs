fn main() {
    let mut args = std::env::args().skip(1);
    let arg = args.next();
    if let Some(arg) = arg {
        if let Ok(lang_id) = arg.parse::<isize>() {
            switch_lang(lang_id);
        } else {
            show_help();
        }
    } else {
        println!("{}", get_lang_id());
    }
}

fn get_lang_id() -> usize {
    // use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
    use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
    let hwnd = get_foreground_window();
    // let ime_wnd = unsafe { ImmGetDefaultIMEWnd(hwnd) };

    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, None) };
    let hkl = unsafe { GetKeyboardLayout(thread_id) };
    // 低16位为语言标识符
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayout
    (hkl.0 as usize) & 0xFFFF
}

fn switch_lang(lang_id: isize) {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_INPUTLANGCHANGEREQUEST};
    let _ = unsafe {
        PostMessageA(
            Some(get_foreground_window()),
            WM_INPUTLANGCHANGEREQUEST,
            WPARAM::default(),
            LPARAM(lang_id),
        )
    };
}
fn get_foreground_window() -> windows::Win32::Foundation::HWND {
    unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() }
}

fn show_help() {
    eprintln!("Usage:");
    eprintln!("  im-switch => 前台窗口的lang_id");
    eprintln!("  im-switch <lang_id> => 切换到指定语言");
    eprintln!("  im-switch [-h | help]");
    std::process::exit(0);
}
