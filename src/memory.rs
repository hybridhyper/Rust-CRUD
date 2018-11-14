use animal::Animal;

// TODO: Rename Memory to be a in memory data provider

pub struct Memory {
    animals: Vec< Box<dyn Animal> >
}

impl Memory {
    pub fn new() -> Memory {
        Memory { animals: Vec::new() }
    }

    pub fn add(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    pub fn get_animals(&self) -> &Vec< Box<dyn Animal> > {
        &self.animals
    }
}

#[cfg(test)]
mod test {

    use ::animal::monkey::Monkey;
    use ::animal::duck::Duck;
    use super::Memory;

    #[test]
    fn test_add() {
        let mut zoo = Memory::new();
        let monkey = Monkey::new(String::from("George"));
        let duck = Duck::new(String::from("Daffy"));

        assert_eq!(zoo.get_animals().len(), 0);

        zoo.add(Box::new(monkey));
        zoo.add(Box::new(duck));

        assert_eq!(zoo.get_animals().len(), 2);

        let animals = zoo.get_animals();
        let mut iter = animals.iter();

        assert_eq!(iter.next().unwrap().get_name(), &"George".to_string());
        assert_eq!(iter.next().unwrap().get_name(), &"Daffy".to_string());
    }
}