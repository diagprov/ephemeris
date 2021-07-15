

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub name : String,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub task : Option<Vec<Task>>,
}
