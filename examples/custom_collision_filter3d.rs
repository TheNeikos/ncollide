extern crate nalgebra as na;
extern crate ncollide;

use na::{Point3, Isometry3};
use ncollide::shape::{Ball, ShapeHandle};
use ncollide::broad_phase::BroadPhasePairFilter;
use ncollide::world::{CollisionObject3, CollisionGroups, GeometricQueryType, CollisionWorld};

struct ParityFilter;

impl BroadPhasePairFilter<Point3<f32>, Isometry3<f32>, ()> for ParityFilter {
    fn is_pair_valid(&self,
                     b1: &CollisionObject3<f32, ()>,
                     b2: &CollisionObject3<f32, ()>)
                     -> bool {
        b1.uid % 2 == b2.uid % 2
    }
}

fn main() {
    let shape  = ShapeHandle::new(Ball::new(0.5f32));
    let groups = CollisionGroups::new();
    let query  = GeometricQueryType::Contacts(0.0);

    let mut world = CollisionWorld::new(0.02, true);

    world.register_broad_phase_pair_filter("Parity filter", ParityFilter);

    world.add(0, na::one(), shape.clone(), groups, query, ());
    world.add(1, na::one(), shape.clone(), groups, query, ());
    world.add(2, na::one(), shape.clone(), groups, query, ());
    world.add(3, na::one(), shape.clone(), groups, query, ());

    world.update();

    // There will be only 2 contacts instead of 6.
    assert!(world.contacts().count() == 2);
}
