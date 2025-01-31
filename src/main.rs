pub mod common;

use common::*;
use imgui::*;

use std::error::Error;

const APP_NAME: &str = "hello world";

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
                // let info = android_native_window::safe_get_display_info();
                // ui.get_foreground_draw_list().add_line([0.0,0.0], [info.width as f32,info.height as f32], [1.0,1.0,1.0]).thickness(4.0).build();

                ui.text_wrapped("Hello world!");
                ui.text_wrapped("你好世界！");
                ui.text_wrapped("こんにちは世界！");
                ui.text_wrapped("안녕하세요, 세계!");
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
    })?;

    Ok(())
}
