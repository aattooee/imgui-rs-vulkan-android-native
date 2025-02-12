pub mod common;
mod imgui_rs_fix;
use common::*;
use imgui::*;
use imgui_rs_fix::ImguiFixedFunctions;


use std::error::Error;

const APP_NAME: &str = "hello world";
static WHITE_OUTER: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 255, 191);
fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    simple_logger::SimpleLogger::new().init()?;
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    System::new(APP_NAME)?.run((), move |run, ui| {
        ui.window("HEllo world")
            .opened(run)
            .size([440.0, 320.0], Condition::FirstUseEver)
            .build(|| {


                ui.text_wrapped("Hello world!");
                ui.text_wrapped("你好世界！");
                ui.text_wrapped("こんにちは世界！");
                ui.text_wrapped("我喜歡看書。");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }
                
                
                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));

                ui.separator();
                ui.text_colored([1.0, 1.0, 1.0, 1.0], format!("fps : {}", ui.io().framerate));
            });
            let draw_list = ui.get_background_draw_list();
                draw_list.add_text_with_font_size([440.0,320.0], WHITE_OUTER, "你好", 20.0);
    })?;

    Ok(())
}
