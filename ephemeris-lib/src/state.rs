
use std::collections::BTreeMap;

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result;
use std::vec::Vec;

use crate::*;
use crate::projects::*;
use crate::tasks::*;

/*pub struct TagTree {
    subtags : BTreeMap<String, Option<TagTree>>,
    refs: Vec<ProjectRef>
}*/

pub struct State {
    pub projects : BTreeMap<String, ProjectRef>,
    pub tasks : Vec<TaskRef>,
    pub tagmap : BTreeMap<String, Vec<ProjectRef>>,
}


impl State {

    pub fn project_add(&mut self, code: &String, name: &String, tags: &Option<Vec<String>>) -> Result<(), String> {
        
        if self.projects.contains_key(code) {
            return Err(String::from(format!("Code {} already exists for another project.", code)));
        }

        let proj = Project::new(code, name, tags.as_ref().unwrap()).unwrap();

        self.projects.insert(code.clone(), Rc::new(RefCell::new(proj)));
        // TODO: inconsistent tag state here.
        Ok(())
    }

    pub fn project_remove(&mut self, code: &String) -> Result<(), String> {
        
        if !self.projects.contains_key(code) {
            return Err(String::from(format!("Code {} does not refer to a project.", code)));
        }
    
        let _p : ProjectRef = self.projects.remove(code).unwrap();
        // TODO: inconsistent tag state here.
        Ok(())
    }

    pub fn task_add(&mut self, name: &String, due: &Option<String>, projectcode: &String, tags: &Option<Vec<String>>) -> Result<(), String> {
        
        if projectcode!= "" && !self.projects.contains_key(projectcode) {
            return Err(String::from(format!("Code {} does not describe a project.", projectcode)));
        }


        //self.projects.insert(code.clone(), Rc::new(RefCell::new(proj)));
        // TODO: inconsistent tag state here.
        Ok(())
    }

    /*pub fn project_remove(&mut self, code: &String) -> Result<(), String> {
        
        if !self.projects.contains_key(code) {
            return Err(String::from(format!("Code {} does not refer to a project.", code)));
        }
    
        let _p : ProjectRef = self.projects.remove(code).unwrap();
        // TODO: inconsistent tag state here.
        Ok(())
    }*/

    fn projects_as_toml(&self) -> String {
        let mut pvec : Vec<Project> = Vec::new();

        for v in self.projects.values() {
            let r : &ProjectRef = v;
            let p : Project = match Rc::try_unwrap(r.clone()) {
            Ok(val) => val.borrow().clone(),
            Err(_) => continue,
            };
            pvec.push(p);
        };
        let plist : ProjectList = ProjectList{project: Some(pvec)};
        toml::to_string(&plist).unwrap()
    }

    fn tasks_as_toml(&self) -> String {
        let mut tvec : Vec<TaskRef> = Vec::new();

        for v in &self.tasks {
            let r : TaskRef = v.clone();
            tvec.push(r.clone());
        };
        let tlist : TaskList = TaskList{task: Some(tvec)};
        toml::to_string(&tlist).unwrap()
    }

    pub fn save(&self) -> Result<(), String> {
        
        let ephemeris_dir = match std::env::var_os(EPHEMERIS_ENV) {
            Some(v) => v.into_string().unwrap(),
            None => {
                match dirs::home_dir() {
                    Some(h) => format!("{}/{}", h.into_os_string().into_string().unwrap(), EPHEMERIS_DIRNAME),
                    None => return Err(String::from("Unable to locate home directory.")),
                }
            },
        };

        let projects_toml = self.projects_as_toml();
        let tasks_toml = self.tasks_as_toml();

        let projectsfilename = format!("{}/{}", ephemeris_dir, EPH_PROJECTNAME);
        fs::write(projectsfilename, projects_toml.as_bytes()).unwrap();
        let tasksfilename = format!("{}/{}", ephemeris_dir, EPH_TASKNAME);
        fs::write(tasksfilename, tasks_toml.as_bytes()).unwrap();
        Ok(())
    }


    pub fn load() -> Result<Box<State>, String> {

        let ephemeris_dir = match std::env::var_os(EPHEMERIS_ENV) {
            Some(v) => v.into_string().unwrap(),
            None => {
                match dirs::home_dir() {
                    Some(h) => format!("{}/{}", h.into_os_string().into_string().unwrap(), EPHEMERIS_DIRNAME),
                    None => return Err(String::from("Unable to locate home directory.")),
                }
            },
        };

        let projectsfilename = format!("{}/{}", ephemeris_dir, EPH_PROJECTNAME);
        let projects_fc = &fs::read(projectsfilename).unwrap();
        let projects_contents : String = String::from_utf8_lossy(projects_fc).parse().unwrap();

        let projectlist : ProjectList = match toml::from_str(&projects_contents) {
            Ok(p) => p,
            Err(s) => return Err(s.to_string()),
        };
        let projects = match projectlist.project {
            Some(v) => v,
            None => return Err(String::from("Unable to find project key in TOML file.")),
        };

        let mut projectmap : BTreeMap<String, ProjectRef> = BTreeMap::new();
        let mut tagmap : BTreeMap<String, Vec<ProjectRef>> = BTreeMap::new();
        for p in projects {

            /*
            FEATURE: nested project.
            match &p.projects {
                None => (),
                Some(p) => {
                    for subp in p {
                        println!("Subproj: {}", subp.name);
                    }
                },
            }*/

            let code = String::from(&p.code.clone());
            let prj = Rc::new(RefCell::new(p));

            projectmap.insert(code.clone(), prj.clone());
            let tags = match &prj.borrow().tags {
                Some(t) => t.clone(),
                None => continue,
            };

            for tag in tags {
                if tagmap.contains_key(&tag) {
                    let v : &mut Vec<ProjectRef> = match tagmap.get_mut(&tag) {
                        Some(v) => v,
                        None => return Err(String::from("Error")),
                    } ;

                    v.push(prj.clone());
                } else {
                    let v : Vec<ProjectRef> = vec![prj.clone()];
                    tagmap.insert(String::from(tag), v);
                }
            }

        }
        let tasksfilename = format!("{}/{}", ephemeris_dir, EPH_TASKNAME);

        let tasks_fc = &fs::read(tasksfilename).unwrap();
        let tasks_contents : String = String::from_utf8_lossy(tasks_fc).parse().unwrap();

        let tasklist : TaskList = match toml::from_str(&tasks_contents) {
            Ok(p) => p,
            Err(s) => return Err(s.to_string()),
        };
        let tasks = match tasklist.task {
            Some(v) => v,
            None => return Err(String::from("Unable to find project key in TOML file.")),
        };
        let state = Box::new(State{projects: projectmap, tagmap: tagmap, tasks: tasks.clone()});
        /*for t in &state.tasks {
            let mut taskref = t.borrow_mut();
            taskref.set_due(); 
        }*/
        Ok(state)
    }
}
