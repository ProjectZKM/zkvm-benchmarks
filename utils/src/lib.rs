use std::{fmt::Display, fs::File, io::Write, time::Duration};

use serde::Serialize;

pub fn benchmark<T: Display + Clone>(func: fn(T) -> (Duration, usize), inputs: &[T], file: &str, input_name: &str) {
    let mut results = Vec::new();
    for input in inputs {
        let (duration, size) = func(input.clone());
        results.push((duration, size));
    }

    write_csv(file, input_name, inputs, &results);
}

pub fn write_csv<T: Display>(file: &str, input_name: &str, inputs: &[T], results: &[(Duration, usize)]) {
    let mut file = File::create(file).unwrap();
    file.write_all(format!("{},prover time (ms),proof size (bytes)\n", input_name).as_bytes()).unwrap();
    inputs.iter().zip(results).for_each(|(input, (duration, size))| {
        file.write_all(format!("{},{},{}\n", input, duration.as_millis(), size).as_bytes()).unwrap();
    });
}

pub fn benchmark_v2<T: Display + Clone>(func: fn(T) -> (Duration, usize, u64), inputs: &[T], file: &str, input_name: &str) {
    let mut results = Vec::new();
    for input in inputs {
        let (duration, size, cycles) = func(input.clone());
        results.push((duration, size, cycles));
    }

    write_csv_v2(file, input_name, inputs, &results);
}

pub fn write_csv_v2<T: Display>(file: &str, input_name: &str, inputs: &[T], results: &[(Duration, usize, u64)]) {
    let mut file = File::create(file).unwrap();
    file.write_all(format!("{},prover time (ms),proof size (bytes),cycles\n", input_name).as_bytes()).unwrap();
    inputs.iter().zip(results).for_each(|(input, (duration, size, cycles))| {
        file.write_all(format!("{},{},{},{}\n", input, duration.as_millis(), size, cycles).as_bytes()).unwrap();
    });
}

pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}
