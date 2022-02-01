use std::sync::Arc;

#[derive(Clone)]
pub struct DigraphNode<T> {
    pub next: DigraphNodeRef<T>, // I made it `pub` to be able `item.next.next()` to remove an item from the middle.
    data: T,
}

impl<T> DigraphNode<T> {
    fn new(next: DigraphNodeRef<T>, data: T) -> Self {
        Self { next, data }
    }
}

pub struct DigraphNodeRef<T> {
    rc: Option<Arc<DigraphNode<T>>>,
}

impl<T> DigraphNodeRef<T> {
    pub fn new() -> Self {
        Self {
            rc: None
        }
    }
    pub fn from_node(value: DigraphNode<T>) -> Self {
        Self::from(Some(Arc::new(value)))
    }
    pub fn from(rc: Option<Arc<DigraphNode<T>>>) -> Self {
        Self {
            rc
        }
    }
    pub fn as_rc(self) -> Option<Arc<DigraphNode<T>>> {
        self.rc
    }
    pub fn is_none(&self) -> bool {
        self.rc.is_none()
    }
    pub fn remove(&mut self) -> bool {
        if let Some(rc) = self.rc.clone() {
            self.rc = rc.next.rc.clone();
            true
        } else {
            false
        }
    }
    pub fn prepend(&mut self, value: T) -> Self {
        let new_node = DigraphNode::new(self.clone(), value);
        let new_node_ref = DigraphNodeRef::from_node(new_node);
        *self = new_node_ref.clone();
        new_node_ref
    }
    pub fn node(&self) -> Option<DigraphNode<T>>
        where T: Clone
    {
        self.rc.clone().map(|node| (*node).clone())
    }
    /// TODO: Should return a reference.
    pub fn data(&self) -> Option<T>
        where T: Clone
    {
        self.rc.clone().map(|node| (*node).data.clone())
    }
    pub fn values(self) -> DigraphNodeValuesIterator<T> {
        DigraphNodeValuesIterator {
            underlying: self.clone()
        }
    }
}

impl<T> Clone for DigraphNodeRef<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}

impl<T> Iterator for DigraphNodeRef<T> {
    type Item = Arc<DigraphNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc) = self.rc.clone() {
            self.rc = rc.next.rc.clone();
            Some(rc.clone())
        } else {
            None
        }
    }
}

pub struct DigraphNodeValuesIterator<T> {
    underlying: DigraphNodeRef<T>,
}

impl<T: Clone> Iterator for DigraphNodeValuesIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|node| node.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::DigraphNodeRef;

    #[test]
    fn insert() {
        let mut list = DigraphNodeRef::new();
        for i in 0..10 {
            list.prepend(i);
        }
        assert_eq!(list.values().collect::<Vec<i32>>(), (0..10).rev().collect::<Vec<i32>>());
    }

    #[test]
    fn pass_two_times() {
        let mut list = DigraphNodeRef::new();
        for i in 0..10 {
            list.prepend(i);
        }
        let iter = list.clone();
        assert_eq!(iter.values().collect::<Vec<i32>>(), (0..10).rev().collect::<Vec<i32>>());
        let iter = list.clone();
        assert_eq!(iter.values().collect::<Vec<i32>>(), (0..10).rev().collect::<Vec<i32>>());
    }

    #[test]
    fn remove() {
        let mut list = DigraphNodeRef::new();
        for i in 0..10 {
            list.prepend(i);
        }
        for _ in 0..5 {
            list.remove();
        }
        assert_eq!(list.values().collect::<Vec<i32>>(), (0..5).rev().collect::<Vec<i32>>());
    }
}