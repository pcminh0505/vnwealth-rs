pub type DispatchResult = Result<(), &'static str>;
pub type DispatchGenericResult<T> = Result<T, &'static str>;

pub trait Dispatch {
    type Caller;
    type Call;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
