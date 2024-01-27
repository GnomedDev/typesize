use typesize::{derive::TypeSize, TypeSize};

#[test]
fn enum_derive() {
    #[derive(TypeSize)]
    #[allow(clippy::large_enum_variant)]
    enum DerivedEnum {
        BigVariant([u8; 1024]),
        ValNamed { field: u8 },
        Vec(Vec<u8>),
        None,
    }

    let vec_variant = DerivedEnum::Vec(vec![0, 1, 2, 3]);
    let big_variant = DerivedEnum::BigVariant([0; 1024]);
    let val_variant = DerivedEnum::ValNamed { field: 0 };
    let none_variant = DerivedEnum::None;

    assert_eq!(
        vec_variant.get_size(),
        core::mem::size_of::<DerivedEnum>() + (core::mem::size_of::<u8>() * 4)
    );
    assert_eq!(val_variant.get_size(), core::mem::size_of::<DerivedEnum>());
    assert_eq!(big_variant.get_size(), core::mem::size_of::<DerivedEnum>());
    assert_eq!(none_variant.get_size(), core::mem::size_of::<DerivedEnum>());
}

#[test]
fn enum_no_data() {
    #[derive(TypeSize)]
    #[allow(dead_code)]
    enum NoData {
        JustVariants,
        OnlyStore,
        TheTag,
        Please,
    }

    assert_eq!(
        NoData::JustVariants.get_size(),
        core::mem::size_of::<NoData>()
    )
}

#[test]
fn enum_padding() {
    #[derive(TypeSize)]
    enum PaddingTest {
        Variant(u8, u64),
    }

    assert_eq!(
        PaddingTest::Variant(0, 0).get_size(),
        core::mem::size_of::<PaddingTest>()
    )
}

#[test]
fn enum_generic() {
    #[derive(TypeSize)]
    #[allow(dead_code)]
    enum Result<T: TypeSize, E: TypeSize> {
        Ok(T),
        Err(E),
    }

    assert_eq!(
        Result::<u8, u8>::Ok(0).get_size(),
        core::mem::size_of::<Result<u8, u8>>()
    )
}
