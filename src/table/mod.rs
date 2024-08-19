pub mod add;
pub mod query;
pub mod contains;
pub mod update;
pub mod remove;
pub mod information;
pub mod save;

pub struct ExTable<T> 
// where T: ()
{
    items: Vec<T>
}