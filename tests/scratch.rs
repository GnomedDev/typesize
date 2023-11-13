use typesize::TypeSize;

#[test]
fn scratch() {
    let vector: Vec<u8> = vec![1, 2, 3, 4];
    assert_eq!(vector.get_size(), 24 + 4);
}
