use super::ExDatabase;

pub trait LoadExDatabase {
    fn load(path: String) -> ExDatabase;
}

impl LoadExDatabase for ExDatabase {
    fn load(path: String) -> ExDatabase {
        todo!()
    }
}