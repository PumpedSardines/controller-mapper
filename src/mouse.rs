use enigo::*;
use winapi::um::winuser::{
	GetCursorPos,
	SetCursorPos,
	ShowCursor,
	GetSystemMetrics,
	ClipCursor,
	GetWindowRect,

	SM_CXSCREEN,
	SM_CYSCREEN
};

use winapi::shared::windef::{
	POINT,
	RECT,
	HWND
};

use std::mem::{MaybeUninit};
use std::ptr::null_mut;

pub struct Mouse {
	pub cur_x: f64,
	pub cur_y: f64,
	pub cur_scroll_x: f64,
	pub cur_scroll_y: f64,
	pub l_pressed: bool,
	pub r_pressed: bool,
	hide_pos_x: i32,
	hide_pos_y: i32
}

impl Mouse {
	pub fn New() -> Mouse {
		Mouse {
			cur_x: 0.0,
			cur_y: 0.0,
			cur_scroll_x: 0.0,
			cur_scroll_y: 0.0,
			l_pressed: false,
			r_pressed: false,
			hide_pos_x: 0,
			hide_pos_y: 0
		}
	}

	pub fn mov(&mut self, x: f64, y: f64) {
		self.cur_x += x;
		self.cur_y += y;

		let mut cur_pt: POINT = unsafe { MaybeUninit::uninit().assume_init() };
		unsafe { GetCursorPos(&mut cur_pt) };
		
		let new_pos_x = match &self.cur_x {
			d if d > &1.0 => {
				self.cur_x -= 1.0;
				1
			},
			d if d < &-1.0 => {
				self.cur_x += 1.0;
				-1
			}
			_ => 0
		};
		
		let new_pos_y = match &self.cur_y {
			d if d > &1.0 => {
				self.cur_y -= 1.0;
				-1
			},
			d if d < &-1.0 => {
				self.cur_y += 1.0;
				1
			}
			_ => 0
		};

		if new_pos_x != 0 || new_pos_y != 0 {
			unsafe { SetCursorPos(
				cur_pt.x + new_pos_x,
				cur_pt.y + new_pos_y,
			); }
		}
	}

	pub fn scroll(&mut self, x: f64, y: f64) {
		self.cur_scroll_x += x;
		self.cur_scroll_y += y;
		
		let new_pos_x = match &self.cur_scroll_x {
			d if d > &1.0 => {
				self.cur_scroll_x -= 1.0;
				1
			},
			d if d < &-1.0 => {
				self.cur_scroll_x += 1.0;
				-1
			}
			_ => 0
		};
		
		let new_pos_y = match &self.cur_scroll_y {
			d if d > &1.0 => {
				self.cur_scroll_y -= 1.0;
				-1
			},
			d if d < &-1.0 => {
				self.cur_scroll_y += 1.0;
				1
			}
			_ => 0
		};

		if new_pos_x != 0 || new_pos_y != 0 {
			let mut enigo = Enigo::new();
			enigo.mouse_scroll_y(new_pos_y);
			enigo.mouse_scroll_x(new_pos_x);
		}
	}

	pub fn l_down(&mut self) {
		let mut enigo = Enigo::new();

		if !self.l_pressed {
			self.l_pressed = true;
			enigo.mouse_down(MouseButton::Left);
		}
	}

	pub fn l_up(&mut self) {
		let mut enigo = Enigo::new();
		
		if self.l_pressed {
			self.l_pressed = false;
			enigo.mouse_up(MouseButton::Left);
		}
	}

	pub fn r_down(&mut self) {
		let mut enigo = Enigo::new();

		if !self.r_pressed {
			self.r_pressed = true;
			enigo.mouse_down(MouseButton::Right);
		}
	}

	pub fn r_up(&mut self) {
		let mut enigo = Enigo::new();
		
		if self.r_pressed {
			self.r_pressed = false;
			enigo.mouse_up(MouseButton::Right);
		}
	}

	pub fn set_show(&mut self, show: bool, hwnd: HWND) {
		if !show {
			let mut cur_pt: POINT = unsafe { MaybeUninit::uninit().assume_init() };
			unsafe { GetCursorPos(&mut cur_pt) };
			self.hide_pos_x = cur_pt.x;
			self.hide_pos_y = cur_pt.y;
			let mut screen: RECT = unsafe { MaybeUninit::uninit().assume_init() };
			unsafe { 
				GetWindowRect(hwnd, &mut screen);
				ClipCursor(&screen);
				ShowCursor(0); 
			};
		} else {
			unsafe {
				ClipCursor(null_mut());
				ShowCursor(1);
				SetCursorPos(
					self.hide_pos_x,
					self.hide_pos_y,
				);
			}
		}
	}
}