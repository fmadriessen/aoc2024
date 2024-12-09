pub fn process(input: &str) -> usize {
    let mut files: Vec<Option<usize>> = input
        .trim()
        .char_indices()
        .flat_map(|(index, c)| {
            let is_file = index % 2 == 0;
            let n = c.to_digit(10).unwrap() as usize;
            match is_file {
                true => {
                    let file_id = index / 2;
                    vec![Some(file_id); n]
                }
                false => vec![None; n],
            }
        })
        .collect();

    let mut high_index = files.len() - 1;
    for (index, file) in files.clone().iter().enumerate() {
        if index >= high_index {
            break;
        }

        match file {
            Some(_) => continue,
            None => {
                // Always need to swap, just don't always know which block with
                while files[high_index].is_none() {
                    high_index -= 1;
                }
                files.swap(index, high_index);
            }
        }
    }

    let checksum = files.iter().filter(|file| file.is_some()).enumerate().fold(
        0,
        |checksum, (index, file)| {
            let file_id = file.unwrap();
            checksum + index * file_id
        },
    );

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!(1928, process(input));
    }
}
