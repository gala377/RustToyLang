use ftl_source::Pointer;

use crate::PRes;

#[allow(dead_code)]
pub(crate) fn pres_lift_fn<R1, R2, P, F1>(f: F1) -> impl FnOnce(PRes<R1, P>) -> PRes<R2, P>
where
    P: Pointer,
    F1: FnOnce(R1) -> R2,
{
    move |res| res.and_then(move |val| Ok(f(val)))
}
