
use crate::interface::DataSampler;
use nalgebra::base::dimension::*;
use nalgebra::*;

#[allow(dead_code)]
pub struct CubicSampler;

impl<T: Copy,V> DataSampler<T,V> for CubicSampler {
    fn sample(past_values: &mut Iterator<Item = (T,V)>, future_values: &mut Iterator<Item = (T,V)>, time: T) -> Option<V> {
        match (past_values.next(), future_values.next()) {

            (Some(p0), Some(f0)) => match (past_values.next(), future_values.next()) {
                //ok
                (Some(p1), Some(f1)) => unimplemented!(),

                (Some(p1), None) => match past_values.next() {
                    Some(p2) => unimplemented!(),
                    None => None
                },

                (None, Some(f1)) => match future_values.next() {
                    Some(f2) => unimplemented!(),
                    None => None
                },

                (None, None) => None
            },

            (Some(p0), None) => match past_values.take(3).collect::<Vec<(T,V)>>().as_slice() {
                [p1,p2,p3] => unimplemented!(),
                _ => None
            }

            (None, Some(f0)) => match future_values.take(3).collect::<Vec<(T,V)>>().as_slice() {
                [f1,f2,f3] => unimplemented!(),
                _ => None
            }

            //No
            (None, None) => None
        }
    }
}

fn cubic_interpolate<N, D: Dim>(p: [(f32,VectorN<N, D>);4], t: f32) -> VectorN<N, D> {
    let M = Matrix4::new(
        p[1].0.powi(3), p[1].0.powi(2), p[1].0, 1.0,
        3.0*p[1].0.powi(2), 2.0*p[1].0, 1.0, 0.0,
        p[2].0.powi(3), p[2].0.powi(2), p[2].0, 1.0,
        3.0*p[2].0.powi(2), 2.0*p[2].0, 1.0, 0.0);
    let C = Vector4::new(
        p[1].1,
        (p[2].1 - p[0].1)/(p[2].0 - p[0].0),
        p[2].1,
        (p[3].1 - p[1].1)/(p[3].0 - p[1].0),
    );
}
