use super::input::MoveEvent;
use std::time::Duration;

use bevy::prelude::*;

use crate::{
    combat::Combat,
    config::TILE_SIZE,
    entities::{Body, Name, Player, Speed},
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(find_an_enemy.system())
            .add_system(monster_ai.system());
    }
}

pub struct ExperiencePoints(pub u32);

#[derive(Bundle)]
pub struct MonsterBundle {
    name: Name,
    monster: Monster,
    body: Body,
    speed: Speed,
    experience: ExperiencePoints,

    #[bundle]
    combat: Combat,
}

impl MonsterBundle {
    pub fn new(name: &str) -> Self {
        MonsterBundle {
            name: Name {
                value: name.to_string(),
            },
            ..Default::default()
        }
    }
}

impl Default for MonsterBundle {
    fn default() -> Self {
        MonsterBundle {
            name: Name {
                value: "Unamed".to_string(),
            },
            monster: Monster::default(),
            body: Body,
            speed: Speed {
                value: 0.,
                ..Default::default()
            },
            combat: Combat::default(),
            experience: ExperiencePoints(100),
        }
    }
}

pub struct Monster {
    pub enemy: Option<Entity>,
    pub vision_range: f32,
}

impl Default for Monster {
    fn default() -> Self {
        Monster {
            enemy: None,
            vision_range: 1000.,
        }
    }
}

fn find_an_enemy(
    mut monsters: Query<(&Transform, &mut Monster)>,
    players: Query<(Entity, &Name, &Transform), With<Player>>,
) {
    for (m_transform, mut monster) in monsters.iter_mut() {
        for (p_entity, _, p_transform) in players.iter() {
            if m_transform.translation.distance(p_transform.translation) < monster.vision_range {
                monster.enemy = Some(p_entity);
                break;
            }
            monster.enemy = None;
        }
    }
}

fn monster_ai(
    mut monsters: Query<(Entity, &mut Monster, &Transform, &mut Speed)>,
    players: Query<(Entity, &Player, &Transform)>,
    mut move_events: EventWriter<MoveEvent>,
) {
    for (m_entity, monster, m_transform, _) in monsters.iter_mut() {
        for (p_entity, _, p_transform) in players.iter() {
            if let Some(me) = monster.enemy {
                if me == p_entity {
                    let vec = follow_enemy(*m_transform, *p_transform);
                    move_events.send(MoveEvent(m_entity, vec));
                }
            }
        }
    }
}

fn follow_enemy(m_transform: Transform, p_transform: Transform) -> Vec3 {
    let x;
    let y;
    let z;
    if m_transform.translation.x < p_transform.translation.x {
        x = TILE_SIZE;
    } else if m_transform.translation.x > p_transform.translation.x {
        x = -TILE_SIZE;
    } else {
        x = 0.;
    }
    if m_transform.translation.y < p_transform.translation.y {
        y = TILE_SIZE;
    } else if m_transform.translation.y > p_transform.translation.y {
        y = -TILE_SIZE;
    } else {
        y = 0.;
    }
    z = m_transform.translation.z;

    Vec3::new(x, y, z)
}
