use crate::impl_Interpolate;
use glam::{DQuat, DVec2, DVec3, DVec4, Quat, Vec2, Vec3, Vec3A, Vec4};

impl_Interpolate!(f32, Vec2, std::f32::consts::PI);
impl_Interpolate!(f32, Vec3, std::f32::consts::PI);
impl_Interpolate!(f32, Vec3A, std::f32::consts::PI);
impl_Interpolate!(f32, Vec4, std::f32::consts::PI);
impl_Interpolate!(f32, Quat, std::f32::consts::PI);

impl_Interpolate!(f64, DVec2, std::f64::consts::PI);
impl_Interpolate!(f64, DVec3, std::f64::consts::PI);
impl_Interpolate!(f64, DVec4, std::f64::consts::PI);
impl_Interpolate!(f64, DQuat, std::f64::consts::PI);
