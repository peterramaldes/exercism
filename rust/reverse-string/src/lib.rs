pub fn reverse(input: &str) -> String {
    if input.len() == 0 { return String::from("") }
    let mut result = String::new(); 
    for i in (0..input.chars().count()).rev() {
       let letter = input.chars().nth(i).expect("retrieve letter from index");
       result.push(letter);
    }
    result
}
