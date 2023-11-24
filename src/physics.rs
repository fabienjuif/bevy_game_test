use std::{collections::HashMap, hash::Hash, hash::Hasher};

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;

#[derive(Event)]
pub enum CollisionEvent {
    /// Event occurring when two colliders start colliding
    Started(Entity, Entity),
    /// Event occurring when two colliders stop colliding
    Stopped(Entity, Entity),
}

#[derive(Clone, Copy)]
pub struct EntityPair {
    pub entity1: Entity,
    pub entity2: Entity,
}

impl EntityPair {
    pub fn new(entity1: Entity, entity2: Entity) -> Self {
        // order is important this is used for the hash trait impl
        if entity1 < entity2 {
            return EntityPair { entity1, entity2 };
        }
        EntityPair {
            entity1: entity2,
            entity2: entity1,
        }
    }
}

impl PartialEq for EntityPair {
    fn eq(&self, other: &Self) -> bool {
        (self.entity1 == other.entity1 && self.entity2 == other.entity2)
            || (self.entity1 == other.entity2 && self.entity2 == other.entity1)
    }
}

impl Eq for EntityPair {}

impl Hash for EntityPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity1.hash(state);
        self.entity2.hash(state);
    }
}

#[derive(Resource, Clone)]
pub struct Collisions {
    pub pairs: HashMap<EntityPair, bool>,
}

impl Collisions {
    pub fn new() -> Self {
        Collisions {
            pairs: HashMap::new(),
        }
    }

    pub fn contains(&self, pair: &EntityPair) -> bool {
        self.pairs.contains_key(pair)
    }

    pub fn add(&mut self, pair: EntityPair) {
        self.pairs.insert(pair, true);
    }

    pub fn remove(&mut self, pair: &EntityPair) {
        self.pairs.remove(pair);
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Collisions::new())
            .add_event::<CollisionEvent>()
            .add_systems(PostUpdate, check_collisions);
    }
}

fn check_collisions(
    mut collisions: ResMut<Collisions>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<CollisionEvent>,
    query: Query<Entity>,
) {
    // let mut before = collisions.clone();
    // let mut events = Vec::<CollisionEvent>::new();

    // // first pass detects intersections
    // for (e1, e2, _) in rapier_context.intersection_pairs() {
    //     if !query.contains(e1) || !query.contains(e2) {
    //         // entity are removed from ECS
    //         // so we are not spawning events for them
    //         // they will be removed in last loop
    //         continue;
    //     }

    //     let pair = EntityPair::new(e1, e2);
    //     before.remove(&pair);
    //     if collisions.contains(&pair) {
    //         // we already have a started event for this intersection
    //         continue;
    //     }

    //     collisions.add(pair);
    //     events.push(CollisionEvent::Started(pair.entity1, pair.entity2));
    // }

    // // second pass detects contacts
    // for c in rapier_context.contact_pairs() {
    //     let e1 = c.collider1();
    //     let e2 = c.collider2();

    //     if !query.contains(e1) || !query.contains(e2) {
    //         // entity are removed from ECS
    //         // so we are not spawning events for them
    //         // they will be removed in last loop
    //         continue;
    //     }

    //     let pair = EntityPair::new(e1, e2);
    //     before.remove(&pair);
    //     if collisions.contains(&pair) {
    //         // we already have a started event for this intersection
    //         continue;
    //     }

    //     collisions.add(pair);
    //     events.push(CollisionEvent::Started(e1, e2));
    // }

    // // emit ended collisions
    // // TODO: implement iter() that return an iterator on the collisions struct
    // for (pair, _) in before.pairs {
    //     collisions.remove(&pair);
    //     events.push(CollisionEvent::Stopped(pair.entity1, pair.entity2));
    // }

    // event_writer.send_batch(events);
}
