use resource_type::ResourceType;


pub struct Resource<'a> {
    pub resource_type: ResourceType,
    pub buffer: &'a [u8],
}
