use std::collections::HashSet;

struct Data<'a> {
    matrix: Vec<&'a[u8]>
}

impl Data<'_> {
    fn get(&self, p: (usize, usize)) -> u8 {
        self.matrix[p.0][p.1] - '0' as u8
    }

    fn adjacent(&self, p: (usize, usize)) -> Vec<(usize,usize)> {
        let mut a = Vec::new();
        if p.0 > 0 { a.push((p.0 - 1,p.1)); }
        if p.1 > 0 { a.push((p.0, p.1 - 1))}
        if p.0 < self.rows() - 1 { a.push((p.0 + 1, p.1))}
        if p.1 < self.cols() - 1 { a.push((p.0, p.1 + 1))}
        return a;
    }

    fn get_basins(&self) -> Vec<(usize, usize)> {
        let mut basins = Vec::new();
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let curr = self.get((r,c));
                let high_count = self.adjacent((r,c)).iter().filter(|p| self.get(**p) <= curr).count();
                if high_count == 0 {
                    basins.push((r,c));
                }
            }
        }
        return basins;
    }

    fn get_basin_size(&self, p: (usize, usize), seen: &mut HashSet<(usize, usize)>) -> usize {
        if seen.contains(&p) || self.get(p) == 9 {
            return 0;
        }

        seen.insert(p);

        let mut size = 1;
        for a in self.adjacent(p) {
            size += self.get_basin_size(a, seen);
        }
        return size;
    }

    fn rows(&self) -> usize {
        self.matrix.len()
    }

    fn cols(&self) -> usize {
        self.matrix[0].len()
    }
}

fn parse(input: &str) -> Data {
    let byte_matrix: Vec<&[u8]> = input
        .trim()
        .split_whitespace()
        .map(|row| row.as_bytes())
        .collect();

    Data { matrix: byte_matrix }
}

fn part_1(data: &Data) -> usize {
    return data.get_basins().iter().map(|b| data.get(*b) as usize + 1).sum();
}

fn part_2(data: &Data) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut sizes: Vec<usize> = data.get_basins().iter().map(|p| {
        data.get_basin_size(*p, &mut seen)
    }).collect();

    sizes.sort();
    
    return sizes.iter().rev().take(3).product();
}

fn main() {
    const DATA: &str = include_str!("../input.txt");
    let data = parse(DATA);
    
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("../test.txt");
    
    #[test]
    fn test_1() {
        let sample_data = parse(SAMPLE_DATA);
        assert_eq!(part_1(&sample_data), 15);
    }

    #[test]
    fn test_2() {
        let sample_data = parse(SAMPLE_DATA);
        assert_eq!(part_2(&sample_data), 1134);
    }
}
