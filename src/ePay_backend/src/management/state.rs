use std::collections::HashSet;

use candid::{Deserialize, CandidType, Principal};

#[derive(CandidType, Deserialize)]
pub struct StateInfo {
    managers: HashSet<Principal>
}

impl StateInfo {
    pub fn add_manager(&mut self, manager: Principal) -> bool {
        self.managers.insert(manager)
    }

    pub fn remove_manager(&mut self, manager: Principal) -> bool {
        self.managers.remove(&manager)
    }

    pub fn is_manager(&self, manager: Principal) -> bool {
        self.managers.contains(&manager)
    }
}

impl Default for StateInfo {
    fn default() -> Self {
        Self { managers: HashSet::new() }
    }
}
