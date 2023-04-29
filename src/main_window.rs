use windows::core::{Error, PCWSTR};
use windows::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Direct2D::{D2D1_ELLIPSE, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1CreateFactory, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1SolidColorBrush};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_SIZE_U};
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, InvalidateRect, PAINTSTRUCT};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, GetClientRect, PostQuitMessage, WM_CREATE, WM_DESTROY, WM_PAINT, WM_SIZE};

use crate::base_window::BaseWindow;

pub struct MainWindow {
    hwnd: HWND,
    factory: Option<ID2D1Factory>,
    render_target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    ellipse: Option<D2D1_ELLIPSE>,
}

impl MainWindow {

    pub fn new() -> Self {
        MainWindow {
            hwnd: Default::default(),
            factory: None,
            render_target: None,
            brush: None,
            ellipse: None,
        }
    }

    fn calculate_layout(&mut self) {
        if let Some(render_target) = &self.render_target {
            let size = unsafe { render_target.GetSize() };
            let x = size.width / 2 as f32;
            let y = size.height / 2 as f32;
            let radius = f32::min(x, y);
            self.ellipse = Some(D2D1_ELLIPSE {
                point: D2D_POINT_2F { x, y },
                radiusX: radius,
                radiusY: radius,
            })
        }
    }

    fn create_graphics_resources(&mut self) -> Result<(), Error> {
        match &self.render_target {
            None => {
                let mut rc = RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                };
                unsafe { GetClientRect(self.hwnd, &mut rc as *mut _ as _); };

                let size = D2D_SIZE_U {
                    width: rc.right as u32,
                    height: rc.bottom as u32,
                };

                let target = unsafe {
                    self.factory.as_ref().unwrap().CreateHwndRenderTarget(
                        &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                            hwnd: Default::default(),
                            pixelSize: Default::default(),
                            presentOptions: Default::default(),
                        } as *const _ as _,
                        &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                            hwnd: self.hwnd,
                            pixelSize: size,
                            presentOptions: Default::default(),
                        })?
                };
                self.render_target = Some(target);
                let color = D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.0,
                };
                let solid_color_brush = unsafe { self.render_target.as_ref().unwrap().CreateSolidColorBrush(&color, None)? };
                self.brush = Some(solid_color_brush);
                self.calculate_layout();
                Ok(())
            }
            Some(_) => { Ok(()) }
        }
    }

    fn resize(&mut self) {
        if let Some(render_target) = &self.render_target {
            let mut rc = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };

            unsafe { GetClientRect(self.hwnd, (&mut rc) as _); };

            let size = D2D_SIZE_U {
                width: rc.right as u32,
                height: rc.bottom as u32,
            };

            unsafe { render_target.Resize(&size) }.expect("Resize Failed");
            self.calculate_layout();
            unsafe { InvalidateRect(self.hwnd, None, BOOL::from(false)) };
        }
    }

    fn on_paint(&mut self) {
        if let Ok(_) = self.create_graphics_resources() {
            let mut ps = PAINTSTRUCT {
                hdc: Default::default(),
                fErase: Default::default(),
                rcPaint: Default::default(),
                fRestore: Default::default(),
                fIncUpdate: Default::default(),
                rgbReserved: [0; 32],
            };
            unsafe {
                BeginPaint(self.hwnd, &mut ps);

                self.render_target.as_ref().unwrap().BeginDraw();

                self.render_target.as_ref().unwrap().Clear(Some(&D2D1_COLOR_F{
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                }));

                self.render_target.as_ref().unwrap().FillEllipse(self.ellipse.as_ref().unwrap(), self.brush.as_ref().unwrap());

                self.render_target.as_ref().unwrap().EndDraw(None, None).unwrap_or(());
                EndPaint(self.hwnd, &ps);
            }
        }
    }
}

impl BaseWindow for MainWindow {
    type T = MainWindow;

    fn get_class_name() -> PCWSTR {
        w!("Circle Window Class")
    }

    // fn new() -> Self {
    //     MainWindow {
    //         hwnd: Default::default(),
    //         factory: None,
    //         render_target: None,
    //         brush: None,
    //         ellipse: None,
    //     }
    // }

    fn handle_message(self: &mut Self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_CREATE => {
                match unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) } {
                    Ok(create_factory) => {
                        self.factory = Some(create_factory);
                        LRESULT(0)
                    }
                    Err(_) => { LRESULT(-1) }
                }
            }
            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
            }
            WM_PAINT => {
                self.on_paint();
                LRESULT(0)
            }
            WM_SIZE => {
                self.resize();
                LRESULT(0)
            }
            _ => { unsafe { DefWindowProcW(window, message, wparam, lparam) } }
        }
    }

    fn set_hwnd(self: &mut Self, hwnd: HWND) {
        self.hwnd = hwnd;
    }

    fn get_hwnd(self: &Self) -> HWND {
        self.hwnd
    }
}