


pub trait DataContainer<'a,T,V> {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>);
}

pub trait DataSampler<T,V> {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V>;
}


pub trait TimeLine<T: Clone> {
    type Item: Clone;
    /// Time should be relative to now
    fn get(&self, time: T) -> Option<Self::Item>;
    fn set(&mut self, time: T, value: Self::Item);
    fn remove(&mut self, time: T) -> Option<Self::Item>;

    fn smooth_correct(&mut self, correction: (T, Self::Item), correction_time_segment: (T,T)) -> Option<Self::Item> {
        let start_value = self.get(correction_time_segment.0.clone())?;
        self.set(correction.0.clone(), correction.1.clone());
        match self.get(correction_time_segment.1.clone()) {
            Some(end_value) => {
                self.set(correction_time_segment.0, start_value);
                self.set(correction_time_segment.1.clone(), end_value.clone());
                Some(end_value.clone())
            }
            None => {
                self.remove(correction.0);
                None
            }
        }
    }
}



