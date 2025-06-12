use rand::Rng;
use std::fs;
use std::io;
use std::time::Instant;

fn read_numbers_from_file(filename: &str) -> io::Result<Vec<i32>> {
    let content = fs::read_to_string(filename)?;
    
    let mut lines = content.lines();
    
    let n: usize = lines.next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "文件为空"))?
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    let numbers_line = lines.next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "缺少数据行"))?;
    
    let numbers: Vec<i32> = numbers_line
        .split_whitespace()
        .map(|s| s.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
        .collect::<Result<_, _>>()?;
    
    if numbers.len() != n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("数据数量不匹配: 预期 {}, 实际 {}", n, numbers.len())
        ));
    }
    
    Ok(numbers)
}
fn random_quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    random_quicksort(&mut arr[..pivot_index]);
    random_quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let mut rng = rand::rng();
    let pivot_index = rng.random_range(0..arr.len());
    arr.swap(pivot_index, arr.len() - 1);
    
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    match read_numbers_from_file("D:/RustProjects/AlgorithmTask2/data/data2.txt") {
        Ok(numbers) => {
            let mut arr = numbers;
            let start = Instant::now();
            random_quicksort(&mut arr);
            let duration = start.elapsed();
            println!("{:?}", arr);
            println!("耗时: {:?} 毫秒", duration.as_millis());
        }
        Err(e) => {
            eprintln!("读取文件出错: {}", e);
        }
    }
}