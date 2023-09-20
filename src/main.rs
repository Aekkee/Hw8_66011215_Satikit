use csv::{ReaderBuilder, StringRecord, Trim, Writer}; 
use std::fs::File; 
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::fs::FileTypeExt;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32 
}

#[derive(Debug)]
struct PolarPoint {
    r: f32,
    z: f32 
}

fn main() {
    let data_file = File::open("books.csv").unwrap();
    println!("{:?}", load_points_to_carte(data_file));
    println!("{:?}", to_polar(vec![(Point{x: 3., y: 4.})]));
    println!("{:?}", to_cartesian(to_polar(vec![(Point{x: 3., y: 4.})])));
    car_to_po("books.csv", "new.csv");
    car_to_po_html("books.csv", "newpolar.html");
    po_to_car_html("new.csv", "newcarte.html");
}


fn to_polar( input: Vec<Point>) -> Vec<PolarPoint> {
    if input.is_empty() {
    return Vec::new()
    }
    let mut result: Vec<PolarPoint> = Vec::new();
    for i in input {
        let r = (i.x.powf(2.)+i.y.powf(2.)).sqrt();
        let z = (i.y/i.x).atan();
        result.push(PolarPoint{r, z});
    }
    result
}

fn to_cartesian( input: Vec<PolarPoint>) -> Vec<Point> {
    if input.is_empty() {
    return Vec::new()
    }
    let mut result: Vec<Point> = Vec::new();
    for i in input {
        result.push(Point{ x: i.r*i.z.cos(), y: i.r*i.z.sin()});
    }
    result
}

fn load_points_to_carte(reader: impl Read) -> Vec<Point> {
    let mut point_list: Vec<Point> = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(reader);
        rdr.records().next();
        for result in rdr.records() {
            let record = result.unwrap();
            if record.len() < 2 {
                continue;
            }
            let x: f32 = record[0].parse().unwrap_or(0.0);
            let y: f32 = record[1].parse().unwrap_or(0.0);
            point_list.push(Point{x,y});
        }
    return point_list;
}

fn load_points_to_polar(reader: impl Read) -> Vec<PolarPoint> {
    let mut point_list: Vec<PolarPoint> = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(reader);
        rdr.records().next();
        for result in rdr.records() {
            let record = result.unwrap();
            let r: f32 = record[0].parse().unwrap_or(0.0);
            let z: f32 = record[1].parse().unwrap_or(0.0);
            point_list.push(PolarPoint{r, z});
        }
    return point_list;
}

fn save_points_polar(pt_list: Vec<PolarPoint>, output_file: &str) {
    let file_path = std::path::Path::new(output_file);
    let mut wtr = Writer::from_path(file_path).unwrap();
    let _ = wtr.write_record(&["R","Zeta"]);
    for i in pt_list {
        let _ = wtr.write_record(&[i.r.to_string(), i.z.to_string()]);
    }
}
#[allow(unused_must_use)]
fn save_points_polar_html(pt_list: Vec<PolarPoint>, output_file: &str) {
    let mut file = File::create(output_file).expect("Failed");
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>R</th>\n\t\t<th>Zeta</th>\n\t</tr>");
    for i in pt_list {
        file.write(
                    format!("\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                    i.r.to_string(),
                    i.z.to_string()
                    ).as_bytes()
                );
            }
            file.write(b"\n</table>");
}

#[allow(unused_must_use)]
fn save_points_carte_html(pt_list: Vec<Point>, output_file: &str) {
    let mut file = File::create(output_file).expect("Failed");
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>X</th>\n\t\t<th>Y</th>\n\t</tr>");
    for i in pt_list {
        file.write(
                    format!("\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                    i.x.to_string(),
                    i.y.to_string()
                    ).as_bytes()
                );
            }
            file.write(b"\n</table>");
}

fn save_points_carte(pt_list: Vec<Point>, output_file: &str) {
    let file_path = std::path::Path::new(output_file);
    let mut wtr = Writer::from_path(file_path).unwrap();
    let _ = wtr.write_record(&["X","Y"]);
    for i in pt_list {
        let _ = wtr.write_record(&[i.x.to_string(), i.y.to_string()]);
    }
}

fn car_to_po(input_file: &str, output_file: &str) {
    let data_file = File::open(input_file).unwrap();
    save_points_polar(to_polar(load_points_to_carte(data_file)),output_file)
}

fn po_to_car(input_file: &str, output_file: &str) {
    let data_file = File::open(input_file).unwrap();
    save_points_carte(to_cartesian(load_points_to_polar(data_file)),output_file)
}

fn car_to_po_html(input_file: &str, output_file: &str) {
    let data_file = File::open(input_file).unwrap();
    save_points_polar_html(to_polar(load_points_to_carte(data_file)),output_file)
}

fn po_to_car_html(input_file: &str, output_file: &str) {
    let data_file = File::open(input_file).unwrap();
    save_points_carte_html(to_cartesian(load_points_to_polar(data_file)),output_file)
}