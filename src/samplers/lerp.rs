use crate::interface::DataSampler;
use lerp::Lerp;
use std::ops::{Sub, Div};

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