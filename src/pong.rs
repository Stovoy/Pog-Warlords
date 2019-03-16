use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        Camera, Flipped, Material, MaterialDefaults, Mesh, PngFormat, PosNormTex, Projection,
        Shape, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use std::f32::consts::PI;

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialize_camera(world: &mut World) {
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

#[derive(PartialEq, Copy, Clone, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
            min_angle: match side {
                Side::Left => -4.0 * PI / 3.0,
                Side::Right => -PI / 3.0,
            },
            max_angle: match side {
                Side::Left => 4.0 * PI / 3.0,
                Side::Right => PI / 3.0,
            },
        }
    }
}

pub const PADDLE_HEIGHT: f32 = 4.0;
pub const PADDLE_WIDTH: f32 = 4.0;

/// initializes one paddle on the left, and one paddle on the right.
fn initialize_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
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

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

/// initializes one ball in the middle-ish of the arena.
fn initialize_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

/// initializes a ui scoreboard
fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { p1_score, p2_score });
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_ball(world, sprite_sheet_handle.clone());

        initialize_paddles(world, sprite_sheet_handle);

        initialize_camera(world);

        initialize_scoreboard(world);

        initialize_circles(world);
    }
}

const CIRCLE_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn initialize_circles(world: &mut World) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let circle_size = 50.0;

    let (mesh, material) = create_circle(world, CIRCLE_COLOR, circle_size);
    world
        .create_entity()
        .with(local_transform.clone())
        .with(mesh)
        .with(material)
        .build();

    let (mesh, material) = create_circle(world, BACKGROUND_COLOR, circle_size - 1.0);
    world
        .create_entity()
        .with(local_transform)
        .with(mesh)
        .with(material)
        .build();
}

fn create_circle(world: &mut World, color: [f32; 4], size: f32) -> (Handle<Mesh>, Material) {
    let loader = world.read_resource::<Loader>();
    let mesh: Handle<Mesh> = loader.load_from_data(
        Shape::Circle(128).generate::<Vec<PosNormTex>>(Some((size, size, size))),
        (),
        &world.read_resource(),
    );
    let albedo = color.into();

    let tex_storage = world.read_resource();
    let material_defaults = world.read_resource::<MaterialDefaults>();
    let albedo = loader.load_from_data(albedo, (), &tex_storage);
    let material = Material {
        albedo,
        ..material_defaults.0.clone()
    };

    (mesh, material)
}
