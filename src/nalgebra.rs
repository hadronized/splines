use crate::impl_Interpolate;
use nalgebra::{Quaternion, Vector1, Vector2, Vector3, Vector4, Vector5, Vector6};

impl_Interpolate!(f32, Vector1<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Vector2<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Vector3<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Vector4<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Vector5<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Vector6<f32>, std::f32::consts::PI);
impl_Interpolate!(f32, Quaternion<f32>, std::f32::consts::PI);

impl_Interpolate!(f64, Vector1<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Vector2<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Vector3<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Vector4<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Vector5<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Vector6<f64>, std::f64::consts::PI);
impl_Interpolate!(f64, Quaternion<f64>, std::f64::consts::PI);
