// #![windows_subsystem = "windows"]
pub mod ai;
mod combat;
mod config;
mod entities;
pub mod input;
mod item;
pub mod items;

use ai::*;
use combat::*;
use config::*;
use entities::*;
use input::*;

use bevy::{
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
    sprite::{QUAD_HANDLE, SPRITE_PIPELINE_HANDLE},
    window::WindowMode,
};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "GameDev".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: false,
            resizable: true,
            // mode: WindowMode::Fullscreen { use_size: false },
            // mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(AiPlugin)
        .add_startup_system(setup.system())
        .insert_resource(LocalPlayer(Entity::new(0)))
        .run();
}

pub struct LocalPlayer(Entity);

pub struct HealthManaBar;
pub struct Bars;

#[derive(Bundle)]
struct HealthManaBarBundle {
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub health_mana_bar: Bars,
}
impl Default for HealthManaBarBundle {
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
                size: Vec2::new(TILE_SIZE, 3.),
                ..Default::default()
            },
            material: Default::default(),
            transform: Transform::from_xyz(0., 0., 7.),
            global_transform: Default::default(),
            health_mana_bar: Bars,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut localplayer: ResMut<LocalPlayer>,
) {
    // #Ent 1
    let player = commands
        .spawn()
        .insert_bundle(PlayerComponents::new("Demnok"))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                ..Default::default()
            },
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(OrthographicCameraBundle::new_2d());
        })
        .id();
    localplayer.0 = player;

    // #Ent 2
    commands
        .spawn()
        .insert_bundle(MonsterBundle::new("#1"))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                ..Default::default()
            },
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(TILE_SIZE, 0., 0.),
            ..Default::default()
        })
        .id();

    // #Ent 3
    commands
        .spawn()
        .insert_bundle(MonsterBundle::new("#2"))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                ..Default::default()
            },
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(-TILE_SIZE, 0., 0.),
            ..Default::default()
        })
        .id();

    // # Ent 4
    commands
        .spawn()
        .insert_bundle(MonsterBundle::new("#3"))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                ..Default::default()
            },
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(-TILE_SIZE, TILE_SIZE, 0.),
            ..Default::default()
        })
        .id();
}
