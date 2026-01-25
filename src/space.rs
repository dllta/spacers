use rstar::{RTree, primitives::GeomWithData};
use slotmap::SlotMap;

type Position = [f32; 2];

/// galaxies store clusters of systems and objects.
struct Galaxy {
    spatial: RTree<GeomWithData<Position, SystemHandle>>,
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

    /// insert an object at a position
    pub fn instantiate_object_at_pos(&mut self, pos: Position) -> ObjectHandle {
        let object_handle = self.objects.insert_with_key(|object_handle| {
            let system_handle = self.systems.insert(System {
                parent: SystemParent::Root(pos),
                orbits: vec![Orbiter {
                    altitude: 0,
                    handle: OrbiterHandle::Object(object_handle),
                }],
            });

            self.spatial.insert(GeomWithData::new(pos, system_handle));

            Object {
                parent: system_handle,
            }
        });

        object_handle
    }

    // insert an object into a system
    pub fn instantiate_object_in_system(
        &mut self,
        system_handle: SystemHandle,
        altitude: usize,
    ) -> ObjectHandle {
        let object_handle = self.objects.insert(Object {
            parent: system_handle,
        });

        self.systems
            .get_mut(system_handle)
            .unwrap()
            .orbits
            .push(Orbiter {
                altitude,
                handle: OrbiterHandle::Object(object_handle),
            });

        object_handle
    }

    /// send an object instructions. used primarily for player controls.
    pub fn orchestrate_object(&mut self, key: ObjectHandle) {}
}

struct Orbiter {
    altitude: usize,
    handle: OrbiterHandle,
}
enum OrbiterHandle {
    Object(ObjectHandle),
    System(SystemHandle),
}

slotmap::new_key_type! { struct SystemHandle; }
struct System {
    parent: SystemParent,
    orbits: Vec<Orbiter>,
}

enum SystemParent {
    Root(Position),
    System(SystemHandle),
}

slotmap::new_key_type! { struct ObjectHandle; }
struct Object {
    parent: SystemHandle,
}

#[test]
fn test() {
    let mut galaxy = Galaxy::new();
    galaxy.instantiate_object_at_pos([0., 0.]);
}
