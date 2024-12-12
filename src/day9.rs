use std::{collections::BTreeMap, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"2333133121414131402
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day9.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[derive(Debug, Clone)]
struct FileMap {
    files: BTreeMap<usize, (usize, usize)>, // start index => (end_index, file_id)
    free: BTreeMap<usize, usize>,           // start index => end_index,
                                            // end_index is inclusive, it's the last block in the thing
}

fn char_to_usize(c: char) -> usize {
    c.to_digit(10).unwrap().try_into().unwrap()
}

impl FileMap {
    fn from_dense(dense_map: &str) -> Self {
        let mut files = BTreeMap::new();
        let mut free = BTreeMap::new();
        let mut file_id = 0;
        let mut length_so_far = 0;
        for (i, c) in dense_map.chars().enumerate() {
            if i % 2 == 0 {
                files.insert(
                    length_so_far,
                    (length_so_far + char_to_usize(c) - 1, file_id),
                );
                file_id += 1;
            } else if c != '0' {
                free.insert(length_so_far, length_so_far + char_to_usize(c) - 1);
            }
            length_so_far += char_to_usize(c);
        }
        Self { files, free }
    }

    fn defrag_last_file(&mut self) {
        let (last_file_start, (last_file_end, last_file_id)) = self.files.pop_last().unwrap();
        self.free.insert(last_file_start, last_file_end);
        let mut remaining_len = last_file_end - last_file_start + 1;
        while remaining_len > 0 {
            let (start, end) = self.free.pop_first().unwrap();

            if end - start + 1 > remaining_len {
                self.files
                    .insert(start, (start + remaining_len - 1, last_file_id));
                self.free.insert(start + remaining_len, end);
                break;
            } else if end - start + 1 == remaining_len {
                self.files
                    .insert(start, (start + remaining_len - 1, last_file_id));
                break;
            } else {
                self.files.insert(start, (end, last_file_id));
                remaining_len -= end - start + 1;
            }
        }
    }

    fn first_free_block(&self) -> usize {
        self.free.first_key_value().map(|(_, x)| *x).unwrap()
    }

    fn last_file_block(&self) -> usize {
        self.files.last_key_value().map(|(_, (x, _))| *x).unwrap()
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (&start, &(end, file_id)) in self.files.iter() {
            for j in start..=end {
                sum += j * file_id;
            }
        }
        sum
    }

    fn defrag_all_pt_2(&mut self) {
        let (mut working_file_start, mut working_file_id) = self
            .files
            .last_key_value()
            .map(|(start, (_, id))| (*start, *id))
            .unwrap();

        loop {
            let (&file_start, &(file_end, file_id)) = self
                .files
                .range(0..=working_file_start)
                .rev()
                .find(|(_, (_, id))| *id <= working_file_id)
                .unwrap();

            if file_id == 0 {
                break;
            } else {
                working_file_start = file_start - 1;
                working_file_id = file_id;
            }

            let file_len = file_end - file_start + 1;

            let Some((&free_start, &free_end)) = self
                .free
                .range(..=file_start)
                .find(|&(start, end)| (end - start + 1 >= file_len))
            else {
                println!("for file {file_id} no free space");
                continue;
            };

            println!("for file {file_id} found open free space from {free_start}..={free_end}");

            self.files.remove(&file_start);
            self.files
                .insert(free_start, (free_start + file_len - 1, file_id));
            self.free.remove(&free_start);
            if free_start + file_len - 1 != free_end {
                self.free.insert(free_start + file_len, free_end);
            }
        }
    }
}

pub fn part1() -> i64 {
    let input = read_input();
    // let input = EXAMPLE;
    // // trim trailing newline
    let input = input.trim();
    let mut map = FileMap::from_dense(input);
    //    println!("{map:?}");
    while map.first_free_block() < map.last_file_block() {
        map.defrag_last_file();
    }
    // println!("{map:?}");

    map.checksum().try_into().unwrap()
}

pub fn part2() -> i64 {
    let input = read_input();
    // let input = EXAMPLE;
    // // trim trailing newline
    let input = input.trim();
    let mut map = FileMap::from_dense(input);
    // println!("{map:?}");
    map.defrag_all_pt_2();
    // println!("{map:?}");

    map.checksum().try_into().unwrap()

    // wrong : 8532582152821
}

/*
          11111111112222222222333333333344
012345678901234567890123456789012345678901

00...111...2...333.44.5555.6666.777.888899
0099.111...2...333.44.5555.6666.777.8888..
0099.1117772...333.44.5555.6666.....8888..
0099.111777244.333....5555.6666.....8888..
00992111777.44.333....5555.6666.....8888..
*/
