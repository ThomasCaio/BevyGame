use std::{process::Child, time::Duration};

use bevy::{ecs::{query::QueryEntityError, system::EntityCommands}, input::{keyboard::KeyboardInput, mouse::MouseButtonInput}, math::{Vec2, Vec3}, prelude::{*
    }, render::{camera::Camera, pipeline::RenderPipeline, render_graph::base::MainPass}, sprite::{QUAD_HANDLE, SPRITE_PIPELINE_HANDLE}};
use crate::{LocalPlayer, combat::{Attack, Combat, LockedTarget}, config::*, entities::{Body, Player, Speed}};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app:&mut AppBuilder) {
        app
        .add_startup_system(setup.system())
        .add_event::<MouseClickEvent>()
        .init_resource::<InputTimer>()
        .init_resource::<Mouse>()
        .add_event::<MoveEvent>()
        .add_event::<ChangePositionEvent>()
        .add_system(track_mouse_position.system())
        .add_system(track_world_mouse_debug.system())
        .add_system(input_handler.system())
        .add_system(mouse_position_trigger.system())
        .add_system(collision_system.system())
        .add_system(movement_system.system())
        .add_system(get_entity_at_mouse_position.system())
        .add_system(lock_on_target.system())
        .add_system(update_player_timers.system())
        .insert_resource(EntityAtMouse(None))
        ;
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn()
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            size: Vec2::new(TILE_SIZE, TILE_SIZE),
            ..Default::default()
        },
        material: materials.add(Color::rgba(Color::YELLOW.r(), Color::YELLOW.g(), 0., 0.2).into()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    })
    .insert(MousePositionDebug)
    ;
}

fn coordinate(xy: Vec2) -> Vec2{
    let x_rest = xy.x % TILE_SIZE;
    let y_rest = xy.y % TILE_SIZE;
    let x_div = xy.x as i32 / TILE_SIZE as i32;
    let y_div = xy.y as i32 / TILE_SIZE as i32;
    let mut x_pos = x_div * TILE_SIZE as i32;
    if x_rest > 0. {
        x_pos += TILE_SIZE as i32;
    }
    let mut y_pos = y_div * TILE_SIZE as i32;
    if y_rest > 0. {
        y_pos += TILE_SIZE as i32;
    }
    let position =  Vec2::new(x_pos as f32, y_pos as f32);
    position
}

#[derive(Debug)]
struct MoveEvent(Entity, Vec3);

#[derive(Default,Debug)]
struct InputTimer(Timer);

fn collision_check(spr1: CollisionObject, spr2: CollisionObject) -> bool {
    if spr1.translation.x < spr2.translation.x + spr2.size.x &&
    spr1.translation.x + spr1.size.x > spr2.translation.x &&
    spr1.translation.y < spr2.translation.y + spr2.size.y &&
    spr1.translation.y + spr1.size.y > spr2.translation.y {
        return true;
    }
    false
}

struct CollisionObject{
    translation: Vec3,
    size: Vec2,
}

fn get_entity_at_mouse_position(sizes: Query<(Entity, &Transform, &Sprite,), (Without<MousePositionDebug>, With<Body>)>, mouse: Res<Mouse>, mut entity_at_mouse: ResMut<EntityAtMouse>) {
    for (entity, transform, sprite) in sizes.iter() {
        let spr1 = CollisionObject{translation: Vec3::new(mouse.coordinated_position.x, mouse.coordinated_position.y, 0.), size: Vec2::new(TILE_SIZE, TILE_SIZE)};
        let spr2 = CollisionObject{translation: transform.translation, size: sprite.size };
        if collision_check(spr1, spr2) {
            entity_at_mouse.0 = Some(entity);
            break;
        }
        else {
            entity_at_mouse.0 = None;
        }
    }
}

fn mouse_position_trigger(mut events: EventReader<MouseClickEvent>, entity_at_mouse: Res<EntityAtMouse>) {
    for event in events.iter() {
        if let Some(t) = entity_at_mouse.0 {
            // println!("{:?}", t);
        }
    }
}

fn input_handler(
    mouse_inputs: Res<Input<MouseButton>>, 
    keyboard_inputs: Res<Input<KeyCode>>, 
    mouse: Res<Mouse>, 
    mut mouse_event: EventWriter<MouseClickEvent>,
    mut move_event: EventWriter<MoveEvent>,
    mut timer: ResMut<InputTimer>,
    time: Res<Time>,
    player: Res<LocalPlayer>
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        if mouse_inputs.just_pressed(MouseButton::Left) {
            mouse_event.send(MouseClickEvent{ 
                button: MouseButton::Left, 
                ui_position: mouse.ui_position, 
                world_position: mouse.world_position, 
                coordinated_position: mouse.coordinated_position 
            })
        }
        if mouse_inputs.just_pressed(MouseButton::Right) {
            mouse_event.send(MouseClickEvent{
                button: MouseButton::Right,
                ui_position: mouse.ui_position,
                world_position: mouse.world_position,
                coordinated_position: mouse.coordinated_position
            })
        }

        // # MOVEMENT # //
    
        if keyboard_inputs.pressed(KeyCode::W) {
            move_event.send(MoveEvent(player.0, Vec3::new(0., TILE_SIZE, 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::A) {
            move_event.send(MoveEvent(player.0, Vec3::new(-TILE_SIZE, 0., 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::S) {
            move_event.send(MoveEvent(player.0, Vec3::new(0., -TILE_SIZE, 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::D) {
            move_event.send(MoveEvent(player.0, Vec3::new(TILE_SIZE, 0., 0.)));
        }

        else if keyboard_inputs.pressed(KeyCode::Q) {
            move_event.send(MoveEvent(player.0, Vec3::new(-TILE_SIZE, TILE_SIZE, 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::E) {
            move_event.send(MoveEvent(player.0, Vec3::new(TILE_SIZE, TILE_SIZE, 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::Z) {
            move_event.send(MoveEvent(player.0, Vec3::new(-TILE_SIZE, -TILE_SIZE, 0.)));
        }
        else if keyboard_inputs.pressed(KeyCode::C) {
            move_event.send(MoveEvent(player.0, Vec3::new(TILE_SIZE, -TILE_SIZE, 0.)));
        }
        timer.0.reset()
    }
}

fn track_world_mouse_debug(mouse: ResMut<Mouse>, mut transforms: Query<&mut Transform, With<MousePositionDebug>>) {
    let mut transform = transforms.single_mut().unwrap();
    transform.translation = Vec3::new(mouse.coordinated_position.x, mouse.coordinated_position.y ,1.);
}

fn track_mouse_position(
    mut moved: EventReader<CursorMoved>,
    mut mouse: ResMut<Mouse>,
    mut player_camera: Query<(&Camera, &GlobalTransform)>,
    windows: ResMut<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let camera = player_camera.single_mut().unwrap().1;
    let camera_offset_x = camera.translation.x;
    let camera_offset_y = camera.translation.y;
    let event = moved.iter().last();

    if let Some(e) = event {
        mouse.position = e.position;
        mouse.ui_position = e.position.clone();
    }
    mouse.world_position = Vec2::new(camera_offset_x + mouse.position.x - (window.width() / 2.), camera_offset_y + mouse.position.y - (window.height() / 2.));
    mouse.coordinated_position = coordinate(mouse.world_position - Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2.));
}

struct ChangePositionEvent(Entity, Vec3);

fn collision_system(
    mut move_events: EventReader<MoveEvent>,
    mut change_position_event: EventWriter<ChangePositionEvent>,
    mut queryset: QuerySet<(
        Query<&Transform, With<Body>>,
        Query<(Entity, &Transform), With<Body>>,
    )>,
) {
    let mut collision = false;
    for event in move_events.iter() {
        let transform1 = queryset.q0_mut().get_mut(event.0).unwrap();
        let delta = transform1.translation.clone() + event.1;

        for (_, transform2) in queryset.q1().iter() {
            if transform2.translation == delta {
                collision = true;
            }
        }
        if !collision {
            change_position_event.send(ChangePositionEvent(event.0, event.1));
        }
    }
}


fn movement_system(mut move_events: EventReader<ChangePositionEvent>, mut players: Query<(&mut Transform, &mut Speed), With<Player>>) {
    for event in move_events.iter() {
        if let Ok((mut transform, mut speed)) = players.get_mut(event.0) { 
            let duration = Duration::from_millis(speed.base_interval as u64 - speed.value as u64);
            speed.interval.set_duration(duration);
            if speed.interval.finished() {
                transform.translation += event.1;
                speed.interval.reset();
            }
        }
    }
}

fn update_player_timers(time :Res<Time>, mut speeds: Query<&mut Speed>, mut attacks: Query<&mut Attack>) {
    for mut speed in speeds.iter_mut() {
        speed.interval.tick(time.delta());
    }
    for mut attack in attacks.iter_mut() {
        attack.interval.tick(time.delta());
        // println!("{:?}", combat.attack.interval);
    }
}


#[derive(Default, Debug)]
pub struct Mouse {
    pub position: Vec2,
    pub ui_position: Vec2,
    pub world_position: Vec2,
    pub coordinated_position: Vec2,
}

#[derive(Debug)]
pub struct MouseClickEvent {
    // pub timestamp: f64,
    pub button: MouseButton,
    pub ui_position: Vec2,
    pub world_position: Vec2,
    pub coordinated_position: Vec2,
}

#[derive(Debug, PartialEq)]
pub struct EntityAtMouse(pub Option<Entity>);


// RightClick
pub fn lock_on_target(mut commands: Commands, 
    entity: Res<EntityAtMouse>, 
    mut target: ResMut<LockedTarget>, 
    mut mouse_events: EventReader<MouseClickEvent>, 
    localplayer: Res<LocalPlayer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    children_query: Query<(Entity, &Parent), With<LockedSprite>>
) {
    for event in mouse_events.iter(){

        // VER SE ISSO INTERFERE NO HP BAR
        if event.button == MouseButton::Right{
            match (entity.0, target.0) {
                (None, None) => (),
                (None, Some(_)) => (),
                (Some(e), None) => {
                    if e == localplayer.0 {}
                    else {
                        if let Some((child, _)) = children_query.iter().last() {
                            commands.entity(child).despawn();
                        }
                        target.0 = Some(e);
                        push_hover_children(e, &mut commands, &mut materials)
                    }
                },
                (Some(e), Some(t)) => {
                    if e == t {
                        target.0 = None;
                        if let Some((child, _)) = children_query.iter().last() {
                            commands.entity(child).despawn();
                        }
                    } else if e == localplayer.0 {}
                    else {
                        if let Some((child, _)) = children_query.iter().last() {
                            commands.entity(child).despawn();
                        }
                        target.0 = Some(e);
                        push_hover_children(e, &mut commands, &mut materials)
                    }
                },
            }
        }
    }
    if let Some(t) = target.0 {
        if let Ok(_) = children_query.get(t) {
            push_hover_children(t, &mut commands, &mut materials)
        }
    }
}

pub struct MousePositionDebug;


fn push_hover_children(entity: Entity, commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let parent = commands.entity(entity)
    .with_children(|entity| {
        entity
        .spawn_bundle(HoverBundle {
            material: materials.add(Color::rgba(1., 0., 0., 0.2).into()),
            ..Default::default()});
    }).id();
}

pub struct LockedSprite;

#[derive(Bundle)]
struct HoverBundle {
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub locked_sprite: LockedSprite,
}
impl Default for HoverBundle {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                ..Default::default()
            },
            material: Default::default(),
            transform: Transform::from_xyz(0., 0., 5.),
            global_transform: Default::default(),
            locked_sprite: LockedSprite,
        }
    }
}