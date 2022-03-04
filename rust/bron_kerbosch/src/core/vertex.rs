use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Vertex(u32);
impl Vertex {
    pub fn new(n: usize) -> Self {
        Self(n as u32)
    }
}

impl<T> Index<Vertex> for [T] {
    type Output = T;
    fn index(&self, i: Vertex) -> &T {
        // Won't actually ever panic on 32- or 64-bit platforms
        let i: usize = i.0.try_into().unwrap();
        &self[i]
    }
}

impl<T> IndexMut<Vertex> for [T] {
    fn index_mut(&mut self, i: Vertex) -> &mut T {
        // Won't actually ever panic on 32- or 64-bit platforms
        let i: usize = i.0.try_into().unwrap();
        &mut self[i]
    }
}

#[derive(Debug)]
pub struct VertexMap<T>(Vec<T>);

impl<T> VertexMap<T> {
    pub fn new(stamp: T, order: usize) -> Self
    where
        T: Clone,
    {
        Self(vec![stamp; order])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, val: &T) -> bool
    where
        T: Eq,
    {
        self.0.contains(val)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vertex, &T)> {
        self.0.iter().enumerate().map(|(i, v)| (Vertex::new(i), v))
    }
}

impl<T> FromIterator<T> for VertexMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> VertexMap<T> {
        Self(Vec::from_iter(iter))
    }
}

impl<T> Index<Vertex> for VertexMap<T> {
    type Output = T;
    fn index(&self, i: Vertex) -> &T {
        // Won't actually ever panic on 32- or 64-bit platforms
        let i: usize = i.0.try_into().unwrap();
        &self.0[i]
    }
}

impl<T> IndexMut<Vertex> for VertexMap<T> {
    fn index_mut(&mut self, i: Vertex) -> &mut T {
        // Won't actually ever panic on 32- or 64-bit platforms
        let i: usize = i.0.try_into().unwrap();
        &mut self.0[i]
    }
}
