use macroquad::prelude::*;

#[macroquad::main("Paint")]
async fn main() {
    let mut lines = Vec::new();
    let mut previous_pos = mouse_position();
    loop {
        clear_background(WHITE);
        draw_text("Paint ", 10., 20., 20., BLACK);
        let position = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let line = (
                previous_pos.0, //previous x
                previous_pos.1, //previous y
                position.0,     //current x
                position.1,     //current y
            );
            lines.push(line);
        }

        previous_pos = position;

        for (x1, y1, x2, y2) in &lines {
            draw_line(*x1, *y1, *x2, *y2, 5., PURPLE);
        }

        next_frame().await
    }
}
