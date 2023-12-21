#[derive(Debug, Hash, PartialEq, Eq)]
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
}
