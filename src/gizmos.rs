use bevy::{color::palettes::css::*, prelude::*};
use rand::Rng;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Initialize gizmo points resource
    let mut points = Vec::new();
    for _ in 0..10 {
        points.push(GizmoPoint {
            base_x: rand::thread_rng().gen_range(-300.0..300.0),
            base_y: rand::thread_rng().gen_range(-200.0..200.0),
            speed: rand::thread_rng().gen_range(0.5..3.0),
            offset: rand::thread_rng().gen_range(0.0..6.40),
            radius: rand::thread_rng().gen_range(50.0..150.0),
            color: match rand::thread_rng().gen_range(0..5) {
                0 => BLUE.into(),
                1 => RED.into(),
                2 => GREEN.into(),
                3 => YELLOW.into(),
                _ => PURPLE.into(),
            },
        });
    }

    commands.insert_resource(GizmoPoints { points });
}

#[derive(Resource)]
struct GizmoPoints {
    points: Vec<GizmoPoint>,
}

struct GizmoPoint {
    base_x: f32,
    base_y: f32,
    speed: f32,
    offset: f32,
    radius: f32,
    color: Color,
}

fn draw_beautiful_gizmos(mut gizmos: Gizmos, time: Res<Time>, points: Res<GizmoPoints>) {
    for point in &points.points {
        let t = time.elapsed_secs() * point.speed + point.offset;
        let x = point.base_x + point.radius * t.sin();
        let y = point.base_y + point.radius * t.cos();

        // Draw main circle
        gizmos.circle_2d(Vec2::new(x, y), 15.0, point.color);

        // Draw orbit trail
        gizmos.circle_2d(
            Vec2::new(point.base_x, point.base_y),
            point.radius,
            Color::srgba(
                point.color.to_srgba().red,
                point.color.to_srgba().green,
                point.color.to_srgba().blue,
                0.3,
            ),
        );

        // Draw line from center to point
        gizmos.line_2d(
            Vec2::new(point.base_x, point.base_y),
            Vec2::new(x, y),
            point.color,
        );
    }

    // Connect all points with lines for web effect
    for i in 0..points.points.len() {
        for j in (i + 1)..points.points.len() {
            let point1 = &points.points[i];
            let point2 = &points.points[j];

            let t1 = time.elapsed_secs() * point1.speed + point1.offset;
            let t2 = time.elapsed_secs() * point2.speed + point2.offset;

            let pos1 = Vec2::new(
                point1.base_x + point1.radius * t1.sin(),
                point1.base_y + point1.radius * t1.cos(),
            );
            let pos2 = Vec2::new(
                point2.base_x + point2.radius * t2.sin(),
                point2.base_y + point2.radius * t2.cos(),
            );

            let distance = pos1.distance(pos2);
            if distance < 200.0 {
                let alpha = 1.0 - (distance / 200.0);
                gizmos.line_2d(pos1, pos2, Color::srgba(1.0, 1.0, 1.0, alpha * 0.3));
            }
        }
    }
}

fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut points: ResMut<GizmoPoints>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let x = cursor_pos.x - (window.width() / 2.0);
                let y = (window.height() / 2.0) - cursor_pos.y;

                // Check if click is near any gizmo point
                points.points.retain(|point| {
                    let t = time.elapsed_secs() * point.speed + point.offset;
                    let gizmo_x = point.base_x + point.radius * t.sin();
                    let gizmo_y = point.base_y + point.radius * t.cos();

                    let distance = ((x - gizmo_x).powi(2) + (y - gizmo_y).powi(2)).sqrt();

                    if distance < 30.0 {
                        info!("Gizmo destroyed at ({}, {})", gizmo_x, gizmo_y);
                        false // Remove this point
                    } else {
                        true // Keep this point
                    }
                });

                // Spawn new point to replace destroyed one
                if points.points.len() < 10 {
                    points.points.push(GizmoPoint {
                        base_x: rand::thread_rng().gen_range(-300.0..300.0),
                        base_y: rand::thread_rng().gen_range(-200.0..200.0),
                        speed: rand::thread_rng().gen_range(0.5..3.0),
                        offset: rand::thread_rng().gen_range(0.0..6.28),
                        radius: rand::thread_rng().gen_range(50.0..150.0),
                        color: match rand::thread_rng().gen_range(0..5) {
                            0 => BLUE.into(),
                            1 => RED.into(),
                            2 => GREEN.into(),
                            3 => YELLOW.into(),
                            _ => PURPLE.into(),
                        },
                    });
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_beautiful_gizmos, handle_click))
        .run();
}
