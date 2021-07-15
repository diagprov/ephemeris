
extern crate dirs;
extern crate regex;
extern crate serde_derive;
extern crate toml;

pub mod projects;

use std::collections::BTreeMap;

use std::fs;
use std::rc::Rc;
use std::result::Result;
use std::vec::Vec;

use crate::projects::*;

pub const EPHEMERIS_ENV : &'static str = "EPHEMERIS_DIR";
pub const EPHEMERIS_DIRNAME : &'static str = ".ephemeris";

const EPH_PROJECTNAME : &'static str = "projects.toml";

pub struct State {
    pub projects : BTreeMap<String, Rc<Box<Project>>>,
    pub tagmap : BTreeMap<String, Vec<Rc<Box<Project>>>>,
}



impl State {

    pub fn project_add(&mut self, code: &String, name: &String, tags: &Option<Vec<String>>) -> Result<(), String> {
        
        if self.projects.contains_key(code) {
            return Err(String::from(format!("Code {} already exists for another project.", code)));
        }

        let proj = Project::new(code, name, tags.as_ref().unwrap()).unwrap();

        self.projects.insert(code.clone(), Rc::new(Box::new(proj)));
        // TODO: inconsistent tag state here.
        Ok(())
    }

    pub fn project_remove(&mut self, code: &String) -> Result<(), String> {
        
        if !self.projects.contains_key(code) {
            return Err(String::from(format!("Code {} does not refer to a project.", code)));
        }
    
        let _p : Rc<Box<Project>> = self.projects.remove(code).unwrap();
        // TODO: inconsistent tag state here.
        Ok(())
    }

    fn projects_as_toml(&self) -> String {
        let mut pvec : Vec<Project> = Vec::new();
        let mut plist : ProjectList = ProjectList{project: Some(pvec)};
        for v in self.projects.values() {
            let r : &Rc<Box<Project>> = v;
            let b : &Box<Project> = r.as_ref();
            let p = b.as_ref().clone();
            let mut v : Vec<Project> = plist.project.unwrap();
            v.push(p);
            plist.project = Some(v);
        };
        toml::to_string(&plist).unwrap()
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

        let projectsfilename = format!("{}/{}", ephemeris_dir, EPH_PROJECTNAME);
        fs::write(projectsfilename, projects_toml.as_bytes()).unwrap();
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

        let mut projectmap : BTreeMap<String, Rc<Box<Project>>> = BTreeMap::new();
        let mut tagmap : BTreeMap<String, Vec<Rc<Box<Project>>>> = BTreeMap::new();
        for p in projects {

            let code = String::from(&p.code.clone());
            let prj = Box::new(p);
            let prjrc : Rc<Box<Project>> = Rc::new(prj);

            projectmap.insert(code.clone(), prjrc.clone());
            let tags = match &prjrc.tags {
                Some(t) => t,
                None => continue,
            };

            for tag in tags {
                if tagmap.contains_key(tag) {
                    let v : &mut Vec<Rc<Box<Project>>> = match tagmap.get_mut(tag) {
                        Some(v) => v,
                        None => return Err(String::from("Error")),
                    } ;

                    v.push(prjrc.clone());
                } else {
                    let v : Vec<Rc<Box<Project>>> = vec![prjrc.clone()];
                    tagmap.insert(String::from(tag), v);
                }
            }

        }
        let state = Box::new(State{projects: projectmap, tagmap: tagmap});
        println!("{}", state.projects_as_toml());
        Ok(state)
    }
}
