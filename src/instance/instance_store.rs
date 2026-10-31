use crate::error::Result;
use crate::models::Instance;
use std::str;

pub trait InstanceStore {
    fn get_instances(&self) -> Vec<String>;
    fn exists(&self, name: &str) -> bool;
    fn load(&self, name: &str) -> Result<Instance>;
    fn store(&self, instance: &Instance) -> Result<()>;

    fn rename(&self, instance: &mut Instance, new_name: &str) -> Result<()>;
    fn delete(&self, instance: &Instance) -> Result<()>;
}
