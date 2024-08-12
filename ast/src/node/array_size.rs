#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ArraySize {
    Fixed(usize),
    Dynamic,
}
