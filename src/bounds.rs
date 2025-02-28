use std::ops::{Add, AddAssign};

use bevy::math::Vec3;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    pub mins: Vec3,
    pub maxs: Vec3,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            mins: Vec3::splat(std::f32::MAX),
            maxs: Vec3::splat(-std::f32::MAX),
        }
    }
}

impl Bounds {

    pub fn from_points(pts: &[Vec3]) -> Self {
        pts.iter().fold(Bounds::default(), |acc, pt| acc + *pt)
    }
    // fn clear(&mut self) {
    //     *self = Self::new();
    // }

    // fn does_intersect(&self, rhs: &Self) -> bool {
    //     if self.maxs.cmplt(rhs.mins).any() {
    //         false
    //     } else if rhs.maxs.cmplt(self.mins).any() {
    //         false
    //     } else {
    //         true
    //     }
    // }

    // fn expand_by_points(&mut self, points: &[Vec3]) {
    //     for point in points {
    //         self.expand_by_point(*point);
    //     }
    // }

    pub fn expand_by_point(&mut self, rhs: Vec3) {
        self.mins = Vec3::select(rhs.cmplt(self.mins), rhs, self.mins);
        self.maxs = Vec3::select(rhs.cmpgt(self.maxs), rhs, self.maxs);
    }

    // fn expand_by_bounds(&mut self, rhs: &Self) {
    //     self.expand_by_point(rhs.mins);
    //     self.expand_by_point(rhs.maxs);
    // }

    pub fn width(&self) -> Vec3 {
        self.maxs - self.mins
    }

    // fn width_x(&self) -> f32 {
    //     self.maxs.x - self.mins.x
    // }

    // fn width_y(&self) -> f32 {
    //     self.maxs.y - self.mins.y
    // }

    // fn width_z(&self) -> f32 {
    //     self.maxs.z - self.mins.z
    // }
}

impl Add<Vec3> for Bounds {
    type Output = Self;
    fn add(self, pt: Vec3) -> Self::Output {
        Bounds {
            mins: Vec3::select(pt.cmplt(self.mins), pt, self.mins),
            maxs: Vec3::select(pt.cmpgt(self.maxs), pt, self.maxs),
        }
    }
}

impl AddAssign<Vec3> for Bounds {
    fn add_assign(&mut self, pt: Vec3) {
        self.mins = Vec3::select(pt.cmplt(self.mins), pt, self.mins);
        self.maxs = Vec3::select(pt.cmpgt(self.maxs), pt, self.maxs);
    }
}
