use std::{io, collections::HashMap};

fn main() {
    let mut company_users_map: HashMap<String, Vec<String>> = HashMap::new();

    println!("Please input department name: ");

    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Error reading department");
    department = department.trim().to_string();
    
    println!("Please input staff username: ");
    let mut staff_name = String::new();
    io::stdin().read_line(&mut staff_name).expect("Error reading staff name");
    staff_name = staff_name.trim().to_string();
    
    let v: Vec<String> = Vec::new();
    let staff_vec = company_users_map.entry(department).or_insert(v);
    staff_vec.push(staff_name);

    println!("{:?}", company_users_map);
    println!("Please input department name to list employees: ");
    let mut list_department = String::new();
    io::stdin().read_line(&mut list_department).expect("Error reading list department");
    list_department = list_department.trim().to_string();

    let list = company_users_map.get(&list_department);
    match list {
        Some(list) => println!("department {} employees is {:?}", list_department, list),
        None => println!("{} department not exist", list_department),
    }
}
