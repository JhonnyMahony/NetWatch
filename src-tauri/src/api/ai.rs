use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa_nn::distance::LInfDist;
use ndarray::Array2;
use ndarray_rand::rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use serde::Serialize;
use tauri::Manager;

use crate::logic::show_packets::FormatedPacket;

#[derive(Serialize)]
pub struct Detect {
    cluster: i32,
    count: usize,
    avg_src_ip_freq: f64,
    avg_dst_ip_freq: f64,
    avg_packet_rate: f64,
    avg_packet_length: f64,
}

#[tauri::command]
pub fn dos_detect(app_handle: tauri::AppHandle) -> Vec<Detect> {
    let mut potential_dos = Vec::new();
    if let Some(packets) = app_handle.try_state::<Mutex<VecDeque<FormatedPacket>>>() {
        let rng = Xoshiro256Plus::seed_from_u64(42);
        let packets = packets.lock().unwrap();
        let mut src_ip_counts = HashMap::new();
        let mut dst_ip_counts = HashMap::new();
        let mut packet_timestamps: Vec<f64> = Vec::new();
        if packets.len() == 0 {
            return vec![];
        }
        for packet in &*packets {
            *src_ip_counts.entry(packet.src_ip).or_insert(0) += 1;
            *dst_ip_counts.entry(packet.dst_ip).or_insert(0) += 1;
            packet_timestamps.push(packet.timestamp);
        }

        // Calculate packet rate
        let time_window = packet_timestamps
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0)
            - packet_timestamps
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0);
        let packet_rate = if time_window > 0.0 {
            packets.len() as f64 / time_window
        } else {
            0.0
        };

        // Create feature matrix
        let mut features = Vec::new();
        for packet in &*packets {
            let src_ip_freq = *src_ip_counts.get(&packet.src_ip).unwrap_or(&0) as f64;
            let dst_ip_freq = *dst_ip_counts.get(&packet.dst_ip).unwrap_or(&0) as f64;
            let protocol_code = match packet.protocol.as_str() {
                "TCP" => 1.0,
                "UDP" => 2.0,
                "ICMP" => 3.0,
                _ => 0.0,
            };
            let packet_length = packet.lenght as f64;
            features.push(vec![
                src_ip_freq,
                dst_ip_freq,
                protocol_code,
                packet_rate,
                packet_length,
            ]);
        }

        // Convert to ndarray
        let n_rows = features.len();
        let n_cols = if n_rows > 0 { features[0].len() } else { 0 };
        if n_rows == 0 || n_cols == 0 {
            log::error!("No valid features extracted. Cannot perform clustering.");
        }
        let feature_array = Array2::from_shape_vec(
            (n_rows, n_cols),
            features.clone().into_iter().flatten().collect::<Vec<f64>>(),
        )
        .expect("Failed to create feature array");
        let dataset = DatasetBase::from(feature_array);

        // Configure and fit KMeans
        let n_clusters = 3;
        let model = KMeans::params_with(n_clusters, rng, LInfDist)
            .max_n_iterations(200)
            .tolerance(1e-5)
            .fit(&dataset)
            .expect("KMeans fitted");

        let predictions = model.predict(dataset);
        log::info!("{:?}", predictions);

        // Analyze clusters for DoS indicators
        let mut cluster_stats: HashMap<i32, (usize, f64, f64, f64, f64)> = HashMap::new(); // (count, avg_src_ip_freq, avg_dst_ip_freq, avg_packet_rate, avg_packet_length)
        for (i, &cluster) in predictions.targets().iter().enumerate() {
            if let Ok(cluster_i32) = cluster.try_into() {
                let stats = cluster_stats
                    .entry(cluster_i32)
                    .or_insert((0, 0.0, 0.0, 0.0, 0.0));
                stats.0 += 1;
                stats.1 += features[i][0]; // src_ip_freq
                stats.2 += features[i][1]; // dst_ip_freq
                stats.3 += features[i][3]; // packet_rate
                stats.4 += features[i][4]; // packet_length
            } else {
                log::warn!("Warning: Cluster index {} too large for i32", cluster);
            }
        }

        // Normalize stats
        for stats in cluster_stats.values_mut() {
            if stats.0 > 0 {
                stats.1 /= stats.0 as f64;
                stats.2 /= stats.0 as f64;
                stats.3 /= stats.0 as f64;
                stats.4 /= stats.0 as f64;
            }
        }

        // Detect potential DoS clusters
        for (
            cluster,
            (count, avg_src_ip_freq, avg_dst_ip_freq, avg_packet_rate, avg_packet_length),
        ) in cluster_stats
        {
            if avg_packet_rate > 100.0 || // High packet rate
           avg_src_ip_freq > 10.0 || // Low source diversity
           avg_dst_ip_freq > 10.0 || // Targeted destination
           (avg_packet_length < 100.0 && avg_packet_rate > 50.0)
            // Small packets at high rate
            {
                let dos = Detect {
                    cluster,
                    count,
                    avg_src_ip_freq,
                    avg_dst_ip_freq,
                    avg_packet_rate,
                    avg_packet_length,
                };
                potential_dos.push(dos);
                log::info!(
                "Potential DoS attack detected in cluster {}: count={}, avg_src_ip_freq={}, avg_dst_ip_freq={}, avg_packet_rate={}, avg_packet_length={}",
                cluster, count, avg_src_ip_freq, avg_dst_ip_freq, avg_packet_rate, avg_packet_length
            );
            }
        }
    }
    potential_dos
}
