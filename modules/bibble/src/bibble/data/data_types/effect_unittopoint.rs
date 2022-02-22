use crate::*;
use super::*;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EffectUnitToPoint{
    APPLY_FORCE,
    NOTHING,
    TO_POINT(EffectToPoint)
}

