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
    let value = Box::new(0_u8);
    assert_eq!(
        value.get_size(),
        std::mem::size_of::<usize>() + std::mem::size_of::<u8>()
    )
}
