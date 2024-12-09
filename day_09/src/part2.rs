#[derive(Clone, Debug)]
struct Chunk {
    fileid: Option<usize>,
    index: usize,
    size: usize,
}

pub fn process(input: &str) -> usize {
    let mut chunks = vec![];
    let mut index = 0_usize;
    for (i, c) in input.trim().char_indices() {
        let size = c.to_digit(10).unwrap() as usize;
        let fileid = match i % 2 == 0 {
            true => Some(i / 2),
            false => None,
        };
        chunks.push(Chunk {
            fileid,
            index,
            size,
        });
        index += size;
    }

    let mut free_space: Vec<_> = chunks
        .clone()
        .into_iter()
        .filter(|chunk| chunk.fileid.is_none())
        .collect();
    let mut files: Vec<_> = chunks
        .into_iter()
        .filter(|chunk| chunk.fileid.is_some())
        .collect();

    for i in (0..files.len()).rev() {
        let file = &mut files[i];

        for j in 0..free_space.len() {
            let space = &mut free_space[j];

            if file.size <= space.size && space.index < file.index {
                file.index = space.index;

                if file.size == space.size {
                    free_space.remove(j);
                } else {
                    space.index += file.size;
                    space.size -= file.size;
                }
                break;
            }
        }
    }

    let checksum = files
        .iter()
        .map(|chunk| {
            (0..chunk.size).fold(0, |checksum, k| {
                checksum + (chunk.index + k) * (chunk.fileid.unwrap())
            })
        })
        .sum();

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!(2858, process(input));
    }
}
