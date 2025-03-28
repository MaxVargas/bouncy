use std::ops;

// https://codereview.stackexchange.com/questions/256345/n-dimensional-array-in-rust
#[derive(PartialEq,Debug)]
pub struct VecNd<const N: usize> {
    data: Vec<f32>,
}

impl<const N: usize> VecNd<N> {
    pub fn from_vec(vec: Vec<f32>) -> Self {
        VecNd {
            data: vec,
        }
    }
}

impl<const N: usize> ops::Add<VecNd<N>> for VecNd<N> {
    type Output = VecNd<N>;

    fn add(self, rhs: VecNd<N>)-> Self::Output {
        let mut sum = vec![0.0; N];
        for i in 0..N {
            sum[i] = self.data[i] + rhs.data[i];
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
        let result = v + w;
        assert_eq!(
            result,
            VecNd{
                data: vec![1.7, 2.4, 2.0],
            }
        );
    }
}

// TODO: Implement these and make tests
// fn sub(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
//     let mut sum = vec![];
//     let n = v1.len();
//     for i in 0..n {
//         sum.push(v1[i] - v2[i]);
//     }
//     sum
// }
// fn scale(v1: &Vec<f32>, s: f32) -> Vec<f32> {
//     let mut v2 = vec![];
//     let n = v1.len();
//     for i in 0..n {
//         v2.push(v1[i] * s);
//     }
//     v2
// }
// fn dot(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
//     let mut sum = 0.0;
//     let n = v1.len();
//     for i in 0..n {
//         sum += v1[i] * v2[i];
//     }
//     sum
// }
// fn norm(v1: &Vec<f32>) -> f32 {
//     dot(v1, v1).pow(0.5)
// }
