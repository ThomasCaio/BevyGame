use crate::combat::{Combat, CombatText, Health};
use crate::config::TILE_SIZE;
use crate::{item::*, Bars, HealthManaBar, HealthManaBarBundle};
use bevy::core::Timer;
use bevy::prelude::*;
use std::time::Duration;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(healthbar_change.system())
            .add_system(insert_entity_name.system())
            .add_system(insert_healthbar.system())
            .add_system(insert_entity_combat.system())
            .add_system(combat_text.system());
    }
}

fn healthbar_change(
    query: Query<(&Health, &Children), Changed<Health>>,
    mut bars: Query<(&mut Sprite, &mut Transform, &Bars)>,
) {
    for (health, children) in query.iter() {
        for child in children.iter() {
            if let Ok((mut sprite, mut transform, _)) = bars.get_mut(*child) {
                let percent = health.value / health.max_value;
                sprite.size = Vec2::new(TILE_SIZE * percent, sprite.size.y);
                transform.translation.x = ((TILE_SIZE / 2.) * percent) - (TILE_SIZE / 2.);
            }
        }
    }
}

fn insert_entity_combat(mut commands: Commands, query: Query<Entity, Added<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(HealthManaBar);
    }
}

fn insert_entity_name(
    mut commands: Commands,
    query: Query<(Entity, &Name), Added<Name>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, name) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    name.value.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 10.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_xyz(0., 16., 500.),
                ..Default::default()
            });
        });
    }
}

const HEALTH_MANA_BAR_POSITION: f32 = 14.;

fn insert_healthbar(
    mut commands: Commands,
    query: Query<Entity, Added<HealthManaBar>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        size: Vec2::new(TILE_SIZE, 4.),
                        ..Default::default()
                    },
                    material: materials.add(Color::BLACK.into()),
                    transform: Transform::from_xyz(0., HEALTH_MANA_BAR_POSITION, 6.),
                    ..Default::default()
                });
            })
            .with_children(|parent| {
                parent.spawn_bundle(HealthManaBarBundle {
                    sprite: Sprite {
                        size: Vec2::new(TILE_SIZE, 3.),
                        ..Default::default()
                    },
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0., HEALTH_MANA_BAR_POSITION, 7.),
                    ..Default::default()
                });
            });
    }
}

fn combat_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Timer, &CombatText)>,
) {
    for (entity, mut transform, timer, _) in query.iter_mut() {
        if timer.finished() {
            transform.translation.y += 0.6;
            if transform.translation.y > 20. {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Body;

#[derive(Debug, Bundle)]
pub struct PlayerComponents {
    player: Player,
    name: Name,
    speed: Speed,
    equipments: Equipments,
    inventory: Vec<Item>,
    body: Body,

    #[bundle]
    combat: Combat,
}

impl PlayerComponents {
    pub fn new(name: &str) -> PlayerComponents {
        let p = PlayerComponents {
            name: Name {
                value: name.to_string(),
            },
            equipments: Equipments {
                mainhand: Item::new("Sword"),
                offhand: Item::new("Wooden Shield"),
                ..Default::default()
            },
            ..Default::default()
        };
        p
    }
}

impl Default for PlayerComponents {
    fn default() -> PlayerComponents {
        PlayerComponents {
            player: Player,
            name: Name {
                value: "Unamed".to_string(),
            },
            speed: Speed::default(),
            combat: Combat::default(),
            equipments: Equipments::default(),
            inventory: Vec::new(),
            body: Body,
        }
    }
}

#[derive(Debug)]
pub struct Name {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Speed {
    pub value: f32,
    pub interval: Timer,
    pub base_interval: f32,
}

impl Default for Speed {
    fn default() -> Speed {
        Speed {
            value: 100.,
            interval: Timer::new(Duration::from_millis(500), false),
            base_interval: 500.,
        }
    }
}
