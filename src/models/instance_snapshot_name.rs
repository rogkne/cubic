use crate::models::{InstanceName, SnapshotName};
use std::fmt;
use std::str::FromStr;

/// A command target that is either a whole instance or one of its snapshots,
/// written as `<instance>` or `<instance>/<snapshot>`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InstanceSnapshotName {
    Instance(InstanceName),
    Snapshot(SnapshotName),
}

impl InstanceSnapshotName {
    pub fn get_instance(&self) -> &InstanceName {
        match self {
            InstanceSnapshotName::Instance(instance) => instance,
            InstanceSnapshotName::Snapshot(snapshot) => snapshot.get_instance(),
        }
    }

    pub fn get_snapshot(&self) -> Option<&SnapshotName> {
        match self {
            InstanceSnapshotName::Instance(_) => None,
            InstanceSnapshotName::Snapshot(snapshot) => Some(snapshot),
        }
    }
}

impl FromStr for InstanceSnapshotName {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if name.contains('/') {
            Ok(InstanceSnapshotName::Snapshot(SnapshotName::from_str(
                name,
            )?))
        } else {
            Ok(InstanceSnapshotName::Instance(InstanceName::from_str(
                name,
            )?))
        }
    }
}

impl fmt::Display for InstanceSnapshotName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            InstanceSnapshotName::Instance(instance) => write!(f, "{instance}"),
            InstanceSnapshotName::Snapshot(snapshot) => write!(f, "{snapshot}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_both_forms() {
        let instance = InstanceSnapshotName::from_str("mymachine").unwrap();
        assert_eq!(instance.get_instance().as_str(), "mymachine");
        assert!(instance.get_snapshot().is_none());

        let snapshot = InstanceSnapshotName::from_str("mymachine/clean").unwrap();
        assert_eq!(snapshot.get_instance().as_str(), "mymachine");
        assert_eq!(snapshot.get_snapshot().unwrap().as_str(), "clean");
        assert_eq!(snapshot.to_string(), "mymachine/clean");
    }

    // The separator must not become a way back into the parent directory.
    #[test]
    fn test_reject_path_traversal() {
        assert!(InstanceSnapshotName::from_str("../../etc").is_err());
        assert!(InstanceSnapshotName::from_str("mymachine/..").is_err());
        assert!(InstanceSnapshotName::from_str("mymachine/clean/extra").is_err());
        assert!(InstanceSnapshotName::from_str("/clean").is_err());
    }
}
