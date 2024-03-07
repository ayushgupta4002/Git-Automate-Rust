use std::process::{Command,exit};
use std::io;

fn automate(){

    let commit_message = commit_msg();

    let add_command = Command::new("git")
    .arg("add")
    .arg("-A")
    .output()
    .expect("add command could not be executed");

    if !add_command.status.success(){
        eprintln!("add command failed !");
        exit(1);
    }

    let commit_command =  Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(commit_message.clone()).output().expect("commit command could not be executed");

    if !commit_command.status.success(){
        eprintln!("Commit command failed !");
        exit(1);
    }

    let push_command = Command::new("git")
    .arg("push")
    .arg("origin")
    .arg("master")
    .output().expect("push command could not be executed");

    if !push_command.status.success(){
        eprintln!("push command could not be executed");
        exit(1);
    }

    println!("Successfully pushed all code with Commit message : {} " ,&commit_message);

}

fn commit_msg() -> String {
    let mut commit_message = String::new();
    println!("please enter your commit message:");
    io::stdin().read_line(&mut commit_message).expect("sorry operation could not be performed");
    commit_message
}

// fn branch_name() -> String {
//     let mut branch = String::new();
//     println!("please enter your branch name:");
//     io::stdin().read_line(&mut branch).expect("sorry operation could not be performed");
//     branch
// }

fn main() {
    println!("W E L C O M E!");
    automate();
}
