use map::{Map, TileType};
use map_builders::random_builder;
use rect::*;
use nannou::prelude::*;

mod map;
mod rect;
mod map_builders;

fn main() {
    nannou::app(model).run()
}

struct Model {
    builder_name: String,
    snapshots: Vec<Map>
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .size(1024, 768)
        .title("Dungeon")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();

    let mut map_builder = random_builder();
    map_builder.build();

    Model {
        builder_name: map_builder.name().to_owned(),
        snapshots: map_builder.get_snapshot_history()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = &app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let mut y = 0;
    let mut x = 0;
    
    let index = if app.time < model.snapshots.len() as f32 {
        (app.time * 2.0) as usize
    } else {
        model.snapshots.len()
    };

    let map = &model.snapshots.get(index).or(model.snapshots.last()).unwrap();

    for tile in map.tiles.iter() {
        let color = match tile {
            TileType::Wall => GREEN,
            TileType::Room => BLUE,
            TileType::Tunnel => RED,
        };

        draw_tile(x as f32, y as f32, color)(draw);

        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }

    draw.text(model.builder_name.as_str())
        .x_y(0.0, 300.0)
        .font_size(32)
        .color(WHITE);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_tile(x: f32, y: f32, color: Rgb<u8>) -> impl Fn(&Draw) -> () {
    return move |draw| {
            draw.quad()
            .x_y(x * 10.0 - 375.0, y * 10.0 - 250.0)
            .w(10.0)
            .h(10.0)
            .color(color);
        }
}