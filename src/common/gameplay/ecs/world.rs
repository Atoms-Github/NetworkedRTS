
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};


use crate::common::gameplay::systems::velocity::*;
use crate::common::gameplay::systems::position::*;
use crate::common::gameplay::systems::size::*;
use crate::common::gameplay::systems::render::*;
use crate::common::gameplay::systems::movershooter::*;
use crate::common::utils::unmoving_vec::*;
use anymap::AnyMap;

use serde::{Serialize, Deserialize};


pub type PogTypeId = u64;
pub type CompositionID = usize;
pub type EntityID = usize;
pub type VerticalStorage<T> = Vec<Vec<T>>;
//pub type TypeSetSerializable = BTreeSet<u64>;
//pub type TypeSetTypes = BTreeSet<TypeId>;
pub type TypeSet = BTreeSet<PogTypeId>;

use std::intrinsics;


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Request {
	pub types: TypeSet,
}


impl Request {
	fn matches(&self, composition: &Composition) -> bool {

		for type_id in &self.types {
			if !composition.types.contains(&type_id) {
				return false;
			}
		}
		true
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct Composition {
	id: CompositionID,
	types: TypeSet,
	pub global_entity_ids: Vec<EntityID>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
struct InternalEntity {
	global_id: EntityID,
	composition_id: CompositionID,
	internal_index: usize,
}

#[derive(Debug)]
pub struct PendingEntity {
	types: TypeSet,
	components_to_add: AnyMap,
}

impl Default for PendingEntity {
	fn default() -> Self {
		Self::new()
	}
}


impl PendingEntity {
	pub fn new() -> PendingEntity {
		PendingEntity {
			types: Default::default(),
			components_to_add: AnyMap::new(),
		}
	}
	
	pub fn add_component<T: 'static>(&mut self, component: T) {
//		self.types.insert(TypeId::of::<T>());
		self.types.insert(unsafe { intrinsics::type_id::<T>()});
		self.components_to_add.insert(component);
	}
	
	pub fn remove_component<T: 'static>(&mut self, component: T) {
		self.types.remove(&unsafe { intrinsics::type_id::<T>()});
		self.components_to_add.remove::<T>();
	}
}
#[derive(Debug)]
pub struct PendingEntities {
	pending_additions: Vec<PendingEntity>,
	pending_deletions: HashSet<EntityID>,
}

impl Default for PendingEntities {
	fn default() -> Self {
		Self::new()
	}
}

impl PendingEntities {
	pub fn new() -> PendingEntities {
		PendingEntities {
			pending_additions: vec![],
			pending_deletions: Default::default(),
		}
	}

//    fn add_comp_to_entity<T>(&mut self, entity_id: EntityID, component: T) { //TODO_richard 3 (Richard's :))
//
//    }
	
	pub fn merge(&mut self, pending_entities: PendingEntities) {
		for entity_id in pending_entities.pending_deletions {
			self.pending_deletions.insert(entity_id);
		}
		for addition in pending_entities.pending_additions {
			self.pending_additions.push(addition);
		}
	}
	
	pub fn create_entity(&mut self, pending_entity: PendingEntity) {
		self.pending_additions.push(pending_entity);
	}
	
	pub fn destroy_entity(&mut self, entity_id: EntityID) {
		self.pending_deletions.insert(entity_id);
	}
}

macro_rules! create_system {
	($($var_name:ident : $sty:ty),*) => {
		#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
		pub struct Storages {
			$(
                    pub $var_name: VerticalStorage<$sty>,
            )*
		}
		impl Default for Storages {
			fn default() -> Self {
				Self::new()
			}
		}
		
		impl Storages {
			pub fn new() -> Storages {
				Storages {
					$(
						$var_name: vec![],
					)*
				}
			}
			
			fn internal_add_composition(&mut self) {
				$(
					self.$var_name.push(vec![]);
				)*
			}
			
			fn internal_add_entity(&mut self, composition_id: CompositionID, pending_entity: &mut PendingEntity) {
				$(
					if let Some(temp_val) = pending_entity.components_to_add.remove::<$sty>() {
						self.$var_name.get_mut(composition_id).unwrap().push(temp_val)
					}
				)*
			}
			
			fn internal_remove_entity(&mut self, composition_id: CompositionID, entity: &InternalEntity) { // Feels quite clunky.
				$(
					{
						let composition_storage = self.$var_name.get_mut(composition_id).unwrap();
						if composition_storage.len() > 0 {
							composition_storage.swap_remove(entity.internal_index);
						}
					}
				)*
			}
		}
	}
}

create_system!(position_s: PositionComp, velocity_s: VelocityComp, render_s: RenderComp, size_s: SizeComp, mover_shooter_s: MoverShooterComp);

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
pub struct World {
	entity_storage: UnmovingVec<InternalEntity>,
	composition_types_to_id_map: BTreeMap<TypeSet, CompositionID>,
	composition_list: Vec<Composition>,

}


impl Default for World {
	fn default() -> Self {
		Self::new()
	}
}
impl World {
	pub fn get_entity_composition_id(&self, id: EntityID) -> CompositionID {
		self.entity_storage.get(id).unwrap().composition_id
	}
	
	pub fn get_entity_internal_index(&self, id: EntityID) -> usize {
		self.entity_storage.get(id).unwrap().internal_index
	}
	
	pub fn update_entities(&mut self, storages: &mut Storages, pending: PendingEntities) {
		for i in pending.pending_deletions {
			let entity = (*self.entity_storage.remove(i)).clone(); // Remove this entity from the data that sits alongside the components.
			storages.internal_remove_entity(entity.composition_id, &entity); // Remove the entity from each of the storages.
			
			let entity_internal_index = entity.internal_index;
			
			// This composition might now be empty. In which case, we're done
			if self.composition_list[entity.composition_id].global_entity_ids.is_empty() {
				continue;
			}
			
			let misplaced_entity_id = self.composition_list[entity.composition_id].global_entity_ids[entity_internal_index]; // Figure out the entity who was moved's global id.
			
			let misplaced_entity = self.entity_storage.get_mut(misplaced_entity_id).unwrap(); // Get the entity who was moved
			misplaced_entity.internal_index = entity_internal_index; // Update the entity who was moved's internal index to the new location.
		}
		
		for mut pending_entity in pending.pending_additions {
			
			let entity_id: EntityID = self.entity_storage.push(InternalEntity {
				global_id: 0,
				composition_id: 0,
				internal_index: 0,
			});
			
			if !self.composition_types_to_id_map.contains_key(&pending_entity.types) {
				self.create_new_composition(&pending_entity.types);
				storages.internal_add_composition();
			}
			
			let composition_id = self.composition_types_to_id_map.get(&pending_entity.types).unwrap();
			let composition_to_add_to = self.composition_list.get_mut(*composition_id).unwrap();
			
			let entity = self.entity_storage.get_mut(entity_id).unwrap();
			entity.global_id = entity_id;
			entity.composition_id = *composition_id;
			entity.internal_index = composition_to_add_to.global_entity_ids.len();
			
			composition_to_add_to.global_entity_ids.push(entity_id);
			
			storages.internal_add_entity(*composition_id, &mut pending_entity);
		}
	}
	
	fn create_new_composition(&mut self, type_set: &TypeSet) {
		let id = self.composition_list.len();
		
		self.composition_list.push(Composition {
			id,
			types: (*type_set).clone(),
			global_entity_ids: vec![],
		});
		
		self.composition_types_to_id_map.insert((*type_set).clone(), id);
	}
	
	pub fn new() -> World {
		World {
			entity_storage: UnmovingVec::new(),
			composition_list: vec![],
			composition_types_to_id_map: Default::default(),
		}
	}
	
	pub fn get_component<'a, T>(&self, storage: &'a mut VerticalStorage<T>, ent_id: EntityID) -> &'a mut T {
		let internal_entity = self.entity_storage.get(ent_id).unwrap();
		storage.get_mut(internal_entity.composition_id).unwrap().get_mut(internal_entity.internal_index).unwrap()
	}
	
	#[inline]
	pub fn internal_get_composition_list(&self) -> &Vec<Composition>{
		&self.composition_list
	}
	
	/// Returns every composition this request needs to touch.
	pub fn internal_make_request(composition_list: &[Composition], request: Request) -> Vec<CompositionID> {
		let mut to_return: Vec<CompositionID> = Vec::new();
		for (index, composition) in composition_list.iter().enumerate() {
			if request.matches(&composition) {
				to_return.push(index);
			}
		}
		to_return
	}
}



































