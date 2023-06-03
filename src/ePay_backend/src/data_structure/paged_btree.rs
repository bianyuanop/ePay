use std::{collections::BTreeMap, borrow::BorrowMut};

pub enum InsertError {
    Occupied,
}

pub enum QueryError {
    Page2Large,
    KeyNotExists,
}

pub enum UpdateError {
    KeyNotExists,
    Page2Large,
}

pub struct PagedBTree<K, V> 
    where K: Ord, V: Clone
{
    pub page: usize,
    pub storage: Vec<BTreeMap<K, V>>,
}

impl<K, V> PagedBTree<K, V> 
    where K: Ord, V: Clone
{
    pub fn insert(&mut self, k: K, v: V) -> Result<(), InsertError> {
        let btree = self.storage.get_mut(self.page).unwrap();
        if btree.contains_key(&k) {
            Err(InsertError::Occupied)
        } else {
            btree.insert(k, v);
            Ok(())
        }
    }

    pub fn get(&self, p: usize, k: K) -> Result<V, QueryError> {
        if p > self.page {
            return Err(QueryError::Page2Large)
        } else {
            let btree = self.storage.get(p).unwrap();
            match btree.get(&k) {
                Some(v) => Ok(v.clone()),
                None => Err(QueryError::KeyNotExists)
            }
        }
    }

    pub fn update(&mut self, p: usize, k: K, v: V) -> Result<(), UpdateError> {
        if p > self.page {
            return Err(UpdateError::Page2Large);
        } else {
            let btree = self.storage.get(p).unwrap(); 
            match btree.get(&k) {
                Some(v) => {
                    Ok(())
                },
                None => Err(UpdateError::KeyNotExists)
            }
        }
    }
}