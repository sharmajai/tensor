#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use num_traits::Num;

// enum Assert<const COND: bool> {}
// trait IsTrue {}
// impl IsTrue for Assert<true> {}

trait Tensor<V: Num, const D: usize> {
    const C: usize;

    fn value(&self, index: usize) -> impl Tensor<V, { D - 1 }>
    where
        [(); D - 1]:;
}

// impl<V: Num, const C: usize, const D: usize> Tensor<V, C, D>
//     for [Box<dyn Tensor<V, C, { D - 1 }>>; C]
// {
//     fn value(&self, index: usize) -> Tensor<i32, 2, 1> {
//         // ...
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
