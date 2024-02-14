mod traits;

use array::uninitialized;
use traits::*;

pub enum Error {
    NoSample,
}

pub fn means<const K: usize, T, D>(points: Vec<T>) -> Result<[T; K], Error>
where
    T: Copy + Eq + std::fmt::Debug,
    D: Distance<T> + Ord,
    Vec<T>: Mean<T> + Sample<T, K>,
{
    let mut centers = points.sample().ok_or(Error::NoSample)?;
    uninitialized!(cluster_points, K);
    loop {
        for i in 0..K {
            cluster_points[i] = Vec::<T>::new();
        }
        for point in &points {
            let (nearest, _) = (0..centers.len() - 1)
                .map(|i| (i, D::distance(&centers[i], &point)))
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            cluster_points[nearest].push(*point);
        }
        let new_centers: [T; K] = cluster_points
            .iter()
            .map(Mean::mean)
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();
        if centers == new_centers {
            break;
        }
        centers = new_centers;
    }
    Ok(centers)
}
