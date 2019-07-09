use crate::interface::*;
use crate::samplers::LerpSampler;
use std::collections::BTreeMap;
use std::cmp::Ord;
use lerp::Lerp;
use std::ops::{Sub, Div};
use std::ops::{Deref,DerefMut};

#[allow(dead_code)]
pub struct LerpBtreeTimeline<T,V>(BTreeMap<T,V>);

impl<T,V> Deref for LerpBtreeTimeline<T,V> {
    type Target = BTreeMap<T,V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T,V> DerefMut for LerpBtreeTimeline<T,V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy + Ord+ Sub<Output = T> + Div<Output = T>,
     V: Copy + Lerp<T>> TimeLine<T,V> for LerpBtreeTimeline<T,V> {
    fn get(&self, time: T) -> Option<V> {
        let (mut past, mut future) = self.split_at(time);
        LerpSampler::sample(&mut past, &mut future, time)
    }
    fn set(&mut self, time: T, value: V) {
        self.insert(time, value);
    }
}