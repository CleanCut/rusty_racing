use rusty_engine::gfx::{GameEvent, Img, Window};
use rusty_engine::glm::{self, Vec2};

fn main() {
    let mut window = Window::new(None, "Rusty Racing");
    let mut car_img = Img::new(
        &window,
        Vec2::new(0.0, 0.6),
        0.2,
        1. / 16.,
        None,
        "resources/car_orange.png",
    );
    'gameloop: loop {
        for game_event in window.poll_game_events() {
            if let GameEvent::Quit = game_event {
                break 'gameloop;
            }
        }

        window.drawstart();
        window.draw(&mut car_img);
        window.drawfinish();

        let final_direction = 3.0 * std::f32::consts::PI;
        if car_img.direction >= final_direction {
            car_img.direction = final_direction;
            continue;
        }

        car_img.direction += 0.03;
        car_img.scale += 0.002;
        car_img.pos += glm::vec2(0., -0.002);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
