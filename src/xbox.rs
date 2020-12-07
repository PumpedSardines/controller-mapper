use winapi::shared::winerror::*;
use winapi::um::xinput::*;
use std::mem::{MaybeUninit};

pub unsafe fn get_availble() -> (bool,bool,bool,bool) {
	let mut exist = (false,false,false,false);

	let mut state: XINPUT_STATE = MaybeUninit::uninit().assume_init();
	let code = XInputGetState(0, &mut state);
	if code == ERROR_SUCCESS { exist.0 = true; }

	let mut state: XINPUT_STATE = MaybeUninit::uninit().assume_init();
	let code = XInputGetState(1, &mut state);
	if code == ERROR_SUCCESS { exist.1 = true; }

	let mut state: XINPUT_STATE = MaybeUninit::uninit().assume_init();
	let code = XInputGetState(2, &mut state);
	if code == ERROR_SUCCESS { exist.2 = true; }

	let mut state: XINPUT_STATE = MaybeUninit::uninit().assume_init();
	let code = XInputGetState(3, &mut state);
	if code == ERROR_SUCCESS { exist.3 = true; }
	
	return exist;
}

#[derive(Clone)]
pub struct Thumb {
	pub x: f64,
	pub y: f64
}

impl Thumb {
	pub fn New() -> Thumb {
		Thumb {
			x: 0.0,
			y: 0.0
		}
	}
}

#[derive(Clone)]
pub struct Button {
	pub pressed: bool,
	pub up: bool,
	pub down: bool
}

impl Button {
	pub fn New() -> Button {
		Button {
			up: false,
			down: false,
			pressed: false
		}
	}

	fn update(&self, current: bool) -> Button {
		Button {
			up: !current && self.pressed,
			pressed: current,
			down: current && !self.pressed,
		}
	}
}



pub struct XboxController {
	pub port: u32,
	pub r_dead_zone: Thumb,
	pub l_dead_zone: Thumb,
	pub l_trigger: f64,
	pub r_trigger: f64,
	pub l_thumb: Thumb,
	pub r_thumb: Thumb,
	pub a: Button,
	pub b: Button,
	pub x: Button,
	pub y: Button,
	pub pad_left: Button,
	pub pad_right: Button,
	pub pad_up: Button,
	pub pad_down: Button,
	pub r_shoulder: bool,
	pub l_shoulder: bool,
	pub lb_thumb: bool,
	pub rb_thumb: bool,
	pub back: Button,
	pub start: Button
}

impl XboxController {
	pub fn New(port: u32) -> XboxController {
		XboxController {
			port,
			r_dead_zone: Thumb::New(),
			l_dead_zone: Thumb::New(),
			l_trigger: 0.0,
			r_trigger: 0.0,
			l_thumb: Thumb::New(),
			r_thumb: Thumb::New(),
			a: 			Button::New(),
			b: 			Button::New(),
			x: 			Button::New(),
			y: 			Button::New(),
			pad_left:  	Button::New(),
			pad_right:	Button::New(),
			pad_up:   	Button::New(),
			pad_down:  	Button::New(),
			r_shoulder: false,
			l_shoulder: false,
			lb_thumb: false,
			rb_thumb: false,
			back: Button::New(),
			start: Button::New(),
		}
	}

	pub fn set_dead_zone(&mut self, dead_zone: Thumb) {
		self.r_dead_zone = dead_zone.clone();
		self.l_dead_zone = dead_zone.clone();
	}

	pub fn load(&mut self) -> bool {
		let mut state: XINPUT_STATE = unsafe { MaybeUninit::uninit().assume_init() };
		let code = unsafe { XInputGetState(self.port, &mut state) };

		if code != ERROR_SUCCESS {
			return false;
		}

		// Buttons
		self.a = self.a.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_A != 0);
		self.y = self.y.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_Y != 0);
		self.b = self.b.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_B != 0);
		self.x = self.x.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_X != 0);

		// L THUMB
		self.l_thumb = Thumb {
			x: state.Gamepad.sThumbLX as f64 / 32767.0,
			y: state.Gamepad.sThumbLY as f64 / 32767.0
		};

		if self.l_thumb.x < self.l_dead_zone.x && self.l_thumb.x > self.l_dead_zone.x * -1.0 {
			self.l_thumb.x = 0.0;
		}

		if self.l_thumb.y < self.l_dead_zone.y && self.l_thumb.y > self.l_dead_zone.y * -1.0 {
			self.l_thumb.y = 0.0;
		}

		// R THUMB
		self.r_thumb = Thumb {
			x: state.Gamepad.sThumbRX as f64 / 32767.0,
			y: state.Gamepad.sThumbRY as f64 / 32767.0
		};

		if self.r_thumb.x < self.r_dead_zone.x && self.r_thumb.x > self.r_dead_zone.x * -1.0 {
			self.r_thumb.x = 0.0;
		}

		if self.r_thumb.y < self.r_dead_zone.y && self.r_thumb.y > self.r_dead_zone.y * -1.0 {
			self.r_thumb.y = 0.0;
		}

		// Back triggers
		self.l_trigger = state.Gamepad.bLeftTrigger as f64 / 255.0;
		self.r_trigger = state.Gamepad.bRightTrigger as f64 / 255.0;
		self.r_shoulder = state.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_SHOULDER != 0;
		self.l_shoulder = state.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_SHOULDER != 0;

		// DPAD
		self.pad_left = self.pad_left.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT != 0);
		self.pad_right = self.pad_right.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT != 0);
		self.pad_up = self.pad_up.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP != 0);
		self.pad_down = self.pad_down.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN != 0);

		// Thumb buttons
		self.lb_thumb = state.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_THUMB != 0;
		self.rb_thumb = state.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_THUMB	!= 0;

		// Center buttons
		self.back = self.back.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_BACK != 0);
		self.start = self.start.update(state.Gamepad.wButtons & XINPUT_GAMEPAD_START != 0);

		true
	}
}