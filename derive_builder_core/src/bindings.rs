use RawTokens;

/// Bindings to be used by the generated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bindings {
    /// Use the standard library.
    Std,
    /// Don't use the standard library.
    NoStd,
}

impl Default for Bindings {
    fn default() -> Self {
        Bindings::Std
    }
}

impl Bindings {
    /// String type.
    pub fn string_ty(&self) -> RawTokens<&'static str> {
        RawTokens(match *self {
            Bindings::Std => ":: std :: string :: String",
            Bindings::NoStd => ":: collections :: string :: String",
        })
    }
}

#[test]
fn std() {
    let b = Bindings::Std;

    assert_eq!(b.string_ty().to_tokens(), quote!(::std::string::String));
}

#[test]
fn no_std() {
    let b = Bindings::NoStd;

    assert_eq!(b.string_ty().to_tokens(),
               quote!(::collections::string::String));
}
