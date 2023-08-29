use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Ident {
    name: String,
    discriminant: u32,
}

pub fn serialize_ident(ident: Ident) -> String {
    bob::other_function();
    serde_json::to_string(&ident).unwrap()
}
