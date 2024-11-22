
pub struct Assert<const COND: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait IsFalse {}

impl IsFalse for Assert<false> {}