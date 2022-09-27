use std::time::*;
use rand::Rng;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    math::vec2
};

const TIME: u64 = 10;
const TIME_STEP: f32 = 1.0 / 60.0;
const UI_FONT_SIZE: f32 = 40.0;
const STANDARD_TEXT_PADDING: Val = Val::Px(5.0);
const TIMER_Y_TEXT_PADDING: Val = Val::Px(0.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const LOCATION_COLOR: Color = Color::rgb(0.0, 0.9, 0.0);
const CUBE_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const CUBE_SIZE: Vec3 = Vec3::new(60.0, 60.0, 0.0);
const CUBE_SPEED: f32 = 240.0;
const CUBE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "cube".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Score { score: 0 })
        .insert_resource(TargetCount { count: 0 })
        .insert_resource( Time{ time: SystemTime::now()} )
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step((TIME_STEP as f64)))
                .with_system(check_collider)
                .with_system(move_cube.before(check_collider))
                .with_system(spawn_target.after(check_collider))
                .with_system(check_time.after(check_collider).before(spawn_target))
        )
        .add_system(update_timer)
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct TargetCount{
    count: usize,
}

struct Time {
    time: SystemTime,
}

struct Score {
    score: usize,
}

#[derive(Component)]
struct ScoreBored;

#[derive(Component)]
struct TimerBored;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Collider;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    //spawn the initial cube
    commands
        .spawn()
        .insert(Cube)
        .insert_bundle(SpriteBundle { sprite: Sprite{color: CUBE_COLOR, ..default()}, transform: Transform{scale: CUBE_SIZE, translation: CUBE_STARTING_POSITION, ..default()}, ..default()})
        .insert(Collider);

    // Scoreboard
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: UI_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: UI_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Relative,
                position: UiRect {
                    top: STANDARD_TEXT_PADDING,
                    left: STANDARD_TEXT_PADDING,
                    ..default()
                },
                ..default()
            }),

    ).insert(ScoreBored);

    //timer
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Time left: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: UI_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: UI_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Relative,
                position: UiRect {
                    top: TIMER_Y_TEXT_PADDING,
                    left: STANDARD_TEXT_PADDING,
                    ..default()
                },
                ..default()
            }),
    ).insert(TimerBored);
}

fn check_collider(mut commands: Commands,
                  mut Cube_query: Query<(&Transform), With<Cube>>,
                  target_query: Query<(Entity, &Transform, Option<&Target>), With<Collider>>,
                  mut collision_events: EventWriter<CollisionEvent>,
                  mut scoreboard: ResMut<Score>,
                  mut tagetcount: ResMut<TargetCount>,

) {
    let cube_transform = Cube_query.single_mut();
    let cube_size = cube_transform.scale.truncate();

    for (collider_entity, transform, maybe_target) in &target_query {
        let collision = collide(
            cube_transform.translation,
            cube_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            if maybe_target.is_some() {
                //todo add the scoreboard adder
                commands.entity(collider_entity).despawn();

                scoreboard.score += 1;
                tagetcount.count -= 1;


            }
        }
    }

}



fn move_cube(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Cube>>
) {
    //todo stop the cube from leaving the screen
    let mut current_position = query.single_mut();
    let mut direction :Vec2 = vec2(0.0,0.0);


    if keyboard_input.pressed(KeyCode::W) {
        direction.y = 1.0;
        //println!("up")
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y = -1.0;
        //println!("down")
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x = 1.0;
        //println!("right")
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x = -1.0;
        //println!("left");
    }



    let new_cube_position = vec2(current_position.translation.x + direction.x * CUBE_SPEED * TIME_STEP, current_position.translation.y + direction.y * CUBE_SPEED * TIME_STEP);


    current_position.translation.x = new_cube_position.x;//.clamp(0.0, WINDOW_WIDTH);
    current_position.translation.y = new_cube_position.y;//.clamp(0.0, WINDOW_HEIGHT);
}

fn spawn_target(
    mut commands: Commands,
    mut tagetcount: ResMut<TargetCount>,
    mut timer: ResMut<Time>,
) {

    if tagetcount.count == 0 {
        //reset timer
        timer.time = SystemTime::now();

        for i in 0..5 {
            let x: f32 = rand::thread_rng().gen_range(-600..600) as f32;
            let y: f32 = rand::thread_rng().gen_range(-200..200) as f32;

            commands
                .spawn()
                .insert(Target)
                .insert_bundle(SpriteBundle { sprite: Sprite { color: Color::rgb(5.0, 0.0, 0.0), ..default() }, transform: Transform { scale: CUBE_SIZE, translation: Vec3::new(x, y, 1.0), ..default() }, ..default() })
                .insert(Collider);

            tagetcount.count += 1;
        }
    }
}

fn check_time(
    mut commands: Commands,
    mut targetcount: ResMut<TargetCount>,
    mut timer: ResMut<Time>,
    mut query: Query<&mut Transform, With<Cube>>
) {
    let mut player = query.single_mut();

    if SystemTime::now().duration_since(timer.time).unwrap() > Duration::new(TIME, 0) {
        if targetcount.count > 0 {
            panic!("you lose")
        }
    }
}

fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text, With<ScoreBored>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}


fn update_timer(scoreboard: Res<Score>, mut query: Query<&mut Text, With<TimerBored>>, mut timer: ResMut<Time>,) {
    let mut text = query.single_mut();
    let time_taken = SystemTime::now().duration_since(timer.time).unwrap();
    let time_left = Duration::new(TIME, 0) - time_taken;
    let secs_left = time_left.as_secs();
    text.sections[1].value = secs_left.to_string();
}