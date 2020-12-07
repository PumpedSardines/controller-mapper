use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::wingdi::*;
use winapi::ctypes::*;
use winapi::um::xinput::*;
use winapi::shared::winerror::*;

use std::mem::{MaybeUninit};
use enigo::*;

mod window;
use window::*;

mod mouse;
use mouse::{
    Mouse
};

mod xbox;
use xbox::*;

mod keyboard;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref X_CONN: Mutex<bool> = Mutex::new(false);
    static ref MOUSE_MODE: Mutex<bool> = Mutex::new(true);
    static ref SHIFT_MODE: Mutex<u8> = Mutex::new(0);

    static ref SELECTED_X: Mutex<i32> = Mutex::new(3);
    static ref SELECTED_Y: Mutex<i32> = Mutex::new(4);
    static ref MOVE_TIMER: Mutex<std::time::Instant> = Mutex::new(std::time::Instant::now());

    static ref FRAME_RATE: Mutex<f64> = Mutex::new(0.0);
}


fn main() {
    let mut mouse: Mouse = Mouse::New();
    let mut xbox: XboxController = XboxController::New(0);

    let mut wnd_x: f64 = unsafe {GetSystemMetrics(SM_CXSCREEN)} as f64 / 2.0 - 353.0;
    let mut wnd_y: f64 = unsafe {GetSystemMetrics(SM_CYSCREEN)} as f64 - 356.0;

    xbox.set_dead_zone(Thumb {
        x: 0.1,
        y: 0.1
    });

    let mut win = unsafe { Window::New(Some(window_proc)).unwrap() };
    unsafe {
        win.set_pos(wnd_x as i32,wnd_y as i32);
        win.set_size(706,256);
        win.set_show(!*MOUSE_MODE.lock().unwrap());
    }

    loop {
        let start_time = std::time::Instant::now();
        let delta_time = *FRAME_RATE.lock().unwrap() / 1000000.0;
        
        let found_controller = xbox.load();
        let mouse_mode = *MOUSE_MODE.lock().unwrap();

        if !found_controller && *X_CONN.lock().unwrap() {
            *X_CONN.lock().unwrap() = false;
            unsafe {
                MessageBoxW(
                    win.handle,
                    win32_string("Xbox controller disconnected, please reconnect to continue using the program").as_ptr(),
                    win32_string("XKey").as_ptr(),
                    MB_OK | MB_ICONERROR
                );
            }
        }else if found_controller {
            *X_CONN.lock().unwrap() = true;
        }
        


        let mut message: MSG = MSG { hwnd: 0 as HWND, message: 0, wParam: 0, lParam: 0, time: 0,pt: POINT{ x:0 , y:0 } };
        unsafe {
            if PeekMessageW( &mut message as *mut MSG, win.handle, 0, 0, 1 ) > 0 {
                TranslateMessage( &message as *const MSG );
                DispatchMessageW( &message as *const MSG );
            }
        }

        let mut speed = 2.0 * delta_time;
        if xbox.x.pressed {
            speed = 0.5 * delta_time;
        }

        let mut scroll_speed = 0.01 * delta_time;
        if xbox.rb_thumb {
            scroll_speed = 0.05 * delta_time;
        }

        if mouse_mode {
            mouse.mov(xbox.l_thumb.x * speed, xbox.l_thumb.y * speed);
            mouse.scroll(xbox.r_thumb.x * scroll_speed, xbox.r_thumb.y * scroll_speed * -1.0);
    
            if xbox.a.pressed {
                mouse.l_down();
            }else {
                mouse.l_up();
            }
    
            if xbox.b.pressed {
                mouse.r_down();
            }else {
                mouse.r_up();
            }
        }else {
            let speed = 2.0 * delta_time;
            let movx = xbox.r_thumb.x * speed;
            let movy = xbox.r_thumb.y * speed;

            if xbox.r_thumb.x != 0.0 || xbox.r_thumb.y != 0.0 {
                wnd_x += movx;
                wnd_y -= movy;
                unsafe { win.set_pos(wnd_x as i32,wnd_y as i32) }
            }

            let key_speed = MOVE_TIMER.lock().unwrap().elapsed().as_nanos() as f64;
            let can_move = key_speed / 1000000.0 > 200.0;


            if xbox.l_thumb.x < -0.5 && can_move {
                *SELECTED_X.lock().unwrap() -= 1;
                *MOVE_TIMER.lock().unwrap() = std::time::Instant::now();
                unsafe { win.rerender(); };
            }
            if xbox.l_thumb.x > 0.5 && can_move {
                *SELECTED_X.lock().unwrap() += 1;
                *MOVE_TIMER.lock().unwrap() = std::time::Instant::now();
                unsafe { win.rerender(); };
            }

            if xbox.l_thumb.y < -0.5 && can_move {
                *SELECTED_Y.lock().unwrap() += 1;
                *MOVE_TIMER.lock().unwrap() = std::time::Instant::now();
                unsafe { win.rerender(); };
            }
            if xbox.l_thumb.y > 0.5 && can_move {
                *SELECTED_Y.lock().unwrap() -= 1;
                *MOVE_TIMER.lock().unwrap() = std::time::Instant::now();
                unsafe { win.rerender(); };
            }

            if xbox.a.down {
                let sel_x = *SELECTED_X.lock().unwrap();
                let sel_y = *SELECTED_Y.lock().unwrap();

                let mut sel_but: keyboard::Button = keyboard::Button {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    font_size: 0,
                    on_click: &|_| {},
                    text: &|_| "t",
                };
    
                for button in keyboard::SWEDISH_KEYBOARD.iter() {
                    if sel_x >= button.x && sel_y >= button.y && sel_x < button.x + button.width && sel_y < button.y + button.height {
                        sel_but = button.clone();
                    }
                }

                unsafe { (*sel_but.on_click)(*SHIFT_MODE.lock().unwrap() > 0); }

                if *SHIFT_MODE.lock().unwrap() != 2 {
                    *SHIFT_MODE.lock().unwrap() = 0;
                    unsafe { win.rerender(); };
                }
            }

            if xbox.b.down {
                let previous_mode = *SHIFT_MODE.lock().unwrap();
                *SHIFT_MODE.lock().unwrap() = match previous_mode {
                    1 => 2,
                    0 => 1,
                    _ => 0,

                };
                unsafe { win.rerender(); };
            }

            if xbox.x.down {
                let mut enigo = Enigo::new();
                enigo.key_click(Key::Backspace);
            }

            if xbox.start.down {
                let mut enigo = Enigo::new();
                enigo.key_click(Key::Return);
            }
        }
        
        if xbox.y.down {
            *MOUSE_MODE.lock().unwrap() = !mouse_mode;
            //mouse.set_show(!mouse_mode, win.handle);
            win.set_show(mouse_mode);
        }

        *FRAME_RATE.lock().unwrap() = start_time.elapsed().as_nanos() as f64;
    }
}


unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            let hdcBuffer = CreateCompatibleDC (hdc);  // OFF screen DC
            let hBitmapBuffer = CreateCompatibleBitmap (hdc,706, 256);  // create memory bitmap for that off screen DC
            SelectObject(hdcBuffer,hBitmapBuffer as HGDIOBJ);

            let dark_one = RGB(20,20,30);
            let dark_two = RGB(33,39,48);
            let white_one = RGB(220,220,220);


            let dark_brush = CreateSolidBrush(dark_one);
            FillRect(hdcBuffer, &mut ps.rcPaint, dark_brush);
            DeleteObject(dark_brush as HGDIOBJ);

            let buttons = keyboard::SWEDISH_KEYBOARD;

            for button in buttons.iter() {
                let mut cur_rect = RECT {
                    left: 4 + 50 * button.x,
                    top: 4 + 50 * button.y,
                    right: 4 + 50 * button.x + 46 + 50 * (button.width - 1),
                    bottom: 4 + 50 * button.y + 46 + 50 * (button.height - 1),
                };

                let dark_brush = CreateSolidBrush(dark_two);
                FillRect(hdcBuffer, &cur_rect, dark_brush);
                DeleteObject(dark_brush as HGDIOBJ);

                let font = CreateFontW(
                    button.font_size,
                    0,
                    GM_COMPATIBLE as i32,
                    0,
                    300,
                    0,
                    0,
                    0,
                    ANSI_CHARSET,
                    OUT_DEFAULT_PRECIS,
                    CLIP_DEFAULT_PRECIS,
                    CLEARTYPE_QUALITY,
                    DEFAULT_PITCH,
                    win32_string("Microsoft Yahei").as_ptr()
                );

                SelectObject(hdcBuffer, font as HGDIOBJ);
                SetTextColor(hdcBuffer, white_one);
                SetBkColor(hdcBuffer, dark_two);

                let output_text = &*(*button.text)(*SHIFT_MODE.lock().unwrap() > 0);

                DrawTextW(
                    hdcBuffer,
                    win32_string(output_text).as_ptr(),
                    output_text.chars().count() as i32,
                    &mut cur_rect,
                    DT_VCENTER | DT_SINGLELINE  | DT_CENTER
                );
                DeleteObject(font as HGDIOBJ);
            }

            let sel_x = *SELECTED_X.lock().unwrap();
            let sel_y = *SELECTED_Y.lock().unwrap();

            let mut sel_but: keyboard::Button = keyboard::Button {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                font_size: 0,
                on_click: &|_| {},
                text: &|_| "t",
            };

            for button in buttons.iter() {
                if sel_x >= button.x && sel_y >= button.y && sel_x < button.x + button.width && sel_y < button.y + button.height {
                    sel_but = button.clone();
                }
            }

            FrameRect(
                hdcBuffer,
                &RECT {
                    left: 3 + 50 * sel_but.x,
                    top: 3 + 50 * sel_but.y,
                    right: 5 + 50 * sel_but.x + 46 + 50 * (sel_but.width - 1),
                    bottom: 5 + 50 * sel_but.y + 46 + 50 * (sel_but.height - 1),
                },
                CreateSolidBrush(RGB(255,255,255))
            );

            BitBlt(hdc,0,0,706,256,hdcBuffer,0,0,SRCCOPY); // copy the content of OFF screen DC to actual screen DC
            DeleteDC (hdcBuffer); // Release the OFF screen DC
            DeleteObject (hBitmapBuffer as HGDIOBJ);

            EndPaint(hwnd, &ps);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam)
    }
    0
}