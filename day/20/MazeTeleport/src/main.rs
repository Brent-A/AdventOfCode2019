mod point;

use point::*;

#[test]
fn example() {
    let maze = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    let map = Map::new(&maze);

    println!("Tiles: {}", map.tiles.len());
    println!("Portals:");
    for (l, (p1, p2)) in map.portals() {
        println!("{}: {:?} -> {:?}", l, p1, p2);
    }

    let zz = map.labeled.get("ZZ").unwrap();
    let aa = map.labeled.get("AA").unwrap();

    let cost_map = map.get_cost_map(&(*zz, 0), Some(&(*aa, 0)));

    let steps = cost_map.get(&(*aa, 0)).unwrap() + 1;
    println!("steps: {}", steps);
    assert_eq!(steps, 396);
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&file);

    println!("Tiles: {}", map.tiles.len());
    println!("Portals:");
    for (l, (p1, p2)) in map.portals() {
        println!("{}: {:?} -> {:?}", l, p1, p2);
    }

    let zz = map.labeled.get("ZZ").unwrap();
    let aa = map.labeled.get("AA").unwrap();

    let cost_map = map.get_cost_map(&(*zz, 0), Some(&(*aa, 0)));

    let steps = cost_map.get(&(*aa, 0)).unwrap() + 1;
    println!("steps: {}", steps);
    assert_eq!(steps, 8314);
}
