


pub trait DataContainer<'a,T,V> {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>);
}

pub trait DataSampler<T,V> {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V>;
}


pub trait TimeLine<T: Clone> {
    type Item: Clone;
    /// Time should be relative to now
    fn get_sample(&self, time: T) -> Option<Self::Item>;
    fn set_key(&mut self, time: T, value: Self::Item);
    fn remove_key(&mut self, time: T) -> Option<Self::Item>;

    fn smooth_correct(&mut self, correction: (T, Self::Item), correction_time_segment: (T,T)) -> Option<Self::Item> {
        let start_value = self.get_sample(correction_time_segment.0.clone())?;
        self.set_key(correction.0.clone(), correction.1.clone());
        match self.get_sample(correction_time_segment.1.clone()) {
            Some(end_value) => {
                self.set_key(correction_time_segment.0, start_value);
                self.set_key(correction_time_segment.1.clone(), end_value.clone());
                Some(end_value.clone())
            }
            None => {
                self.remove_key(correction.0);
                None
            }
        }
    }
}



