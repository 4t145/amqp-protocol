use super::Type;

pub trait Restrict<'a>: Sized {
    type Source: Type<'a>;
    fn restrict(source: Self::Source) -> Result<Self, Self::Source>;
    fn source(self) -> Self::Source;
}