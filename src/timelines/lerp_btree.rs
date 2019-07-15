use crate::interface::{TimeLine, DataContainer, DataSampler};
use crate::samplers::{LerpSampler, Lerp};
use std::collections::BTreeMap;
use std::cmp::Ord;
use std::ops::{Sub};
use num_traits::Float;
use std::ops::{Deref, DerefMut};


/// Linear interpolation sampler and b-tree map container
#[allow(dead_code)]
pub struct LerpBtreeTimeline<T,V: Lerp<F>,F: Float> {
    inner: BTreeMap<T,V>,
    _phantom: std::marker::PhantomData<F>
}

impl<T,V,F> LerpBtreeTimeline<T,V,F>
where
    F: Float,
    T: Clone + Ord + Sub<Output = T> + Into<F>,
    V: Clone + Lerp<F> {
    #[allow(dead_code)]
    pub fn new(btreemap: BTreeMap<T,V>) -> Self {
        LerpBtreeTimeline{
            inner: btreemap,
            _phantom: std::marker::PhantomData::<F>
        }
    }
}

impl<T,V,F> Deref for LerpBtreeTimeline<T,V,F> 
where
    V: Lerp<F>,
    F: Float {
    type Target = BTreeMap<T,V>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V,T,F> DerefMut for LerpBtreeTimeline<T,V,F> 
where
    V: Lerp<F>,
    F: Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T,V,F: Float> TimeLine<T> for LerpBtreeTimeline<T,V,F>
where 
    F: Float,
    T: Clone + Ord + Sub<Output = T> + Into<F>,
    V: Clone + Lerp<F> {
    type Item = V;
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

#[cfg(feature = "nalgebra_impl")]
#[cfg(test)]
mod tests_nalgebra {
    use super::*;
    use nalgebra::Vector1;
    use std::iter::FromIterator;
    use nalgebra::Scalar;
    fn adapt<T,N: Scalar>(input: Vec<(T,N)>) -> Vec<(T, Vector1<N>)> {
        input.into_iter()
            .map(|p|(p.0, Vector1::new(p.1)))
            .collect()
    }

    #[test]
    fn interpolation() {
        let data = adapt(vec![(15, 20), (10, 10)]);
        let bmap = LerpBtreeTimeline::<_,_,f64>::new(BTreeMap::from_iter(data));
        assert_eq!(bmap.get_sample(10), Some(Vector1::new(10)));
        assert_eq!(bmap.get_sample(15), Some(Vector1::new(20)));
        assert_eq!(bmap.get_sample(13), Some(Vector1::new(16)));
    }

    #[test]
    fn extrapolation_future() {
        let data = adapt(vec![(15, 20), (10, 10)]);
        let bmap = LerpBtreeTimeline::<_,_,f64>::new(BTreeMap::from_iter(data));
        assert_eq!(bmap.get_sample(10), Some(Vector1::new(10)));
        assert_eq!(bmap.get_sample(15), Some(Vector1::new(20)));
        assert_eq!(bmap.get_sample(17), Some(Vector1::new(24)));
    }

    #[test]
    fn extrapolation_past() {
        let data = adapt(vec![(15, 20), (10, 10)]);
        let bmap = LerpBtreeTimeline::<_,_,f64>::new(BTreeMap::from_iter(data));
        assert_eq!(bmap.get_sample(10), Some(Vector1::new(10)));
        assert_eq!(bmap.get_sample(15), Some(Vector1::new(20)));
        assert_eq!(bmap.get_sample(7), Some(Vector1::new(4)));
    }
}