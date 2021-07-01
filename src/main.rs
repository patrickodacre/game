use std::io::stdin;

#[derive(Debug)]
enum VisitorAction
{
    Accept,
    AcceptWithNote
    {
        note: String,
    },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor
{
    name: String,
    greeting: String,
    action: VisitorAction,
    age: u8,
}

impl Visitor
{
    fn new(name: &str, greeting: &str, action: VisitorAction, age: u8) -> Self
    {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet(&self) -> &String
    {
        &self.greeting
    }
}

fn get_name() -> String
{
    println!("What's your name?");

    let mut name = String::new();

    stdin().read_line(&mut name).expect("bad input");

    name.trim().to_string()
}
fn main()
{
    let mut visitor_list = vec![
        Visitor::new("Evan", "Hey, Evan!", VisitorAction::Accept, 14),
        Visitor::new("Christian", "Hey, Christian!", VisitorAction::Refuse, 7),
        Visitor::new("Daniel", "Hey, Daniel!", VisitorAction::Refuse, 5),
    ];

    loop {
        let name = get_name().to_lowercase();

        if name.is_empty() {
            println!("Updated list: {:?}\n", visitor_list);
            println!("Bye!");
            break;
        }

        let known_visitor = visitor_list
            .iter()
            .find(|v| v.name.to_lowercase().contains(name.as_str()));

        match known_visitor {
            Some(v) => println!("{}\n", v.greet()),
            _ => {
                println!("You're not on the list!\n");
                visitor_list.push(Visitor::new(
                    name.as_str(),
                    format!("Hey {}!", name).as_str(),
                    VisitorAction::AcceptWithNote {
                        note: "Some note".to_string(),
                    },
                    0,
                ));
            }
        }
    }
}
