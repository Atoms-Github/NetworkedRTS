use crate::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UnitStructureComp{
    pub structure_info: StructureFlavourInfo,
}
