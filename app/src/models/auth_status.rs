use bounce::Atom;

#[derive(Atom, PartialEq, Clone, Debug)]
pub enum AuthStatus {
    Fetching,
    Authenticated,
    Unauthenticated,
}

impl Default for AuthStatus {
    fn default() -> Self {
        Self::Fetching
    }
}
