
use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;
use serde_derive::{Serialize, Deserialize};

pub type ProjectRef = Rc<RefCell<Project>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub code : String,
    pub name : String,
    pub tags : Option<Vec<String>>,
    // FEATURE: do we support project nesting like this?
    // pub projects: Option<Vec<ProjectRef>>,
}



#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectList {
    pub project : Option<Vec<Project>>,
}


impl Project {

    fn validate_code(code: &String) -> bool {
        let re = Regex::new(r"^[A-Za-z0-9-_]+$").unwrap();
        re.is_match(code)
    }

    pub fn new(code: &String, name: &String, tags: &Vec<String>) -> Result<Project, String> {
        
        if !Project::validate_code(code) {
            return Err(String::from(format!("The code {} is not a valid shortcode.", code)));
        } 

        Ok(Project{
            code: code.clone(),
            name: name.clone(),
            tags: Some(tags.clone()),
            //projects: None,
        })
    }

}
