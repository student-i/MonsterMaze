use std::io::{stdin, stdout, Write};
use rand::Rng;


fn get_input_string(prompt: &str) -> String
{
    /*Get user input and return it as String 
    Input: None
    Output: User input as String */
    print!("{} ", prompt);
    stdout().flush().unwrap();
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}



fn key(inventory: &mut Vec<String>)
{
    //Function to pick up key
    println!("As you walk the path turns to the left.");
    println!("When you walk over you see a dead end.");
    println!("As you turn to go back you see a key and pick it up.");
    inventory.push("Key".to_string());

    println!("You walk back and go the other way.\n");
    weapon(inventory);

}

fn weapon(inventory: &mut Vec<String>)
{
    //Function to pick up weapon and make next choice

    println!("After walking for a while you see another fork in the path.");
    let mut correct: bool = false;
    let mut input: String = String::new();
    while correct == false{
        input = get_input_string("Do you go FORWARD or RIGHT? ").to_lowercase();
        if input.trim() == "forward" || input.trim() == "right" {
            correct = true;
        }
    }
    if input.trim() == "forward" {
        println!("The path turns to the right.");
        print!("After the turn there is a dead end with a sword.");
        println!("You pick it up and walk back. The path turns to the right.\n");
        inventory.push("Sword".to_string());
    }
    println!("You take the path to the right and see another fork in the path.");
    correct = false;
    while correct == false{
        input = get_input_string("Do you go FORWARD or RIGHT? ").to_lowercase();
        if input.trim() == "forward" || input.trim() == "right" {
            correct = true;
        }
    }
    if input.trim() == "forward" {
        monster_forword(inventory);
    }
    if input.trim() == "right" {
        monster_right1(inventory);
    }
}

fn monster_forword(inventory: &mut Vec<String>)
{

    //Function to fight snake

    println!("You walk forward and see a large snake.");

    //Generate random number
    let mut rng = rand::thread_rng();
    let potential: i32 = rng.gen_range(0..10);
 
    //Determine who wins
    if inventory.contains(&"Sword".to_string()) 
    {
        println!("You attack the snake with your sword.");
        if potential <= 2 {
            dead();
        }
        else {
            println!("YOU WIN!");
            gate(inventory);
        }
    }
    else {
        println!("You attack the snake with stick.");
        if potential <= 5 {
            dead();
        }
        else {
            println!("YOU WIN!");
            gate(inventory);
        }
    }
}

fn monster_right1(inventory: &mut Vec<String>)
{

    //Function to fight drat
    println!("\nYou walk right and see a monster that looks like it is a cross between a cat and a dragon.");
    
    //Generate random number
    let mut rng = rand::thread_rng();
    let potential: i32 = rng.gen_range(0..10);
    
    //Determine who wins
    if inventory.contains(&"Sword".to_string()) 
    {
        println!("You attack the drat with your sword.");
        if potential <= 3 {
            dead();
        }
        else 
        {
            println!("YOU WIN!");
            monster_right2(inventory);
        }
    }
    else {
        println!("You attack the cat drat with stick.");
        if potential <= 6 {
            dead();
        }
        else
        {
            println!("YOU WIN!");
            monster_right2(inventory);
        }
    }
}

fn monster_right2(inventory: &mut Vec<String>)
{

    //Function to fight dragon
    println!("\nAfter defeating the drat the path goes to the right");
    println!("and you run into a dragon.");

    //Generate random number
    let mut rng = rand::thread_rng();
    let potential: i32 = rng.gen_range(0..10);

    //Determine who wins
    if inventory.contains(&"Sword".to_string()) 
    {
        println!("You attack the drat with your sword.");
        if potential <= 4 {
            dead();
        }
        else 
        {
            println!("YOU WIN!");
            escape();
        }
    }
    else {
        println!("You attack the cat drat with stick.");
        if potential <= 7 {
            dead();
        }
        else 
        {
            println!("YOU WIN!");
            escape();
        }
    }
}

fn gate(inventory: &mut Vec<String>)
{

    //Function deal with gate
    println!("\nAs you round aother corner you see a gate.");
    if inventory.contains(&"Key".to_string()) {
        println!("As you aproach the gate you remember the key you picked up earlier.");
        println!("You try the key and it works.");
        println!("You walk through the gate and see that the path turns to the left.");
        escape();
    }
    else {
        print!("Ypu walk up to the gate and notice that it is locked.");
        println!("You try for a while but can't get through so you turn around and follow the other");
        println!("path. From your last fork.");
        monster_right1(inventory);
    }
}

fn escape()
{
    //Function to escape
    println!("\nYou finally see the way out. You win!");
}

fn dead()
{
    //Function to die
    println!("\nThe monster kills you.");
    println!("Game over.");
}

fn main() {

    let mut inventory: Vec<String> = Vec::new();
    inventory.push("Stick".to_string());

    //Welcome message
    println!("Welcome to the Monster Maze.");
    println!("Your goal is to escape the maze alive.");
    println!("You start out armed with a stick.");
    println!("Good luck!\n\n");
    println!("As you enter the maze you see two paths.");

    let mut correct: bool = false;
    let mut input: String = String::new();
    while correct == false 
    {
        input = get_input_string("Do you go LEFT or RIGHT? ").to_lowercase();
        if input.trim() == "left" || input.trim() == "right" {
            correct = true;
        }
    }
    if input.trim() == "left" {
        weapon(&mut inventory);
    }
    if input.trim() == "right" {
        key(&mut inventory);
    } 

}
