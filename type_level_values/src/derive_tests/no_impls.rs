/**
```compile_fail
use type_level_values::prelude::*;;
use type_level_values::derive_tests::no_impls::ConstTupledNoImpls;

format!("{:?}",ConstTupledNoImpls::<U10,U20>::MTVAL);
```
*/
#[derive(TypeLevel)]
#[typelevel(reexport(Variants))]
pub struct TupledNoImpls(pub u32, pub u32);
