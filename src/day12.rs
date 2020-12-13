use std::fs::read_to_string;

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

pub fn run() {
    let input = read_to_string("src/day12-input.txt").unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|x| {
            let (instruction, value) = x.split_at(1);
            let value = value.parse::<i32>().unwrap();
            let instruction = match instruction {
                "N" => Instruction::North,
                "S" => Instruction::South,
                "E" => Instruction::East,
                "W" => Instruction::West,
                "L" => Instruction::Left,
                "R" => Instruction::Right,
                "F" => Instruction::Forward,
                x => unreachable!(format!("unexpected '{:?}'", x)),
            };
            instruction(value)
        })
        .collect();

    let (north, east, _) =
        instructions
            .iter()
            .fold((0, 0, 0), |(north, east, faces), instruction| {
                match (instruction, faces) {
                    //faces is index of [E,S,W,N]
                    (Instruction::North(v), _) | (Instruction::Forward(v), 3) => {
                        (north + v, east, faces)
                    }

                    (Instruction::South(v), _) | (Instruction::Forward(v), 1) => {
                        (north - v, east, faces)
                    }
                    (Instruction::East(v), _) | (Instruction::Forward(v), 0) => {
                        (north, east + v, faces)
                    }
                    (Instruction::West(v), _) | (Instruction::Forward(v), 2) => {
                        (north, east - v, faces)
                    }

                    (Instruction::Left(v), _) => (north, east, (faces - v / 90).rem_euclid(4)),
                    (Instruction::Right(v), _) => (north, east, (faces + v / 90).rem_euclid(4)),

                    (Instruction::Forward(_), _) => (north, east, faces),
                }
            });
    let result = north.abs() + east.abs();
    assert_eq!(2280, result);
    println!("{}", result);

    let (north, east, _, _) = instructions.iter().fold(
        (0, 0, 1, 10),
        |(north, east, waypoint_north, waypoint_east), instruction| match instruction {
            Instruction::North(v) => (north, east, waypoint_north + v, waypoint_east),
            Instruction::South(v) => (north, east, waypoint_north - v, waypoint_east),
            Instruction::East(v) => (north, east, waypoint_north, waypoint_east + v),
            Instruction::West(v) => (north, east, waypoint_north, waypoint_east - v),

            Instruction::Left(v) => {
                let new_waypoint =
                    (0..v / 90).fold((waypoint_north, waypoint_east), |(wn, we), _| (we, -wn));
                (north, east, new_waypoint.0, new_waypoint.1)
            }
            Instruction::Right(v) => {
                let new_waypoint =
                    (0..v / 90).fold((waypoint_north, waypoint_east), |(wn, we), _| (-we, wn));
                (north, east, new_waypoint.0, new_waypoint.1)
            }

            Instruction::Forward(v) => (
                north + waypoint_north * v,
                east + waypoint_east * v,
                waypoint_north,
                waypoint_east,
            ),
        },
    );
    let result = north.abs() + east.abs();
    assert_eq!(38693, result);
    println!("{}", result);
}
