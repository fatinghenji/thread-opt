use crate::policy::name_match::policy_common::Policy;
use hashbrown::HashMap;
use libc::pid_t;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 1] = [" "];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &HashMap<pid_t, String>) {
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map);
}
