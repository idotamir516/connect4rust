use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Move {
    pub row: u8,
    pub column: u8,
}