const input: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

/// For example:
/// 2413432311323
/// 3215453535623
/// 3255245654254
/// 3446585845452
/// 4546657867536
/// 1438598798454
/// 4457876987766
/// 3637877979653
/// 4654967986887
/// 4564679986453
/// 1224686865563
/// 2546548887735
/// 4322674655533
/// Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

/// Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

/// One way to minimize heat loss is this path:

/// 2>>34^>>>1323
/// 32v>>>35v5623
/// 32552456v>>54
/// 3446585845v52
/// 4546657867v>6
/// 14385987984v4
/// 44578769877v6
/// 36378779796v>
/// 465496798688v
/// 456467998645v
/// 12246868655<v
/// 25465488877v5
/// 43226746555v>
/// This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only 102.
fn part1() {
    // write code that fulfills problem description above
}
