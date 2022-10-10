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