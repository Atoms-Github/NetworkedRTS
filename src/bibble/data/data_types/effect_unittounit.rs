use super::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EffectUnitToUnit{
    LAUNCH_PROJECTILE(ProjectileID),
    INSTA_AFFECT_TARGET(EffectToUnit)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectileMould {
    pub actor_id: ActorID,
    pub speed: f32,
    pub hit_effect: EffectUnitToUnit,
}