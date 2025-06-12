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
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    match read_numbers_from_file("D:/RustProjects/AlgorithmTask2/data/data2.txt") {
        Ok(numbers) => {
            let mut arr = numbers;
            let start = Instant::now();
            insertion_sort(&mut arr);
            let duration = start.elapsed();
            println!("{:?}", arr);
            println!("耗时: {:?} 毫秒", duration.as_millis());
        }
        Err(e) => {
            eprintln!("读取文件出错: {}", e);
        }
    }
}