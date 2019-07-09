
pub trait DataContainer<'a,T,V> where T: Copy {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>);
}

pub trait TimeLine<'a,T,V>: DataContainer<'a,T,V> where T: Copy {
    fn sample_value_at<S: DataSampler<T,V>>(&'a self, time: T) -> Option<V> {
        let (mut past, mut future) = self.split_at(time);
        S::sample(&mut past, &mut future, time)
    }
}

impl<'a,T: Copy, V, D> TimeLine<'a,T,V> for D where D: DataContainer<'a,T,V> {}


pub trait DataSampler<T,V> where T: Copy {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V>;
}

