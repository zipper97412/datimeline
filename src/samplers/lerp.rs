use crate::interface::DataSampler;
use nalgebra::{VectorN, DimName, allocator::Allocator, DefaultAllocator, Scalar};
use alga::general::{ClosedAdd, ClosedSub};
use num_traits::{Float};
use std::ops::{Add, Sub, Mul, Div};


pub trait FloatMul<P: Float = f64> {
    fn mul(self, rhs: P) -> Self;
}

impl<T,P> FloatMul<P> for T 
where
    P: Float,
    T: Mul<P, Output = T> {
    fn mul(self, rhs: P) -> Self {
        self * rhs
    }
}



#[allow(dead_code)]
pub struct LerpSampler;

impl<T, N, D> DataSampler<T,VectorN<N,D>> for LerpSampler
where
    T: ClosedSub + Clone + Into<f64>,
    VectorN<N,D>: FloatMul,
    N: Scalar + ClosedAdd + ClosedSub,
    D: DimName,
    DefaultAllocator: Allocator<N, D> {
    fn sample(past_values: &mut Iterator<Item = (T,VectorN<N,D>)>, future_values: &mut Iterator<Item = (T,VectorN<N,D>)>, time: T) -> Option<VectorN<N,D>> {
        match (past_values.next(), future_values.next()) {

            //Interpolation
            (Some((pt, pv)), Some((ft, fv))) => Some(lerp(pv, fv, (time - pt.clone()).into()/(ft - pt).into())),

            //Try Extrapolation future
            (Some((pt, pv)),None) => match past_values.next() {
                //Extrapolation future from the last 2 values
                Some((pt2, pv2)) => Some(lerp(pv2, pv, (time - pt2).into()/(pt - pt2).into())),
                //Extrapolation failed
                None => None
            },

            //Try Extrapolation past
            (None, Some((ft, fv))) => match future_values.next() {
                //Extrapolation past from the next 2 values
                Some((ft2, fv2)) => Some(lerp(fv, fv2, (time - ft).into()/(ft2 - ft).into())),
                //Extrapolation failed
                None => None
            },

            //No value to (Inter/Extra)polate
            (None, None) => None
        }
    }
}

fn lerp<T,V>(a: V, b: V, t: T) -> V 
where
    T: Float,
    V: Clone + Add<Output = V> + Sub<Output = V> + FloatMul<T>,
{
    a.clone() + ((b - a).mul(t))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;
    fn adapt(input: Vec<(i32,i32)>) -> Vec<(i32, Vector1<i32>)> {
        input.into_iter()
            .map(|p|(p.0, Vector1::new(p.1)))
            .collect()
    }

    #[test]
    fn interpolation_success() {
        let past = adapt(vec![(15_i32, 2_i32), (10, 1)]);
        let future = adapt(vec![(20_i32, 3_i32), (25, 4)]);
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 17_i32), Some(Vector1::new(2)));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15), Some(Vector1::new(2)));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20), Some(Vector1::new(3)));

    }
    #[test]
    fn interpolation_failed() {
        let past: Vec<(f32,Vector1<f32>)> = Vec::new();
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 42.0), None);

    }
    #[test]
    fn extrapolation_future_success() {
        let past = adapt(vec![(15_f32, 2_f32), (10.0, 1.0)]);
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), Some(Vector1::new(2.0)));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20.0), Some(Vector1::new(3.0)));

    }
    #[test]
    fn extrapolation_future_failed() {
        let past = adapt(vec![(10.0, 1.0)]);
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), None);

    }
    #[test]
    fn extrapolation_past_success() {
        let future = adapt(vec![(15_f32, 2_f32), (10.0, 1.0)]);
        let past = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 10.0), Some(Vector1::new(1.0)));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5.0), Some(Vector1::new(0.0)));

    }
    #[test]
    fn extrapolation_past_failed() {
        let future = adapt(vec![(10.0, 1.0)]);
        let past = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5.0), None);

    }
}