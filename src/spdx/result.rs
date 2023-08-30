use super::SPDXId;

pub enum SPDXFind<'a> {
    Exact(&'a str),
    Closest(Vec<SPDXId<'a>>),
}
