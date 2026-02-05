
unsafe extern "C" {
fn gui_init(w: i32, h: i32);
fn gui_update(vram: *const u32);
fn gui_should_close() -> bool;
fn gui_terminate();
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
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            gui_terminate();
        }
    }
}
