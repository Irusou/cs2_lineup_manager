use uuid::Uuid;

use crate::domain::lineup::Lineup;

#[derive(Debug)]
pub struct Manager {
    lineups: Vec<Lineup>,
    backup_version: i32,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            lineups: Vec::new(),
            backup_version: 1,
        }
    }

    fn save_to_file(path: String) {
        todo!()
    }

    fn read_from_file(path: String) {
        todo!()
    }

    fn create_backup() {
        todo!()
    }

    fn load_from_backup(version: String) {
        todo!()
    }

    fn list(filter: String) -> Vec<Lineup> {
        todo!()
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Lineup> {
        match self.lineups.iter().find(|lineup| lineup.id() == id) {
            Some(lineup) => Some(lineup),
            None => None,
        }
    }

    fn add_lineup(&mut self, lineup: Lineup) -> Uuid {
        let id = lineup.id();
        self.lineups.push(lineup);
        id
    }

    fn update_lineup(id: Uuid) -> Uuid {
        todo!()
    }

    fn delete_lineup(id: Uuid) -> Uuid {
        todo!()
    }

    pub fn lineups(&self) -> &[Lineup] {
        &self.lineups
    }

    pub fn lineups_mut(&mut self) -> &mut Vec<Lineup> {
        &mut self.lineups
    }

    pub fn set_lineups(&mut self, lineups: Vec<Lineup>) {
        self.lineups = lineups;
    }
}

#[cfg(test)]
mod test {
    use crate::domain::types::{Map, ThrowType, UtilityKind};

    use super::*;

    #[test]
    fn test_create_manager() {
        let m = Manager::new();
        assert_eq!(m.lineups.len(), 0);
        assert_eq!(m.backup_version, 1);
    }

    #[test]
    fn test_create_and_add_lineup_to_list() {
        let mut m = Manager::new();

        let lineup = Lineup::new(
            String::from("ramp pop flash"),
            Map::Mirage,
            UtilityKind::Flash,
            ThrowType::Stand,
        );

        let line_id = lineup.id();
        let added_id = m.add_lineup(lineup);

        assert_eq!(m.lineups.len(), 1);
        assert_eq!(line_id, added_id);
    }

    #[test]
    fn test_get_by_id() {
        let mut m = Manager::new();

        let lineup = Lineup::new(
            String::from("ramp pop flash"),
            Map::Mirage,
            UtilityKind::Flash,
            ThrowType::Stand,
        );

        let added_id = m.add_lineup(lineup);

        if let Some(lnp) = m.get_by_id(added_id) {
            assert_eq!(lnp.id(), added_id)
        }
    }
}
