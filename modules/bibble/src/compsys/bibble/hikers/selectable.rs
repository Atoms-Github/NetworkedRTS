use crate::*;
use crate::bibble::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SelectableComp {
    pub is_selected: bool
}
