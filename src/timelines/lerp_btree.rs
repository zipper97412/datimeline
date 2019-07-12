use crate::interface::{TimeLine, DataContainer, DataSampler};
use crate::samplers::LerpSampler;
use std::collections::BTreeMap;
use std::cmp::Ord;
use nalgebra::{VectorN, DimName, allocator::Allocator, DefaultAllocator, Scalar};
use alga::general::{ClosedAdd, ClosedSub, ClosedMul, ClosedDiv};
use num_traits::{Zero, One};

#[allow(dead_code)]
pub type LerpBtreeTimeline<N,D> = BTreeMap<N,VectorN<N,D>>;

impl<N,D> TimeLine<N> for LerpBtreeTimeline<N,D>
where 
    N: Clone + Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + Ord,
    D: DimName,
    DefaultAllocator: Allocator<N, D> {
    type Item = VectorN<N,D>;
    fn get(&self, time: N) -> Option<Self::Item> {
        let (mut past, mut future) = self.split_at(time);
        LerpSampler::sample(&mut past, &mut future, time)
    }
    fn set(&mut self, time: N, value: Self::Item) {
        self.insert(time, value);
    }
    fn remove(&mut self, time: N) -> Option<Self::Item> {
        self.remove(&time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector1;
    use std::iter::FromIterator;
    use std::cmp::Ord;
    fn adapt(input: Vec<(f32,f32)>) -> Vec<(f32, Vector1<f32>)> {
        input.into_iter()
            .map(|p|(p.0, Vector1::new(p.1)))
            .collect()
    }

    #[test]
    fn it_works() {
        let data = adapt(vec![(15_f32, 2_f32), (10.0, 1.0)]);
        let mut bmap = BTreeMap::from_iter(data);

        let (mut past, mut future) = bmap.split_at(17);
        assert_eq!(past.next(), Some((15,2)));
        assert_eq!(past.next(), Some((10,1)));

        assert_eq!(future.next(), Some((20,3)));
        assert_eq!(future.next(), Some((25,4)));
    }
}