use resources::Resources;


pub trait ResourceLoader<'a, _Resources>
    where
        _Resources: Resources<'a> {
    type Error;

    fn load(&self) -> Result<_Resources, Self::Error>;
}
