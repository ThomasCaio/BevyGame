use std::time::Duration;
use super::input::{MoveEvent};


use bevy::prelude::*;

use crate::{combat::Combat, config::TILE_SIZE, entities::{Body, Player, Speed, Name}};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(find_an_enemy.system())
        .add_system(monster_ai.system())
        ;
    }
}

#[derive(Bundle)]
pub struct MonsterBundle {
    name: Name,
    monster: Monster,
    body: Body,
    speed: Speed,

    #[bundle]
    combat: Combat,
}

impl MonsterBundle {
    pub fn new(name: &str) -> Self {
        MonsterBundle {
            name: Name{value: name.to_string()},
            ..Default::default()
        }
    }
}

impl Default for MonsterBundle {
    fn default() -> Self {
        MonsterBundle {
            name: Name{value: "Unamed".to_string()},
            monster: Monster::default(),
            body: Body,
            speed: Speed{value: 0., ..Default::default()},
            combat: Combat::default(),
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

fn calculate_distance(t1: Vec3, t2: Vec3) -> f32 {
    f32::sqrt(f32::powf(t2.x - t1.x, 2.) + f32::powf(t2.y - t1.y, 2.))
}

fn find_an_enemy(mut monsters: Query<(&Transform, &mut Monster)>, players: Query<(Entity, &Name, &Transform), With<Player>>) {
    for (m_transform, mut monster) in monsters.iter_mut() {
        for (p_entity, p_name, p_transform) in players.iter() {
            if calculate_distance(m_transform.translation, p_transform.translation) < monster.vision_range {
                monster.enemy = Some(p_entity);
                println!("{:?}", (p_name));
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
    for (m_entity, mut monster, m_transform, mut m_speed) in monsters.iter_mut() {
        for (p_entity, _, p_transform) in players.iter() {
            if m_speed.interval.finished() {
                if let Some(me) = monster.enemy {
                    if me == p_entity {
                        let vec = follow_enemy(*m_transform, *p_transform);
                        move_events.send(MoveEvent(m_entity, vec));
                    }
                }
            }
        }
    }
}

fn follow_enemy(m_transform: Transform, p_transform: Transform) -> Vec3 {
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut z: f32 = 0.;
    if m_transform.translation.x < p_transform.translation.x {
        x = TILE_SIZE;
    }
    else if m_transform.translation.x > p_transform.translation.x {
        x = -TILE_SIZE;
    }
    else {
        x = 0.;
    }
    if m_transform.translation.y < p_transform.translation.y {
        y = TILE_SIZE;
    }
    else if m_transform.translation.y > p_transform.translation.y {
        y = -TILE_SIZE;
    }
    else {
        y = 0.;
    }
    z = m_transform.translation.z;

    Vec3::new(x, y, z)
}