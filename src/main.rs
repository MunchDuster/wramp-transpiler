use std::fs;
use std::env;

fn main() {
    println!("This is the unofficial WRAMP Transpiler.");

    // get file name
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];


    let path = env::current_dir().unwrap();
    
    let output_file = path.join(format!("{file_name}.s"));
    let input_file = path.join(format!("{file_name}.sname"));

    println!("{:#?}", path.display());

    let i2 = input_file.clone();

    println!("Looking for file: {}", input_file.display());
    let text: String = match fs::read_to_string(input_file) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file \"{}\": {}", i2.display(), error),
    };

    let mut lines: Vec<&str> = text.split("\n").collect();
    let mut variables: Vec<&str> = Vec::new();
    let mut variable_registers: Vec<usize> = Vec::new();
    let mut variable_declaration_line_indices: Vec<usize> = Vec::new();

    // find variable declarations
    let variable_declaration = "register";
    for (index, line) in lines.iter().enumerate() {
        if line.contains(variable_declaration) { // check if this contains a variable

            let stuff_after_keyword: Vec<&str> = line.split(variable_declaration).collect::<Vec<&str>>()[1].trim().split(" ").collect();
            
            // add to variables vector
            variables.push(stuff_after_keyword[0].trim());

            // keep track of what register this variable is for
            variable_registers.push(stuff_after_keyword[1].trim().parse::<usize>().unwrap());

            // add line index to delete it later
            variable_declaration_line_indices.push(index);
        }
    }
    println!("Found variables: {}", variables.join(","));
    
    // delete variable declarations
    for index in variable_declaration_line_indices.iter().rev() {
        lines.remove(*index);
    }
    
    // combine lines back into single text
    let mut contents: String = lines.join("\n");
    
    // replace each use of each variable with a register
    for (index, variable) in variables.iter().enumerate() {
        if contents.contains(variable) { // check if this contains a variable
            let register = format!("${}", variable_registers[index]);
            contents = contents.replace(*variable, &register);
        }
    }

    // save to file
    match fs::write(output_file, contents) {
        Ok(_message) => println!("Write to file success") ,
        Err(error) => println!("Write to file FAIL: {}", error)
    }
}