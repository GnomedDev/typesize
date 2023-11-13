use typesize::{derive::TypeSize, TypeSize};

#[test]
fn struct_named_fields() {
    #[derive(Default, TypeSize)]
    struct NamedFields {
        a: u8,
        b: u8,
    }

    assert_eq!(NamedFields::default().get_size(), 0_u8.get_size() * 2)
}

#[test]
fn struct_unnamed_field() {
    #[derive(Default, TypeSize)]
    struct UnnamedFields(u8, u8);

    assert_eq!(UnnamedFields::default().get_size(), 0_u8.get_size() * 2)
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

    assert_eq!(
        PaddingTest::default().get_size(),
        std::mem::size_of::<PaddingTest>()
    );
}

#[test]
fn struct_generic() {
    #[derive(Default, TypeSize)]
    struct GenericTest<T: TypeSize>(T);

    assert_eq!(GenericTest::<u8>::default().get_size(), 0_u8.get_size())
}
