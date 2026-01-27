use std::collections::HashMap;

use rstar::{RTree, primitives::GeomWithData};
use slotmap::{SlotMap, SparseSecondaryMap};

pub type Position = [f32; 2];

/// galaxies store clusters of systems and objects.
#[derive(Debug)]
pub struct Galaxy {
    spatial: RTree<GeomWithData<Position, ObjectHandle>>,
    objects: SlotMap<ObjectHandle, Object>,
    maneuvers: SparseSecondaryMap<ObjectHandle, Maneuver>,
}

impl Galaxy {
    /// create an empty galaxy object
    pub fn new() -> Self {
        Self {
            spatial: RTree::new(),
            objects: SlotMap::with_key(),
            maneuvers: SparseSecondaryMap::new(),
        }
    }

    /// instantiate an object at a position or as a child
    pub fn spawn_object(&mut self, object: ObjectBuilder, parent: ParentBuilder) -> ObjectHandle {
        let object_handle = self.objects.insert(Object {
            parent: match parent {
                ParentBuilder::Position(pos) => Parent::Position(pos),
                ParentBuilder::Relation(object_handle, ref _relation) => {
                    Parent::Relation(object_handle)
                }
            },
            children: None,
            mass: object.mass,
            name: object.name,
        });

        match parent {
            ParentBuilder::Position(pos) => {
                self.spatial.insert(GeomWithData::new(pos, object_handle));
            }
            ParentBuilder::Relation(parent_handle, relation) => {
                self.objects
                    .get_mut(parent_handle)
                    .unwrap()
                    .insert_child(object_handle, relation);
            }
        }

        if let Some(maneuver) = object.maneuver {
            self.maneuvers.insert(object_handle, maneuver);
        }

        if let Some(children) = object.children {
            self.spawn_children(object_handle, children);
        }

        object_handle
    }

    fn spawn_children(
        &mut self,
        object_handle: ObjectHandle,
        children: Vec<(ObjectBuilder, Relation)>,
    ) {
        for (object, relation) in children {
            self.spawn_object(object, ParentBuilder::Relation(object_handle, relation));
        }
    }

    pub fn get_object(&self, object_handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(object_handle)
    }

    pub fn get_maneuver(&self, object_handle: ObjectHandle) -> Option<&Maneuver> {
        self.maneuvers.get(object_handle)
    }
}

pub struct ObjectBuilder {
    pub children: Option<Vec<(ObjectBuilder, Relation)>>,
    pub mass: f64,
    pub name: &'static str,
    maneuver: Option<Maneuver>,
}

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self {
            children: None,
            mass: 1_000_000.,
            name: "Object",
            maneuver: None,
        }
    }
}

impl ObjectBuilder {
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }
    pub fn child(mut self, child: ObjectBuilder, relation: Relation) -> Self {
        match &mut self.children {
            Some(children) => children.push((child, relation)),
            None => self.children = Some(vec![(child, relation)]),
        }
        self
    }
    pub fn maneuver(mut self, maneuver: Maneuver) -> Self {
        self.maneuver = Some(maneuver);
        self
    }
}

slotmap::new_key_type! { pub struct ObjectHandle; }

#[derive(Debug)]
pub struct Object {
    pub parent: Parent,
    pub children: Option<HashMap<ObjectHandle, Relation>>,
    pub mass: f64,
    pub name: &'static str,
}

#[derive(Debug, Clone)]
pub enum Parent {
    Position(Position),
    Relation(ObjectHandle),
}
pub enum ParentBuilder {
    Position(Position),
    Relation(ObjectHandle, Relation),
}

#[derive(Debug, Clone)]
pub enum Relation {
    Orbit(usize),
}

impl Object {
    pub fn builder() -> ObjectBuilder {
        ObjectBuilder::default()
    }
    fn insert_child(&mut self, object_handle: ObjectHandle, relation: Relation) {
        match &mut self.children {
            Some(children) => {
                children.insert(object_handle, relation);
            }
            None => self.children = Some(HashMap::from([(object_handle, relation)])),
        };
    }
    pub fn children_count(&self) -> usize {
        match &self.children {
            Some(children) => children.len(),
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct Maneuver;
