use crate::instance::Instance;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Selection {
    #[default]
    Overview,
    AddInstance(Instance),
    Instance(usize, Instance),
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectionChanged(Selection),
    InstanceNameChanged(String),
    Delete,
    Save,
}
