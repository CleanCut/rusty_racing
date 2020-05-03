use rusty_engine::gfx::{ShapeStyle, Sprite};
use rusty_engine::glm::{self, Vec2};
use rusty_engine::prelude::{Color, GameEvent, Window};

fn main() {
    let mut window = Window::new(None, "Rusty Racing");
    let mut sprites = Vec::new();
    // Left 3 line boxes

    // Top left: purple line box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(-0.6, 1.2),
        0.0,
        1. / 16.,
        0.25,
        0.5,
        Color::new(0.8, 0.6, 1.0),
        ShapeStyle::Line,
    ));
    // Middle left: blue line box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(-0.6, 0.6),
        0.0,
        1. / 16.,
        0.75,
        0.5,
        Color::new(0.4, 0.6, 1.0),
        ShapeStyle::Line,
    ));
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(-0.6, 0.6),
        0.0,
        1. / 16.,
        0.75,
        0.5,
        Color::new(0.4, 0.6, 1.0),
        ShapeStyle::Line,
    ));
    // Bottom left: teal line box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(-0.6, 0.0),
        0.0,
        1. / 16.,
        1.0,
        0.5,
        Color::new(0.4, 1.0, 1.0),
        ShapeStyle::Line,
    ));

    // Right 3 fill boxes

    // Top right: red fill box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(0.6, 1.2),
        0.0,
        1. / 16.,
        0.75,
        0.75,
        Color::new(1.0, 0.2, 0.3),
        ShapeStyle::Fill,
    ));
    // Middle right: blue fill box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(0.6, 0.6),
        0.0,
        1. / 16.,
        0.5,
        0.5,
        Color::new(0.0, 1.0, 1.0),
        ShapeStyle::Fill,
    ));
    // Bottom right: green fill box
    sprites.push(Sprite::new_rectangle(
        &window,
        Vec2::new(0.6, 0.0),
        0.0,
        1. / 16.,
        0.35,
        0.35,
        Color::new(0.5, 1.0, 0.5),
        ShapeStyle::Fill,
    ));

    // Middle: circles

    // Top middle: orange circle
    sprites.push(Sprite::new_circle(
        &window,
        Vec2::new(0.0, 1.2),
        0.0,
        1.0 / 16.,
        0.30,
        Color::new(1.0, 0.647, 0.0),
        ShapeStyle::Line,
    ));
    // Bottom middle: yellow circle
    sprites.push(Sprite::smooth_circle(
        &window,
        Vec2::new(0.0, 0.0),
        0.0,
        1.0 / 100.,
        Color::new(1.0, 1.0, 0.2),
    ));

    // Top middle:

    // Center: Orange Car
    sprites.push(Sprite::new_image(
        &window,
        Vec2::new(0.0, 0.6),
        0.0,
        1. / 16.,
        None,
        "resources/car_orange.png",
    ));
    sprites.push(Sprite::new_image(
        &window,
        Vec2::new(0.0, 0.6),
        0.,
        1. / 16.,
        None,
        "resources/car_orange.png",
    ));
    let final_direction = 3.0 * std::f32::consts::PI;
    'gameloop: loop {
        for game_event in window.poll_game_events() {
            if let GameEvent::Quit = game_event {
                break 'gameloop;
            }
        }

        window.drawstart();
        for sprite in sprites.iter_mut() {
            sprite.draw(&mut window);
        }
        window.drawfinish();

        for sprite in sprites.iter_mut() {
            if sprite.transform.direction >= final_direction {
                sprite.transform.direction = final_direction;
                continue;
            }
            sprite.transform.direction += 0.03;
            sprite.transform.scale += 0.002;
            sprite.transform.pos += glm::vec2(0., -0.002);
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
