#[derive(Copy,Clone)]
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
    // is_shift
    pub text:  *const dyn Fn(bool) -> *const str,
    pub on_click: *const dyn Fn(bool) -> ()
}

/*
#[allow(dead_code)]
pub const ENGLISH_US_KEYBOARD: Keyboard = Keyboard { };
*/
use enigo::*;

fn press_key(shift: bool, key: char) {
    let mut enigo = Enigo::new();
    if shift {
        enigo.key_down(Key::Shift);
        enigo.key_click(Key::Layout(key));
        enigo.key_up(Key::Shift);
    } else {
        enigo.key_click(Key::Layout(key));
    }
}

fn press_special_key(key: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(key);
}

struct Keyboard {
    shift: bool,
    x: i32,
    y: i32,
    keys: &'static [Button]
}

const SWEDISH_KEYBOARD_T: Keyboard = Keyboard {
    shift: false,
    x: 0,
    y: 0,
    keys: SWEDISH_KEYBOARD
};

pub const SWEDISH_KEYBOARD: &'static [Button] = &[
    // ROW ONE
    Button {
        x: 0,
        y: 0,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "esc",
        on_click: &|_| press_special_key(Key::Escape),
    },
    Button {
        x: 1,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "!", false => "1"},
        on_click: &|is_shift| press_key(is_shift,'1'),
    },
    Button {
        x: 2,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "\"", false => "2"},
        on_click: &|is_shift| press_key(is_shift,'2'),
    },

    Button {
        x: 3,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "#", false => "3"},
        on_click: &|is_shift| press_key(is_shift,'3'),
    },
    Button {
        x: 4,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "$", false => "4"},
        on_click: &|is_shift| println!("test"),
    },
    Button {
        x: 5,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "%", false => "5"},
        on_click: &|is_shift| press_key(is_shift,'5'),
    },
    Button {
        x: 6,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "@", false => "6"},
        on_click: &|is_shift| println!("test"),
    },
    Button {
        x: 7,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "/", false => "7"},
        on_click: &|is_shift| press_key(is_shift,'7'),
    },
    Button {
        x: 8,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "(", false => "8"},
        on_click: &|is_shift| press_key(is_shift,'8'),
    },
    Button {
        x: 9,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => ")", false => "9"},
        on_click: &|is_shift| press_key(is_shift,'9'),
    },
    Button {
        x: 10,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "=", false => "0"},
        on_click: &|is_shift| press_key(is_shift,'0'),
    },
    
    Button {
        x: 11,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "?", false => "+"},
        on_click: &|is_shift| press_key(is_shift,'+'),
    },
    Button {
        x: 12,
        y: 0,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "£", false => "*"},
        on_click: &|is_shift| println!("test"),
    },

    Button {
        x: 13,
        y: 0,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "copy",
        on_click: &|_| {
            let mut enigo = Enigo::new();

            enigo.key_down(Key::Control);
            enigo.key_click(Key::Layout('c'));
            enigo.key_up(Key::Control);
        },
    },
    Button {
        x: 13,
        y: 1,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "paste",
        on_click: &|_| {
            let mut enigo = Enigo::new();

            enigo.key_down(Key::Control);
            enigo.key_click(Key::Layout('v'));
            enigo.key_up(Key::Control);
        },
    },

    Button {
        x: 13,
        y: 2,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "undo",
        on_click: &|_| {
            let mut enigo = Enigo::new();

            enigo.key_down(Key::Control);
            enigo.key_click(Key::Layout('z'));
            enigo.key_up(Key::Control);
        },
    },

    Button {
        x: 13,
        y: 3,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "redo",
        on_click: &|_| {
            let mut enigo = Enigo::new();

            enigo.key_down(Key::Control);
            enigo.key_click(Key::Layout('y'));
            enigo.key_up(Key::Control);
        },
    },

    Button {
        x: 11,
        y: 2,
        font_size: 18,
        width: 2,
        height: 1,
        text: &|_| "enter",
        on_click: &|_| press_special_key(Key::Return),
    },

    Button {
        x: 11,
        y: 1,
        font_size: 18,
        width: 2,
        height: 1,
        text: &|_| "backspace",
        on_click: &|_| press_special_key(Key::Backspace),
    },

    Button {
        x: 13,
        y: 4,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "left",
        on_click: &|_| press_special_key(Key::LeftArrow),
    },

    Button {
        x: 12,
        y: 4,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "down",
        on_click: &|_| press_special_key(Key::DownArrow),
    },

    Button {
        x: 12,
        y: 3,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "up",
        on_click: &|_| press_special_key(Key::UpArrow),
    },

    Button {
        x: 11,
        y: 4,
        font_size: 18,
        width: 1,
        height: 1,
        text: &|_| "right",
        on_click: &|_| press_special_key(Key::RightArrow),
    },

    Button {
        x: 0,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Q", false => "q"},
        on_click: &|is_shift| press_key(is_shift,'q'),
    },
    Button {
        x: 1,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "W", false => "w"},
        on_click: &|is_shift| press_key(is_shift,'w'),
    },
    Button {
        x: 2,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "E", false => "e"},
        on_click: &|is_shift| press_key(is_shift,'e'),
    },
    Button {
        x: 3,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "R", false => "r"},
        on_click: &|is_shift| press_key(is_shift,'r'),
    },
    Button {
        x: 4,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "T", false => "t"},
        on_click: &|is_shift| press_key(is_shift,'t'),
    },
    Button {
        x: 5,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Y", false => "y"},
        on_click: &|is_shift| press_key(is_shift,'y'),
    },
    Button {
        x: 6,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "U", false => "u"},
        on_click: &|is_shift| press_key(is_shift,'u'),
    },
    Button {
        x: 7,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "I", false => "i"},
        on_click: &|is_shift| press_key(is_shift,'i'),
    },
    Button {
        x: 8,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "O", false => "o"},
        on_click: &|is_shift| press_key(is_shift,'o'),
    },
    Button {
        x: 9,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "P", false => "p"},
        on_click: &|is_shift| press_key(is_shift,'p'),
    },
    Button {
        x: 10,
        y: 1,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Å", false => "å"},
        on_click: &|is_shift| press_key(is_shift,'å'),
    },

    Button {
        x: 0,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "A", false => "a"},
        on_click: &|is_shift| press_key(is_shift,'a'),
    },
    Button {
        x: 1,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "S", false => "s"},
        on_click: &|is_shift| press_key(is_shift,'s'),
    },
    Button {
        x: 2,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "D", false => "d"},
        on_click: &|is_shift| press_key(is_shift,'d'),
    },
    Button {
        x: 3,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "F", false => "f"},
        on_click: &|is_shift| press_key(is_shift,'f'),
    },
    Button {
        x: 4,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "G", false => "g"},
        on_click: &|is_shift| press_key(is_shift,'g'),
    },
    Button {
        x: 5,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "H", false => "h"},
        on_click: &|is_shift| press_key(is_shift,'h'),
    },
    Button {
        x: 6,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "J", false => "j"},
        on_click: &|is_shift| press_key(is_shift,'j'),
    },
    Button {
        x: 7,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "K", false => "k"},
        on_click: &|is_shift| press_key(is_shift,'k'),
    },
    Button {
        x: 8,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "L", false => "l"},
        on_click: &|is_shift| press_key(is_shift,'l'),
    },
    Button {
        x: 9,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Ö", false => "ö"},
        on_click: &|is_shift| press_key(is_shift,'ö'),
    },
    Button {
        x: 10,
        y: 2,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Ä", false => "ä"},
        on_click: &|is_shift| press_key(is_shift,'ä'),
    },

    Button {
        x: 0,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "Z", false => "z"},
        on_click: &|is_shift| press_key(is_shift,'z'),
    },
    Button {
        x: 1,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "X", false => "x"},
        on_click: &|is_shift| press_key(is_shift,'x'),
    },
    Button {
        x: 2,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "C", false => "c"},
        on_click: &|is_shift| press_key(is_shift,'c'),
    },
    Button {
        x: 3,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "V", false => "v"},
        on_click: &|is_shift| press_key(is_shift,'v'),
    },
    Button {
        x: 4,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "B", false => "b"},
        on_click: &|is_shift| press_key(is_shift,'b'),
    },
    Button {
        x: 5,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "N", false => "n"},
        on_click: &|is_shift| press_key(is_shift,'n'),
    },
    Button {
        x: 6,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "M", false => "m"},
        on_click: &|is_shift| press_key(is_shift,'m'),
    },
    Button {
        x: 7,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => ";", false => ","},
        on_click: &|is_shift| press_key(is_shift,','),
    },
    Button {
        x: 8,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => ":", false => "."},
        on_click: &|is_shift| press_key(is_shift,'.'),
    },
    Button {
        x: 9,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "_", false => "-"},
        on_click: &|is_shift| press_key(is_shift,'-'),
    },
    Button {
        x: 10,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => "<", false => "<"},
        on_click: &|is_shift| press_key(is_shift,'w'),
    },
    Button {
        x: 11,
        y: 3,
        font_size: 24,
        width: 1,
        height: 1,
        text: &|is_shift| match is_shift { true => ">", false => ">"},
        on_click: &|is_shift| press_key(is_shift,'w'),
    },

    Button {
        x: 0,
        y: 4,
        font_size: 18,
        width: 2,
        height: 1,
        text: &|is_shift| "shift",
        on_click: &|is_shift| press_key(is_shift,'w'),
    },
    Button {
        x: 2,
        y: 4,
        font_size: 18,
        width: 9,
        height: 1,
        text: &|is_shift| "spacebar",
        on_click: &|is_shift| Enigo::new().key_click(Key::Space),
    },
];