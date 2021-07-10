use crate::combat::{create_combat_text, Combat, CombatText, Health};
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
            .add_system(exp_change.system())
            .add_system(level_up.system())
            .add_system(combat_text.system());
    }
}

#[derive(Debug)]
pub struct Level(pub u32);
#[derive(Debug)]
pub struct CurrentExperience(pub u32);
#[derive(Debug)]
pub struct NextLevelExperience(pub u32);

#[derive(Bundle, Debug)]
struct Levelling {
    level: Level,
    current_experience: CurrentExperience,
    next_level_experience: NextLevelExperience,
}

impl Default for Levelling {
    fn default() -> Self {
        Self {
            level: Level(0),
            current_experience: CurrentExperience(0),
            next_level_experience: NextLevelExperience(100),
        }
    }
}

fn exp_change(
    mut query: Query<(&mut CurrentExperience, &mut NextLevelExperience, &mut Level), Changed<CurrentExperience>>,
) {
    for (mut cur, mut next, mut lvl) in query.iter_mut() {
        if cur.0 >= next.0 {
            cur.0 -= next.0;
            lvl.0 += 1;
            next.0 = lvl.0 * (((lvl.0 * 100) as f32 * 1.2) as u32);
            println!("{:?}", (lvl, cur, next));
        }
    }
}

fn level_up(
    mut commands: Commands,
    query: Query<Entity, Changed<Level>>,
    asset_server: Res<AssetServer>,
) {
    for entity in query.iter() {
        create_combat_text(
            entity,
            "Level up!".to_string(),
            &mut commands,
            &asset_server,
            None,
            None,
            None,
            None,
        )
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
    #[bundle]
    levelling: Levelling,
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
            levelling: Levelling::default(),
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
