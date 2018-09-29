use std::hash::Hash;
use resource::Resource;


pub trait AddResource<'a> {
    type Error;

    fn add(&mut self, id: impl Hash, resource: Resource<'a>) -> Result<(), Self::Error>;
}

pub trait GetResource {
    type Error;

    fn get<'a>(&self, id: impl Hash) -> Result<Resource<'a>, Self::Error>;
}

pub trait Resources<'a>: AddResource<'a> + GetResource {}
