use support::{decl_storage, decl_module};

pub trait Trait: system::Trait{}

decal_storage!{
    trait Store for Module<T: Trait> as KittyStore{

    }
}

decal_module!{
    pub struct Module<T: Trait> for enum Call where origin: T::Origin{
        
    }
}