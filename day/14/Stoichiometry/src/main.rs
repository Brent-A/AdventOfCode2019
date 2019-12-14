#[test]
fn test1() {
   let input = "9 ORE => 2 A
   8 ORE => 3 B
   7 ORE => 5 C
   3 A, 4 B => 1 AB
   5 B, 7 C => 1 BC
   4 C, 1 A => 1 CA
   2 AB, 3 BC, 4 CA => 1 FUEL";

   let n = NanoFactory::parse(&input);
   assert_eq!(n.needed_ore(1), 165);
}

#[test]
fn test2() {
    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    let n = NanoFactory::parse(&input);
    let needed = n.needed_ore(1);
    assert_eq!(n.needed_ore(1), 13312);
 
    let trillion : i128 = 1000000000000;

   let x = trillion / (needed as i128);
   println!("x:{}", x );

   println!("ore for n-fuel: {}", n.needed_ore(82392753));

   let m = n.max_fuel(trillion);

   assert_eq!(m, 82892753 );
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Material(String);


#[derive(Debug)]
struct Quantity {
    amount: i128,
    kind: Material,
}

impl Quantity {
    fn parse(input: &str) -> Self {
        let mut halfs = input.trim().split(&" ");
        let amounttext = halfs.next().unwrap();
        let kindtext = halfs.next().unwrap();

        Self {
            amount: amounttext.parse::<i128>().unwrap(),
            kind: Material(kindtext.to_string()),
        }
    }
}

#[derive(Debug)]
struct Reaction {
    input : Vec<Quantity>,
    output : Quantity,
}

impl Reaction {
    fn parse(input: &str) -> Self {
        let input = input.trim();
        let mut halfs = input.split(&"=>");
        let inputtext = halfs.next().unwrap();
        let outputtext = halfs.next().unwrap();
        
        let mut inputs = Vec::new();

        for inputitemtext in inputtext.split(",") {
            let inputitem = Quantity::parse(inputitemtext);
            inputs.push(inputitem);
        }

        Self {
            input: inputs,
            output: Quantity::parse(outputtext),
        }
    }
}
use std::collections::HashMap;


#[derive(Debug)]
struct NanoFactory {
    reactions: HashMap<Material, Reaction>,
}

type Inventory = HashMap<Material, i128>;

impl NanoFactory {
    fn parse(input: &str) -> Self {
        let mut reactions = HashMap::new();
        for line in input.lines() {
            let reaction = Reaction::parse(line);
            reactions.insert(reaction.output.kind.clone(), reaction);
        }
        Self {
            reactions: reactions,
        }
    }

    fn apply_reaction(inventory: &mut Inventory, reaction: &Reaction, quantity: i128) {

        let mut rcount = quantity / reaction.output.amount;
        if rcount == 0 {
            rcount = 1;
        }
        inventory.insert(reaction.output.kind.clone(), inventory.get(&reaction.output.kind).unwrap_or(&0) + reaction.output.amount * rcount);
        for i in &reaction.input {
            inventory.insert(i.kind.clone(), inventory.get(&i.kind).unwrap_or(&0) - i.amount * rcount);    
        }
    }

    fn max_fuel(&self, ore: i128) -> i128{

        let mut last_guess = -1;
        let mut guess = 0;
        let mut increment = 1000000;

        loop {
            let n = self.needed_ore(guess);
            println!("guess: {} ore: {}", guess, n);

            if n < ore {
                guess = guess + increment;
            }
            if n > ore {
                if increment == 1 {
                    return guess - 1;
                }
                guess = guess - increment;
                
                increment = increment / 10;
            }

        }
    }

    fn needed_ore(&self, fuel: i128) -> i128 {
        let mut inventory : Inventory = HashMap::new();

        NanoFactory::apply_reaction(&mut inventory, self.reactions.get(&Material("FUEL".to_string())).unwrap(), fuel);

        loop {
            let negativecategory = inventory.iter().filter(|&(m,&q)| q < 0 && *m != Material("ORE".to_string())).next();
            if negativecategory.is_some() {
                let m = negativecategory.unwrap().0.clone();
                let q = - *negativecategory.unwrap().1;

                NanoFactory::apply_reaction(&mut inventory, self.reactions.get(&m).unwrap(), q);
            }
            else {
                break;
            }

        }

        //println!("inventory: {:?}", inventory);
        0 - *inventory.get(&Material("ORE".to_string())).unwrap()
    }
    
}

fn main() {
    
    let input = std::fs::read_to_string("input.txt").unwrap();
    
   let n = NanoFactory::parse(&input);
   let ore_per_fuel = n.needed_ore(1);
   println!("Needed ore: {}", n.needed_ore(1));
   let trillion : i128 = 1000000000000;

   let m = n.max_fuel(trillion);
   println!("x:{}", m );
}
