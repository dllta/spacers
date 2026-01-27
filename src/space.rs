use std::collections::HashMap;

use rstar::{RTree, primitives::GeomWithData};
use slotmap::SlotMap;

pub type Position = [f32; 2];

/// galaxies store clusters of systems and objects.
#[derive(Debug)]
pub struct Galaxy {
    spatial: RTree<GeomWithData<Position, ObjectHandle>>,
    objects: SlotMap<ObjectHandle, Object>,
}

impl Galaxy {
    /// create an empty galaxy object
    pub fn new() -> Self {
        Self {
            spatial: RTree::new(),
            objects: SlotMap::with_key(),
        }
    }

    /// instantiate an object at a position or as a child
    pub fn spawn_object(&mut self, mass: isize, parent: ParentBuilder) -> ObjectHandle {
        let object_handle = self.objects.insert(Object {
            parent: match parent {
                ParentBuilder::Position(pos) => Parent::Position(pos),
                ParentBuilder::Relation(object_handle, ref _relation) => {
                    Parent::Relation(object_handle)
                }
            },
            children: None,
            mass,
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

        object_handle
    }

    pub fn get_object(&self, object_handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(object_handle)
    }
}

slotmap::new_key_type! { pub struct ObjectHandle; }

#[derive(Debug)]
pub struct Object {
    pub parent: Parent,
    pub children: Option<HashMap<ObjectHandle, Relation>>,
    pub mass: isize,
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
    fn insert_child(&mut self, object_handle: ObjectHandle, relation: Relation) {
        match &mut self.children {
            Some(children) => {
                children.insert(object_handle, relation);
            }
            None => self.children = Some(HashMap::from([(object_handle, relation)])),
        };
    }
}
