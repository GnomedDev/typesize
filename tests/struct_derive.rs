use typesize::TypeSize;

#[derive(Default)]
#[allow(dead_code)]
struct NotTypeSize(Box<u64>);

#[test]
fn struct_named_fields() {
    #[derive(Default, TypeSize)]
    struct NamedFields {
        a: u8,
        b: u8,
    }

    assert_eq!(NamedFields::default().get_size(), 0_u8.get_size() * 2);
}

#[test]
fn struct_unnamed_field() {
    #[derive(Default, TypeSize)]
    struct UnnamedFields(u8, u8);

    assert_eq!(UnnamedFields::default().get_size(), 0_u8.get_size() * 2);
}

#[test]
#[allow(dead_code)]
fn struct_skip_attr() {
    #[derive(Default, TypeSize)]
    struct NamedSkip {
        #[typesize(skip)]
        field: NotTypeSize,
    }

    #[derive(Default, TypeSize)]
    struct UnnamedSkip(#[typesize(skip)] NotTypeSize);

    // Just the `usize` for the pointer, not the u64 behind the pointer.
    assert_eq!(NamedSkip::default().get_size(), 0_usize.get_size());
    assert_eq!(UnnamedSkip::default().get_size(), 0_usize.get_size());
}

#[test]
fn struct_with_attr() {
    const EXTRA_SIZE: usize = 42;

    fn my_extra_size(_field: &usize) -> usize {
        EXTRA_SIZE
    }

    #[derive(Default, TypeSize)]
    struct NamedWith {
        #[typesize(with = my_extra_size)]
        field: usize,
    }

    #[derive(Default, TypeSize)]
    struct UnnamedWith(#[typesize(with = my_extra_size)] usize);

    assert_eq!(NamedWith::default().extra_size(), EXTRA_SIZE);
    assert_eq!(UnnamedWith::default().extra_size(), EXTRA_SIZE);
}

#[test]
fn struct_unit() {
    #[derive(TypeSize)]
    struct Unit;

    #[derive(TypeSize)]
    struct NamedUnit {}

    #[derive(TypeSize)]
    struct UnnamedUnit();

    assert_eq!(Unit.get_size(), 0);
    assert_eq!(NamedUnit {}.get_size(), 0);
    assert_eq!(UnnamedUnit().get_size(), 0);
}

#[test]
fn struct_padding() {
    #[derive(Default, TypeSize)]
    struct PaddingTest(u8, u64);

    #[repr(packed)]
    #[derive(Default, TypeSize)]
    struct PackedTest(u8, u64);

    assert_eq!(
        PaddingTest::default().get_size(),
        core::mem::size_of::<PaddingTest>()
    );

    assert_eq!(
        PackedTest::default().get_size(),
        0_u64.get_size() + 0_u8.get_size()
    );
}

#[test]
fn struct_generic() {
    #[derive(Default, TypeSize)]
    struct GenericTest<T: TypeSize>(T);

    assert_eq!(GenericTest::<u8>::default().get_size(), 0_u8.get_size());
}
