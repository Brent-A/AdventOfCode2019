mod point;

use point::*;

#[test]
fn example() {
    let maze = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    let map = Map::new(&maze);

    println!("Tiles: {}", map.tiles.len());
    println!("Portals: {:?}", map.portals());

    let cost_map = map.get_cost_map(map.labeled.get("ZZ").unwrap());

    let aa = map.labeled.get("AA").unwrap();
    let steps = cost_map.get(&aa).unwrap() + 1;
    println!("steps: {}", steps);
    assert_eq!(steps, 58);
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&file);

    println!("Tiles: {}", map.tiles.len());
    println!("Portals: {:?}", map.portals());

    let cost_map = map.get_cost_map(map.labeled.get("ZZ").unwrap());

    let aa = map.labeled.get("AA").unwrap();
    let steps = cost_map.get(&aa).unwrap() + 1;
    println!("steps: {}", steps);
}
