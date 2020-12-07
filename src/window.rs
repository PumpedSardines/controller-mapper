use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::mem::{MaybeUninit};

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub struct Window {
    pub handle: HWND,
}

impl Window {
	pub unsafe fn New(
		proc: Option<unsafe extern "system" fn(HWND,u32,usize,isize) -> isize>
	) -> Option<Window> {
		let name = win32_string("name");
		let title = win32_string("title");

		let hinstance = GetModuleHandleW(null_mut());

		// Lowkey no idea what this does exactly, just followed the msdn documentation
		let wnd_class = WNDCLASSW {
			style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
			lpfnWndProc: proc,
			hInstance: hinstance,
			lpszClassName: name.as_ptr(),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hIcon: null_mut(),
			hCursor: null_mut(),
			hbrBackground: null_mut(),
			lpszMenuName: null_mut(),
		};

		RegisterClassW(&wnd_class);

		let handle = CreateWindowExW(
			WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_COMPOSITED, // Keps the window on top
			name.as_ptr(),
			title.as_ptr(),
			WS_POPUPWINDOW, // This makes the window borderless
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			0 as HWND,
			0 as HMENU,
			hinstance,
			0 as LPVOID,
		);

		Some(Window { 
			handle,
		})
	}

	pub unsafe fn get_size(&mut self) -> (i32, i32) {
        let mut rect: RECT = MaybeUninit::uninit().assume_init();
        GetWindowRect(self.handle, &mut rect);

        (rect.left - rect.right, rect.top - rect.bottom)
    }

    pub unsafe fn get_pos(&mut self) -> (i32, i32) {
        let mut rect: RECT = MaybeUninit::uninit().assume_init();
        GetWindowRect(self.handle, &mut rect);

        (rect.left, rect.top)
	}
	
	pub unsafe fn set_pos(&mut self, x: i32, y: i32) -> () {
		SetWindowPos(
			self.handle,
			HWND_TOPMOST,
			x,
			y,
			0,
			0,
			SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOREDRAW
		);
	}

	pub unsafe fn set_size(&mut self, width: i32, height: i32) -> () {
		SetWindowPos(
			self.handle,
			HWND_TOPMOST,
			0,
			0,
			width,
			height,
			SWP_SHOWWINDOW | SWP_NOMOVE 
		);
	}

	pub fn set_show(&mut self, show: bool) {
		let mut flag = SW_HIDE;
		let current_show = unsafe { IsWindowVisible(self.handle) != 0 };

		if show {
			flag = SW_RESTORE;
		}
		
		if current_show == show {
			return ();
		}

		unsafe { ShowWindow(self.handle,flag) };
	}

    pub unsafe fn rerender(&mut self) {
        InvalidateRect(self.handle, 0 as LPRECT, 1);
    }
}