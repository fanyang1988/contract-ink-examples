pub mod modules {
    pub trait Module<T>{
        fn get_module(&self) -> &T;
        fn get_module_mut(&mut self) -> &mut T;
    }
}

pub use modules::{
    Module
};