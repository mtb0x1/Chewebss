use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn normalise(weights: &[f64]) -> Vec<f64> {
    //sum
    let sum: f64 = weights.iter().map(|&w| 10f64.powf(w)).sum();

    // Calculate the reciprocal of the sum.
    let reciprocal = 1.0 / sum;

    // Multiply each weight by the reciprocal and collect.
    weights
        .iter()
        .map(|&w| 10f64.powf(w) * reciprocal)
        .collect()
}

fn normalise1(weights: &[f64]) -> Vec<f64> {
    // Create a HashMap to cache pow calculations.
    let mut pow_cache: HashMap<String, f64> = HashMap::new();

    // Function to calculate 10^w, using the cache if available.
    let mut calculate_pow = |w: f64| -> f64 {
        let key = w.to_string(); //hack since f64 doesn't have Eq
        *pow_cache
            .entry(key.clone())
            .or_insert_with(|| 10f64.powf(w))
    };

    // Calculate the sum and populate the cache.
    let sum: f64 = weights.iter().map(|&w| calculate_pow(w)).sum();

    // Calculate the reciprocal of the sum.
    let reciprocal = 1.0 / sum;

    // Multiply each weight by the reciprocal and collect.
    weights
        .iter()
        .map(|&w| calculate_pow(w) * reciprocal)
        .collect()
}

pub fn normalise2(weights: &[f64]) -> Vec<f64> {
    let mut weights = weights.iter().map(|a| 10f64.powf(*a)).collect::<Vec<f64>>();

    let sum = weights.iter().sum::<f64>();

    for weight in &mut weights {
        *weight /= sum;
    }
    weights
}

pub fn normalise3(weights: &[f64]) -> Vec<f64> {
    let mut weights = weights.iter().map(|a| 10f64.powf(*a)).collect::<Vec<f64>>();

    let reciprocal = 1.0 / weights.iter().sum::<f64>();

    for weight in &mut weights {
        *weight *= reciprocal;
    }
    weights
}

pub fn normalise4(weights: &[f64]) -> Vec<f64> {
    let mut weights = weights.iter().map(|a| 10f64.powf(*a)).collect::<Vec<f64>>();

    let sum = weights.iter().sum::<f64>();

    for weight in &mut weights.iter_mut() {
        *weight /= sum;
    }
    weights
}

pub fn normalise5(weights: &[f64]) -> Vec<f64> {
    let mut weights = weights.iter().map(|a| 10f64.powf(*a)).collect::<Vec<f64>>();

    let reciprocal = 1.0 / weights.iter().sum::<f64>();

    for weight in &mut weights.iter_mut() {
        *weight *= reciprocal;
    }
    weights
}

#[derive(Debug, Clone, Copy)]
struct FloatWrapper(f64);

impl Eq for FloatWrapper {}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        // Implement your custom equality logic here if needed.
        // For example, you can compare the two f64 values with a small epsilon.
        (self.0 - other.0).abs() < 1e-6
    }
}

impl Hash for FloatWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Implement a custom hashing strategy.
        // Here, we use the raw bits representation of the f64 for hashing.
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

fn normalise6(weights: &[f64]) -> Vec<f64> {
    // Create a HashMap to cache pow calculations.
    let mut pow_cache: HashMap<FloatWrapper, f64> = HashMap::new();

    // Function to calculate 10^w, using the cache if available.
    let mut calculate_pow = |w: f64| -> f64 {
        *pow_cache
            .entry(FloatWrapper(w))
            .or_insert_with(|| 10f64.powf(w))
    };

    // Calculate the sum and populate the cache.
    let sum: f64 = weights.iter().map(|&w| calculate_pow(w)).sum();

    // Calculate the reciprocal of the sum.
    let reciprocal = 1.0 / sum;

    // Multiply each weight by the reciprocal and collect.
    weights
        .iter()
        .map(|&w| calculate_pow(w) * reciprocal)
        .collect()
}

fn normalise7(weights: &[f64]) -> Vec<f64> {
    // Create a HashMap to cache pow calculations.
    let mut pow_cache: HashMap<FloatWrapper, f64> = HashMap::new();

    // Function to calculate 10^w, using the cache if available.
    let mut calculate_pow = |w: f64| -> f64 {
        *pow_cache
            .entry(FloatWrapper(w))
            .or_insert_with(|| 10f64.powf(w))
    };

    let mut weights = weights
        .iter()
        .map(|a| calculate_pow(*a))
        .collect::<Vec<f64>>();

    let reciprocal = 1.0 / weights.iter().sum::<f64>();

    for weight in &mut weights {
        *weight *= reciprocal;
    }
    weights
}

fn normalise_benchmark(c: &mut Criterion) {
    let mut weights = Vec::new();
    for i in 0..1000000 {
        weights.push(i as f64);
    }

    c.bench_function("normalise with reciprocal", |b| {
        b.iter(|| normalise(black_box(&weights)))
    });
    //c.bench_function("normalise with reciprocal+str_w_hash", |b| b.iter(|| normalise1(black_box(&weights))));//wut ?
    c.bench_function("normalise with loop", |b| {
        b.iter(|| normalise2(black_box(&weights)))
    });
    c.bench_function("normalise with reciprocal+loop", |b| {
        b.iter(|| normalise3(black_box(&weights)))
    });
    c.bench_function("normalise with mut_iter", |b| {
        b.iter(|| normalise4(black_box(&weights)))
    });
    c.bench_function("normalise with reciprocal+mut_iter", |b| {
        b.iter(|| normalise5(black_box(&weights)))
    });
    c.bench_function("normalise with reciprocal+FW(w)_hash", |b| {
        b.iter(|| normalise6(black_box(&weights)))
    });
    c.bench_function("normalise with reciprocal+loop+FW(w)_hash", |b| {
        b.iter(|| normalise7(black_box(&weights)))
    });
}

criterion_group!(benches, normalise_benchmark);
criterion_main!(benches);
