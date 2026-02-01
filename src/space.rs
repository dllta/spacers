use std::fmt::Display;

use rstar::{RTree, primitives::GeomWithData};
use slotmap::SlotMap;

use crate::object::{Body, Composition, Object, ObjectBuilder, ObjectHandle, ObjectKind, Parent, ParentBuilder, Relation};


pub type Position = [f32; 2];

#[derive(Clone, Copy, Debug)]
pub struct Mass(pub f64);
#[derive(Clone, Copy, Debug)]
pub struct Distance(pub f64);


// interface from ratatui app to a game world
pub trait World {
    fn spawn_object(&mut self, object: ObjectBuilder, parent: ParentBuilder) -> ObjectHandle;
    fn get_object(&self, object_handle: ObjectHandle) -> Option<&Object>;
    fn get_handle(&self) -> Option<ObjectHandle>;
}

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kg", self.0)
    }
}
impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}km", self.0)
    }
}

/// galaxies store clusters of systems and objects.
#[derive(Debug)]
pub struct Galaxy {
    spatial: RTree<GeomWithData<Position, ObjectHandle>>,
    objects: SlotMap<ObjectHandle, Object>,
    handle: Option<ObjectHandle>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self { spatial: RTree::new(), objects: SlotMap::with_key(), handle: None }
    }
}

impl Galaxy {
    pub fn new() -> Self {
        let mut galaxy = Galaxy::default();

        let sun = galaxy.spawn_object(
            ObjectBuilder::default().name("Sun").mass(Mass(1_000_000.)).kind(ObjectKind::Body(Body { composition: Composition::default(), radius: Distance(100.) })),
            ParentBuilder::Position([0.3, 0.2]),
        );
        let ship = galaxy.spawn_object(
            ObjectBuilder::default().name("Ship").mass(Mass(2_000.)),
            ParentBuilder::Relation(sun, Relation::Orbit(300)),
        );
        galaxy.handle = Some(ship);
        
        galaxy
    }

    pub fn new_cluster(stars: u32) -> Self {
        todo!()
    }

    // recursively spawns children from spawn_object
    fn spawn_children(
        &mut self,
        object_handle: ObjectHandle,
        children: Vec<(ObjectBuilder, Relation)>,
    ) {
        for (object, relation) in children {
            self.spawn_object(object, ParentBuilder::Relation(object_handle, relation));
        }
    }
}

impl World for Galaxy {
    fn spawn_object(&mut self, object: ObjectBuilder, parent: ParentBuilder) -> ObjectHandle {
        let object_handle = self.objects.insert(Object {
            parent: match parent {
                ParentBuilder::Position(pos) => Parent::Position(pos),
                ParentBuilder::Relation(object_handle, ref _relation) => {
                    Parent::Relation(object_handle)
                }
            },
            children: None,
            mass: object.mass.unwrap_or(Mass(1_000.)),
            name: object.name.unwrap_or("Body"),
            kind: object.kind.unwrap_or(ObjectKind::Body(Body { composition: Composition::default(), radius: Distance(300.) })),
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

        if let Some(children) = object.children {
            self.spawn_children(object_handle, children);
        }

        object_handle
    }

    fn get_object(&self, object_handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(object_handle)
    }

    fn get_handle(&self) -> Option<ObjectHandle> {
        self.handle
    }
}