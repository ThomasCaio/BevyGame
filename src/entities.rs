use std::time::Duration;
use bevy::core::Timer;
use bevy::prelude::{*};
use crate::combat::{Combat, CombatText, Health};
use crate::config::TILE_SIZE;
use crate::{HealthManaBar, item::*};


pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(healthbar_change.system())
        .add_system(combat_text.system())
        ;
    }
}

fn healthbar_change(
    query: Query<(Entity, &Health, &Children), Changed<Health>>,
    mut bars: Query<(&mut Sprite, &mut Transform, &HealthManaBar)>,
) {
    for (entity, health, children) in query.iter() {
        for child in children.iter() {
            if let Ok((mut sprite, mut transform, _)) = bars.get_mut(*child) {
                let percent = health.value / health.max_value;
                sprite.size = Vec2::new(TILE_SIZE * percent, sprite.size.y);
                transform.translation.x = ((TILE_SIZE/2.) * percent) - (TILE_SIZE/2.);
            }
        }
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
        let p = PlayerComponents{name: Name{value: name.to_string()}, equipments: Equipments{mainhand: Item::new("Sword"), offhand: Item::new("Wooden Shield"),..Default::default()}, ..Default::default()};
        p
    }
}

impl Default for PlayerComponents {
    fn default() -> PlayerComponents {
        PlayerComponents{
            player: Player,
            name: Name{value: "Unamed".to_string()},
            speed: Speed::default(),
            combat: Combat::default(),
            equipments: Equipments::default(),
            inventory: Vec::new(),
            body: Body,
        }
    }
}


#[derive(Debug)]
pub struct Name{
    pub value: String
}

#[derive(Debug, Clone)]
pub struct Speed{
    pub value: f32,
    pub interval: Timer,
    pub base_interval: f32
}

impl Default for Speed {
    fn default() -> Speed {
        Speed{
            value: 100.,
            interval: Timer::new(Duration::from_millis(500), false),
            base_interval: 500.
        }
    }
}