use crate::interface::DataSampler;
use nalgebra::{MatrixMN, Dim, Scalar, allocator::Allocator, DefaultAllocator};
use num_traits::{Float};
use std::ops::{Add, Sub};


pub trait Lerp<F> {
    fn lerp(self, other: Self, t: F) -> Self;
}

impl<N, R, C, F> Lerp<F> for MatrixMN<N, R, C>
where
    N: Clone + Scalar + Lerp<F>,
    MatrixMN<N, R, C>: Clone + Add<Output=MatrixMN<N, R, C>> + Sub<Output=MatrixMN<N, R, C>>, 
    F: Float,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C> {
    fn lerp(self, other: Self, t: F) -> Self {
        let mut self_matrix = self.into_owned();
        let mut other_matrix = other.into_owned();
        for (s, o) in self_matrix.as_mut_slice().iter_mut().zip(other_matrix.as_mut_slice().iter_mut()) {
            *s = s.lerp(*o,t);
        }
        self_matrix
    }
}

macro_rules! impl_lerp {
    ($p:ty, $t:ty) => {
        impl Lerp<$p> for $t {
            fn lerp(self, other: Self, t: $p) -> Self {
                self + ((other - self) as $p * t) as Self
            }
        }
    };
}

impl_lerp!(f32, u8);
impl_lerp!(f32, i8);
impl_lerp!(f32, u16);
impl_lerp!(f32, i16);
impl_lerp!(f32, u32);
impl_lerp!(f32, i32);
impl_lerp!(f32, f32);
impl_lerp!(f32, u64);
impl_lerp!(f32, i64);
impl_lerp!(f32, f64);
impl_lerp!(f32, u128);
impl_lerp!(f32, i128);

impl_lerp!(f64, u8);
impl_lerp!(f64, i8);
impl_lerp!(f64, u16);
impl_lerp!(f64, i16);
impl_lerp!(f64, u32);
impl_lerp!(f64, i32);
impl_lerp!(f64, f32);
impl_lerp!(f64, u64);
impl_lerp!(f64, i64);
impl_lerp!(f64, f64);
impl_lerp!(f64, u128);
impl_lerp!(f64, i128);

#[allow(dead_code)]
pub struct LerpSampler<F = f64> {
    _phantom: std::marker::PhantomData<F>
}

impl<T, V, F> DataSampler<T,V> for LerpSampler<F>
where
    F: Float,
    T: Sub<Output = T> + Clone + Into<F>,
    V: Lerp<F> {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V> {
        match (past_values.next(), future_values.next()) {

            //Interpolation
            (Some((pt, pv)), Some((ft, fv))) => Some(pv.lerp(fv, (time - pt.clone()).into()/(ft - pt).into())),

            //Try Extrapolation future
            (Some((pt, pv)),None) => match past_values.next() {
                //Extrapolation future from the last 2 values
                Some((pt2, pv2)) => Some(pv2.lerp(pv, (time - pt2.clone()).into()/(pt - pt2).into())),
                //Extrapolation failed
                None => None
            },

            //Try Extrapolation past
            (None, Some((ft, fv))) => match future_values.next() {
                //Extrapolation past from the next 2 values
                Some((ft2, fv2)) => Some(fv.lerp(fv2, (time - ft.clone()).into()/(ft2 - ft).into())),
                //Extrapolation failed
                None => None
            },

            //No value to (Inter/Extra)polate
            (None, None) => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;
    fn adapt<T,N: Scalar>(input: Vec<(T,N)>) -> Vec<(T, Vector1<N>)> {
        input.into_iter()
            .map(|p|(p.0, Vector1::new(p.1)))
            .collect()
    }

    #[test]
    fn interpolation_success() {
        let past = adapt(vec![(15_i32, 20_i32), (10, 1)]);
        let future = adapt(vec![(20_i32, 30_i32), (25, 4)]);
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 17), Some(Vector1::new(24)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), Some(Vector1::new(20)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20), Some(Vector1::new(30)));

    }
    #[test]
    fn interpolation_success_i32_f32() {
        let past = adapt(vec![(15, 20.26_f32), (10, 1.0)]);
        let future = adapt(vec![(20, 30_f32), (25, 4.0)]);
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 18), Some(Vector1::new(26.104)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), Some(Vector1::new(20.26)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20), Some(Vector1::new(30.0)));

    }
    #[test]
    fn interpolation_success_i32_f64() {
        let past = adapt(vec![(15, 20.26_f64), (10, 1.0)]);
        let future = adapt(vec![(20, 30_f64), (25, 4.0)]);
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 18), Some(Vector1::new(26.104)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), Some(Vector1::new(20.26)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20), Some(Vector1::new(30.0)));

    }
    #[test]
    fn interpolation_success_f32_i32() {
        let past = adapt(vec![(15_f32, 20), (10.0, 1)]);
        let future = adapt(vec![(20_f32, 30), (25.0, 4)]);
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 18.0), Some(Vector1::new(26)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), Some(Vector1::new(20)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20.0), Some(Vector1::new(30)));
    }
    #[test]
    fn interpolation_success_f64_i32() {
        let past = adapt(vec![(15_f64, 20), (10.0, 1)]);
        let future = adapt(vec![(20_f64, 30), (25.0, 4)]);
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 18.0), Some(Vector1::new(26)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), Some(Vector1::new(20)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20.0), Some(Vector1::new(30)));
    }
    
    #[test]
    fn interpolation_failed() {
        let past: Vec<(i32,Vector1<i32>)> = Vec::new();
        let future = Vec::new();
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 42), None);

    }
    #[test]
    fn extrapolation_future_success() {
        let past = adapt(vec![(15, 2), (10, 1)]);
        let future = Vec::new();
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), Some(Vector1::new(2)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20), Some(Vector1::new(3)));

    }
    #[test]
    fn extrapolation_future_failed() {
        let past = adapt(vec![(10, 1)]);
        let future = Vec::new();
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), None);

    }
    #[test]
    fn extrapolation_past_success() {
        let future = adapt(vec![(15, 2), (10, 1)]);
        let past = Vec::new();
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 10), Some(Vector1::new(1)));
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5), Some(Vector1::new(0)));

    }
    #[test]
    fn extrapolation_past_failed() {
        let future = adapt(vec![(10, 1)]);
        let past = Vec::new();
        assert_eq!(LerpSampler::<f64>::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5), None);

    }
}