use crate::task_info::TaskInfo;
use crate::sytem_info::SystemInfo;

pub trait TaskManagerCore
{

    fn kill_process(&self, process_id: i32)->Result<(),dyn Error>;
    fn list_processes(&self)->Result<Vec<TaskInfo>,Box<dyn Error>>;

    fn get_process_info(&self, process_id: i32)->Result<TaskInfo,Box<dyn Error>>;

    fn get_system_info(&self)->Result<SystemInfo,Box<dyn Error>>;
    //list processes
    //get_proces_info
    //system_resources
}