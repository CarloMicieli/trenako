use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Mutex;

/// It represents an in-memory repository, useful for the unit tests
pub struct InMemoryRepository<Id: Debug + Eq + Hash, T: Debug + Clone> {
    storage: Mutex<RefCell<HashMap<Id, T>>>,
}

impl<Id: Debug + Eq + Hash, T: Debug + Clone> InMemoryRepository<Id, T> {
    /// Creates an empty in-memory repository
    pub fn empty() -> Self {
        InMemoryRepository {
            storage: Mutex::new(RefCell::new(HashMap::new())),
        }
    }

    /// Creates an in-memory repository initialized with a single element
    pub fn of(key: Id, item: T) -> Self {
        let mut storage = HashMap::new();
        storage.insert(key, item);
        InMemoryRepository {
            storage: Mutex::new(RefCell::new(storage)),
        }
    }

    /// Returns true if the in-memory repository contains the id
    pub fn contains(&self, id: &Id) -> bool {
        let items = self.storage.lock().expect("unable to acquire the items lock");
        items.borrow().contains_key(id)
    }

    /// Adds a new element to the in-memory repository
    pub fn add(&self, id: Id, new_item: T) {
        let items = self.storage.lock().expect("Unable to acquire the lock");
        items.borrow_mut().insert(id, new_item);
    }

    /// Returns the number of elements in the in-memory repository
    pub fn len(&self) -> usize {
        let items = self.storage.lock().expect("unable to acquire the items lock");
        items.borrow().len()
    }

    /// Returns true if the in-memory repository contains no elements.
    pub fn is_empty(&self) -> bool {
        let items = self.storage.lock().expect("unable to acquire the items lock");
        items.borrow().is_empty()
    }

    /// Find the item with the `id` id (if any)
    pub fn find_by_id(&self, id: &Id) -> Option<T> {
        let items = self.storage.lock().expect("unable to acquire the items lock");
        items.borrow().get(id).cloned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod in_memory_repository {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_an_in_memory_repository() {
            let repository = InMemoryRepository::<Id, String>::empty();
            assert_eq!(0, repository.len());
            assert_eq!(false, repository.contains(&Id(42)));
            assert_eq!(true, repository.is_empty());

            repository.add(Id(42), String::from("answer"));
            assert_eq!(1, repository.len());
            assert_eq!(true, repository.contains(&Id(42)));
            assert_eq!(false, repository.is_empty());
        }

        #[test]
        fn it_should_create_a_singleton_in_memory_repository() {
            let repository = InMemoryRepository::of(Id(42), "answer");
            assert_eq!(1, repository.len());
            assert_eq!(false, repository.is_empty());
            assert_eq!(true, repository.contains(&Id(42)));
        }

        #[derive(Debug, Hash, PartialEq, Eq)]
        struct Id(u32);
    }
}
