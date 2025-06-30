use bevy::{color::palettes::css::BLUE, prelude::*};
use rand::Rng;

// making a simple click game so first i need once a create a Sprite i need to save its postion so
// when i click on the sprite with the left mouse they got ditruced

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
}
#[derive(Component)]
struct AnimateTranslation;
#[derive(Component)]
struct Distroy;
#[derive(Debug)]
struct RandomCorinates {
    x: f32,
    y: f32,
}
impl RandomCorinates {
    fn new() -> Self {
        Self {
            x: rand::thread_rng().gen_range(-500.0..500.0),
            y: rand::thread_rng().gen_range(-500.0..500.0),
        }
    }
}

// Click at sprite to distroy them
// so i need a component to distroy every clicking thing So
// first step i have to see when i click on some sprite it get dispawnde
// spawning at mouse position
fn spawn_in_random_position(mut commands: Commands, assets: Res<AssetServer>) {
    for _i in 0..3 {
        let font = assets.load("fonts/PixelatedEleganceRegular-ovyAA.ttf");
        let text_font = TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        };
        let radom_fuck = RandomCorinates::new();
        commands.spawn((
            Text2d::new("Catch Me"),
            text_font.clone(),
            TextColor(BLUE.into()),
            Transform::from_xyz(radom_fuck.x, radom_fuck.y, 0.0),
            AnimateTranslation,
        ));
    }
}

// ECS system to handle fading and despawning sprites
fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        info_once!("{}", transform.translation.x);
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
        info_once!("{}", transform.translation.y);
    }
}
fn distroy_if_clicked_left(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity)>,
    ms: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    for (transform, entity) in query.iter_mut() {
        if ms.just_pressed(MouseButton::Left) {
            if let Ok(window) = windows.single() {
                if let Some(pos) = window.cursor_position() {
                    info!("Mouse at: {:?}", pos);
                    let x = pos.x - (window.width() / 2.0);
                    let y = (window.height() / 2.0) - pos.y;
                    // The sprite's center position is:
                    //    // how to get the sprite position and  i just a simple
                    // if statment i think i just need some basic knwodldge
                    // + i didint define a Component for distroying Sprites
                    let sprite_x = transform.translation.x;
                    let sprite_y = transform.translation.y;

                    // Hardcode sprite size for now (adjust to match your sprite size)
                    let sprite_width = 80.0;
                    let sprite_height = 80.0;

                    // Calculate sprite boundaries
                    let sprite_left = sprite_x - (sprite_width / 2.0);
                    let sprite_right = sprite_x + (sprite_width / 2.0);
                    let sprite_top = sprite_y + (sprite_height / 2.0);
                    let sprite_bottom = sprite_y - (sprite_height / 2.0);

                    // Check if click is inside sprite
                    if x >= sprite_left
                        && x <= sprite_right
                        && y >= sprite_bottom
                        && y <= sprite_top
                    {
                        commands.entity(entity).despawn();
                        info!("Sprite destroyed!");
                    }
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_translation,
                distroy_if_clicked_left,
                spawn_in_random_position,
            )
                .chain(),
        )
        .run();
}
