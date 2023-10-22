use bevy::prelude::*;
use bevy::math::*;
use bevy::window::WindowResized;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use std::f32::consts::*;

// constants
const PLAYER_SPEED: f32 = 400.0;
const GRAVITY: f32 = 9.81 * 100.0;
const BOUNCE_CONST: f32 = 0.2;
const RESPONSE_CONST: f32 = 1.0;
const LINEAR_FRICTION_CONST: f32 = 0.95;
const ROT_FRICTION_CONST: f32 = 0.20;
const MARGIN:f32 = 2.0;

const LEFT_WALL: f32 = -200.;
const RIGHT_WALL: f32 = 200.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

const FRUIT_N: usize = 11;
const FRUIT_RADII: [f32; FRUIT_N] = [
    10.0,
    20.0,
    30.0,
    40.0,
    50.0,
    60.0,
    70.0,
    80.0,
    90.0,
    100.0,
    110.0,
];
// const FRUIT_COLORS: [Color; FRUIT_N] = [
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
//     Color::rgb(1.0, 0.0, 0.0),
// ];
const FRUIT_HUE: [f32; FRUIT_N] = [
    0.0,
    10.0,
    20.0,
    30.0,
    40.0,
    50.0,
    60.0,
    70.0,
    80.0,
    90.0,
    100.0,
];



#[derive(Component)]
struct FruitIterator{
    next_id: u32,
    next_group: u8,
}

#[derive(Component)]
struct Fruit {
    id: u32,
    group: u8, // in range 0..=11
    pos: Vec2,
    pos_last: Vec2,
    vel: Vec2,
    acc: Vec2,
    a_pos: f32,
    a_vel: f32,
    a_acc: f32,
    radius: f32,
    color: Color,
}

// impl Fruit{
//     fn setVelocity(Vec2, )
//     {
//         position_last = position - (v * dt);
//     }
// }

#[derive(Component)]
struct Player;
// struct Player {
    // pos: Pos,
    // fruit: Fruit,
// }

// Wall code from Rust Brick Breaker example
#[derive(Component)]
struct Collider;
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}


fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            input_handler, 
            apply_gravity,
            apply_collisions,
            apply_constraint,
            physics_update,
            update_sprites,
        ))
        .run();

}

fn setup(mut commands: Commands){
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle{
            transform: Transform { 
                translation: vec3(0.0, TOP_WALL, 0.0),
                ..default()
                // rotation: (), scale: () 
            },
            sprite: Sprite{
                color: Color::rgb(0.3, 0.3, 0.7),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            ..default()
        },
        Player,
        FruitIterator{
            next_id: 0,
            next_group: rng.gen_range(0..5),
        },
    ));

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    // commands.spawn(WallBundle::new(WallLocation::Top));

}

fn spawn_fruit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fruit_iterator: Mut<'_, FruitIterator>,
    player_translation: Vec3,
    asset_server: Res<AssetServer>,
){
    let fruit_icon = asset_server.load("fruit_icon.png");
    let mut rng = rand::thread_rng();
    commands.spawn((
        // MaterialMesh2dBundle {
        //     transform: Transform { 
        //         translation: vec3(player_translation.x, player_translation.y, 0.0),
        //         ..default()
        //         // rotation: (), scale: () 
        //     },
        //     mesh: meshes.add(shape::Circle::new(FRUIT_RADII[fruit_iterator.next_group as usize]).into()).into(),
        //     material: materials.add(ColorMaterial::from(FRUIT_COLORS[fruit_iterator.next_group as usize])),
        //     ..default()
        // },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(2.0*FRUIT_RADII[fruit_iterator.next_group as usize])),
                color: Color::hsla(FRUIT_HUE[fruit_iterator.next_group as usize], 0.9, 0.6, 1.0),
                ..default()
            },
            texture: fruit_icon.clone(),
            transform: Transform { 
                translation: vec3(player_translation.x, player_translation.y, 0.0),
                rotation: Quat::from_rotation_z(FRAC_PI_4), // 45 degree rotation
                ..default()
                // rotation: (), scale: () 
            },
            ..default()
        },
        Fruit{
            id: fruit_iterator.next_id,
            group: fruit_iterator.next_group,
            pos: Vec2{
                x: player_translation.x,
                y: player_translation.y,
            },
            pos_last: Vec2{
                x: player_translation.x,
                y: player_translation.y,
            },
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            a_pos: FRAC_PI_4,
            a_vel: 0.0,
            a_acc: 0.0,
            color: Color::RED,
            radius: FRUIT_RADII[fruit_iterator.next_group as usize],
        },
    ));
    fruit_iterator.next_id += 1;
    fruit_iterator.next_group = rng.gen_range(0..5);
}

fn input_handler(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<(&mut Transform, &mut FruitIterator), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let (mut player_transform, mut fruit_iterator) = query.single_mut();
    
    let mut direction: f32 = 0.0;
    if input.pressed(KeyCode::A){
        direction -= 1.0;
    }
    if input.pressed(KeyCode::D){
        direction += 1.0;
    }
    if input.just_pressed(KeyCode::Space) {
        spawn_fruit(commands, meshes, materials, fruit_iterator, player_transform.translation, asset_server);
    }

    let new_x: f32 = player_transform.translation.x + direction * PLAYER_SPEED * time_step.period.as_secs_f32();

    player_transform.translation.x = new_x;
}

fn apply_gravity(
    time_step: Res<FixedTime>,
    mut fruit_query: Query<&mut Fruit>,    
){
    let mut fruits: Vec<_> = fruit_query.iter_mut().collect();
    for i in 0..fruits.len() {
        fruits[i].acc.y -= GRAVITY;
    }
}

fn apply_collisions(
    time_step: Res<FixedTime>,
    mut fruit_query: Query<&mut Fruit>,
){
    let mut fruits: Vec<_> = fruit_query.iter_mut().collect();
    let mut r_ij: Vec2 = Vec2::ZERO;
    let mut r_ij_mag: f32 = 0.0;
    let mut r_ij_hat: Vec2 = Vec2::ZERO;
    let mut v_ij: Vec2 = Vec2::ZERO;
    let mut min_dist: f32 = 0.0;
    let mut ratio_i: f32 = 0.0;
    let mut ratio_j: f32 = 0.0;
    let mut delta: f32 = 0.0;
    let dt = time_step.period.as_secs_f32();
    if fruits.len() < 2{
        return;
    }
    for i in 0..(fruits.len()-1) {
        for j in (i+1)..fruits.len() {
            r_ij = fruits[j].pos - fruits[i].pos;
            r_ij_mag = r_ij.length();
            min_dist = fruits[j].radius + fruits[i].radius;
            if r_ij_mag < min_dist{
                r_ij_hat = r_ij / r_ij_mag;
                ratio_i = fruits[i].radius / min_dist;
                ratio_j = fruits[j].radius / min_dist;
                delta =  0.5 * RESPONSE_CONST * (r_ij_mag - min_dist);

                fruits[i].pos += r_ij_hat * (ratio_j * delta);
                fruits[j].pos -= r_ij_hat * (ratio_i * delta);
                fruits[i].vel += r_ij_hat * (ratio_j * delta) / dt;
                fruits[j].vel -= r_ij_hat * (ratio_i * delta) / dt;

                fruits[i].a_acc -= ROT_FRICTION_CONST * ratio_j *((fruits[i].vel - fruits[j].vel).perp_dot(r_ij_hat) + fruits[i].a_vel*fruits[i].radius - fruits[j].a_vel*fruits[j].radius);
                fruits[j].a_acc += ROT_FRICTION_CONST * ratio_i *((fruits[i].vel - fruits[j].vel).perp_dot(r_ij_hat) + fruits[i].a_vel*fruits[i].radius - fruits[j].a_vel*fruits[j].radius);

                // println!("{:?}, {:?}", fruits[i].a_acc, fruits[j].a_acc);
            }
        }
    }
}

fn apply_constraint(
    time_step: Res<FixedTime>,
    mut fruit_query: Query<&mut Fruit>, 
){

    let mut fruits: Vec<_> = fruit_query.iter_mut().collect();
    for i in 0..fruits.len() {
        if (fruits[i].pos.y - fruits[i].radius) < (BOTTOM_WALL + WALL_THICKNESS/2.0){
            fruits[i].pos.y = BOTTOM_WALL + WALL_THICKNESS/2.0 + fruits[i].radius;
            fruits[i].vel.y = -fruits[i].vel.y * BOUNCE_CONST;
            fruits[i].vel.x = fruits[i].vel.x * LINEAR_FRICTION_CONST;
            fruits[i].a_acc += LINEAR_FRICTION_CONST * (-fruits[i].vel.x - fruits[i].a_vel*fruits[i].radius);
        }
        if (fruits[i].pos.x - fruits[i].radius) < (LEFT_WALL + WALL_THICKNESS/2.0){
            fruits[i].pos.x = LEFT_WALL + WALL_THICKNESS/2.0 + fruits[i].radius;
            fruits[i].vel.x = -fruits[i].vel.x * BOUNCE_CONST;
            fruits[i].vel.y = fruits[i].vel.y * LINEAR_FRICTION_CONST;
            fruits[i].a_acc += LINEAR_FRICTION_CONST * (fruits[i].vel.y - fruits[i].a_vel*fruits[i].radius);
        }
        if (fruits[i].pos.x + fruits[i].radius) > (RIGHT_WALL - WALL_THICKNESS/2.0){
            fruits[i].pos.x = RIGHT_WALL - WALL_THICKNESS/2.0 - fruits[i].radius;
            fruits[i].vel.x = -fruits[i].vel.x * BOUNCE_CONST;
            fruits[i].vel.y = fruits[i].vel.y * LINEAR_FRICTION_CONST;
            fruits[i].a_acc += LINEAR_FRICTION_CONST * (-fruits[i].vel.y - fruits[i].a_vel*fruits[i].radius);
        }
    }

    // for (auto& obj : m_objects) {
    //     const sf::Vector2f v    = m_constraint_center - obj.position;
    //     const float        dist = sqrt(v.x * v.x + v.y * v.y);
    //     if (dist > (m_constraint_radius - obj.radius)) {
    //         const sf::Vector2f n = v / dist;
    //         obj.position = m_constraint_center - n * (m_constraint_radius - obj.radius);
    //     }
    // }
}

// fn physics_update(
//     time_step: Res<FixedTime>,
//     mut fruit_query: Query<&mut Fruit>, 
// ){
//     let dt = time_step.period.as_secs_f32();
//     let mut displacement: Vec2 = Vec2::ZERO;
//     for mut fruit_i in fruit_query.iter_mut(){
//         displacement = fruit_i.pos - fruit_i.pos_last;
//         fruit_i.pos_last = fruit_i.pos;
//         fruit_i.pos = fruit_i.pos + displacement + fruit_i.acc * dt * dt;
//         fruit_i.acc = Vec2::ZERO;
//     }

// }

fn physics_update(
    time_step: Res<FixedTime>,
    mut fruit_query: Query<&mut Fruit>, 
){
    let dt = time_step.period.as_secs_f32();
    for mut fruit_i in fruit_query.iter_mut(){
        fruit_i.vel.x += dt * fruit_i.acc.x;
        fruit_i.vel.y += dt * fruit_i.acc.y;
        fruit_i.a_vel += dt * fruit_i.a_acc;
        fruit_i.pos.x += dt * fruit_i.vel.x;
        fruit_i.pos.y += dt * fruit_i.vel.y;
        fruit_i.a_pos += dt * fruit_i.a_vel;

        fruit_i.acc.x = 0.0;
        fruit_i.acc.y = 0.0;
        fruit_i.a_acc = 0.0;
    }

}

// fn calc_vel(
//     time_step: Res<FixedTime>,
//     mut fruit_query: Query<&mut Fruit>,
//     wall_query: Query<(Entity, &Transform), With<Collider>>,
// ){
//     let dt = time_step.period.as_secs_f32();
//     let mut accel = Vec2{
//         x: 0.0,
//         y: 0.0,
//     };

//     let mut fruits: Vec<_> = fruit_query.iter_mut().collect();
//     let mut r_ij: Vec2 = Vec2::ZERO;
//     let mut v_ij: Vec2 = Vec2::ZERO;

//     let mut reflect_x = false;
//     let mut reflect_y = false;
//     let mut pos_vec: Vec3;

//     for i in 0..fruits.len() {
//         accel.x = 0.0;
//         accel.y = -GRAVITY;

//         // Check ball collisions
//         for j in 0..fruits.len() {
//             if i == j {
//                 continue; // skip the same fruit
//             }
//             r_ij = fruits[j].pos - fruits[i].pos;
//             v_ij = fruits[j].pos - fruits[i].pos;
//             if r_ij.length() < (fruits[i].radius + fruits[j].radius){
//                 accel += -NORMAL_CONST * r_ij.normalize() * v_ij.length();
//                 println!("{:?}", accel);
//             }
//         }

//         // Check wall collisions

//         for (wall_entity, wall_transform) in &wall_query {
//             let collision = collide(
//                 Vec3::from((fruits[i].pos, 0.0)),
//                 Vec2::splat(2.0*fruits[i].radius),
//                 wall_transform.translation,
//                 wall_transform.scale.truncate(),
//             );

//             if let Some(collision) = collision{
//                 match collision {
//                     Collision::Left => reflect_x = fruits[i].vel.x > 0.0,
//                     Collision::Right => reflect_x = fruits[i].vel.x < 0.0,
//                     Collision::Top => reflect_y = fruits[i].vel.y < 0.0,
//                     Collision::Bottom => reflect_y = fruits[i].vel.y > 0.0,
//                     Collision::Inside => { /* do nothing */ }
//                 }
//             }
//         }

//         if reflect_x{
//             fruits[i].vel.x = -(fruits[i].vel.x * NORMAL_CONST);
//             accel.x = 0.0;
//         }
//         if reflect_y{
//             fruits[i].vel.y = -(fruits[i].vel.y * NORMAL_CONST);
//             accel.y = 0.0;
//         }

//         // if fruits[i].pos.y < -200.0{
//         //     accel.y = NORMAL_CONST * (-fruits[i].pos.y - 400.0);
//         //     fruits[i].vel.y = -(fruits[i].vel.y * NORMAL_CONST);
//         // }

//         fruits[i].vel += dt * accel;
//         // println!("{:?}", accel);
//         // println!("{:?}", fruits[i].pos);
//     }
// }

// fn calc_pos(
//     time_step: Res<FixedTime>,
//     mut query: Query<&mut Fruit>,
// ){
//     let dt = time_step.period.as_secs_f32();

//     for mut fruit_i in query.iter_mut(){
//         fruit_i.pos.x += dt * fruit_i.vel.x;
//         fruit_i.pos.y += dt * fruit_i.vel.y;
//     }
// }

fn update_sprites(
    mut query: Query<(&mut Transform, &Fruit)>,
){
    for (mut transform, fruit) in query.iter_mut(){
        transform.translation.x = fruit.pos.x;
        transform.translation.y = fruit.pos.y;
        transform.rotation = Quat::from_rotation_z(fruit.a_pos);
    }
}