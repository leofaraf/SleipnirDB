use super::ExDatabase;

pub trait SaveExDatabase {
    fn save(&self) -> ExDatabase;
}

impl SaveExDatabase for ExDatabase {
    fn save(&self) -> ExDatabase {
        todo!()
    }
}