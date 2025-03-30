use std::ops;

// https://codereview.stackexchange.com/questions/256345/n-dimensional-array-in-rust
#[derive(PartialEq,Debug)]
pub struct VecNd<const N: usize> {
    pub data: Vec<f32>,
}

impl<const N: usize> VecNd<N> {
    pub fn from_vec(vec: Vec<f32>) -> Self {
        VecNd {
            data: vec,
        }
    }
    pub fn zero() -> Self {
        VecNd {
            data: vec![0.0; N],
        }
    }
}

impl<const N: usize> VecNd<N> {
    pub fn dot(&self, v: &VecNd<N>) -> f32 {
        (self * v).data.iter().sum()
    }

    pub fn norm(&self) -> f32 {
        self.dot(&self).powf(0.5)
    }
}

impl<const N: usize> ops::Add<&VecNd<N>> for &VecNd<N> {
    type Output = VecNd<N>;

    fn add(self, rhs: &VecNd<N>)-> Self::Output {
        let mut sum = vec![0.0; N];
        for i in 0..N {
            sum[i] = self.data[i] + rhs.data[i];
        }
        VecNd::from_vec(sum)
    }
}

impl<const N: usize> ops::Sub<&VecNd<N>> for &VecNd<N> {
    type Output = VecNd<N>;

    fn sub(self, rhs: &VecNd<N>)-> Self::Output {
        let mut sum = vec![0.0; N];
        for i in 0..N {
            sum[i] = self.data[i] - rhs.data[i];
        }
        VecNd::from_vec(sum)
    }
}

impl<const N: usize> ops::Mul<&VecNd<N>> for &VecNd<N> {
    type Output = VecNd<N>;

    fn mul(self, rhs: &VecNd<N>)-> Self::Output {
        let mut sum = vec![0.0; N];
        for i in 0..N {
            sum[i] = self.data[i] * rhs.data[i];
        }
        VecNd::from_vec(sum)
    }
}

impl<const N: usize> ops::Mul<&VecNd<N>> for f32 {
    type Output = VecNd<N>;

    fn mul(self, rhs: &VecNd<N>)-> Self::Output {
        let mut sum = vec![0.0; N];
        for i in 0..N {
            sum[i] = self * rhs.data[i];
        }
        VecNd::from_vec(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_defn() {
        let v = vec![0.3, 0.2, 1.7];
        let result: VecNd<3> = VecNd::from_vec(v);
        assert_eq!(result, VecNd{
            data: vec![0.3, 0.2, 1.7],
        });
    }

    #[test]
    fn add() {
        let v: VecNd<3> = VecNd::from_vec(vec![0.3, 0.2, 1.7]);
        let w: VecNd<3>= VecNd::from_vec(vec![1.4, 2.2, 0.3]);
        let result = &v + &w;
        assert_eq!(
            result,
            VecNd{
                data: vec![1.7, 2.4, 2.0],
            }
        );
    }
}
