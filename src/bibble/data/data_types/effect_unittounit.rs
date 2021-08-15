use super::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EffectUnitToUnit{
    LAUNCH_PROJECTILE(ProjectileID),
    INSTA_AFFECT_TARGET(EffectToUnit)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EutuLaunchProjectile{
    pub actor_id: ActorID
}