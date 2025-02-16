use windows::Win32::UI::Input::KeyboardAndMouse::*;

// So what we will do is keep two bools
// first is the last toggle state we saw on the key
// second is whether we saw the toggle state and want to set to true for we are toggled
// Calling functions should set the second bool to false if they performed an operation using the bool
#[derive(Debug)]
pub struct Keys {
    pub up: (bool, bool),
    pub down: (bool, bool),
    pub left: (bool, bool),
    pub right: (bool, bool),
    pub k: (bool, bool),
    pub enter: (bool, bool),
}

impl Keys {
    pub fn new() -> Keys {
        let up;
        let down;
        let left;
        let right;
        let k;
        let enter;

        unsafe {
            up = (GetKeyState(VK_UP.0 as i32) & 1) != 0;
            down = (GetKeyState(VK_DOWN.0 as i32) & 1) != 0;
            left = (GetKeyState(VK_LEFT.0 as i32) & 1) != 0;
            right = (GetKeyState(VK_RIGHT.0 as i32) & 1) != 0;
            k = (GetKeyState(VK_K.0 as i32) & 1) != 0;
            enter = (GetKeyState(VK_RETURN.0 as i32) & 1) != 0;
        }

        Keys {
            up: (up, false),
            down: (down, false),
            left: (left, false),
            right: (right, false),
            k: (k, false),
            enter: (enter, false),
        }
    }

    pub fn check_keys_toggle(&mut self) {
        let up;
        let down;
        let left;
        let right;
        let k;
        let enter;

        unsafe {
            up = (GetKeyState(VK_UP.0 as i32) & 1) != 0;
            down = (GetKeyState(VK_DOWN.0 as i32) & 1) != 0;
            left = (GetKeyState(VK_LEFT.0 as i32) & 1) != 0;
            right = (GetKeyState(VK_RIGHT.0 as i32) & 1) != 0;
            k = (GetKeyState(VK_K.0 as i32) & 1) != 0;
            enter = (GetKeyState(VK_RETURN.0 as i32) & 1) != 0;
        }
        set_val(&mut self.up, up);
        set_val(&mut self.down, down);
        set_val(&mut self.left, left);
        set_val(&mut self.right, right);
        set_val(&mut self.k, k);
        set_val(&mut self.enter, enter);
    }

    // This will iterate thru all the keys and see if any were pressed
    pub fn was_key_pressed(&self) -> bool {
        if self.up.1 == true {
            return true;
        } else if self.down.1 == true {
            return true;
        } else if self.left.1 == true {
            return true;
        } else if self.right.1 == true {
            return true;
        } else if self.k.1 == true {
            return true;
        } else if self.enter.1 == true {
            return true;
        }

        false
    }
}

fn set_val(val: &mut (bool, bool), check: bool) {
    if val.0 != check {
        val.0 = check;
        val.1 = true;
    }
}

// loop {
//     unsafe {
//         // let keys: &mut [u8; 256] = &mut [0; 256];
//         // GetKeyboardState(keys);
//         // \x1b[2K erases the whole line
//         print!("\x1b[1;1H\x1b[2K");
//         let key = GetKeyState(VK_UP.0 as i32);
//         let b1 = key & 0x8000u16 as i16;
//         let b2 = key & 0x1;
//         print!("{:x}, {:x}", b1, b2);
//         print!("\x1b[2;1H\x1b[2K");
//         print!("{}", GetKeyState(VK_DOWN.0 as i32));
//         print!("\x1b[3;1H\x1b[2K");
//         print!("{}", GetKeyState(VK_LEFT.0 as i32));
//         print!("\x1b[4;1H\x1b[2K");
//         print!("{}", GetKeyState(VK_RIGHT.0 as i32));
//         println!("");
//     }
// }
