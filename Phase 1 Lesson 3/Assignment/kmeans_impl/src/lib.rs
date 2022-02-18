//! An implementation of the K-means clustering algorithm for the Assignment of Lesson 3.

use std::sync::{Arc, Mutex};

use rand::Rng;

pub struct KMeansCluster {
    members: Vec<usize>,
    centroid: usize,
}

impl KMeansCluster {
    pub fn new(centroid: usize) -> Self {
        Self {
            members: Vec::new(),
            centroid,
        }
    }
}

/// A thread-safe implementation of the K-means clustering algorithm.
pub fn compute_kmeans(
    thread_guard: Arc<Mutex<i8>>,
    data_points: Arc<Vec<Vec<f32>>>,
    k: usize,
) -> (f32, Vec<Vec<f32>>) {
    // assert that the number of clusters is smaller than the number of data points of
    // the data
    assert!(k < data_points.len());

    // allocate the right amount of memory to avoid dynamically allocating more memory
    // when we push additional clusters to the vector
    let mut clusters = Vec::<KMeansCluster>::with_capacity(k);

    let mut rng = rand::thread_rng();

    // randomly pick the initial seeds for the centroids of the clusters
    (0..k).for_each(|_| {
        let centroid = rng.gen_range(0..data_points.len());
        clusters.push(KMeansCluster::new(centroid));
    });

    let mut iter = data_points.iter().enumerate();

    // iterate through each item in the data
    while let Some((index, current)) = iter.next() {
        let mut distances = Vec::<f32>::with_capacity(data_points.len());

        // for each of the cluster centroids, compute the Euclidean distance of the current
        // data point from that centroid
        clusters.iter().for_each(|cluster| {
            distances.push(euclidean_distance(
                &current[..],
                &data_points[cluster.centroid],
            ));
        });

        // find the cluster centroid we are the closest to for the current data point
        let closest = distances.iter().cloned().fold(1f32 / 0f32, f32::min);
        // find the cluster (its position in the cluster vector) we want from the computed
        // closest centroid
        let cluster = distances
            .iter()
            .position(|&centroid| centroid == closest)
            .unwrap();

        // update the cluster vector with the updated cluster with the new data point included
        // in it
        if let Some(cluster) = clusters.get_mut(cluster) {
            cluster.members.push(index);
        }
    }

    // now we have the initial arrangement of the clusters
    // attempt to optimize the centroids to make the clusters more optimal through each
    // iteration
    for _ in 0..data_points.len().pow(2) {
        // pick the best centroid for each cluster
        clusters.iter_mut().for_each(|cluster| {
            let mut distances: Vec<f32> = Vec::with_capacity(cluster.members.len());

            // simulate each cluster member as the cluster centroid
            cluster.members.iter().for_each(|simulated_centroid| {
                let mut sum = 0f32;

                cluster.members.iter().for_each(|member| {
                    sum += euclidean_distance(
                        &data_points[*member],
                        &data_points[*simulated_centroid],
                    );
                });

                distances.push(sum);
            });

            let min = distances.iter().cloned().fold(1. / 0., f32::min);
            if let Some(best) = distances.iter().position(|&x| x == min) {
                cluster.centroid = best;
            }

            // clear all members and re-assign membership for each cluster
            cluster.members.clear();
        });

        // iterate through each row in the matrix, and pick the best cluster membership
        let mut iter = data_points.iter().enumerate();
        while let Some((index, current)) = iter.next() {
            let mut distances = Vec::<f32>::with_capacity(data_points.len());

            // for each of the cluster centroids, compute the Euclidean distance of the current
            // data point from that centroid
            clusters.iter().for_each(|cluster| {
                distances.push(euclidean_distance(
                    &current[..],
                    &data_points[cluster.centroid],
                ));
            });

            // find the cluster centroid we are the closest to for the current data point
            let closest = distances.iter().cloned().fold(1f32 / 0f32, f32::min);
            // find the cluster (its position in the cluster vector) we want from the computed
            // closest centroid
            let cluster = distances
                .iter()
                .position(|&centroid| centroid == closest)
                .unwrap();

            // update the cluster vector with the updated cluster with the new data point included
            // in it
            if let Some(cluster) = clusters.get_mut(cluster) {
                cluster.members.push(index);
            }
        }
    }

    let _ = thread_guard.lock().unwrap();
    let mut membership = (0..data_points.len()).collect::<Vec<_>>();
    
    let mut score = 0f32;
    for (index, cluster) in clusters.iter().enumerate() {
        let sum = cluster
            .members
            .iter()
            .inspect(|&member| membership[*member] = index)
            .fold(0f32, |acc, &x| {
                acc + euclidean_distance(&data_points[x], &data_points[cluster.centroid])
            });
        score += sum;
    }

    // generate a new data point matrix containing the cluster index appended to the end
    let mut results = Vec::with_capacity(data_points.len());
    for (row, cluster_num) in data_points.iter().zip(membership.iter()) {
        let mut data = row.clone();

        data.push(*cluster_num as f32);
        results.push(data);
    }

    (score, results)
}

fn euclidean_distance(x1: &[f32], x2: &[f32]) -> f32 {
    let mut sum = 0.0f32;

    for (xi, xj) in x1.iter().zip(x2) {
        sum += (xi - xj).powi(2);
    }

    sum.sqrt()
}
