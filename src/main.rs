extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::{
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawFlat, DrawFlat2D, Flipped, Pipeline, PngFormat, PosNormTex,
        Projection, RenderBundle, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Stage, Texture, TextureMetadata,
    },
    utils::application_root_dir,
};

struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Paddle>();

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Paddle>();

        initialise_paddles(world, sprite_sheet_handle);

        initialise_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
