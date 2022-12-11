use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
    directories: Vec<Rc<RefCell<Directory>>>,
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    size: usize,
}

impl Directory {
    pub fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Directory {
            directories: vec![],
            name,
            parent,
            size: 0,
        }
    }
}

enum Compare {
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo
}

enum Type {
    Command,
    Directory,
    File,
}

pub fn get_first_solution() {
    let output = crate::read_file("src/input/day7.txt");
    let file_system = populate_file_system(&output);

    println!("Total size: {}", get_directories(&file_system, 100000, &Compare::LessThanOrEqualTo).iter().sum::<usize>());
}

pub fn get_second_solution() {
    let output = crate::read_file("src/input/day7.txt");
    let file_system = populate_file_system(&output);
    let max_size = 70000000;
    let free_space_needed = 30000000;

    let mut directories = get_directories(&file_system, max_size, &Compare::LessThan);
    let free_space = max_size - directories[directories.len() - 1];
    let space_to_clear = free_space_needed - free_space;
    directories = get_directories(&file_system, space_to_clear, &Compare::GreaterThanOrEqualTo);

    println!("Total size: {}", directories[0]);
}

fn populate_file_system(output: &str) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new(String::from("/"), None)));
    let mut current = Rc::clone(&root);

    for line in output.lines() {
        let temp = Rc::clone(&current);
        match get_line_type(line) {
            Type::Command => {
                if line.contains("cd") {
                    if line.contains("..") {
                        if let Some(dir) = &temp.borrow().parent {
                            current = Rc::clone(dir);
                        } else {
                            println!("not found {:?}", current.borrow().name);
                        }
                    } else if line.contains('/') {
                    } else if let Some(i) = temp
                        .borrow()
                        .directories
                        .iter()
                        .position(|d| d.borrow().name == line[4..].trim())
                    {
                        current = Rc::clone(&temp.borrow().directories[i]);
                    }
                }
            }
            Type::Directory => add_directory(&current, line[4..].trim().to_string()),
            Type::File => {
                if let Some(size) = line.split(' ').next() {
                    if let Ok(s) = size.parse::<usize>() {
                        add_file(&current, s);
                    }
                }
            }
        }
    }

    root
}

fn get_line_type(line: &str) -> Type {
    if line.starts_with('$') {
        return Type::Command;
    }

    if line.starts_with("dir") {
        return Type::Directory;
    }

    Type::File
}

fn add_directory(parent: &Rc<RefCell<Directory>>, dir_name: String) {
    let current = Rc::clone(parent);
    let mut current_mut = current.borrow_mut();

    if !current_mut
        .directories
        .iter()
        .any(|d| d.borrow().name == dir_name)
    {
        let new_dir = Rc::new(RefCell::new(Directory::new(
            dir_name,
            Some(Rc::clone(parent)),
        )));
        current_mut.directories.push(Rc::clone(&new_dir));
    };
}

fn add_file(initial: &Rc<RefCell<Directory>>, file_size: usize) {
    let current = Rc::clone(initial);

    if let Some(p) = &current.borrow().parent {
        add_file(p, file_size);
    }

    current.borrow_mut().size += file_size;
}

fn get_directories(directory: &Rc<RefCell<Directory>>, max_size: usize, compare: &Compare) -> Vec<usize> {
    let current = Rc::clone(directory);
    let borrowed = current.borrow();
    let mut directories = Vec::new();

    for dir in &borrowed.directories.clone() {
        directories.append(&mut get_directories(dir, max_size, compare));
    }

    let should_add = match compare {
        Compare::GreaterThan => borrowed.size > max_size,
        Compare::GreaterThanOrEqualTo => borrowed.size >= max_size,
        Compare::LessThan => borrowed.size < max_size,
        Compare::LessThanOrEqualTo => borrowed.size <= max_size
    };

    if should_add {
        directories.push(borrowed.size);
    }

    directories.sort();
    directories
}
