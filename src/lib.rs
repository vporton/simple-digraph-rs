use std::rc::Rc;

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
    rc: Option<Rc<DigraphNode<T>>>,
}

impl<T> DigraphNodeRef<T> {
    pub fn new(value: DigraphNode<T>) -> Self {
        Self::from(Some(Rc::new(value)))
    }
    pub fn from(rc: Option<Rc<DigraphNode<T>>>) -> Self {
        Self {
            rc
        }
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
        let new_node_ref = DigraphNodeRef::new(new_node);
        *self = new_node_ref.clone();
        new_node_ref
    }
    #[allow(dead_code)]
    fn node(&self) -> Option<DigraphNode<T>>
        where T: Clone
    {
        self.rc.clone().map(|node| (*node).clone())
    }
}

impl<T> Clone for DigraphNodeRef<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}

impl<T> Iterator for DigraphNodeRef<T> {
    type Item = Rc<DigraphNode<T>>;

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

// TODO: Test.