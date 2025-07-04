use bevy::{color::palettes::css::BLUE, prelude::*};
use rand::Rng;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Load font for score display
    let font = assets.load("fonts/PixelatedEleganceRegular-ovyAA.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    // Spawn score text
    commands.spawn((
        Text2d::new("Score: 0"),
        text_font.clone(),
        TextColor(BLUE.into()),
        Transform::from_xyz(-400.0, 350.0, 0.0),
        ScoreText,
    ));

    // Spawn initial SpongeBob sprites
    for _ in 0..8 {
        spawn_spongebob(&mut commands, &assets);
    }
}

#[derive(Component)]
struct AnimateTranslation {
    speed: f32,
    offset: f32,
    radius_x: f32,
    radius_y: f32,
}

#[derive(Component)]
struct SpongeBobSprite;

#[derive(Debug, Component)]
struct Grab;
struct RandomCoordinates {
    x: f32,
    y: f32,
}

impl RandomCoordinates {
    fn new() -> Self {
        Self {
            x: rand::thread_rng().gen_range(-800.0..800.0),
            y: rand::thread_rng().gen_range(-800.0..800.0),
        }
    }
}

#[derive(Resource)]
struct Score(pub usize);

#[derive(Component)]
struct ScoreText;

fn spawn_spongebob(commands: &mut Commands, assets: &Res<AssetServer>) {
    let coords = RandomCoordinates::new();

    commands.spawn((
        Sprite {
            image: assets.load("spong_Ass.png"),
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        Transform::from_xyz(coords.x, coords.y, 0.0),
        AnimateTranslation {
            speed: rand::thread_rng().gen_range(1.0..10.0),
            offset: rand::thread_rng().gen_range(-400.0..800.0),
            radius_x: rand::thread_rng().gen_range(50.0..300.0),
            radius_y: rand::thread_rng().gen_range(50.0..300.0),
        },
        SpongeBobSprite,
    ));
}

// Animate SpongeBob sprites in circular/elliptical patterns
fn animate_translation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &AnimateTranslation), With<SpongeBobSprite>>,
) {
    for (mut transform, animation) in &mut query {
        let t = time.elapsed_secs() * animation.speed + animation.offset;
        transform.translation.x = animation.radius_x * t.sin() + -400.0;
        transform.translation.y = animation.radius_y * t.cos();
    }
}

// Handle clicking on SpongeBob sprites
fn destroy_if_clicked_left(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<SpongeBobSprite>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to world coordinates
                let world_x = cursor_pos.x - (window.width() / 2.0);
                let world_y = (window.height() / 2.0) - cursor_pos.y;

                for (transform, entity) in &query {
                    let sprite_x = transform.translation.x;
                    let sprite_y = transform.translation.y;

                    // SpongeBob sprite hitbox (adjust based on your sprite size)
                    let sprite_width = 64.0; // Adjust to match your sprite
                    let sprite_height = 64.0; // Adjust to match your sprite

                    // Calculate sprite boundaries
                    let left = sprite_x - (sprite_width / 2.0);
                    let right = sprite_x + (sprite_width / 2.0);
                    let top = sprite_y + (sprite_height / 2.0);
                    let bottom = sprite_y - (sprite_height / 2.0);

                    // Check if click is inside sprite
                    if world_x >= left && world_x <= right && world_y >= bottom && world_y <= top {
                        // Destroy the clicked sprite
                        commands.entity(entity).despawn();

                        // Spawn a new SpongeBob sprite
                        spawn_spongebob(&mut commands, &assets);

                        // Increase score
                        score.0 += 1;
                        info!("SpongeBob caught! Score: {}", score.0);

                        // Only destroy one sprite per click
                        break;
                    }
                }
            }
        }
    }
}

// Update score display
fn update_score(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.3, 0.5)))
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, destroy_if_clicked_left, update_score),
        )
        .run();
}
