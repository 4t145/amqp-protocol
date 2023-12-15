use crate::Type;

// pub trait Provide<T>: Type
// where
//     Self::Source: Clone,
//     T: Type,
//     <T as Type>::Source: for<'a> From<&'a Self::Source>,
// {
//     fn provide(self) -> T {
//         T::restrict(self.unrestrict().into()).expect("invalid provide value")
//     }
// }

// pub trait Require: Type {}
