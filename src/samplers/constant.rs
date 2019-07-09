use crate::interface::DataSampler;

pub struct ConstSampler;

impl<T: Copy,V> DataSampler<T,V> for ConstSampler {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, _: &mut Iterator<Item = (T,V)>, _: T) -> Option<V> {
        Some(past_values.next()?.1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        assert_eq!(2 + 2, 4);
    }
}
