use std::str::Lines;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<FileSystem>,
    size: usize,
}

#[derive(Debug)]
enum FileSystem {
    File(File),
    Directory(Directory),
}

fn get_dir_contents(dir_name: &str, input: &mut Lines) -> FileSystem {
    let mut dir = Directory { name: dir_name.to_string(), children: vec!(), size: 0 };

    while let Some(line) = input.next() {
        if line == "$ cd .." {
            break;
        }

        if line.starts_with("$ cd ") {
            let child = get_dir_contents(line.split_at(5).1, input);
            dir.children.push(child);

            if let Some(FileSystem::Directory(d)) = dir.children.last() {
                dir.size += d.size;
            }
        }
        else if !line.starts_with("dir ") && !line.starts_with("$ ls") {
            let parts: Vec<&str> = line.split(' ').collect();
            let f = File {
                size: parts[0].parse().unwrap_or_default(),
                name: parts[1].to_string(),
            };
            dir.size += f.size;
            dir.children.push(FileSystem::File(f));
        }
    }

    FileSystem::Directory(dir)
}

fn display_fs(indent: usize, fs: &FileSystem, max_size: usize) -> Vec<(String, usize)> {
    //let mut total = 0;
    let mut dirs = vec!();

    match fs {
        FileSystem::File(f) => { 
            /*if f.size <= max_size {
                println!("{}{} {} bytes", " ".repeat(indent), f.name, f.size); 
            }*/
        },
        FileSystem::Directory(d) => {
            println!("{}.{}/ {} bytes", " ".repeat(indent), d.name, d.size);

            if d.size >= max_size {
                //total = d.size;
                dirs.push((d.name.clone(), d.size));
            }

            for child in d.children.iter() {
                //total += display_fs(indent+2, child, max_size);
                dirs.append(&mut display_fs(indent+2, child, max_size));
            }
        },
    }

    //total
    dirs
}

fn main() {
    let input = std::fs::read_to_string("day_7 input.txt").unwrap();
    let mut itr = input.lines();
    
    itr.next(); // enter root folder

    let total_space = 70000000;
    let space_used = 44125990;
    let space_needed = 30000000;
    let free_space = total_space - space_used;
    let amount_to_delete = space_needed - free_space;

    println!("{} - {} = {}", total_space, space_used, free_space);
    println!("{} - {} = {}", space_needed, free_space, amount_to_delete);

    let fs = get_dir_contents("/", &mut itr);
    let mut dirs = display_fs(0, &fs, amount_to_delete);
    dirs.sort_by(|a, b| a.1.cmp(&b.1));

    for dir in dirs {
        println!("{:?}", dir);
    }
}