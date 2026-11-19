#![cfg(feature = "details")]

use typesize::{derive::TypeSize, Field, TypeSize};

#[test]
fn test_details() {
    const UNKNOWN_SIZE: usize = 67;

    #[derive(Default)]
    struct UnknownTypeSize { }

    fn size_of_unknown(value: &UnknownTypeSize) -> usize {
        UNKNOWN_SIZE
    }

    #[derive(Default, TypeSize)]
    struct TestDetails {
        likes: Vec<String>,
        name: String,
        age: u8,
        #[typesize(with = size_of_unknown)]
        unknown: UnknownTypeSize,
    }

    let test = TestDetails {
        likes: vec![String::from("Cats"), String::from("Foxes")],
        name: String::from("Example"),
        age: 18,
        unknown: UnknownTypeSize::default(),
    };

    let test_fields = [
        Field {
            name: "likes",
            collection_items: Some(2),
            size: test.likes.get_size(),
        },
        Field {
            name: "name",
            size: test.name.get_size(),
            collection_items: Some(test.name.len()),
        },
        Field {
            name: "age",
            collection_items: None,
            size: test.age.get_size(),
        },
        Field {
            name: "unknown",
            collection_items: None,
            size: UNKNOWN_SIZE,
        },
    ];

    assert_eq!(test.get_size_details(), test_fields);
}
