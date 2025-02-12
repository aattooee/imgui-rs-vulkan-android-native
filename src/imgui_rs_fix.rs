use imgui::*;

pub trait ImguiFixedFunctions {
    fn add_text_with_font_size(&self,
        pos: impl Into<mint::Vector2<f32>>,
        col: impl Into<ImColor32>,
        text: impl AsRef<str>,
        font_size:f32
    );
}
impl ImguiFixedFunctions for DrawListMut<'_> {
    fn add_text_with_font_size(&self,
        pos: impl Into<mint::Vector2<f32>>,
        col: impl Into<ImColor32>,
        text: impl AsRef<str>,
        font_size:f32
    ) {
        unsafe {
            let text = text.as_ref();
            let start = text.as_ptr() as *const std::ffi::c_char;
            let end = (start as usize + text.len()) as *const std::ffi::c_char;
            sys::ImDrawList_AddText_FontPtr(sys::igGetBackgroundDrawList(), std::ptr::null(), font_size, pos.into().into(), col.into().into(), start, end, 0.0, std::ptr::null());
        }
    }
}