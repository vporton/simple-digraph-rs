use std::{rc::Rc, ops::Deref};

pub struct DigraphNode<T> {
    next: DigraphNodeRef<T>,
    data: T,
}

pub struct DigraphNodeRef<T> {
    rc: Rc<DigraphNode<T>>,
}

impl<T> DigraphNodeRef<T> {
    fn from(rc: Rc<DigraphNode<T>>) -> Self {
        Self {
            rc
        }
    }
    pub fn iter(&self) -> DigraphNodesIterator<T> {
        DigraphNodesIterator::from(self)
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


pub struct DigraphNodesIterator<T> {
    current: DigraphNodeRef<T>,
}

impl<T> DigraphNodesIterator<T> {
    pub fn from(ptr: &DigraphNodeRef<T>) -> Self {
        Self { current: DigraphNodeRef::from((*ptr).rc.clone()) }
    }
}

impl<T> Iterator for DigraphNodesIterator<T> {
    type Item = DigraphNodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if Rc::strong_count(&self.current.rc) != 0 { // FIXME
            self.current = DigraphNodeRef::from((*self.current.rc).next.rc.clone());
            Some(self.current.clone())
        } else {
            None
        }
    }
}

pub struct DigraphNodeValuesIterator<T> {
    underlying: DigraphNodesIterator<T>,
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