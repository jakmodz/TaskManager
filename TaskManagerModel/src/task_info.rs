pub struct TaskInfo
{
    pid: u32,
    name: String,
    description: String,

}

impl TaskInfo
{
    pub fn new(pid: u32, name: String, description: String) -> TaskInfo
    {
        TaskInfo
        { pid, name, description }
    }
}