use crate::interface::DataSampler;


/// Implement StepSampler for last velue strategy
/// 
/// This sampler only return the last value of the data relative to intant t
/// > Note: since the generation of past_values and future_values iterators 
/// has already be done by the [DataContainer](../interface/trait.DataContainer.html)
/// the variables `future_values` and `t` are note used and the first value of 
/// `past_values` is simply returned
#[allow(dead_code)]
pub struct StepSampler;

impl<T,V> DataSampler<T,V> for StepSampler {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, _: &mut Iterator<Item = (T,V)>, _: T) -> Option<V> {
        Some(past_values.next()?.1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut past = vec![(15_i32, 2_i32), (10, 1)].into_iter();
        let mut future = vec![(20_i32, 3_i32), (25, 4)].into_iter();
        let sample = StepSampler::sample(&mut past, &mut future, 42_i32);
        assert_eq!(sample, Some(2));
    }
}
