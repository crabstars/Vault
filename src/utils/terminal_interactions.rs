use std::io::{stdout, Write, stdin};

use anyhow::Ok;

pub fn prompt_user(question: &str) -> String{
    print!("{}", question);
    let mut answer = String::new(); 
    let _ = stdout().flush();
    stdin().read_line(&mut answer).expect("This was not a correct string");
    if let Some('\n')=answer.chars().next_back() {
        answer.pop();
    }
    if let Some('\r')=answer.chars().next_back() {
        answer.pop();
    }
    answer
}

pub fn prompt_password() -> Result<String, anyhow::Error>{
    let mut password: String;
    loop {
        password = rpassword::prompt_password("Please enter the password:")?;
        if password != rpassword::prompt_password("Please re-enter the password:")?{
            println!("Your passwords didn't match, pls try again!")
        } else {
            return Ok(password)
        }
    }
}
