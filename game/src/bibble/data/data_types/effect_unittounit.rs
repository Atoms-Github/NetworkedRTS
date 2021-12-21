use super::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EffectUnitToUnit{
    LAUNCH_SEEKING_PROJECTILE(SeekingProjectileMould),
    INSTA_AFFECT_TARGET(EffectToUnit)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SeekingProjectileMould {
    pub actor: ActorMould,
    pub speed: f32,
    pub hit_effect: EffectToUnit,
    pub size: f32,
}