extern crate nalgebra;
extern crate num_traits;
extern crate alga;

mod interface;
mod samplers;
mod containers;
mod timelines;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
