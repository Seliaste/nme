use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    AddLine,
    DeleteLine,
    DeleteChar,
    SaveFile,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Exit,
    None,
}

//TODO: too tedious, find another way (maybe with serde?)
impl From<String> for Action {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "addline" => Self::AddLine,
            "deleteline" => Self::DeleteLine,
            "deletechar" => Self::DeleteChar,
            "savefile" => Self::SaveFile,
            "moveleft" => Self::MoveLeft,
            "moveright" => Self::MoveRight,
            "moveup" => Self::MoveUp,
            "movedown" => Self::MoveDown,
            "exit" => Self::Exit,
            _ => Self::None,
        }
    }
}
