mod frunk {
    #[cfg(feature = "0_4")]
    pub(crate) use frunk_0_4::*;

    #[cfg(feature = "0_3")]
    pub(crate) use frunk_0_3::*;
}

mod frunk_core {
    #[cfg(feature = "0_4")]
    pub(crate) use frunk_core_0_4::*;

    #[cfg(feature = "0_3")]
    pub(crate) use frunk_core_0_3::*;
}

#[derive(Debug, PartialEq, Eq, frunk_enum_derive::LabelledGenericEnum)]
enum Colour<T> where T: std::fmt::Display {
    Red,
    Green,
    Blue,
    Named(T),
    Other(i32),
}

#[derive(Debug, PartialEq, Eq, frunk_enum_derive::LabelledGenericEnum)]
enum Color<T> where T: std::fmt::Display {
    Red,
    Green,
    Blue,
    Named(T),
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk::labelled::Transmogrifier;

    #[test]
    fn unit_variants() {
        let color = Color::<String>::Red;
        let colour: Colour<_> = color.transmogrify();
        assert_eq!(colour, Colour::Red);
    }

    #[test]
    fn generics() {
        let color = Color::Named("Silver");
        let colour: Colour<_> = color.transmogrify();
        assert_eq!(colour, Colour::Named("Silver"));
    }
}
