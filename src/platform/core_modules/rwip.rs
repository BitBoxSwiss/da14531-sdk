pub use crate::bindings::{
    KE_API_ID as KeApiId, KE_MEM_ATT_DB, KE_MEM_ENV, KE_MEM_KE_MSG, KE_MEM_NON_RETENTION,
    KE_TASK_TYPE as KeTaskType,
};

#[inline]
pub fn rwip_schedule() {
    unsafe {
        crate::bindings::rwip_schedule();
    }
}
