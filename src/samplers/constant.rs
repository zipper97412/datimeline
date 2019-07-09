use crate::interface::DataSampler;

#[allow(dead_code)]
pub struct ConstSampler;

impl<T: Copy,V> DataSampler<T,V> for ConstSampler {
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
        let sample = ConstSampler::sample(&mut past, &mut future, 42_i32);
        assert_eq!(sample, Some(2));
    }
}
