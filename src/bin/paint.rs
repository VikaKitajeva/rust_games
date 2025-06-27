use macroquad::prelude::*;

#[macroquad::main("Paint")]
async fn main() {
    let mut points = Vec::new();
    loop {
        clear_background(WHITE);
        draw_text("Paint ", 10., 20., 20., BLACK);
        if is_mouse_button_down(MouseButton::Left) {
            let position = mouse_position();
            points.push(position);
        }
        if points.len() > 2 {
            for i in 0..points.len() - 2 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[i + 1];
                draw_line(x1, y1, x2, y2, 5., PURPLE);
            }
        }
        next_frame().await
    }
}
