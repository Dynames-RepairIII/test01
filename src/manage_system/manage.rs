use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Student {
    id:u32,
    name:String,
}

#[derive(debug)]
struct Class{
    id:u32,
    name:String,
    student:Vec<student>,
}

#[derive(debug)]
struct Course{
    id:u32,
    name:String,
}

#[derive(debug)]
struct Club{
    id:u32,
    name:String,
    members:Vec<Student>,
}

pub struct StudentManagementSystem{
    students: HashMap<u32,Student>,
    classes: HashMap<u32,Class>,
    courses: HashMap<u32,Course>,
    clubs:HashMap<u32,Club>,
}

impl StudentManagementSystem{
    pub fn new() -> StudentManagementSystem{
        StudentManagementSystem{
            students: HashMap::new(),
            classes:HashMap::new(),
            courses:HashMap::new(),
            clubs:HashMap::new(),
        }
    }
    pub fn create_student(&mut self, id: u32, name: String){
        let student = Student{id,name};
        self.students.insert(id,student);
    }
    
}