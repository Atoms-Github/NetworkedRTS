use game::bibble::data::data_types::noneffects::StructureFlavourInfo;
use serde::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitStructureComp{
    pub structure_info: StructureFlavourInfo,
}