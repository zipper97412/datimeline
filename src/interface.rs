/*!
 * Trais to implement Timelines
 * 
 * Those trait are not interdependant but can be used to implement concrete timelines.
 * 
 * [timelines](../timelines/index.html) module use those traits to implement timelines with different
 * datastructures and samplers strategies
 */


/// Datastrucutres that can hold data keys
/// 
/// The datastructure need to hold the data keys but also give back an iterator of past values
/// and an iterator of future values for a particular instant of time
/// 
/// T: the type that encode time (ex: u32)
/// V: the data type that we try to represent (ex: player position)
pub trait DataContainer<'a,T,V> {
    /// Split the data collection in half, returning an iterator of past values relative to instant `time` (most recent first)
    /// and an iterator of future values relative to instant `time` (oldest first)
    /// (past, future)
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>);
}

/// Represents a strategy used to inter/extrapolate values
/// ex: Linear
/// T: the type that encode time (ex: u32)
/// V: the data type that we try to represent (ex: player position)
pub trait DataSampler<T,V> {
    /// take all the past and future data keys relative to time to feed to the sampling algorithme
    /// if not enough data keys are available (or if they are not in order), the sampling algorithme can fail by returning None
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V>;
}


/// A Timeline that represent the evolution of a value across time
/// 
/// Implementations provides a way to sample (guess) a value at any given time.
/// Also it should be possible to add and remove any data key at any point of time.
/// 
/// By default the time variable for every method is absolute.
/// Adapters can be used to change the behaviour of any timeline
/// 
/// T: the type that encode time (ex: u32)
pub trait TimeLine<T: Clone> {
    type Item: Clone;
    /// Try to guess a value for a particular instant 
    /// The strategy used to inter/extrapolate depend on the implementation 
    /// and can fail by returning None
    fn get_sample(&self, time: T) -> Option<Self::Item>;

    /// Add a new data key at a given time.
    /// WARNING depending on the sample strategy used, this action may change timeline value around this instant
    fn set_key(&mut self, time: T, value: Self::Item);

    /// Remove a data key at a given time.
    /// WARNING depending on the sample strategy used, this action may change timeline value around this instant
    fn remove_key(&mut self, time: T) -> Option<Self::Item>;

    /// Add a new data key to the timeline with a smooth correction,
    /// `correction` is the new data key to add
    /// `correction_time_segment.0` is the instant at which the data should stay the same (typically Now)
    /// `correction_time_segment.1` is the instant at which the correction should end (ex: 200 ms)
    /// 
    /// This method can be used to imlement dead reckoning with smooth correction
    /// The smooth correction might fail due to sampling fail, in this case the timeline is cleaned up before returning None
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



