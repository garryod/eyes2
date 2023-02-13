use direction::Direction;

use crate::Settings;

#[derive(Debug)]
pub enum BadGenomeError {
    InvalidGenome,
}

// Every creature has a Genotype which defines their behaviour. It is
// expected that the Genotype will be defined by a genome, and that the
// genome (with mutations as appropriate) will be passed to the
// descendant creatures.
pub trait Genotype {
    fn new(config: Settings) -> Self;
    // execute the next instruction of your Genomic code
    fn tick(&mut self) -> GenotypeActions<Self>
    where
        Self: Sized;
    // change your internal energy level (this is for reference only as
    // the canonical energy level in in Creature itself)
    fn set_energy(&mut self, energy: i32);
}

// The genotype's tick method returns one of these actions. The creature
// will pass the request on to the world which will will verify the
// action is valid and then update the world state accordingly.
pub enum GenotypeActions<T: Genotype> {
    Reproduce(T),
    Move(Direction),
    Look(Direction),
    None,
}

// For each new Genotype defined the developer must add an arm to this
// genotype constructor function. This constructor provides a polymorphic
// interface to the Genotype trait.
// TODO replace this in some fashion now we have Genotype::new

// pub fn _new_genotype(which: &str, config: Settings) -> Result<Box<dyn Genotype>, BadGenomeError> {
//     let genotype: Box<dyn Genotype> = match which {
//         "giles" => Box::new(GilesGenotype { energy: 0 }),
//         "random" => Box::new(RandomGenomeType::new(config)),
//         _ => return Err(BadGenomeError::InvalidGenome),
//     };
//     Ok(genotype)
// }
