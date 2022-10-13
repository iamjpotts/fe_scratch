use std::fs::File;
use fe_scratch::add;

#[test]
fn adds_together() {
    assert_eq!(13, add(9, 4));
}

#[test]
#[ignore]
fn fails() {
    assert_eq!(11, 13);
}

#[test]
fn sewer() {
    let pipe_name = "\\\\.\\pipe\\docker_engine";

    println!("Opening {}", pipe_name);

    let mut pipe = File::open(&pipe_name)
        .unwrap();

    drop(pipe);

    println!("Done.");
}