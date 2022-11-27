use std::mem;
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use either::Either;

pub struct RwList<T> {
    root: RwLock<Option<Arc<RWListNode<T>>>>
}

impl<'a, T> IntoIterator for &'a RwList<T> {
    type Item = RWListValue<'a, T>;
    type IntoIter = RWListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RWListIterator::from(self.root.read().unwrap())
    }
}

impl<T> RwList<T> {
    pub fn new() -> RwList<T> {
        RwList {
            root: RwLock::new(None)
        }
    }

    pub fn add(&self, value: T) {
        let mut lock = self.root.write().unwrap();
        if let Some(root) = lock.take() {
            *lock = Some(Arc::new(RWListNode::with_next(value, root)));
        } else {
            *lock = Some(Arc::new(RWListNode::new(value)));
        }
    }
}

struct RWListNode<T> {
    value: T,
    next: Option<Arc<RWListNode<T>>>,
}

impl<T> RWListNode<T> {
    fn new(value: T) -> RWListNode<T> {
        RWListNode {
            value,
            next: None
        }
    }

    fn with_next(value: T, next: Arc<RWListNode<T>>) -> RWListNode<T> {
        RWListNode {
            value,
            next: Some(next)
        }
    }
}

pub struct RWListIterator<'a, T>(RWListValue<'a, T>);

impl<'a, T> Iterator for RWListIterator<'a, T> {
    type Item = RWListValue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let new = match &self.0.0 {
            Either::Left(l) => {
                if !l.is_some() {
                    return None;
                }

                if let Some(next) = &l.as_ref().unwrap().next {
                    RWListValue::from(next.clone())
                } else {
                    return None;
                }
            }
            Either::Right(r) => {
                if let Some(next) = &r.next {
                    RWListValue::from(next.clone())
                } else {
                    return None;
                }
            }
        };

        Some(mem::replace(&mut self.0, new))
    }
}

impl<'a, T> From<RwLockReadGuard<'a, Option<Arc<RWListNode<T>>>>> for RWListIterator<'a, T> {
    fn from(lock_node: RwLockReadGuard<'a, Option<Arc<RWListNode<T>>>>) -> Self {
        RWListIterator(RWListValue::from(lock_node))
    }
}

pub struct RWListValue<'a, T>(Either<RwLockReadGuard<'a, Option<Arc<RWListNode<T>>>>, Arc<RWListNode<T>>>);

impl<T> Deref for RWListValue<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            Either::Left(l) => &l.as_ref().unwrap().value,
            Either::Right(r) => &r.value
        }
    }
}

impl<'a, T> From<RwLockReadGuard<'a, Option<Arc<RWListNode<T>>>>> for RWListValue<'a, T> {
    fn from(value: RwLockReadGuard<'a, Option<Arc<RWListNode<T>>>>) -> Self {
        RWListValue(Either::Left(value))
    }
}

impl<'a, T> From<Arc<RWListNode<T>>> for RWListValue<'a, T> {
    fn from(value: Arc<RWListNode<T>>) -> Self {
        RWListValue(Either::Right(value))
    }
}