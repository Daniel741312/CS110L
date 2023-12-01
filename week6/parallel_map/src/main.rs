use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let input_len = input_vec.len();
    let mut output_vec: Vec<U> = Vec::with_capacity(input_len);
    output_vec.resize_with(input_len, Default::default);
    // TODO: implement parallel map!
/*
                    main
   (workers2main_rx)↑   |(main2workers_tx)
                    |   |
                    |   |
   (workers2main_tx)|   ↓(main2workers_rx)
      worker1 worker2 ... workern
*/

    let (main2workers_tx, main2workers_rx) = crossbeam_channel::unbounded();
    let (workers2main_tx, workers2main_rx) = crossbeam_channel::unbounded();

    let mut handles = vec![];
    for _ in 0..num_threads {
        let main2workers_rx = main2workers_rx.clone();
        let workers2main_tx = workers2main_tx.clone();
        let handle = thread::spawn(move || {
            while let Ok((idx, value)) = main2workers_rx.recv() {
                workers2main_tx.send((idx, f(value))).expect("return result error in worker");
            }
        });
        handles.push(handle);
    }

    for i in 0..input_len {
        main2workers_tx.send((input_len - 1 - i, input_vec.pop().unwrap())).expect("dispatch job error in main");
    }

    drop(main2workers_tx);
    drop(workers2main_tx);

    while let Ok((idx, value)) = workers2main_rx.recv() {
        output_vec[idx] = value;
    }

    for handle in handles {
        handle.join().expect("join handle error");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
