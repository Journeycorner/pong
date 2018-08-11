extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput,
                         Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};

mod pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use pong::Pong;

    let path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), false)?;
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();
    Ok(())
}

