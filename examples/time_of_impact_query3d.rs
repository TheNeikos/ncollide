extern crate nalgebra as na;
extern crate ncollide;

use na::{Isometry3, Vector3};
use ncollide::shape::{Cuboid, Ball};
use ncollide::query;

fn main() {
    let cuboid = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
    let ball   = Ball::new(1.0);

    let cuboid_pos            = na::one();
    let ball_pos_intersecting = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), na::zero());
    let ball_pos_will_touch   = Isometry3::new(Vector3::new(2.0, 2.0, 2.0), na::zero());
    let ball_pos_wont_touch   = Isometry3::new(Vector3::new(3.0, 3.0, 3.0), na::zero());

    let box_vel1 = Vector3::new(-1.0, 1.0, 1.0);
    let box_vel2 = Vector3::new(1.0, 1.0, 1.0);

    let ball_vel1 = Vector3::new(2.0, 2.0, 2.0);
    let ball_vel2 = Vector3::new(-0.5, -0.5, -0.5);

    let toi_intersecting = query::time_of_impact(&ball_pos_intersecting, &ball_vel1, &ball,
                                                 &cuboid_pos,            &box_vel1,  &cuboid);
    let toi_will_touch = query::time_of_impact(&ball_pos_will_touch, &ball_vel2, &ball,
                                               &cuboid_pos,          &box_vel2,  &cuboid);
    let toi_wont_touch = query::time_of_impact(&ball_pos_wont_touch, &ball_vel1, &ball,
                                               &cuboid_pos,          &box_vel1,  &cuboid);

    assert_eq!(toi_intersecting, Some(0.0));
    assert!(toi_will_touch.is_some() && toi_will_touch.unwrap() > 0.0);
    assert_eq!(toi_wont_touch, None);
}
