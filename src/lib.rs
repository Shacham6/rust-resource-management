use std::hash::Hash;

pub mod resources;
pub mod resource_loader;
pub mod resource;
pub mod resource_type;


#[cfg(tests)]
mod tests {
    #[test]
    fn test_something() {
        let arr = [1, 2, 3, 4, 5, 6];
        let slice = &arr;
    }
}
