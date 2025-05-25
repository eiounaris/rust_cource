use prost::Message as PBMessage;

// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    }
}

use snazzy::items;

/// Returns a large shirt of the specified color
pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

fn main() {
    let shirt = create_large_shirt("white".to_string());
    println!("{shirt:?}");
    let mut buf = Vec::with_capacity(shirt.encoded_len());
    shirt.encode(&mut buf).unwrap();
    println!("{buf:?}");
    let decoded_shirt = items::Shirt::decode(buf.as_ref()).unwrap();
    println!("{decoded_shirt:?}");
}