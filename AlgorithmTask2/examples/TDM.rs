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

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    
    merge_sort(&mut left);
    merge_sort(&mut right);
    
    merge(arr, &left, &right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn main() {

    match read_numbers_from_file("D:/RustProjects/AlgorithmTask2/data/data1.txt") {
        Ok(numbers) => {
            let mut arr = numbers;
            let start = Instant::now();
            merge_sort(&mut arr);
            let duration = start.elapsed();
            println!("{:?}", arr);
            println!("耗时: {:?} 毫秒", duration.as_millis());
        }
        Err(e) => {
            eprintln!("读取文件出错: {}", e);
        }
    }
}