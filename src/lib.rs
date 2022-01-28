use std::{rc::Rc, ops::Deref};

pub struct DigraphNode<T> {
    pub next: DigraphNodeRef<T>, // I made it `pub` to be able `item.next.next()` to remove an item from the middle.
    data: T,
}

pub struct DigraphNodeRef<T> {
    rc: Rc<DigraphNode<T>>,
}

impl<T> DigraphNodeRef<T> {
    pub fn from(rc: Rc<DigraphNode<T>>) -> Self {
        Self {
            rc
        }
    }
    pub fn remove(&mut self) {
        self.rc = self.rc.next.rc.clone()
    }
}

impl<T> Clone for DigraphNodeRef<T> {
    fn clone(&self) -> Self {
        Self { rc: self.rc.clone() }
    }
}

impl<T> Deref for DigraphNodeRef<T> {
    type Target = DigraphNode<T>;

    fn deref(&self) -> &Self::Target {
        self.rc.deref()
    }
}

impl<T> Iterator for DigraphNodeRef<T> {
    type Item = DigraphNodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if Rc::strong_count(&self.rc) != 0 { // FIXME
            self.rc = (*self.rc).next.rc.clone();
            Some(self.clone())
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
        if let Some(next) = self.underlying.next() {
            Some(next.rc.data.clone())
        } else {
            None
        }
    }
}

// TODO: Test.