use std::time::{Duration, Instant};

use crate::sorts::SortingMethod;

use super::distributions::*;
use fxhash::FxHashMap;
use rand::Rng;

pub trait HarnessInputProvider<T> {
    fn generate_input(rng: &mut impl Rng, lengths: &[usize]) -> Vec<DistributionResult<T>>;
}

macro_rules! harness_input_provider {
    ($($t:ident),*) => {
        impl<T,  $($t: InputDistribution<T>),*> HarnessInputProvider<T>
            for ($($t),*,)
        {
            fn generate_input(
                rng: &mut impl Rng,
                lengths: &[usize],
            ) -> Vec<DistributionResult<T>> {
                let mut v = Vec::new();
                $(
                v.extend(
                    lengths
                        .into_iter()
                        .map(|&len| <$t>::generate(rng, len))
                        .map(|data| DistributionResult {
                            name: <$t>::name(),
                            data,
                        }),
                );
                )*
                v
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct DistributionResult<T> {
    name: String,
    data: Vec<T>,
}

harness_input_provider!(D1);
harness_input_provider!(D1, D2);
harness_input_provider!(D1, D2, D3);
harness_input_provider!(D1, D2, D3, D4);
harness_input_provider!(D1, D2, D3, D4, D5);
harness_input_provider!(D1, D2, D3, D4, D5, D6);
harness_input_provider!(D1, D2, D3, D4, D5, D6, D7);
harness_input_provider!(D1, D2, D3, D4, D5, D6, D7, D8);

#[derive(Debug, Clone, Copy)]
pub struct BenchmarkResult {
    pub time: Duration,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct BenchmarkData {
    pub result: BenchmarkResult,
    pub name: String,
    pub stable: bool,
}

pub trait SortingMethodProvider<T> {
    fn run_all(data: &[DistributionResult<T>], runs: usize) -> Vec<BenchmarkData>;
}

macro_rules! sorting_provider {
    ($($t:ident),*) => {
        impl<T: PartialOrd + Clone, $($t: SortingMethod<T>),*> SortingMethodProvider<T> for ($($t),* ,) {
            fn run_all(data: &[DistributionResult<T>], runs: usize) -> Vec<BenchmarkData> {
                let mut v = Vec::with_capacity(data.len());
                $(
                v.extend(data.iter().map(|DistributionResult { name: _, data }| {
                    let mut inputs: Vec<Vec<T>> = (0..runs).map(|_| data.to_vec()).collect();
                    let start = Instant::now();
                    for input in &mut inputs {
                        <$t>::sort(input)
                    }
                    let time = start.elapsed().div_f64(runs as f64);
                    let success = inputs
                        .into_iter()
                        .all(|v| v.windows(2).all(|s| s[0] <= s[1]));
                    let result = BenchmarkResult { time, success };
                    BenchmarkData {
                        result,
                        name: <$t>::name(),
                        stable: <$t>::stable(),
                    }
                }));)*
                v
            }
        }
    };
}

sorting_provider!(S1);
sorting_provider!(S1, S2);
sorting_provider!(S1, S2, S3);
sorting_provider!(S1, S2, S3, S4);
sorting_provider!(S1, S2, S3, S4, S5);
sorting_provider!(S1, S2, S3, S4, S5, S6);
sorting_provider!(S1, S2, S3, S4, S5, S6, S7);

pub type HashMap<K, V> = FxHashMap<K, V>;

/// The results are presented in this format:
/// Two hashmaps, the first one containing unstable sorting algorithms, the second - stable
///
/// The first key of each hashmap is the number of elements
/// The second key is the name of the sorting algorithm
/// The values are the name of the input distribution and its runtime
pub fn bench<T: std::fmt::Debug, D: HarnessInputProvider<T>, S: SortingMethodProvider<T>>(
    rng: &mut impl Rng,
    lengths: &[usize],
    runs: usize,
) -> [HashMap<usize, HashMap<String, Vec<(String, BenchmarkResult)>>>; 2] {
    let inputs = D::generate_input(rng, lengths);
    let results = S::run_all(&inputs, runs);
    let mut maps: [HashMap<usize, HashMap<String, Vec<(String, BenchmarkResult)>>>; 2] =
        std::array::from_fn(|_| HashMap::default());
    inputs.iter().cycle().zip(results.into_iter()).for_each(
        |(
            DistributionResult {
                name: distribution_name,
                data,
            },
            BenchmarkData {
                result,
                name: sort_name,
                stable,
            },
        )| {
            let map = if stable { &mut maps[1] } else { &mut maps[0] };
            let size = data.len();
            let size_map: &mut HashMap<String, _> = map.entry(size).or_default();
            size_map
                .entry(sort_name)
                .or_default()
                .push((distribution_name.to_string(), result));
        },
    );
    maps
}
