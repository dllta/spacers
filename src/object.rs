use std::{collections::HashMap};

use crate::space::{Distance, Mass, Position};

slotmap::new_key_type! { pub struct ObjectHandle; }

#[derive(Debug)]
pub struct Object {
    pub parent: Parent,
    pub children: Option<HashMap<ObjectHandle, Relation>>,
    pub mass: Mass,
    pub name: &'static str,
    pub kind: ObjectKind,
}

#[derive(Debug, Clone)]
pub enum Parent {
    Position(Position),
    Relation(ObjectHandle),
}

#[derive(Debug, Clone)]
pub enum Relation {
    Orbit(usize),
}

#[derive(Debug)]
pub enum ObjectKind {
    Body(Body),
    Field(Field),
    Structure(Structure),
}


impl Object {
    pub fn get_child(&self, object_handle: ObjectHandle) -> Option<&Relation> {
        match &self.children {
            Some(children) => children.get(&object_handle),
            None => None,
        }
    }
    pub fn insert_child(&mut self, object_handle: ObjectHandle, relation: Relation) {
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


#[derive(Clone, Debug)]
pub struct Body {
    pub composition: Composition,
    pub radius: Distance,
    // temperature
}

#[derive(Clone, Debug)]
pub struct Field {
    pub composition: Composition,
    pub morphology: FieldMorphology,
}
#[derive(Clone, Debug)]
pub enum FieldMorphology {
    Cloud {
        radius: Distance, // from own center of mass
    },
    Disk {
        radius: Distance // from parent center of mass
    },
    Belt {
        inner: Distance, // from parent center of mass
        outer: Distance, // from parent center of mass
    },
}

#[derive(Clone, Debug)]
pub struct Composition {
    pub hydrogen: f32,
    pub helium: f32,
    pub rock: f32,
    pub ice: f32,
    pub metals: f32,
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            hydrogen: 0.,
            helium: 0.,
            rock: 1.,
            ice: 0.,
            metals: 0.,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    components: Vec<Component>,
    index: ComponentIndex,
}
#[derive(Clone, Debug)]
pub enum Component {
    Reactor(()),
    Cargo,
    Thruster,
    Drill,
}
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum ComponentKind {
    Reactor,
    Cargo,
    Thruster,
    Drill,
}
#[derive(Clone, Debug)]
pub struct ComponentIndex {
    by_kind: Vec<Vec<usize>>,
}

impl Structure {
    // example
    fn reactors(&self) -> impl Iterator<Item = &()> {
        self.index.by_kind[ComponentKind::Reactor as usize]
            .iter()
            .map(|&i| match &self.components[i] {
                Component::Reactor(a) => a,
                _ => unreachable!(),
            })
    }
}


pub struct ObjectBuilder {
    pub children: Option<Vec<(ObjectBuilder, Relation)>>,
    pub mass: Option<Mass>,
    pub name: Option<&'static str>,
    pub kind: Option<ObjectKind>,
}

pub enum ParentBuilder {
    Position(Position),
    Relation(ObjectHandle, Relation),
}

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self {
            children: None,
            mass: None,
            name: None,
            kind: None
        }
    }
}

impl ObjectBuilder {
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn mass(mut self, mass: Mass) -> Self {
        self.mass = Some(mass);
        self
    }
    pub fn child(mut self, child: ObjectBuilder, relation: Relation) -> Self {
        match &mut self.children {
            Some(children) => children.push((child, relation)),
            None => self.children = Some(vec![(child, relation)]),
        }
        self
    }
    pub fn kind(mut self, kind: ObjectKind) -> Self {
        self.kind = Some(kind);
        self
    }
}

