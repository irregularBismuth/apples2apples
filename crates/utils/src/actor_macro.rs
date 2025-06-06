///Simplify the actor implementation via macro providing the types used in ractor
///
/// `Msg`
/// `State`
/// `Arguments`
#[macro_export]
macro_rules! actor_types {
    ($msg:ty, $state:ty, $args:ty) => {
        type Msg = $msg;
        type State = $state;
        type Arguments = $args;
    };
}
