use std::collections::{HashSet, HashMap};

struct Trait{
    members: HashSet<usize>
}

struct Expert{
    traits: HashMap<String, Trait>,
    entities: HashMap<usize, String>,
    //trait to question map
    questions: HashMap<String, String>
}

enum Answer{
    Yes,
    No
}

impl Expert{
    fn new() -> Self{
        Expert{
            traits: HashMap::new(),
            entities: HashMap::new(),
            questions: HashMap::new()
        }
    }

    fn add_entity(&mut self, name: &str, traits: &Vec<&str>) -> usize{
        let mut id = 0;
        let mut done = false;
        while !done{
            if self.entities.contains_key(&id){
                id += 1;
                continue;
            } else{
                self.entities.insert(id, name.to_string());
                done = true;
                continue;
            }
        }
        for tr in traits{
            let mut tr = match self.traits.get_mut(&tr.to_string()){
                Some(v) => v,
                None => {
                    panic!("Entity {} has non-existent trait {}", name, tr);
                }
            };
            tr.members.insert(id);
        }
        id
    }

    fn add_trait(&mut self, tr: &str, question: &str){
        self.questions.insert(tr.to_string(), question.to_string());
        self.traits.insert(tr.to_string(), Trait{members: HashSet::new()});
    }

    fn start_guessing_session(&self){
        //at first all entities are in the guess
        let mut guess: HashSet<usize> = self.entities.iter().map(|(k,_)|{k.clone()}).collect();
        let mut confirmed_traits = HashSet::<String>::new(); //there are no confirmed traits
        //all traits are questioned
        let mut questioned_traits: HashSet<String> = self.traits.iter().map(|(tr,_)|{tr.clone()}).collect();
        
        while guess.len() > 1 && questioned_traits.len() > 0{
            let question = self.choose_question(&guess, &questioned_traits);
            questioned_traits.remove(&question);
            println!("{} (y)es/(n)o", *self.questions.get(&question).unwrap());
            let mut done = false;
            let mut ans = Answer::No;
            while !done{
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                match input.trim() {
                    "y" => {
                        ans = Answer::Yes;
                        done = true;},
                    "n" =>{
                        ans = Answer::No;
                        done = true;},
                    _ => {println!("Incorrect input")}
                }
            }
            self.narrow_guess(&ans, &question, &mut guess);
        }

        println!("Guess: {}", self.entities.get(guess.iter().next().unwrap()).unwrap());
    }

    fn narrow_guess(&self, answer: &Answer, tr: &String, guess: &mut HashSet<usize>){
        let mut remove = HashSet::new();
        let tr = self.traits.get(tr).unwrap();
        for entity in guess.iter(){
            let presence = tr.members.get(entity);
            match answer{
                Answer::Yes => {
                    if presence.is_none(){
                        remove.insert(*entity);}
                },
                Answer::No =>{
                    if presence.is_some(){
                        remove.insert(*entity);}
                }
            }
        }
        *guess = guess.difference(&remove).map(|ent|{ent.clone()}).collect();
    }

    fn choose_question(&self, guess: &HashSet<usize>, questioned: &HashSet<String>) -> String{
        let mut best_q = questioned.iter().next().unwrap().clone();
        let mut best_occurences = 0;

        for q in questioned{
            let mut occurences = 0;
            let tr = self.traits.get(q).unwrap();
            for g in guess{
                if let Some(_) = tr.members.get(g){
                    occurences += 1;
                }
            }
            if (guess.len() as i64 /2 - occurences).abs() < (guess.len() as i64/2 - best_occurences).abs(){
                best_q = q.clone();
                best_occurences = occurences;
            }
        }
        best_q
    }
}

fn main() {
    let mut expert = Expert::new();
    expert.add_trait("wool", "Has wool?");
    expert.add_trait("bird", "A bird?");
    expert.add_trait("predator", "A predator?");
    expert.add_trait("long_neck", "Has a long neck?");
    expert.add_trait("black_white", "Is it black-white colored?");
    expert.add_trait("yellow_brown", "Is it yellow-brown colored?");
    expert.add_trait("swim", "Does it swim?");
    expert.add_trait("fly", "Does it fly?");
    expert.add_trait("hooved", "Has hooves?");

    expert.add_entity("penguin", &vec!["bird", "black_white", "swim"]);
    expert.add_entity("giraffe", &vec!["long_neck", "hooved", "wool"]);
    expert.add_entity("ostrich", &vec!["bird", "long_neck"]);
    expert.add_entity("hawk", &vec!["predator", "bird", "fly"]);
    expert.add_entity("cheetah", &vec!["predator", "yellow_brown", "wool"]);

    expert.start_guessing_session();
}
