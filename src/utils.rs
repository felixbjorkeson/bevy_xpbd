//! Miscallaneous utility functions.

use crate::prelude::*;

#[cfg(feature = "2d")]
pub(crate) fn make_isometry(pos: Vector, rot: &Rotation) -> Isometry<Scalar> {
    Isometry::<Scalar>::new(pos.into(), (*rot).into())
}

#[cfg(feature = "3d")]
pub(crate) fn make_isometry(pos: Vector, rot: &Rotation) -> Isometry<Scalar> {
    Isometry::<Scalar>::new(pos.into(), rot.to_scaled_axis().into())
}

pub(crate) fn entity_from_index_and_gen(index: u32, generation: u32) -> bevy::prelude::Entity {
    bevy::prelude::Entity::from_bits((generation as u64) << 32 | index as u64)
}

#[cfg(feature = "3d")]
pub(crate) fn get_rotated_inertia_tensor(inertia_tensor: Matrix3, rot: Quaternion) -> Matrix3 {
    let rot_mat3 = Matrix3::from_quat(rot);
    (rot_mat3 * inertia_tensor) * rot_mat3.transpose()
}

/// Computes the magnitude of the impulse caused by dynamic friction.
pub(crate) fn compute_dynamic_friction(
    tangent_speed: Scalar,
    generalized_inv_mass_sum: Scalar,
    coefficient: Scalar,
    normal_lagrange: Scalar,
    sub_dt: Scalar,
) -> Scalar {
    let normal_impulse = normal_lagrange / sub_dt;

    // Compute impulse caused by dynamic friction, clamped to never exceed the tangential speed.
    // Note: This is handled differently from the XPBD paper because it treated mass incorrectly.
    -(coefficient * normal_impulse.abs()).min(tangent_speed / generalized_inv_mass_sum)
}

/// Computes the speed correction caused by restitution.
pub(crate) fn compute_restitution(
    normal_speed: Scalar,
    pre_solve_normal_speed: Scalar,
    mut coefficient: Scalar,
    gravity: Vector,
    sub_dt: Scalar,
) -> Scalar {
    // If normal velocity is small enough, use restitution of 0 to avoid jittering
    if normal_speed.abs() <= 2.0 * gravity.length() * sub_dt {
        coefficient = 0.0;
    }

    -normal_speed + (-coefficient * pre_solve_normal_speed).min(0.0)
}
