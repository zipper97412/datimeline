
pub trait DataContainer<'a,T,V> {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>);
}

pub trait TimeLine<T,V> {
    fn get(&self, time: T) -> Option<V>;
    fn set(&mut self, time: T, value: V);
}


pub trait DataSampler<T,V> {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V>;
}

