#[macro_export]
macro_rules! builder {
    (
        $(
            $with:ident => $ty:ty
        ),*
    ) => {
        paste::paste! {
            $(
                pub fn [<with_ $with>]<W: $ty + 'static>(mut self, $with: W) -> Self {
                    self.$with.push(Box::new($with));
                    self
                }
            )*
        }
    };
}