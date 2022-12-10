use competition_io::{input_new, Input};

use Node::*;

#[derive(Debug)]
struct Directory {
    name : String,
    children : Vec<Node>,

}

impl Directory {
    fn new(name : String) -> Self {
        Directory{name, children : Vec::new()}

    }
}

#[derive(Debug)]
enum Node {
    Directory(Directory),
    File(String, u64)
}

impl Node {
    fn new_directory(name : String) -> Node {
        Directory(Directory{name, children : Vec::new()})
    }

    fn new_file(name : String, size : u64) -> Node {
        File(name, size)
    }
}


fn main() {
    let mut inp = input_new();

    let mut root = Directory::new("/".to_owned());

    handle_dir(&mut root, &mut inp);

    let mut dir_sizes = Vec::new();

    let tot_size = find_dir_sizes(&mut root, &mut dir_sizes);   

    let to_free = tot_size - 40_000_000;

    dir_sizes.sort();

    // dbg!(to_free);
    // println!("{dir_sizes:?}");
    let i = dir_sizes.partition_point(|x| *x< to_free);
    let to_delete = dir_sizes[i];

    println!("The size of the deleted directory is {to_delete}");
}

enum CDDestination<'a> {
    Child(&'a mut Directory),
    Parent,
    Root,
}

fn cd<'a, I>(directory : &'a mut Directory, inp : &mut Input<I>) -> CDDestination<'a> where I : Iterator<Item = char> {
    let name : String = inp.next();

    if name == "/" {
        return CDDestination::Root;
    }
    if name == ".." {
        return CDDestination::Parent;
    }
    
    let mut child_iter = directory.children.iter_mut();
    let child = loop {
        let Directory(child) = child_iter.next().unwrap() else {continue};
        if name == child.name {
            break child;
        }
    };

    return CDDestination::Child(child);
}

fn ls<I>(directory : &mut Directory, inp : &mut Input<I>) where I : Iterator<Item = char> {
    'ls : loop {
        let style : String = inp.next();

        if style == "$" {
            return;
        }

        let name : String = inp.next();

        for child in directory.children.iter() {
            let child_name = match child {
                Directory(dir) => &dir.name,
                File(name, _) => name
            };

            if child_name == &name {
                continue 'ls;
            }
        }

        if style == "dir" {
            directory.children.push(Node::new_directory(name));
        }

        else {
            let size : u64 = style.parse().unwrap();

            directory.children.push(Node::new_file(name, size))
        }
    }

}

enum HandleDirReturn {
    Parent,
    Root,
    Quit,
}

fn handle_dir<I>(directory : &mut Directory, inp : &mut Input<I>) -> HandleDirReturn where I : Iterator<Item = char> {
    loop {
        let cmd : String = inp.next();
        if cmd == "$" {
            continue
        }

        match cmd.as_ref() {
            "cd" => {
                let child = match cd(directory, inp) {
                    CDDestination::Child(child) => child,
                    CDDestination::Parent => return HandleDirReturn::Parent,
                    CDDestination::Root => 
                        if directory.name != "/" {
                            return HandleDirReturn::Root;
                        }
                        else {
                            continue
                        }
                };

                match handle_dir(child, inp) {
                    HandleDirReturn::Parent => continue,
                    HandleDirReturn::Root => 
                        if directory.name != "/" {
                            return HandleDirReturn::Root
                        }
                        else {
                            continue
                        },
                    HandleDirReturn::Quit => return HandleDirReturn::Quit,
                }
            }
            "ls" => {
                ls(directory, inp);
            }
            "q" => {
                return HandleDirReturn::Quit;
            }
            _ => unreachable!()
        }
    }
}

fn find_dir_sizes(directory : &Directory, dir_sizes : &mut Vec<u64>) -> u64{
    // dbg!(&directory);
    let mut dir_sum = 0;
    for child in directory.children.iter() {
        match child {
            File(_, size) => dir_sum += size,
            Directory(child) => {
                dir_sum += find_dir_sizes(child, dir_sizes);
            }
        }
        
    }

    dir_sizes.push(dir_sum);

    dir_sum
    
}
