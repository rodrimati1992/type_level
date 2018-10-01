use crate_::ops::iteration_ops::{Pop_, Push_};
use crate_::std_types::option::{None_, Some_};

use typenum::consts::{U0, U1, U10, U11, U12, U13, U14, U15, U2, U3, U4, U5, U6, U7, U8, U9};

impl<Value> Push_<Value> for () {
    type Output = (Value,);
}

impl<L0, Value> Push_<Value> for (L0,) {
    type Output = (L0, Value);
}

impl<L0, L1, Value> Push_<Value> for (L0, L1) {
    type Output = (L0, L1, Value);
}

impl<L0, L1, L2, Value> Push_<Value> for (L0, L1, L2) {
    type Output = (L0, L1, L2, Value);
}

impl<L0, L1, L2, L3, Value> Push_<Value> for (L0, L1, L2, L3) {
    type Output = (L0, L1, L2, L3, Value);
}

impl<L0, L1, L2, L3, L4, Value> Push_<Value> for (L0, L1, L2, L3, L4) {
    type Output = (L0, L1, L2, L3, L4, Value);
}

impl<L0, L1, L2, L3, L4, L5, Value> Push_<Value> for (L0, L1, L2, L3, L4, L5) {
    type Output = (L0, L1, L2, L3, L4, L5, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, Value> Push_<Value> for (L0, L1, L2, L3, L4, L5, L6) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, Value> Push_<Value> for (L0, L1, L2, L3, L4, L5, L6, L7) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, Value> Push_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
{
    type Output = (
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
        Value,
    );
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, Value> Push_<Value>
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
        Value,
    );
}

////////////////////////////////////////////////////////////////////////

impl Pop_ for () {
    type Output = None_;
}

impl<L0> Pop_ for (L0,) {
    type Output = Some_<(L0, ())>;
}

impl<L0, L1> Pop_ for (L0, L1) {
    type Output = Some_<(L1, (L0,))>;
}

impl<L0, L1, L2> Pop_ for (L0, L1, L2) {
    type Output = Some_<(L2, (L0, L1))>;
}

impl<L0, L1, L2, L3> Pop_ for (L0, L1, L2, L3) {
    type Output = Some_<(L3, (L0, L1, L2))>;
}

impl<L0, L1, L2, L3, L4> Pop_ for (L0, L1, L2, L3, L4) {
    type Output = Some_<(L4, (L0, L1, L2, L3))>;
}

impl<L0, L1, L2, L3, L4, L5> Pop_ for (L0, L1, L2, L3, L4, L5) {
    type Output = Some_<(L5, (L0, L1, L2, L3, L4))>;
}

impl<L0, L1, L2, L3, L4, L5, L6> Pop_ for (L0, L1, L2, L3, L4, L5, L6) {
    type Output = Some_<(L6, (L0, L1, L2, L3, L4, L5))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7> Pop_ for (L0, L1, L2, L3, L4, L5, L6, L7) {
    type Output = Some_<(L7, (L0, L1, L2, L3, L4, L5, L6))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8> Pop_ for (L0, L1, L2, L3, L4, L5, L6, L7, L8) {
    type Output = Some_<(L8, (L0, L1, L2, L3, L4, L5, L6, L7))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9> Pop_ for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9) {
    type Output = Some_<(L9, (L0, L1, L2, L3, L4, L5, L6, L7, L8))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10> Pop_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
{
    type Output = Some_<(L10, (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11> Pop_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
{
    type Output = Some_<(L11, (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12> Pop_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
{
    type Output = Some_<(L12, (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13> Pop_
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
{
    type Output = Some_<(L13, (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12))>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14> Pop_
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
    type Output = Some_<(
        L14,
        (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13),
    )>;
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15> Pop_
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
    type Output = Some_<(
        L15,
        (
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
        ),
    )>;
}
