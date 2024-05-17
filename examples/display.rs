use quadtree_simple::*;
use macroquad::prelude::*;
use macroquad::rand::*;



#[macroquad::main("BasicShapes")]
async fn main() {
    let mut qt:Quadtree<bool> = Quadtree::new(Qrect::screen_size(screen_width(), screen_height()), 4);


    loop {
        clear_background(BLACK);

        if is_mouse_button_down(MouseButton::Left) {
            let x = mouse_position().0;
            let y = mouse_position().1;
            qt.insert(&Point::new(x, y, false));
        }
        if is_key_pressed(KeyCode::Space) {
            qt = Quadtree::new(Qrect::screen_size(screen_width(), screen_height()), 4);
        }


        let rects = qt.get_rects();
        let points = qt.collect();

        for rect in rects {
            draw_rectangle_lines(rect.x - rect.w, rect.y - rect.h, rect.w * 2., rect.h * 2., 1.0, BLUE);
        }
        for point in points {
            draw_circle(point.x, point.y, 0.5, RED);
        }

        next_frame().await
    }
}