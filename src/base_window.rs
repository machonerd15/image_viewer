use std::mem;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CREATESTRUCTW, CreateWindowExW, DefWindowProcW, GetWindowLongPtrW, GWLP_USERDATA, HCURSOR, HICON, HMENU, RegisterClassExW, SetWindowLongPtrW, WINDOW_EX_STYLE, WINDOW_STYLE, WM_CREATE, WNDCLASS_STYLES, WNDCLASSEXW};

pub trait BaseWindow {
    type T: BaseWindow;

    fn get_class_name() -> PCWSTR;
    // fn new() -> Self;
    fn handle_message(self: &mut Self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    fn set_hwnd(self: &mut Self, hwnd: HWND);
    fn get_hwnd(self: &Self) -> HWND;
    fn get_window(self: &Self) -> HWND {Self::get_hwnd(self)}

    unsafe extern "system" fn window_proc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let this: *mut Self::T;
        if message == WM_CREATE {
            let create = lparam.0 as *const CREATESTRUCTW;
            this = (*create).lpCreateParams as *mut Self::T;
            SetWindowLongPtrW(window, GWLP_USERDATA, this as _);

            Self::T::set_hwnd(this.as_mut().unwrap(), window);
        } else {
            this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self::T;
        }

        if let Some(p_this) = this.as_mut() {
            Self::T::handle_message(p_this, window, message, wparam, lparam)
        } else {
            DefWindowProcW(window, message, wparam, lparam)
        }
    }



    fn create(
        self: &Self,
        window_name: PCWSTR,
        style: WINDOW_STYLE,
        ex_style: WINDOW_EX_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: HWND,
        menu: HMENU,
    ) -> bool {
        let wc = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: Self::get_wndclass_style(),
            lpfnWndProc: Some(Self::window_proc),
            cbClsExtra: Self::get_cls_extra(),
            cbWndExtra: Self::get_wnd_extra(),
            hInstance: Self::get_instance(),
            hIcon: Self::get_icon(),
            hCursor: Self::get_cursor(),
            hbrBackground: Self::get_background(),
            lpszMenuName: Self::get_menu_name(),
            lpszClassName: Self::get_class_name(),
            hIconSm: Self::get_icon_sm(),
        };
        assert_ne!(unsafe { RegisterClassExW(&wc) }, 0);

        let mut lpparam = Box::new(self);

        let window = unsafe {
            CreateWindowExW(
                ex_style,
                Self::get_class_name(),
                window_name,
                style,
                x,
                y,
                width,
                height,
                parent,
                menu,
                Self::get_instance(),
                Some(lpparam.as_mut() as *mut _ as _),
            )
        };
        if window.0 == 0 {return false}
        true
    }

    fn get_wndclass_style() -> WNDCLASS_STYLES {
        Default::default()
    }

    fn get_cls_extra() -> i32 {0}

    fn get_wnd_extra() -> i32 {0}

    fn get_instance() -> HMODULE {
        unsafe {
            match GetModuleHandleW(None){
                Ok(module) => {module}
                Err(_) => {Default::default()}
            }
        }
    }

    fn get_icon() -> HICON {Default::default()}

    fn get_cursor() -> HCURSOR {Default::default()}

    fn get_background() -> HBRUSH {Default::default()}

    fn get_menu_name() -> PCWSTR {PCWSTR::null()}

    fn get_icon_sm() -> HICON {Default::default()}
}



// impl BaseWindow<T=()> {
//     unsafe extern "system" fn window_proc(
//         window: HWND,
//         message: u32,
//         wparam: WPARAM,
//         lparam: LPARAM,
//     ) -> LRESULT {
//         let p_this: *mut T;
//         if message == WM_CREATE {
//             let p_create = lparam.0 as *const CREATESTRUCTA;
//             p_this = (*p_create).lpCreateParams as *mut T;
//             SetWindowLongPtrA(window, GWLP_USERDATA, p_this as isize);
//
//             (*p_this).m_hwnd = window;
//         } else {
//             p_this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut T;
//         }
//
//         if let Some(p_this) = p_this.as_mut() {
//             p_this.handle_message(window, message, wparam, lparam)
//         } else {
//             DefWindowProcA(window, message, wparam, lparam)
//         }


        // if (p_this as isize) == 0 {
        //     return DefWindowProcA(window, message, wparam, lparam);
        // }
        // return p_this.handle_message(message, wparam, lparam);
        //
        // match message {
        //     WM_DESTROY => {PostQuitMessage(0); LRESULT::default()},
        //     WM_PAINT => {
        //         let mut p: PAINTSTRUCT = PAINTSTRUCT::default();
        //         let ps: *mut PAINTSTRUCT = &mut p as *mut PAINTSTRUCT;
        //         let hdc: HDC = BeginPaint(window, ps);
        //
        //         let rect: *const RECT = &(*ps).rcPaint as *const RECT;
        //
        //         FillRect(hdc, rect, HBRUSH { 0: (COLOR_3DSHADOW.0 + 1) as isize });
        //
        //         EndPaint(window, ps);
        //         LRESULT::default()
        //     }
        //     _ => {DefWindowProcA(window, message, wparam, lparam)}
        // }
//
//     }
// }