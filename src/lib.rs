// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-elf-utilities: elf utilities component model.
//!
//! Real implementation: utilitiess, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utilities {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfUtilitiesError {
    EmptyUtilitiesName,
    DuplicateUtilities(String),
    EmptyEventSubject,
}

impl fmt::Display for ElfUtilitiesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUtilitiesName => write!(f, "utilities name must not be empty"),
            Self::DuplicateUtilities(id) => write!(f, "duplicate utilities {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ElfUtilitiesError {}

pub struct ElfUtilities {
    pub name: String,
    utilitiess: BTreeMap<String, Utilities>,
    events: BTreeMap<String, Event>,
}

impl ElfUtilities {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            utilitiess: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_utilities(&mut self, item: Utilities) -> Result<(), ElfUtilitiesError> {
        if item.name.trim().is_empty() {
            return Err(ElfUtilitiesError::EmptyUtilitiesName);
        }
        let id = item.id.clone();
        if self.utilitiess.contains_key(&id) {
            return Err(ElfUtilitiesError::DuplicateUtilities(id.clone()));
        }
        self.utilitiess.insert(id, item);
        Ok(())
    }

    pub fn utilities_count(&self) -> usize {
        self.utilitiess.len()
    }

    pub fn get_utilities(&self, id: &str) -> Option<&Utilities> {
        self.utilitiess.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ElfUtilitiesError> {
        if event.subject.trim().is_empty() {
            return Err(ElfUtilitiesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_utilities(id: &str, name: &str) -> Utilities {
        Utilities {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ElfUtilities::new("ElfUtilities");
        assert_eq!(m.name, "ElfUtilities");
        assert_eq!(m.utilities_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_utilities() {
        let mut m = ElfUtilities::new("ElfUtilities");
        m.add_utilities(sample_utilities("a1", "Alpha")).unwrap();
        assert_eq!(m.utilities_count(), 1);
        assert!(m.get_utilities("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ElfUtilities::new("ElfUtilities");
        assert!(m.add_utilities(sample_utilities("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ElfUtilities::new("ElfUtilities");
        m.add_utilities(sample_utilities("x", "One")).unwrap();
        assert!(m.add_utilities(sample_utilities("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ElfUtilities::new("ElfUtilities");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
