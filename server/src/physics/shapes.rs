// Yoinked this from bevy::render::Primitives so the render feature doesn't have to be
// added.

use bevy::ecs::{component::Component, reflect::ReflectComponent};
use bevy::math::DVec3;
use bevy::reflect::Reflect;

/// An Axis-Aligned Bounding Box
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Aabb {
    pub center: DVec3,
    pub half_extents: DVec3,
}

impl Aabb {
    #[inline]
    pub fn from_min_max(minimum: DVec3, maximum: DVec3) -> Self {
        let minimum = DVec3::from(minimum);
        let maximum = DVec3::from(maximum);
        let center = 0.5 * (maximum + minimum);
        let half_extents = 0.5 * (maximum - minimum);
        Self {
            center,
            half_extents,
        }
    }

    /// Calculate the relative radius of the AABB with respect to a plane
    #[inline]
    pub fn relative_radius(&self, p_normal: &DVec3, axes: &[DVec3]) -> f64 {
        // NOTE: dot products on Vec3A use SIMD and even with the overhead of conversion are net faster than Vec3
        let half_extents = self.half_extents;
        DVec3::new(
            p_normal.dot(axes[0]),
            p_normal.dot(axes[1]),
            p_normal.dot(axes[2]),
        )
        .abs()
        .dot(half_extents)
    }

    #[inline]
    pub fn min(&self) -> DVec3 {
        self.center - self.half_extents
    }

    #[inline]
    pub fn max(&self) -> DVec3 {
        self.center + self.half_extents
    }
}

//impl From<Sphere> for Aabb {
//    #[inline]
//    fn from(sphere: Sphere) -> Self {
//        Self {
//            center: sphere.center,
//            half_extents: Vec3A::splat(sphere.radius),
//        }
//    }
//}
//
//#[derive(Clone, Debug, Default)]
//pub struct Sphere {
//    pub center: Vec3A,
//    pub radius: f32,
//}
//
//impl Sphere {
//    #[inline]
//    pub fn intersects_obb(&self, aabb: &Aabb, local_to_world: &Mat4) -> bool {
//        let aabb_center_world = *local_to_world * aabb.center.extend(1.0);
//        let axes = [
//            Vec3A::from(local_to_world.x_axis),
//            Vec3A::from(local_to_world.y_axis),
//            Vec3A::from(local_to_world.z_axis),
//        ];
//        let v = Vec3A::from(aabb_center_world) - self.center;
//        let d = v.length();
//        let relative_radius = aabb.relative_radius(&(v / d), &axes);
//        d < self.radius + relative_radius
//    }
//}
