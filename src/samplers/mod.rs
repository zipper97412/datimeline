/*!
 * Contains [Samplers](../interface/trait.DataSampler.html) implementations
 */
mod step;
mod lerp;
//mod cubic;

pub use self::step::StepSampler;
pub use self::lerp::{LerpSampler, Lerp};
//pub use self::cubic::CubicSampler;