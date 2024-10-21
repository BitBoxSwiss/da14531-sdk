pub use crate::bindings::{
    ke_state_handler as KeStateHandler, ke_state_t as KeState, ke_task_desc as KeTaskDesc,
    ke_task_id_t as KeTaskId,
};

unsafe impl Sync for KeTaskDesc {}

#[inline]
pub fn ke_state_set(task_id: KeTaskId, state_id: KeState) {
    unsafe {
        crate::bindings::ke_state_set(task_id, state_id);
    }
}

#[inline]
pub fn ke_state_get(task_id: KeTaskId) -> KeState {
    unsafe { crate::bindings::ke_state_get(task_id) }
}

#[inline]
pub fn ke_task_create(task_type: u8, task_desc: &KeTaskDesc) {
    unsafe {
        crate::bindings::ke_task_create(task_type, task_desc);
    }
}

/// Retrieve the task Type (task index) from message
#[inline]
pub const fn msg_t(msg: KeTaskId) -> u8 {
    (msg >> 8) as u8
}

/// Retrieve the Task ID (message index) from the message
#[inline]
pub const fn msg_i(msg: KeTaskId) -> u8 {
    (msg & 0xff) as u8
}
