use crate::interface::DataSampler;
use lerp::Lerp;
use std::ops::{Sub, Div};

#[allow(dead_code)]
pub struct LerpSampler;

impl<T: Copy + Sub<Output = T> + Div<Output = T>, V: Lerp<T>> DataSampler<T,V> for LerpSampler {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V> {
        match (past_values.next(), future_values.next()) {

            //Interpolation
            (Some((pt, pv)), Some((ft, fv))) => Some(pv.lerp(fv, (time - pt)/(ft - pt))),

            //Try Extrapolation future
            (Some((pt, pv)),None) => match past_values.next() {
                //Extrapolation future from the last 2 values
                Some((pt2, pv2)) => Some(pv2.lerp(pv, (time - pt2)/(pt - pt2))),
                //Extrapolation failed
                None => None
            },

            //Try Extrapolation past
            (None, Some((ft, fv))) => match future_values.next() {
                //Extrapolation past from the next 2 values
                Some((ft2, fv2)) => Some(fv.lerp(fv2, (time - ft)/(ft2 - ft))),
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
    #[test]
    fn interpolation_success() {
        let past = vec![(15_f32, 2_f32), (10.0, 1.0)];
        let future = vec![(20_f32, 3_f32), (25.0, 4.0)];
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 17.5), Some(2.5));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), Some(2.0));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20.0), Some(3.0));

    }
    #[test]
    fn interpolation_failed() {
        let past: Vec<(f32,f32)> = Vec::new();
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 42.0), None);

    }
    #[test]
    fn extrapolation_future_success() {
        let past = vec![(15_f32, 2_f32), (10.0, 1.0)];
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), Some(2.0));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 20.0), Some(3.0));

    }
    #[test]
    fn extrapolation_future_failed() {
        let past = vec![(10.0, 1.0)];
        let future = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 15.0), None);

    }
    #[test]
    fn extrapolation_past_success() {
        let future = vec![(15_f32, 2_f32), (10.0, 1.0)];
        let past = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 10.0), Some(1.0));
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5.0), Some(0.0));

    }
    #[test]
    fn extrapolation_past_failed() {
        let future = vec![(10.0, 1.0)];
        let past = Vec::new();
        assert_eq!(LerpSampler::sample(&mut past.clone().into_iter(), &mut future.clone().into_iter(), 5.0), None);

    }
}