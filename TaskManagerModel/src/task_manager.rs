use std::Box;
use crate::TaskManagerCore::TaskManagerCore;
use crate::system_info::SystemInfo;

pub struct TaskManager
{
    tasks:vec<TaskInfo>,
    task_manager_core: Box<dyn TaskManagerCore>,
    system_info: SystemInfo,

}

impl TaskManager
{
    pub fn new(tasks:Vec<TaskInfo>, warkusz:Box<dyn TaskManagerCore>) -> TaskManager
    {

    }
}