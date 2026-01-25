use rstar::{RTree, primitives::GeomWithData};
use slotmap::SlotMap;

type Position = [f32; 2];

/// galaxies store clusters of systems and objects.
struct Galaxy {
    spatial: RTree<GeomWithData<Position, OrbiterHandle>>,
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

    pub fn instantiate_object(&mut self, parent: Parent) -> ObjectHandle {
        let object_handle = self.objects.insert(Object {
            parent: parent.clone(),
        });
        match parent {
            Parent::Root(pos) => {
                // insert object at global position
                self.spatial
                    .insert(GeomWithData::new(pos, OrbiterHandle::Object(object_handle)));
            }
            Parent::System(system_handle, orbit) => {
                // insert object on system
                self.systems
                    .get_mut(system_handle)
                    .unwrap()
                    .orbits
                    .push(Orbiter {
                        orbit,
                        handle: OrbiterHandle::Object(object_handle),
                    });
            }
        }
        object_handle
    }

    /// send an object instructions. used primarily for player controls.
    pub fn orchestrate_object(&mut self, key: ObjectHandle) {}
}

struct Orbiter {
    orbit: Orbit,
    handle: OrbiterHandle,
}
enum OrbiterHandle {
    Object(ObjectHandle),
    System(SystemHandle),
}

#[derive(Debug, Clone)]
struct Orbit {
    // prone to change: will be periapsis and apoapsis in future
    altitude: usize,
}

slotmap::new_key_type! { struct SystemHandle; }
struct System {
    parent: Parent,
    orbits: Vec<Orbiter>,
}

#[derive(Debug, Clone)]
enum Parent {
    Root(Position),
    System(SystemHandle, Orbit),
}

slotmap::new_key_type! { struct ObjectHandle; }
struct Object {
    parent: Parent,
}

#[test]
fn test() {
    let mut galaxy = Galaxy::new();
    galaxy.instantiate_object(Parent::Root([0., 0.]));
}
