use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SelectableComp {
    pub is_selected: bool
}