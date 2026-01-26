use rstar::{RTree, primitives::GeomWithData};
use slotmap::SlotMap;

type Position = [f32; 2];

/// galaxies store clusters of systems and objects.
#[derive(Debug)]
pub struct Galaxy {
    spatial: RTree<GeomWithData<Position, Orbiter>>,
    systems: SlotMap<SystemHandle, System>,
    objects: SlotMap<ObjectHandle, Object>,
}

impl Galaxy {
    /// create an empty galaxy object
    pub fn new() -> Self {
        Self {
            spatial: RTree::new(),
            systems: SlotMap::with_key(),
            objects: SlotMap::with_key(),
        }
    }

    // inserts an orbiter into spatial or a system
    fn insert_orbiter(&mut self, parent: &Parent, orbiter: Orbiter) {
        match parent {
            // insert orbiter at global position
            Parent::Root(pos) => self.spatial.insert(GeomWithData::new(*pos, orbiter)),
            // insert orbiter within a system
            Parent::System(system_handle, _orbit) => self
                .systems
                .get_mut(*system_handle)
                .unwrap()
                .orbits
                .push(orbiter),
        }
    }

    /// instantiate a system with a set of objects
    pub fn instantiate_system(&mut self, system: System) -> SystemHandle {
        let parent = system.parent.clone();
        let system_handle = self.systems.insert(system);

        self.insert_orbiter(&parent, Orbiter::System(system_handle));

        system_handle
    }

    /// instantiate an object at a global position or inside of a system at a specific orbit
    /// TODO collapse to systems if in proximity
    pub fn instantiate_object(&mut self, object: Object) -> ObjectHandle {
        let parent = object.parent.clone();
        let object_handle = self.objects.insert(object);

        self.insert_orbiter(&parent, Orbiter::Object(object_handle));

        object_handle
    }

    pub fn get_object(&self, object_handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(object_handle)
    }

    pub fn get_system(&self, system_handle: SystemHandle) -> Option<&System> {
        self.systems.get(system_handle)
    }

    /// send an object instructions. used primarily for player controls.
    pub fn orchestrate_object(&mut self, key: ObjectHandle) {}
}

#[derive(Debug)]
enum Orbiter {
    Object(ObjectHandle),
    System(SystemHandle),
}

#[derive(Debug, Clone)]
pub struct Orbit {
    // prone to change: will be periapsis and apoapsis in future
    pub altitude: usize,
}

impl Orbit {
    pub fn new(altitude: usize) -> Self {
        Self { altitude }
    }
}

slotmap::new_key_type! { pub struct SystemHandle; }
#[derive(Debug)]
pub struct System {
    pub parent: Parent,
    pub orbits: Vec<Orbiter>,
}

impl System {
    pub fn new(parent: Parent) -> Self {
        Self {
            parent,
            orbits: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Parent {
    Root(Position),
    System(SystemHandle, Orbit),
}

slotmap::new_key_type! { pub struct ObjectHandle; }
#[derive(Debug)]
pub struct Object {
    pub parent: Parent,
    pub mass: isize,
}
