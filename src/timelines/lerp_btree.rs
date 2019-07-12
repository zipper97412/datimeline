use crate::interface::{TimeLine, DataContainer, DataSampler};
use crate::samplers::{LerpSampler, Lerp};
use std::collections::BTreeMap;
use std::cmp::Ord;
use std::ops::{Sub};
use nalgebra::{VectorN, DimName, allocator::Allocator, DefaultAllocator, Scalar};

#[allow(dead_code)]
pub type LerpBtreeTimeline<T,N,D> = BTreeMap<T,VectorN<N,D>>;

impl<T,N,D> TimeLine<T> for LerpBtreeTimeline<T,N,D>
where 
    T: Clone + Ord + Sub<Output = T> + Into<f64>,
    N: Scalar,
    VectorN<N,D>: Lerp<f64>,
    D: DimName,
    DefaultAllocator: Allocator<N, D> {
    type Item = VectorN<N,D>;
    fn get_sample(&self, time: T) -> Option<Self::Item> {
        let (mut past, mut future) = self.split_at(time.clone());
        LerpSampler::sample(&mut past, &mut future, time)
    }
    fn set_key(&mut self, time: T, value: Self::Item) {
        self.insert(time, value);
    }
    fn remove_key(&mut self, time: T) -> Option<Self::Item> {
        self.remove(&time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;
    use std::iter::FromIterator;
    fn adapt<T,N: Scalar>(input: Vec<(T,N)>) -> Vec<(T, Vector1<N>)> {
        input.into_iter()
            .map(|p|(p.0, Vector1::new(p.1)))
            .collect()
    }

    #[test]
    fn interpolation_i32_i32() {
        let data = adapt(vec![(15, 20), (10, 10)]);
        let bmap = BTreeMap::from_iter(data);
        assert_eq!(bmap.get_sample(10), Some(Vector1::new(10)));
        assert_eq!(bmap.get_sample(15), Some(Vector1::new(20)));
        assert_eq!(bmap.get_sample(13), Some(Vector1::new(16)));
    }
}