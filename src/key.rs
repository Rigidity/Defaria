#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Key<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
}
