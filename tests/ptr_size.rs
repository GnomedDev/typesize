use typesize::{with::ref_size, TypeSize};

struct NotTypesize;

#[test]
fn ptr_size() {
    assert_eq!(
        ref_size::<[NotTypesize]>(&[NotTypesize].as_slice()),
        core::mem::size_of::<usize>() * 2
    );
    assert_eq!(
        ref_size::<NotTypesize>(&&NotTypesize),
        core::mem::size_of::<usize>()
    );
}

#[test]
fn box_size() {
    let value: Box<u8> = Box::new(0_u8);
    let fat_str: Box<str> = Box::from("data");
    let fat_arr: Box<[u8]> = Box::from(*b"data");

    assert_eq!(
        value.get_size(),
        core::mem::size_of::<usize>() + core::mem::size_of::<u8>()
    );

    let expected_fat = core::mem::size_of::<usize>() // thin ptr
        + core::mem::size_of::<usize>() // len
        + (core::mem::size_of::<u8>() * fat_str.len()); // chars

    assert_eq!(fat_arr.get_size(), expected_fat);
    assert_eq!(fat_str.get_size(), expected_fat);
}
