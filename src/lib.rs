
/*! 
 * Timelines datastructures to represent the evolution of a value across time.
 * 
 * Timelines can use interpolation/extrapolation strategies to guess the value
 * at a given time (past or future) based on key values
 * 
 * Different *polation strategies can be used to guess the value of the data.
 * [sampler](samplers/index.html) contains different interpolation strategies
 */
#[cfg(feature = "nalgebra_impl")]
extern crate nalgebra;
extern crate num_traits;
//extern crate alga;

pub mod interface;
pub mod samplers;
pub mod containers;
pub mod timelines;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
