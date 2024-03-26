use core::marker::PhantomData;

pub trait Config {
    type AccountId: ToString;
}

pub struct Pallet<T: Config> {
    _config: PhantomData<T>,
}

impl<T> Pallet<T>
where
    T: Config,
{
    pub fn new() -> Self {
        return Pallet {
            _config: PhantomData::default(),
        };
    }
}
