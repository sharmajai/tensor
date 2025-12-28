use num_traits::Num;
use std::ops::Index;

trait Tensor0<V: Num> {
    fn value(&self) -> &V;
}

impl<V: Num> Tensor0<V> for V {
    fn value(&self) -> &V {
        self
    }
}

trait Tensor1<V: Num, const C1: usize>: Index<usize>
where
    Self::Output: Tensor0<V>,
{
}

impl<V: Num, const C1: usize> Tensor1<V, C1> for [V; C1] {}

trait Tensor2<V: Num, const C1: usize, const C2: usize>: Index<usize>
where
    Self::Output: Tensor1<V, C2>,
    <<Self as Index<usize>>::Output as Index<usize>>::Output: Tensor0<V>,
{
}

impl<V: Num, const C1: usize, const C2: usize> Tensor2<V, C1, C2> for [[V; C2]; C1] {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(*it(&a, 0, 0), 1);
        assert_eq!(*it(&a, 0, 1), 2);
        assert_eq!(*it(&a, 0, 2), 3);
        assert_eq!(*it(&a, 1, 0), 4);
        assert_eq!(*it(&a, 1, 1), 5);
        assert_eq!(*it(&a, 1, 2), 6);
    }
    fn it(t: &impl Tensor2<i32, 2, 3, Output = [i32; 3]>, i: usize, j: usize) -> &i32 {
        return &t[i][j];
    }
}
