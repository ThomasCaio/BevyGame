extern crate rand;
use rand::{thread_rng, Rng};

use bevy::{
    ecs::bundle,
    prelude::*,
    render::{color, render_graph::base::MainPass},
    text::Text2dSize,
};
use std::{borrow::Borrow, collections::btree_map::Range, time::Duration};

use crate::{
    ai::ExperiencePoints,
    entities::{CurrentExperience, Name, NextLevelExperience, Player},
    item::{AttributeType, Equipments, Item},
    main, LocalPlayer,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LockedTarget(None))
            .add_event::<AttackEvent>()
            .add_event::<ResistanceEvent>()
            .add_event::<MissEvent>()
            .add_event::<BlockEvent>()
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_event::<SpawnEvent>()
            .add_event::<CombatText>()
            .add_system(attack_system.system())
            .add_system(miss_system.system())
            .add_system(attack_system.system())
            .add_system(hit_system.system())
            .add_system(resistance_system.system())
            .add_system(block_system.system())
            .add_system(damage_system.system())
            .add_system(death_system.system())
            .add_system(death_drop.system());
    }
}

pub struct DeathEvent {
    attacker: Entity,
    defender: Entity,
    damage: DamageSet,
}

#[derive(Debug)]
pub struct CombatText;

pub struct SpawnEvent(Entity);

#[derive(Debug)]
pub struct LockedTarget(pub Option<Entity>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DamageType {
    Physical,
    Fire,
    Water,
    Air,
    Earth,
    Holy,
    Death,
    LifeDrain,
    ManaDrain,
}

#[derive(Debug, Clone, Copy)]
pub struct Damage {
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
pub(crate) struct Combat {
    pub health: Health,
    pub mana: Mana,
    pub attack: Attack,
    pub defense: Defense,
}

impl Default for Combat {
    fn default() -> Self {
        Combat {
            health: Health {
                max_value: 100.,
                value: 100.,
            },
            mana: Mana {
                max_value: 50.,
                value: 50.,
            },
            attack: Attack::default(),
            defense: Defense::default(),
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
        Defense {
            value: 1.,
            rate: 50.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attack {
    pub damage: Damage,
    pub range: f32,
    pub interval: Timer,
    pub base_interval: u64,
    pub rate: f32,
}

impl Default for Attack {
    fn default() -> Attack {
        Attack {
            damage: Damage {
                value: 10.,
                dtype: DamageType::Physical,
            },
            range: 50.,
            interval: Timer::new(Duration::from_millis(500), false),
            base_interval: 2000,
            rate: 75.,
        }
    }
}

#[derive(Debug)]
struct AttackEvent {
    attacker: Entity,
    defender: Entity,
}
#[derive(Debug)]
struct MissEvent {
    attacker: Entity,
    defender: Entity,
}
#[derive(Debug)]
struct ResistanceEvent {
    attacker: Entity,
    defender: Entity,
    damage: DamageSet,
}
struct BlockEvent {
    attacker: Entity,
    defender: Entity,
    damage: DamageSet,
}
#[derive(Debug)]
struct DamageEvent {
    attacker: Entity,
    defender: Entity,
    damage: DamageSet,
}

#[derive(Debug)]
struct PassiveEvent {
    attacker: Entity,
    defender: Entity,
    passive_spell: PassiveSpell,
}

#[derive(Debug)]
struct PassiveSpell;

#[derive(Debug, Clone)]
pub struct DamageSet(pub Vec<Damage>);

fn passive_trigger() {
    // TODO: Thorns, Critical Strike, Bleeding, Burning, Poison, Etc.
}

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
                if t1.translation.distance(t2.translation) < attacker_range {
                    if attack_attack.interval.finished() {
                        let chance = thread_rng().gen_range(0.0..=100.);
                        if attack_attack.rate > chance {
                            hit_events.send(AttackEvent {
                                attacker: player.0,
                                defender: t,
                            });
                        } else {
                            miss_events.send(MissEvent {
                                attacker: player.0,
                                defender: t,
                            });
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
        let mut damage_set = DamageSet(vec![]);
        if let Ok((attacker_attack, Some(attacker_equipment))) = attacks.get_mut(hit.attacker) {
            damage_set = attacker_equipment.get_damage_set();
            damage_set.0.iter_mut().for_each(|&mut mut dmg| {
                if dmg.dtype == DamageType::Physical {
                    dmg.value += attacker_attack.damage.value;
                }
            });
        }
        resistance_events.send(ResistanceEvent {
            attacker: hit.attacker,
            defender: hit.defender,
            damage: damage_set,
        });
    }
}

fn miss_system(
    mut commands: Commands,
    mut miss_events: EventReader<MissEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in miss_events.iter() {
        commands.entity(event.defender).with_children(|parent| {
            parent.spawn_bundle(CombatTextBundle {
                text: Text::with_section(
                    "Miss",
                    TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 12.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
    }
}

fn resistance_system(
    mut resistance_events: EventReader<ResistanceEvent>,
    mut block_events: EventWriter<BlockEvent>,
    mut query: Query<Option<&Equipments>>,
) {
    for block in resistance_events.iter() {
        let mut damage_set = block.damage.clone();
        if let Ok(Some(defender_equipments)) = query.get_mut(block.defender) {
            damage_set.0.iter_mut().for_each(|&mut mut dmg| {
                let factor =
                    defender_equipments.get_attributes(AttributeType::Resistance(dmg.dtype)) / 100.;
                dmg.value -= dmg.value * factor;
            });
        }
        block_events.send(BlockEvent {
            attacker: block.attacker,
            defender: block.defender,
            damage: damage_set,
        });
    }
}

fn block_system(
    mut block_events: EventReader<BlockEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut query: Query<Option<&Equipments>>,
) {
    for resistance in block_events.iter() {
        let mut damage_set = resistance.damage.clone();
        if let Ok(Some(defender_equipments)) = query.get_mut(resistance.defender) {
            let total_block = defender_equipments.get_attributes(AttributeType::Block);
            damage_set.0.iter_mut().for_each(|&mut mut dmg| {
                if dmg.dtype == DamageType::Physical {
                    dmg.value -= total_block;
                    if dmg.value < 0. {
                        dmg.value = 0.;
                    }
                }
            });
        }
        damage_events.send(DamageEvent {
            attacker: resistance.attacker,
            defender: resistance.defender,
            damage: damage_set,
        })
    }
}

// N??o est?? printando o valor do dano. Revisar todo esquema do DamageSet atrav??s dos eventos.

fn damage_color(damage: DamageSet) -> Color {
    let mut dtype = DamageType::Physical;
    let last_value = 0.;
    for dmg in damage.0 {
        if dmg.value > last_value {
            dtype = dmg.dtype;
        }
    }
    match dtype {
        DamageType::Physical => return Color::GRAY,
        DamageType::Fire => return Color::RED,
        DamageType::Water => return Color::BLUE,
        DamageType::Air => return Color::PINK,
        DamageType::Earth => return Color::GREEN,
        DamageType::Holy => return Color::YELLOW,
        DamageType::Death => return Color::BLACK,
        DamageType::LifeDrain => return Color::rgba(1., 0., 0., 0.5),
        DamageType::ManaDrain => return Color::rgba(0., 0., 1., 0.5),
    }
}

fn damage_system(
    mut commands: Commands,
    mut damage_event: EventReader<DamageEvent>,
    mut query: Query<&mut Health>,
    asset_server: Res<AssetServer>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for dmg in damage_event.iter() {
        let damage_set = dmg.damage.clone();
        if let Ok(mut health) = query.get_mut(dmg.defender) {
            let mut total_damage = 0.;
            for d in damage_set.0.iter() {
                total_damage += d.value;
            }
            health.value -= total_damage;
            if health.value <= 0. {
                death_events.send(DeathEvent {
                    attacker: dmg.attacker,
                    defender: dmg.defender,
                    damage: dmg.damage.clone(),
                });
            }
            create_combat_text(
                dmg.defender,
                format!("{:.0}", total_damage),
                &mut commands,
                &asset_server,
                None,
                None,
                None,
                None,
            )
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

fn death_drop(
    mut queryset: QuerySet<(
        Query<&ExperiencePoints>,
        Query<(Entity, &mut CurrentExperience, &NextLevelExperience)>,
    )>,
    mut events: EventReader<DeathEvent>,
) {
    for event in events.iter() {
        if let Ok(experience) = queryset.q0().get(event.defender) {
            let drop_experience = experience.0.clone();
            if let Ok((_, mut cur, _)) = queryset.q1_mut().get_mut(event.attacker) {
                cur.0 += drop_experience;
            }
        }
    }
}

pub fn create_combat_text(
    parent: Entity,
    text: String,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    font_color: Option<Color>,
    font_size: Option<f32>,
    vertical_align: Option<VerticalAlign>,
    horizontal_align: Option<HorizontalAlign>,
) {
    let font_color: Color = font_color.unwrap_or(Color::WHITE);
    let font_size: f32 = font_size.unwrap_or(12.);
    let vertical_align: VerticalAlign = vertical_align.unwrap_or(VerticalAlign::Bottom);
    let horizontal_align: HorizontalAlign = horizontal_align.unwrap_or(HorizontalAlign::Center);

    commands.entity(parent).with_children(|parent| {
        parent.spawn_bundle(CombatTextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/font.ttf"),
                    font_size: font_size,
                    color: font_color,
                    ..Default::default()
                },
                TextAlignment {
                    vertical: vertical_align,
                    horizontal: horizontal_align,
                },
            ),
            ..Default::default()
        });
    });
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
