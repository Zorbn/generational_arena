#![allow(dead_code)]

#[derive(PartialEq, Eq)]
struct GenerationalIndex {
    index: usize,
    generation: usize,
}

enum GenerationalElement<T> {
    Some { value: T, generation: usize },
    None { generation: usize },
}

impl<T> GenerationalElement<T> {
    fn is_some(&self) -> bool {
        match self {
            GenerationalElement::Some { value: _, generation: _ } => true,
            GenerationalElement::None { generation: _ } => false,
        }
    }

    fn is_none(&self) -> bool {
        !self.is_some()
    }

    fn generation(&self) -> usize {
        match self {
            GenerationalElement::Some {
                value: _,
                generation,
            } => *generation,
            GenerationalElement::None { generation } => *generation,
        }
    }
}

struct GenerationalArena<T> {
    elements: Vec<GenerationalElement<T>>,
    next_index_to_fill: usize,
}

impl<T> GenerationalArena<T> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            next_index_to_fill: 0,
        }
    }

    fn push(&mut self, value: T) -> GenerationalIndex {
        while self.next_index_to_fill < self.elements.len()
            && self.elements[self.next_index_to_fill].is_some()
        {
            self.next_index_to_fill += 1;
        }

        if self.next_index_to_fill >= self.elements.len() {
            self.elements.push(GenerationalElement::Some {
                value,
                generation: 0,
            });
        } else {
            let generation = self.elements[self.next_index_to_fill].generation();
            self.elements[self.next_index_to_fill] = GenerationalElement::Some {
                value,
                generation: generation + 1,
            }
        }

        GenerationalIndex {
            index: self.next_index_to_fill,
            generation: self.elements[self.next_index_to_fill].generation(),
        }
    }

    fn remove(&mut self, generational_index: GenerationalIndex) {
        if let Some(GenerationalElement::Some {
            value: _,
            generation,
        }) = self.elements.get_mut(generational_index.index)
        {
            self.elements[generational_index.index] = GenerationalElement::None {
                generation: *generation,
            };
        };
    }

    fn remove_at(&mut self, index: usize) {
        if let Some(GenerationalElement::Some {
            value: _,
            generation,
        }) = self.elements.get_mut(index)
        {
            self.elements[index] = GenerationalElement::None {
                generation: *generation,
            };
        };
    }

    fn at(&self, index: usize) -> Option<&T> {
        match self.elements.get(index) {
            Some(GenerationalElement::Some {
                value,
                generation: _,
            }) => Some(value),
            _ => None,
        }
    }

    fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.elements.get_mut(index) {
            Some(GenerationalElement::Some {
                value,
                generation: _,
            }) => Some(value),
            _ => None,
        }
    }

    fn get(&self, generational_index: GenerationalIndex) -> Option<&T> {
        match self.elements.get(generational_index.index) {
            Some(GenerationalElement::Some { value, generation })
                if *generation == generational_index.generation =>
            {
                Some(value)
            }
            _ => None,
        }
    }

    fn get_mut(&mut self, generational_index: GenerationalIndex) -> Option<&mut T> {
        match self.elements.get_mut(generational_index.index) {
            Some(GenerationalElement::Some { value, generation })
                if *generation == generational_index.generation =>
            {
                Some(value)
            }
            _ => None,
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

struct Entity {
    health: i32,
    damage: i32,
}

fn entity_update(self_index: usize, entities: &mut GenerationalArena<Entity>) {
    for i in 0..entities.len() {
        if i == self_index {
            continue;
        }

        entity_attack(self_index, i, entities);
    }
}

fn entity_attack(self_index: usize, other_index: usize, entities: &mut GenerationalArena<Entity>) -> Option<()> {
    entities.at_mut(other_index)?.health -= entities.at_mut(self_index)?.damage;

    if entities.at_mut(other_index)?.health < 0 {
        entities.remove_at(other_index);
    }

    Some(())
}

fn main() {
    let mut entities = GenerationalArena::<Entity>::new();

    entities.push(Entity {
        health: 100,
        damage: 30,
    });

    entities.push(Entity {
        health: 200,
        damage: 10,
    });

    for _ in 0..7 {
        for i in 0..entities.len() {
            entity_update(i, &mut entities);
        }
    }

    for i in 0..entities.len() {
        let entity = match entities.at(i) {
            Some(a) => a,
            _ => continue,
        };

        println!("{}", entity.health);
    }

    println!("Hello, world!");
}
