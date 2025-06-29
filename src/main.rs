use bevy::prelude::*;

// making a simple click game so first i need once a create a Sprite i need to save its postion so
// when i click on the sprite with the left mouse they got ditruced

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let font = assets.load("fonts/PixelatedEleganceRegular-ovyAA.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((Text2d::new("Rotate"), text_font.clone(), AnimateRotation));
    commands.spawn((
        Text2d::new("Transform"),
        text_font.clone(),
        AnimateTranslation,
    ));

    commands.spawn((Text2d::new("Rotate"), text_font.clone(), AnimateRotation));
    commands.spawn((
        Text2d::new("Scale"),
        text_font.clone(),
        Transform::from_xyz(288.0, 288.0, 0.0),
        AnimateScale,
    ));
    let spong = assets.load("spong_Ass.png");
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(200.0, 200.0)),
            image: spong.clone(),
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
        AnimateSpong,
    ));
}
#[derive(Component)]
struct FadingSprite {
    timer: Timer,
}
#[derive(Component)]
struct AnimateRotation;
#[derive(Component)]
struct AnimateScale;
#[derive(Component)]
struct AnimateSpong;
#[derive(Component)]
struct AnimateTranslation;
#[derive(Component)]
struct Distroy;

// Click at sprite to distroy them
// so i need a component to distroy every clicking thing So
// first step i have to see when i click on some sprite it get dispawnde
// spawning at mouse position
fn spawn_in_mouse_position(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ms: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    if ms.just_pressed(MouseButton::Right) {
        if let Ok(window) = windows.single() {
            if let Some(pos) = window.cursor_position() {
                info!("Mouse at: {:?}", pos);
                let x = pos.x - (window.width() / 2.0);
                let y = (window.height() / 2.0) - pos.y;
                commands.spawn((
                    Sprite::from_image(asset_server.load("hello.png")),
                    Transform::from_xyz(x, y, 0.0),
                    FadingSprite {
                        timer: Timer::from_seconds(3.0, TimerMode::Once),
                    },
                ));
            }
        }
    } else {
        info_once!("Just press with the right mouse button");
    }
}

// ECS system to handle fading and despawning sprites
fn fade_sprites_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FadingSprite, &mut Sprite)>,
) {
    for (entity, mut fading, mut sprite) in query.iter_mut() {
        // Tick the timer
        fading.timer.tick(time.delta());

        // Calculate alpha based on timer progress (1.0 = fully visible, 0.0 = invisible)
        let alpha = 1.0 - fading.timer.fraction();
        sprite.color.set_alpha(alpha);

        // Despawn the entity when timer is finished
        if fading.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
fn animate_roattion(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
    }
}
fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        info!("{}", transform.translation.x);
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
        info!("{}", transform.translation.y);
    }
}
fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut TextFont, (With<Text2d>, With<AnimateScale>)>,
) {
    for mut size in &mut query {
        size.font_size = ops::cos(time.elapsed_secs()) * 50.0 + 100.0;
    }
}
fn animate_spong(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Sprite>, With<AnimateSpong>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::sin(time.elapsed_secs()));
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
                    let sprite_width = 200.0;
                    let sprite_height = 200.0;

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
        .add_systems(Update, spawn_in_mouse_position)
        .add_systems(Update, fade_sprites_system)
        .add_systems(
            Update,
            (
                animate_roattion,
                animate_translation,
                animate_scale,
                animate_spong,
                distroy_if_clicked_left,
            )
                .chain(),
        )
        .run();
}
