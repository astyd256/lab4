use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: f32,
    b: f32,
    c: f32
}
// Finding cooefficients of linear equation Ax+By-C=0
fn find_cooefficients(x1: f32, y1: f32, x2: f32, y2: f32) -> (f32, f32, f32){
    return(y2 - y1, 
        x1 - x2, 
        x2*y1 - x1*y2);
}
// Returns true and coordinates if intersection exists or false and (0;0) coordinates if it doesn't 
fn find_intersection(first_line: Line, second_line: Line) -> (f32, f32, bool){
    let system_det = (first_line.a*second_line.b - second_line.a*first_line.b).abs();
    if system_det < 0.0001 { return (0.0,0.0, false); }
    
    //После проверки на параллельность прямых ищем точку пересечения
    let intersection = Point{
        x: (first_line.c*second_line.b - second_line.c*first_line.b) / system_det,
        y: (first_line.a*second_line.c - second_line.a*first_line.c) / system_det,
    };
    return(intersection.x, intersection.y, true);
}
// Methdo for reading files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
//Checking if a dot (x,y) is between a line (x1, y1, x2, y2) 
fn is_between(x: f32, y:f32, x1: f32, y1: f32, x2: f32, y2: f32) -> bool{
    
    let cross = (x - x1) * (y - y1) - (x2 - x1) * (y2 - y1); //cheking if dot is on the line
    if cross.abs() > 0.01 {return false};
    
    if f32::abs(x2 - x1) >= f32::abs(y2 - y1) {
        if (x2 - x1) > 0.0 {return x1 <= x && x <= x2} else {return x2 <= x && x <= x1}
    } else {
        if (y2 - y1) > 0.0 {return y1 <= y && y <= y2} else {return y2 <= y && y <= y1}
    }
}

fn main() {
    let mut file_lines = Vec::new();

    // read the file
    if let Ok(lines) = read_lines("./text.txt") {
        // read each line from file
        for line in lines {
            if let Ok(str) = line { file_lines.push(str);}      
        }   
    }

    let mut points_array = Vec::new();

    // reading lines from file
    for i in 0..file_lines.len(){
        //Transform string
        file_lines[i]= file_lines[i].replacen(" ", ",", 1).to_string();
        let string_value: Vec<&str> = file_lines[i].split_terminator(',').collect(); // seperate elements by comma
        for j in 0..4{
            let number: f32 = string_value[j].trim().parse().unwrap(); // parse string to float
            points_array.push(number);
        }
    }

    let mut main_line = Line{ 
        a: 0.0,
        b: 0.0,
        c: 0.0
    };

    (main_line.a, main_line.b, main_line.c) = find_cooefficients(points_array[0], points_array[1], 
        points_array[2], points_array[3]);
    println!("Main line have have coordinates ({},{}) и ({},{})",points_array[0], points_array[1], points_array[2], points_array[3]);
    
    let mut segment_line = Line{
        a: 0.0,
        b: 0.0,
        c: 0.0
    };

    for i in 1..file_lines.len() {
       
        (segment_line.a, segment_line.b, segment_line.c) = find_cooefficients(points_array[i * 4], points_array[i * 4 + 1], 
            points_array[i * 4 + 2], points_array[i * 4 + 3]);
        let intersect_answer = find_intersection(main_line, segment_line); 
        println!("intersect_answer: {} {} {}", intersect_answer.0, intersect_answer.1, intersect_answer.2);
        if intersect_answer.2 && is_between(intersect_answer.0, intersect_answer.1, 
            points_array[i * 4], points_array[i * 4 + 1], 
            points_array[i * 4 + 2], points_array[i * 4 + 3]) {
                println!("{} line ({},{}) и ({},{}). Intersection point is ({},{})!",i, points_array[i * 4], points_array[i * 4 + 1], 
                points_array[i * 4 + 2], points_array[i * 4 + 3], intersect_answer.0, intersect_answer.1 );
            }
        else {
            println!("{} line have coordinates ({},{}) and ({},{}). Doesn't have an intersection point.", i, points_array[i * 4], points_array[i * 4 + 1], 
            points_array[i * 4 + 2], points_array[i * 4 + 3]);
        }
    } 
}