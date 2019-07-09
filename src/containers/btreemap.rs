use std::collections::BTreeMap;
use std::cmp::Ord;
use crate::interface::DataContainer;


impl<'a, T: Copy+Ord,V: Copy> DataContainer<'a,T,V> for BTreeMap<T,V> {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>) {

        (Box::new(self.range(..time)
            .rev()
            .map(|(&t,&v)| (t,v))), 
         Box::new(self.range(time..)
            .map(|(&t,&v)| (t,v))))
    }
}