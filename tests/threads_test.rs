use rayon::{current_num_threads, current_thread_index, max_num_threads, prelude::*};

#[test]
fn test_thread_indexing() {
    println!("Max Num threads: {}", max_num_threads());

    let cur_num_threads = current_num_threads();
    println!("Cur Num threads: {}", cur_num_threads);

    let result = (0..cur_num_threads * 3)
        .into_par_iter()
        .map(|i| 100 * current_thread_index().unwrap() + i)
        .collect::<Vec<_>>();

    println!("Collection: {:?}", result);
}
