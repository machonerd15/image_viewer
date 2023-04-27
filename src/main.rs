use std::mem;
use std::ops::BitOr;
use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, COLOR_3DSHADOW, COLOR_WINDOW, EndPaint, FillRect, HBRUSH, HDC, PAINTSTRUCT};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExA, CS_DROPSHADOW, CS_HREDRAW, CW_USEDEFAULT, DefWindowProcA, DispatchMessageA, GetMessageA, MSG, PostQuitMessage, RegisterClassExA, ShowWindow, SW_NORMAL, TranslateMessage, WM_DESTROY, WM_PAINT, WNDCLASSEXA, WS_OVERLAPPEDWINDOW};

fn main() {
    const CLASS_NAME: PCSTR = s!("imgv");
    let wndclass = windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXA {
        cbSize: mem::size_of::<WNDCLASSEXA>() as u32,
        style: CS_DROPSHADOW.bitor(CS_HREDRAW),
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: Default::default(),
        hIcon: Default::default(),
        hCursor: Default::default(),
        hbrBackground: Default::default(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: CLASS_NAME,
        hIconSm: Default::default(),
    };

    unsafe { RegisterClassExA(&wndclass); }

    let hwnd = unsafe { CreateWindowExA(Default::default(), CLASS_NAME, s!("Test window"), WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, None, None, None, None) };

    unsafe { ShowWindow(hwnd, SW_NORMAL); }

    unsafe {
        let mut message: MSG = MSG::default();
        let msg = &mut message as *mut MSG;
        while GetMessageA(msg, None, 0, 0).as_bool() {
            TranslateMessage(msg);
            DispatchMessageA(msg);
        }
    }
}

unsafe extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_DESTROY => {PostQuitMessage(0); LRESULT::default()},
        WM_PAINT => {
            let mut p: PAINTSTRUCT = PAINTSTRUCT::default();
            let ps: *mut PAINTSTRUCT = &mut p as *mut PAINTSTRUCT;
            let hdc: HDC = BeginPaint(window, ps);

            let rect: *const RECT = &(*ps).rcPaint as *const RECT;

            FillRect(hdc, rect, HBRUSH { 0: (COLOR_3DSHADOW.0 + 1) as isize });

            EndPaint(window, ps);
            LRESULT::default()
        }
        _ => {DefWindowProcA(window, message, wparam, lparam)}
    }

}
