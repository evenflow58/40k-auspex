use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectWithId<T> {
    pub id: String,
    pub object: T
}

impl<T> ObjectWithId<T> {
    pub fn new(id: String, object: T) -> Self {
        ObjectWithId {
            id,
            object,
        }
    }

}