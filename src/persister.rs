use actix_web::web::Data;

use crate::persistence::Persist;

use super::Database;

pub struct DatabasePersisterFactory<D: Database> {
    db: D,
}

impl<D: Database> DatabasePersisterFactory<D> {
    pub fn with(db: D) -> Self {
        Self { db }
    }
    pub fn get(&self, ts: u64) -> DatabasePersister<'_, D> {
        DatabasePersister { db: &self.db, ts }
    }
}
pub struct DatabasePersister<'a, D: Database> {
    db: &'a D,
    ts: u64,
}

impl<'a, D: Database> Persist for DatabasePersister<'_, D> {
    fn load_occupancy(&self) -> Vec<map::Update> {
        todo!()
    }

    fn update(&mut self, update: map::Update) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_persister_factory() -> DatabasePersisterFactory<sqlite::Connection> {
        DatabasePersisterFactory::with(sqlite::open(":memory:").unwrap())
    }

    #[test]
    fn load_is_empty_if_never_saved() {
        let persister_factory = create_persister_factory();
        let persister = persister_factory.get(0);
        let data = persister.load_occupancy();
        assert!(data.is_empty());
    }

    #[test]
    fn update_then_load_gives_correct_occupancy() {
        let persister_factory = create_persister_factory();
        let mut persister = persister_factory.get(0);
        let update = (('A', 3), map::SeatStatus::Sold);
        persister.update(update);
        let data = persister.load_occupancy();
        assert_eq!(vec![update], data);
    }

    #[test]
    fn update_multiple_times_then_load_gives_correct_occupancy() {
        let persister_factory = create_persister_factory();
        let mut persister = persister_factory.get(0);
        let update = (('A', 3), map::SeatStatus::Sold);
        persister.update(update.clone());
        persister.update(update.clone());
        let data = persister.load_occupancy();
        assert_eq!(vec![update], data);
    }

    #[test]
    fn multiple_updates_give_last_applied() {
        let persister_factory = create_persister_factory();
        let mut persister = persister_factory.get(0);
        let sell = (('A', 3), map::SeatStatus::Sold);
        let lock = (('A', 3), map::SeatStatus::Locked);
        persister.update(lock);
        persister.update(sell);
        let data = persister.load_occupancy();
        assert_eq!(vec![sell], data);
    }

    #[test]
    fn sell_then_free_gives_empty_status() {
        let persister_factory = create_persister_factory();
        let mut persister = persister_factory.get(0);
        let sell = (('A', 3), map::SeatStatus::Sold);
        let free = (('A', 3), map::SeatStatus::Free);
        persister.update(sell);
        persister.update(free);
        let data = persister.load_occupancy();
        assert!(data.is_empty());
    }
}
