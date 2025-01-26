use crate::policy::name_match::common::Policy;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
use std::sync::Arc;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 1] = ["GameThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &HashMap<pid_t, CompactString>) {
    let task_map = Arc::new(task_map.clone());

    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(&Arc::clone(&task_map));
}
