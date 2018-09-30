use crate_::ops::iteration_ops::{PopBack_, PushBack_};
use prelude::*;





impl<Value> PushBack_<Value> for () {
    type Output = (Value,);
}

impl<L0, Value> PushBack_<Value> for (L0,) {
    type Output = (L0, Value);
}

impl<L0, L1, Value> PushBack_<Value> for (L0, L1) {
    type Output = (L0, L1, Value);
}

impl<L0, L1, L2, Value> PushBack_<Value> for (L0, L1, L2) {
    type Output = (L0, L1, L2, Value);
}

impl<L0, L1, L2, L3, Value> PushBack_<Value> for (L0, L1, L2, L3) {
    type Output = (L0, L1, L2, L3, Value);
}

impl<L0, L1, L2, L3, L4, Value> PushBack_<Value> for (L0, L1, L2, L3, L4) {
    type Output = (L0, L1, L2, L3, L4, Value);
}

impl<L0, L1, L2, L3, L4, L5, Value> PushBack_<Value> for (L0, L1, L2, L3, L4, L5) {
    type Output = (L0, L1, L2, L3, L4, L5, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, Value> PushBack_<Value> for (L0, L1, L2, L3, L4, L5, L6) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, Value> PushBack_<Value> for (L0, L1, L2, L3, L4, L5, L6, L7) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, Value> PushBack_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Value> PushBack_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Value> PushBack_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Value> PushBack_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Value> PushBack_<Value>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Value);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, Value> PushBack_<Value>
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

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, Value> PushBack_<Value>
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

impl PopBack_ for () {
    type Output = None_;
}


impl<L0,> PopBack_
for (L0,)
{
    type Output=Some_<(L0,())>;
}

impl<L0,L1,> PopBack_
for (L0,L1,)
{
    type Output=Some_<(L1,(L0,))>;
}

impl<L0,L1,L2,> PopBack_
for (L0,L1,L2,)
{
    type Output=Some_<(L2,(L0,L1,))>;
}

impl<L0,L1,L2,L3,> PopBack_
for (L0,L1,L2,L3,)
{
    type Output=Some_<(L3,(L0,L1,L2,))>;
}

impl<L0,L1,L2,L3,L4,> PopBack_
for (L0,L1,L2,L3,L4,)
{
    type Output=Some_<(L4,(L0,L1,L2,L3,))>;
}

impl<L0,L1,L2,L3,L4,L5,> PopBack_
for (L0,L1,L2,L3,L4,L5,)
{
    type Output=Some_<(L5,(L0,L1,L2,L3,L4,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,)
{
    type Output=Some_<(L6,(L0,L1,L2,L3,L4,L5,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,)
{
    type Output=Some_<(L7,(L0,L1,L2,L3,L4,L5,L6,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,)
{
    type Output=Some_<(L8,(L0,L1,L2,L3,L4,L5,L6,L7,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,)
{
    type Output=Some_<(L9,(L0,L1,L2,L3,L4,L5,L6,L7,L8,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,)
{
    type Output=Some_<(L10,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,)
{
    type Output=Some_<(L11,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,)
{
    type Output=Some_<(L12,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,)
{
    type Output=Some_<(L13,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,)
{
    type Output=Some_<(L14,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,))>;
}

impl<L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,> PopBack_
for (L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,L15,)
{
    type Output=Some_<(L15,(L0,L1,L2,L3,L4,L5,L6,L7,L8,L9,L10,L11,L12,L13,L14,))>;
}

