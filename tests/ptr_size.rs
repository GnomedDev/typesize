use typesize::{Ref, TypeSize};

#[test]
fn ptr_size() {
    let fat_ptr: Ref<[u8]> = Ref(&[]);
    let thin_ptr: Ref<u8> = Ref(&0);

    assert_eq!(fat_ptr.get_size(), std::mem::size_of::<usize>() * 2);
    assert_eq!(thin_ptr.get_size(), std::mem::size_of::<usize>());
}

#[test]
fn box_size() {
    let value: Box<u8> = Box::new(0_u8);
    let fat_str: Box<str> = Box::from("data");
    let fat_arr: Box<[u8]> = Box::from(*b"data");

    assert_eq!(
        value.get_size(),
        std::mem::size_of::<usize>() + std::mem::size_of::<u8>()
    );

    let expected_fat = std::mem::size_of::<usize>() // thin ptr
        + std::mem::size_of::<usize>() // len
        + (std::mem::size_of::<u8>() * fat_str.len()); // chars

    assert_eq!(fat_arr.get_size(), expected_fat);
    assert_eq!(fat_str.get_size(), expected_fat);
}
