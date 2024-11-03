#![cfg(feature = "details")]

use typesize::{Field, TypeSize};

#[test]
fn test_details() {
    #[derive(Default, TypeSize)]
    struct TestDetails {
        likes: Vec<String>,
        name: String,
        age: u8,
    }

    let test = TestDetails {
        likes: vec![String::from("Cats"), String::from("Foxes")],
        name: String::from("Example"),
        age: 18,
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
    ];

    assert_eq!(test.get_size_details(), test_fields);
}
