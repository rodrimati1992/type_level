use crate_::collection_ops::Reverse_;

impl Reverse_ for () {
    type Output = ();
}

impl<L0> Reverse_ for (L0,) {
    type Output = (L0,);
}
impl<L0, L1> Reverse_ for (L0, L1) {
    type Output = (L1, L0);
}
impl<L0, L1, L2> Reverse_ for (L0, L1, L2) {
    type Output = (L2, L1, L0);
}
impl<L0, L1, L2, L3> Reverse_ for (L0, L1, L2, L3) {
    type Output = (L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4> Reverse_ for (L0, L1, L2, L3, L4) {
    type Output = (L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5> Reverse_ for (L0, L1, L2, L3, L4, L5) {
    type Output = (L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6> Reverse_ for (L0, L1, L2, L3, L4, L5, L6) {
    type Output = (L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7> Reverse_ for (L0, L1, L2, L3, L4, L5, L6, L7) {
    type Output = (L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8> Reverse_ for (L0, L1, L2, L3, L4, L5, L6, L7, L8) {
    type Output = (L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9> Reverse_ for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9) {
    type Output = (L9, L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10> Reverse_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
{
    type Output = (L10, L9, L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11> Reverse_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
{
    type Output = (L11, L10, L9, L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12> Reverse_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
{
    type Output = (L12, L11, L10, L9, L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13> Reverse_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
{
    type Output = (L13, L12, L11, L10, L9, L8, L7, L6, L5, L4, L3, L2, L1, L0);
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14> Reverse_
    for (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
    )
{
    type Output = (
        L14,
        L13,
        L12,
        L11,
        L10,
        L9,
        L8,
        L7,
        L6,
        L5,
        L4,
        L3,
        L2,
        L1,
        L0,
    );
}
impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15> Reverse_
    for (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
        L15,
    )
{
    type Output = (
        L15,
        L14,
        L13,
        L12,
        L11,
        L10,
        L9,
        L8,
        L7,
        L6,
        L5,
        L4,
        L3,
        L2,
        L1,
        L0,
    );
}
