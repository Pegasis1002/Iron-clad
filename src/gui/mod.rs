
unsafe extern "C" {
fn gui_init(w: i32, h: i32);
fn gui_update(vram: *const u32);
fn gui_should_close() -> bool;
fn gui_terminate();
fn gui_get_key() -> u32;
fn gui_get_char() -> u32;
}

pub struct Screen;

impl Screen {
    pub fn new(w: i32, h:i32) -> Self {
        unsafe { gui_init(w, h); }
        Screen
    }

    pub fn refresh(&self, vram: &[u32]) {
        unsafe { gui_update(vram.as_ptr()); }
    }

    pub fn is_open(&self) -> bool {
        unsafe { !gui_should_close() }
    }

    pub fn get_key_pressed(&self) -> Option<u32>  {
        let key =unsafe { gui_get_key() };

        if key == 0 {
            None
        } else {
            Some(key)
        }
    }

    pub fn get_char_pressed(&self) -> Option<u32>  {
        let key = unsafe { gui_get_char() };

        if key == 0 {
            None
        } else {
            Some(key)
        }

    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            gui_terminate();
        }
    }
}
