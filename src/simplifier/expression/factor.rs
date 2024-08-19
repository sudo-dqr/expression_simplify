use super::super::reform::poly::Poly;

pub trait Factor {
    fn to_polynomial(&self) -> Poly;
}