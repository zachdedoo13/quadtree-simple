use quadtree_simple::*;
use macroquad::prelude::*;



#[macroquad::main("BasicShapes")]
async fn main() {
    let mut qt:Quadtree<bool> = Quadtree::new(Qrect::screen_size(screen_width(), screen_height()), 4);


    loop {
        clear_background(BLACK);

        // inset points
        if is_mouse_button_down(MouseButton::Left) {
            let x = mouse_position().0;
            let y = mouse_position().1;
            qt.insert(&Point::new(x, y, false));
        }
        
        // clear the quadtree
        if is_key_pressed(KeyCode::Space) {
            qt = Quadtree::new(Qrect::screen_size(screen_width(), screen_height()), 4);
        }

        // query points
        if is_mouse_button_down(MouseButton::Right) {
            let x = mouse_position().0;
            let y = mouse_position().1;
            let check = qt.query_circle(x, y, 50.);
            for point in check {
                draw_circle(point.x, point.y, 2.0, GREEN);
            }
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
