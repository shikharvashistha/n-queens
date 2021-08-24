extern crate oxigen;// Import the oxigen crate
extern crate rand;// Import the rand crate

use oxigen::prelude::*;// Import the oxigen prelude
use rand::distributions::Uniform;// Import the uniform distribution
use rand::prelude::*;// Import the rand prelude
use std::fmt::Display;// Import the Display trait
use std::fs::File;// Import the file system module

#[derive(Clone, PartialEq, Eq, std::hash::Hash)]// Implement the Clone, PartialEq, Eq, and Hash traits
struct QueensBoard(Vec<u8>);// Define a custom type for the board
impl Display for QueensBoard {// Implement the Display trait for the custom type
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {// Implement the Display trait for the custom type
        let mut s = String::new();// Create a new string
        for row in self.iter() {// Iterate over the rows
            let mut rs = String::from("|");// Create a new string
            for i in 0..self.0.len() {// Iterate over the columns
                if i == *row as usize {// If the column is the queen's column
                    rs.push_str("Q|");// Add a Q to the string
                } else {// If the column is not the queen's column
                    rs.push_str(" |")// Add a space to the string
                }// End if
            }
            rs.push('\n');// Add a newline to the string
            s.push_str(&rs);// Add the row to the string
        }
        write!(f, "{}", s)// Write the string to the formatter
    }
}

impl Genotype<u8> for QueensBoard {// Implement the Genotype trait for the custom type
    type ProblemSize = u8;// Define the problem size type
    //type GenotypeHash = Self;// Define the genotype hash type

    fn iter(&self) -> std::slice::Iter<u8> {// Implement the iter method for the custom type
        self.0.iter()// Return the row iterator
    }
    fn into_iter(self) -> std::vec::IntoIter<u8> {// Implement the into_iter method for the custom type
        self.0.into_iter()// Return the row iterator
    }
    fn from_iter<I: Iterator<Item = u8>>(&mut self, genes: I) {// Implement the from_iter method for the custom type
        self.0 = genes.collect();// Collect the genes into the row
    }

    fn generate(size: &Self::ProblemSize) -> Self {// Implement the generate method for the custom type
        let mut individual = Vec::with_capacity(*size as usize);// Create a new vector with the size of the problem
        let mut rgen = SmallRng::from_entropy();// Create a new random number generator
        for _i in 0..*size {// Iterate over the size of the problem
            individual.push(rgen.sample(Uniform::from(0..*size)));// Push a random number between 0 and the size of the problem
        }
        QueensBoard(individual)// Return the individual
    }

    // This function returns the maximum score possible (n, since in the
    // worst case n queens collide) minus the number of queens that collide with others
    fn fitness(&self) -> f64 {// Implement the fitness method for the custom type
        let size = self.0.len();// Get the size of the problem
        let diags_exceed = size as isize - 1_isize;// Calculate the maximum diagonal index
        let mut collisions = Vec::with_capacity(size);// Create a new vector with the size of the problem
        let mut verticals: Vec<isize> = Vec::with_capacity(size);// Create a new vector with the size of the problem
        let mut diagonals: Vec<isize> = Vec::with_capacity(size + diags_exceed as usize);// Create a new vector with the size of the problem
        let mut inv_diags: Vec<isize> = Vec::with_capacity(size + diags_exceed as usize);// Create a new vector with the size of the problem
        for _i in 0..size {// Iterate over the size of the problem
            verticals.push(-1);// Push a -1 to the verticals vector
            diagonals.push(-1);// Push a -1 to the diagonals vector
            inv_diags.push(-1);// Push a -1 to the inverse diagonals vector
            collisions.push(false);// Push a false to the collisions vector
        }
        for _i in 0..diags_exceed as usize {// Iterate over the maximum diagonal index
            diagonals.push(-1);// Push a -1 to the diagonals vector
            inv_diags.push(-1);// Push a -1 to the inverse diagonals vector
        }

        for (row, queen) in self.0.iter().enumerate() {// Iterate over the rows and the queens
            let mut collision = verticals[*queen as usize];// Get the vertical collision
            if collision > -1 {// If there is a vertical collision
                collisions[row] = true;// Set the collision to true
                collisions[collision as usize] = true;// Set the collision to true
            }// End if
            verticals[*queen as usize] = row as isize;// Set the vertical collision

            // A collision exists in the diagonal if col-row have the same value
            // for more than one queen
            let diag = ((*queen as isize - row as isize) + diags_exceed) as usize;// Calculate the diagonal index
            collision = diagonals[diag];// Get the diagonal collision
            if collision > -1 {// If there is a diagonal collision
                collisions[row] = true;// Set the collision to true
                collisions[collision as usize] = true;// Set the collision to true
            }
            diagonals[diag] = row as isize;// Set the diagonal collision

            // A collision exists in the inverse diagonal if n-1-col-row have the
            // same value for more than one queen
            let inv_diag =// Calculate the inverse diagonal index
                ((diags_exceed - *queen as isize - row as isize) + diags_exceed) as usize;
            collision = inv_diags[inv_diag];// Get the inverse diagonal collision
            if collision > -1 {// If there is a inverse diagonal collision
                collisions[row] = true;// Set the collision to true
                collisions[collision as usize] = true;// Set the collision to true
            }
            inv_diags[inv_diag] = row as isize;//   Set the inverse diagonal collision
        }

        (size - collisions.into_iter().filter(|r| *r).count()) as f64// Return the fitness
    }

    fn mutate(&mut self, rgen: &mut SmallRng, index: usize) {// Implement the mutate method for the custom type
        self.0[index] = rgen.sample(Uniform::from(0..self.0.len())) as u8;// Mutate the index
    }

    fn is_solution(&self, fitness: f64) -> bool {// Implement the is_solution method for the custom type
        fitness as usize == self.0.len()// Return true if the fitness is equal to the size of the problem
    }

    /*fn hash(&self) -> Self {// Implement the hash method for the custom type
        self.clone()// Return the individual
    }*/
}

fn main() {
    let n_queens: u8 = std::env::args()// Get the number of queens from the command line
        .nth(1)// Get the first argument
        .expect("Enter a number between 4 and 255 as argument")// Get the number of queens from the command line
        .parse()// Parse the argument
        .expect("Enter a number between 4 and 255 as argument");// Get the number of queens from the command line
    let progress_log = File::create("progress.csv").expect("Error creating progress log file");// Create the progress log file
    let population_log =// Create the population log file
        File::create("population.txt").expect("Error creating population log file");// Create the population log file
    let log2 = (f64::from(n_queens) * 4_f64).log2().ceil();// Calculate the log of the number of queens
    let mut population_size = 2_i32.pow(log2 as u32) as usize;// Calculate the population size
    if n_queens <= 8 {// If the number of queens is less than or equal to 8
        population_size *= 2;// Double the population size
    }

    let (solutions, generation, progress, _population) = GeneticExecution::<u8, QueensBoard>::new()// Create a new genetic execution
        .population_size(population_size)// Set the population size
        .genotype_size(n_queens as u8)// Set the genotype size
        .mutation_rate(Box::new(MutationRates::Linear(SlopeParams {// Set the mutation rate
            start: f64::from(n_queens) / (2_f64 + 4_f64 * log2) / 100_f64,// Calculate the start of the mutation rate
            bound: 0.005,// Set the mutation bound
            coefficient: -0.00002,// Set the mutation coefficient
        })))
        .selection_rate(Box::new(SelectionRates::Linear(SlopeParams {// Set the selection rate
            start: 3_f64,// Set the start of the selection rate
            bound: 6_f64,// Set the selection bound
            coefficient: 0.05,// Set the selection coefficient
        })))
        .select_function(Box::new(SelectionFunctions::Tournaments(NTournaments(// Set the selection function
            population_size / 2,// Set the number of tournaments
        ))))
        .crossover_function(Box::new(CrossoverFunctions::UniformCross))// Set the crossover function
        .population_refitness_function(Box::new(PopulationRefitnessFunctions::Niches(// Set the population refitness function
            NichesAlpha(0.8),// Set the alpha of the niche function
            Box::new(NichesBetaRates::Linear(SlopeParams {// Set the beta rate of the niche function
                start: 0.0025,// Set the start of the beta rate
                bound: 10.0_f64.min(log2 * log2 / 6.0),// Set the beta bound
                coefficient: 0.000001 * log2 * log2,// Set the beta coefficient
            })),
            NichesSigma(0.6),// Set the sigma of the niche function
        )))
        // Fighting to parents works but the evolution is slower with many queens
        /*
        .survival_pressure_function(Box::new(// Set the survival pressure function
            SurvivalPressureFunctions::ChildrenFightParentsAndTheRestWorst,// Set the survival pressure function
        ))*/
        .survival_pressure_function(Box::new(SurvivalPressureFunctions::Worst))// Set the survival pressure function
        .age_function(Box::new(AgeFunctions::Linear(// Set the age function
            AgeThreshold(5),// Set the age threshold
            AgeSlope(0.5),// Set the age slope
        )))
        .stop_criterion(Box::new(StopCriteria::SolutionsFound(// Set the stop criterion
            4.min(n_queens as usize / 2),// Set the number of solutions to find
        )))// Set the stop criterion
        .progress_log(2_000, progress_log)// Set the progress log
        .population_log(2_000, population_log)// Set the population log
        .run();// Run the genetic execution

    println!(
        "Finished in the generation {} with a progress of {}",// Print the generation and progress
        generation, progress// Print the generation and progress
    );
    for sol in &solutions {// For each solution
        println!("{}", sol);// Print the solution
    }
}