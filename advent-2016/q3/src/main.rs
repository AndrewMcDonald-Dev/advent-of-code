use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let part = 2;

    let file = File::open("src/input.txt").expect("File not found.");
    let reader = BufReader::new(file);

    if part == 1 {
        println!("{}", count_triangles(reader));
    } else if part == 2 {
        println!("{}", count_triangles_transposed(reader));
    }
}

fn good_triangle(tri: &mut [i32; 3]) -> bool {
    tri.sort();
    tri[0] + tri[1] > tri[2]
}

fn count_triangles(reader: BufReader<File>) -> u32 {
    //grab file and read it line by line

    let mut count = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let line_strs = line.split_whitespace().collect::<Vec<&str>>();
        if line_strs.len() != 3 {
            panic!("Line does not have three lengths")
        }

        let mut tri: [i32; 3] = [0; 3];
        for i in 0..3 {
            tri[i] = line_strs[i].parse().expect("tri has non-int");
        }

        //Add to count
        if good_triangle(&mut tri) {
            count += 1;
        }
    }

    count
}

fn count_triangles_transposed(reader: BufReader<File>) -> u32 {
    let mut count = 0;
    let mut index = 0;
    let mut buffer = [[0i32; 3]; 3];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let line_strs = line.split_whitespace().collect::<Vec<&str>>();
        if line_strs.len() != 3 {
            panic!("line does not contain 3 sides");
        }

        for i in 0..3 {
            buffer[i][index] = line_strs[i].parse().expect("tri has non-int")
        }

        // Process a group if one is ready
        index += 1;

        if index == 3 {
            for b in &mut buffer {
                if good_triangle(b) {
                    count += 1;
                }
            }
            index = 0;
        }
    }

    if index != 0 {
        panic!("uneven number of lines in input");
    }

    count
}
