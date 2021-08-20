use super::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EffectUnitToUnit{
    LAUNCH_PROJECTILE(ProjectileMould),
    INSTA_AFFECT_TARGET(EffectToUnit)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectileMould {
    pub actor: ActorMould,
    pub speed: f32,
    pub hit_effect: EffectToUnit,
}