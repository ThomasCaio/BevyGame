extern crate rand;
use rand::{Rng, thread_rng};

use std::{borrow::Borrow, collections::btree_map::Range, time::Duration};
use bevy::{ecs::bundle, prelude::*, render::{color, render_graph::base::MainPass}, text::Text2dSize};

use crate::{LocalPlayer, entities::{Name, Player}, item::{AttributeType, Equipments, Item}, main};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(LockedTarget(None))
        .add_event::<AttackEvent>()
        .add_event::<ResistanceEvent>()
        .add_event::<MissEvent>()
        .add_event::<BlockEvent>()
        .add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        .add_event::<SpawnEvent>()
        .add_event::<CombatText>()
        .add_event::<ComplexDamageEvent>()
        .add_system(attack_system.system())
        .add_system(miss_system.system())
        .add_system(attack_system.system())
        .add_system(hit_system.system())
        .add_system(resistance_system.system())
        .add_system(block_system.system())
        .add_system(damage_system.system())
        .add_system(death_system.system())
        ;
    }
}


pub struct DeathEvent{attacker: Entity, defender: Entity, damage: ComplexDamage}

#[derive(Debug)]
pub struct CombatText;

pub struct SpawnEvent(Entity);

#[derive(Debug)]
pub struct LockedTarget(pub Option<Entity>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DamageType{
    Physical,
    Fire,
    Water,
    Air,
    Earth,
    Holy,
    Death,
    LifeDrain,
    ManaDrain
}

#[derive(Debug, Clone, Copy)]
pub struct Damage{
    pub value: f32,
    pub dtype: DamageType,
}

#[derive(Debug)]
pub struct Health {
    pub max_value: f32,
    pub value: f32,
}

#[derive(Debug)]
pub struct Mana {
    pub max_value: f32,
    pub value: f32,
}

#[derive(Debug, Bundle)]
pub(crate)struct Combat {
    pub health: Health,
    pub mana: Mana,
    pub attack: Attack,
    pub defense: Defense,
}

impl Default for Combat {
    fn default() -> Self {
        Combat {
            health: Health { max_value: 10000., value: 10000. },
            mana: Mana { max_value: 50., value: 50. },
            attack: Attack::default(),
            defense: Defense::default()
        }
    }
}

#[derive(Debug)]
pub struct Defense {
    value: f32,
    rate: f32,
}

impl Default for Defense {
    fn default() -> Self {
        Defense{value: 1., rate: 50.}
    }
}

#[derive(Debug, Clone)]
pub struct Attack{
    pub damage: Damage,
    pub range: f32,
    pub interval: Timer,
    pub base_interval: u64,
    pub rate: f32,
}

impl Default for Attack{
    fn default() -> Attack {
        Attack{
            damage: Damage{value: 100., dtype: DamageType::Physical},
            range: 50.,
            interval: Timer::new(Duration::from_millis(500), false),
            base_interval: 2000,
            rate: 75.,
        }
    }
}

fn calculate_distance(t1: Vec3, t2: Vec3) -> f32 {
    f32::sqrt(f32::powf(t2.x - t1.x, 2.) + f32::powf(t2.y - t1.y, 2.))
}

#[derive(Debug)]
struct AttackEvent{attacker: Entity, defender: Entity}
#[derive(Debug)]
struct MissEvent{attacker: Entity, defender: Entity}
#[derive(Debug)]
struct ResistanceEvent{attacker: Entity, defender: Entity, damage: ComplexDamage}
struct BlockEvent{attacker: Entity, defender: Entity, damage: ComplexDamage}
#[derive(Debug)]
struct DamageEvent{attacker: Entity, defender: Entity, damage: ComplexDamage}

#[derive(Debug)]
struct ComplexDamageEvent{attacker: Entity, defender: Entity, damage: ComplexDamage}

#[derive(Debug)]
struct PassiveEvent{attacker: Entity, defender: Entity, passive_spell: PassiveSpell}

#[derive(Debug)]
struct PassiveSpell;

#[derive(Debug, Clone)]
struct ComplexDamage{physical: Damage, fire: Damage, water: Damage, air: Damage, earth: Damage, holy: Damage, death: Damage, lifedrain: Damage, manadrain: Damage}

impl IntoIterator for ComplexDamage {
    type Item = Damage;
    type IntoIter = ComplexIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ComplexIntoIterator {
            complex: self,
            index: 0,
        }
    }
}

pub struct ComplexIntoIterator {
    complex: ComplexDamage,
    index: usize,
}

impl Iterator for ComplexIntoIterator {
    type Item = Damage;

    fn next(&mut self) -> Option<Damage> {
        let result = match self.index {
            0 => self.complex.physical,
            1 => self.complex.fire,
            2 => self.complex.water,
            3 => self.complex.air,
            4 => self.complex.earth,
            5 => self.complex.holy,
            6 => self.complex.death,
            7 => self.complex.lifedrain,
            8 => self.complex.manadrain,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl Default for ComplexDamage {
    fn default() -> Self {
        ComplexDamage{
            physical: Damage{value: 0., dtype: DamageType::Physical},
            fire: Damage{value: 0., dtype: DamageType::Fire}, 
            water: Damage{value: 0., dtype: DamageType::Water}, 
            air: Damage{value: 0., dtype: DamageType::Air}, 
            earth: Damage{value: 0., dtype: DamageType::Earth}, 
            holy: Damage{value: 0., dtype: DamageType::Holy}, 
            death: Damage{value: 0., dtype: DamageType::Death}, 
            lifedrain: Damage{value: 0., dtype: DamageType::LifeDrain}, 
            manadrain: Damage{value: 0., dtype: DamageType::ManaDrain}}
    }
}

fn passive_trigger() {}

fn attack_system(
    target: ResMut<LockedTarget>,
    player: ResMut<LocalPlayer>,
    transforms: Query<&Transform>,
    mut attacks: Query<&mut Attack>,
    mut hit_events: EventWriter<AttackEvent>,
    mut miss_events: EventWriter<MissEvent>,
) {
    if let Ok(mut attack_attack) = attacks.get_mut(player.0) {
        let attacker_range = attack_attack.range;
        if let Some(t) = target.0 {
            let t1 = transforms.get(player.0);
            let t2 = transforms.get(t);
            if let (Ok(t1), Ok(t2)) = (t1, t2) {
                if calculate_distance(t1.translation, t2.translation) < attacker_range {
                    if attack_attack.interval.finished() {
                        let chance = thread_rng().gen_range(0.0..=100.);
                        if attack_attack.rate > chance {
                            hit_events.send(AttackEvent{attacker: player.0, defender: t});
                        } else {
                            miss_events.send(MissEvent{attacker: player.0, defender: t});
                        }
                        attack_attack.interval.reset();
                    }
                }
            }
        }
    }
}

fn hit_system(
    mut hit_events: EventReader<AttackEvent>,
    mut resistance_events: EventWriter<ResistanceEvent>,
    mut attacks: Query<(&mut Attack, Option<&Equipments>)>,
) {
    for hit in hit_events.iter() {
        let mut complex_damage = ComplexDamage::default();
        if let Ok((attacker_attack, Some(attacker_equipment))) = attacks.get_mut(hit.attacker) {
            complex_damage.physical = attacker_attack.damage;
            complex_damage.physical.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Physical));
            complex_damage.fire.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Fire));
            complex_damage.water.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Water));
            complex_damage.air.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Air));
            complex_damage.earth.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Earth));
            complex_damage.holy.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Holy));
            complex_damage.death.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::Death));
            complex_damage.lifedrain.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::LifeDrain));
            complex_damage.manadrain.value += attacker_equipment.get_attributes(AttributeType::Damage(DamageType::ManaDrain));
        }
        resistance_events.send(ResistanceEvent{attacker: hit.attacker, defender: hit.defender, damage: complex_damage});
    }
}

fn miss_system(
    mut commands: Commands,
    mut miss_events: EventReader<MissEvent>,
    asset_server: Res<AssetServer>
) {
    for event in miss_events.iter() {
        commands.entity(event.defender)
        .with_children(|parent| 
            {parent.spawn_bundle(CombatTextBundle {
            text: Text::with_section(
                "Miss",
                TextStyle{
                    font: asset_server.load("fonts/font.ttf"),
                    font_size: 12.0,
                    color: Color::WHITE,
                    },
                TextAlignment{
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
            });});
    }
}

fn resistance_system(
    mut resistance_events: EventReader<ResistanceEvent>,
    mut block_events: EventWriter<BlockEvent>,
    mut query: Query<Option<&Equipments>>,
) {
    for block in resistance_events.iter() {
        let mut complex_damage = block.damage.clone();
        if let Ok(Some(defender_equipments)) = query.get_mut(block.defender) {
            complex_damage.physical.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Physical))/100.);
            complex_damage.fire.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Fire))/100.);
            complex_damage.water.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Water))/100.);
            complex_damage.air.value -= complex_damage.physical.value *  (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Air))/100.);
            complex_damage.earth.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Earth))/100.);
            complex_damage.holy.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Holy))/100.);
            complex_damage.death.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::Death))/100.);
            complex_damage.lifedrain.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::LifeDrain))/100.);
            complex_damage.manadrain.value -= complex_damage.physical.value * (defender_equipments.get_attributes(AttributeType::Resistance(DamageType::ManaDrain))/100.);
        }
        block_events.send(BlockEvent{attacker: block.attacker, defender: block.defender, damage: complex_damage})
    }
}

fn block_system(
    mut block_events: EventReader<BlockEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut query: Query<Option<&Equipments>>,
) {
    for resistance in block_events.iter() {
        let mut complex_damage = resistance.damage.clone();
        if let Ok(Some(defender_equipments)) = query.get_mut(resistance.defender) {
            let total_block = defender_equipments.get_attributes(AttributeType::Block);
            complex_damage.physical.value -= total_block;
            if complex_damage.physical.value < 0. {
                complex_damage.physical.value = 0.;
            }
        }
        damage_events.send(DamageEvent{attacker: resistance.attacker, defender: resistance.defender, damage: complex_damage})
    }
}

fn damage_system(
    mut commands: Commands,
    mut damage_event: EventReader<DamageEvent>,
    mut query: Query<&mut Health>,
    asset_server: Res<AssetServer>,
) {
    for dmg in damage_event.iter() {
        let complex_damage = dmg.damage.clone();
        if let Ok(mut health) = query.get_mut(dmg.defender) {
            let mut total_damage = 0.;
            for d in complex_damage {
                total_damage += d.value;
            }
            health.value -= total_damage;
            if health.value < 0. {
                health.value = 0.;
            }

            commands.entity(dmg.defender)
            .with_children(|parent| 
                {parent.spawn_bundle(CombatTextBundle {
                text: Text::with_section(
                    format!("{:.1}", total_damage),
                    TextStyle{
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 12.0,
                        color: Color::GRAY,
                        },
                    TextAlignment{
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
                });});
        }
    }
}

fn death_system(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0. {
            commands.entity(entity).despawn_recursive();
        }

    }
}


#[derive(Bundle)]
pub struct CombatTextBundle {
    pub draw: Draw,
    pub visible: Visible,
    pub text: Text,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub main_pass: MainPass,
    pub text_2d_size: Text2dSize,
    pub timer: Timer,
    c: CombatText,
}

impl Default for CombatTextBundle {
    fn default() -> Self {
        Self {
            draw: Draw {
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            text: Default::default(),
            transform: Transform::from_xyz(0., 0., 501.),
            global_transform: Default::default(),
            main_pass: MainPass {},
            text_2d_size: Text2dSize {
                size: Size::default(),
            },
            timer: Timer::new(Duration::from_millis(25), true),
            c: CombatText,
        }
    }
}