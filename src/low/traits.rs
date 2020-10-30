pub(crate) trait CType {
    type CItem;

    fn to_c_type(self) -> Self::CItem;
}
