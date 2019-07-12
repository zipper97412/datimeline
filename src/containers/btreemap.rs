use std::collections::BTreeMap;
use std::cmp::Ord;
use crate::interface::DataContainer;


impl<'a, T: Clone+Ord, V: Clone> DataContainer<'a,T,V> for BTreeMap<T,V> {
    fn split_at(&'a self, time: T) -> (Box<Iterator<Item = (T,V)> + 'a>, Box<Iterator<Item = (T,V)> + 'a>) {
        (Box::new(self.range(..&time)
            .rev()
            .map(|(t,v)| (t.clone(),v.clone()))), 
         Box::new(self.range(&time..)
            .map(|(t,v)| (t.clone(),v.clone()))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut bmap = BTreeMap::new();
        bmap.insert(10, 1);
        bmap.insert(15, 2);
        bmap.insert(20, 3);
        bmap.insert(25, 4);

        let (mut past, mut future) = bmap.split_at(17);
        assert_eq!(past.next(), Some((15,2)));
        assert_eq!(past.next(), Some((10,1)));

        assert_eq!(future.next(), Some((20,3)));
        assert_eq!(future.next(), Some((25,4)));
    }
}