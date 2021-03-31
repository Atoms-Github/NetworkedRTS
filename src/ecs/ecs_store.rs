use serde::*;
pub type PogTypeId = u64;
pub type CompositionID = usize;
pub type GlobalEntityID = usize;
pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
//pub type TypeSetTypes = BTreeSet<TypeId>;
pub type TypeSet = BTreeSet<PogTypeId>;



use serde::{Deserialize, Serialize};
use crate::ecs::ecs_manager::*;
use std::collections::{HashMap, BTreeSet};
use std::any::{Any, TypeId};
use anymap::any::CloneAny;
use std::fmt::Debug;
use std::hash::Hash;










