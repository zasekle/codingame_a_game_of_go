use std::str;

#[derive(PartialEq, Debug, Clone)]
enum Space {
    BLACK,
    WHITE,
    EMPTY,
}

#[derive(PartialEq, Debug)]
struct Loc {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Debug)]
struct SingleMove {
    color: Space,
    loc: Loc,
}

#[derive(PartialEq, Debug)]
struct Group {
    locs: Vec<Loc>,
    owner: Space,
}

#[derive(PartialEq, Debug, Clone)]
struct BoardSpace {
    element: Space,
    group_index: i32,
}

fn main() {
    // Test #1
    // let raw_board: Vec<String> = Vec::from([
    //     ".....".to_string(),
    //     ".....".to_string(),
    //     ".....".to_string(),
    //     ".....".to_string(),
    //     ".....".to_string()
    // ]);
    //
    // let moves = Vec::<SingleMove>::from([
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 2, col: 2 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 2, col: 3 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 3, col: 2 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 3, col: 4 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 4, col: 3 } },
    // ]);

    // Test #4 Suicidal move
    // let raw_board: Vec<String> = Vec::from([
    //     ".BW..".to_string(),
    //     "BW...".to_string(),
    //     ".W...".to_string(),
    //     ".....".to_string(),
    //     ".....".to_string(),
    // ]);
    //
    // let moves = Vec::<SingleMove>::from([
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 3, col: 4 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 4, col: 4 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 3, col: 3 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 2, col: 0 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 0, col: 0 } },
    // ]);

    // Test #5 Non-Suicidal move
    // let raw_board: Vec<String> = Vec::from([
    //     ".B...".to_string(),
    //     "BW...".to_string(),
    //     "WB..W".to_string(),
    //     "...W.".to_string(),
    //     "...BW".to_string(),
    // ]);
    //
    // let moves = Vec::<SingleMove>::from([
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 3, col: 4 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 0, col: 2 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 0, col: 3 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 0, col: 0 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 1, col: 2 } },
    //     SingleMove { color: Space::WHITE, loc: Loc { row: 0, col: 1 } },
    //     SingleMove { color: Space::BLACK, loc: Loc { row: 1, col: 0 } },
    // ]);

    //Test #6 Ko rule violated
    let raw_board: Vec<String> = Vec::from([
        "..B..".to_string(),
        ".B.B.".to_string(),
        ".WBW.".to_string(),
        ".BW..".to_string(),
        ".....".to_string(),
    ]);

    let moves = Vec::<SingleMove>::from([
        SingleMove { color: Space::BLACK, loc: Loc { row: 3, col: 3 } },
        SingleMove { color: Space::WHITE, loc: Loc { row: 1, col: 2 } },
        SingleMove { color: Space::BLACK, loc: Loc { row: 2, col: 2 } },
    ]);

    let mut board: Vec<Vec<BoardSpace>> = Vec::new();

    for board_row in raw_board {
        let mut new_row: Vec<BoardSpace> = Vec::new();
        for board_space in board_row.chars() {
            new_row.push(
                BoardSpace {
                    element: match board_space {
                        'B' => {
                            Space::BLACK
                        }
                        'W' => {
                            Space::WHITE
                        }
                        _ => {
                            Space::EMPTY
                        }
                    },
                    group_index: -1,
                }
            )
        }
        board.push(new_row);
    }

    let mut groups = Vec::<Group>::new();
    let mut one_prev_round_board : Vec<Vec<Space>>= Vec::new();
    let mut two_prev_round_board : Vec<Vec<Space>>= Vec::new();

    print_board(
        &board,
        &groups,
    );

    calculate_groups(
        &mut groups,
        &mut board,
    );

    remove_surrounded_pieces(
        &mut groups,
        &mut board,
        -1,
        -1,
    );

    for single_move in moves.iter() {
        let row = single_move.loc.row as usize;
        let col = single_move.loc.col as usize;

        if board[row][col].element != Space::EMPTY {
            println!("row {row} col {col} BOARD SPACE NOT EMPTY!");
        }

        board[row][col].element = single_move.color.clone();

        calculate_groups(
            &mut groups,
            &mut board,
        );

        remove_surrounded_pieces(
            &mut groups,
            &mut board,
            row as i32,
            col as i32,
        );

        for group in &groups {
            println!("{:?}", group);
        }

        println!("\nboard");
        print_board(
            &board,
            &groups,
        );

        println!("\ntwo_prev_round_board");
        for (i, row) in two_prev_round_board.iter().enumerate() {
            let mut print_row_str = String::new();
            for (j, col) in row.iter().enumerate() {
                if j != 0 {
                    print_row_str.push(' ');
                }
                print_row_str.push(
                    match col {
                        Space::BLACK => {
                            'B'
                        }
                        Space::WHITE => {
                            'W'
                        }
                        Space::EMPTY => {
                            'E'
                        }
                    }
                )
            }
            println!("{print_row_str}");
        }

        // suicide move
        if board[single_move.loc.row as usize][single_move.loc.col as usize].element != single_move.color {
            println!("NON EQUAL COLOR");
        }

        // ko move
        if !two_prev_round_board.is_empty() {
            let mut boards_equal = true;
            'outer: for (i, row) in two_prev_round_board.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    if *col != board[i][j].element {
                        println!("!boards_equal {i} {j}");
                        boards_equal = false;
                        break 'outer;
                    }
                }
            }

            if boards_equal {
                println!("NOT_VALID");
            }
        }

        two_prev_round_board = one_prev_round_board;
        one_prev_round_board = Vec::new();

        for board_row in board.iter() {
            let mut prev_board_row = Vec::<Space>::new();
            for board_ele in board_row.iter() {
                prev_board_row.push(board_ele.element.clone());
            }
            one_prev_round_board.push(prev_board_row);
        }
    }
}

fn calculate_groups(
    groups: &mut Vec<Group>,
    board: &mut Vec<Vec<BoardSpace>>,
) {
    groups.clear();

    for board_row in &mut *board {
        for element in &mut *board_row {
            element.group_index = -1;
        }
    }

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let board_space = &board[row][col];

            if board_space.group_index == -1 {
                let group_index = groups.len() as i32;
                let group_owner = board_space.element.clone();
                board[row][col].group_index = group_index;
                groups.push(
                    Group {
                        locs: Vec::from([Loc { row, col }]),
                        owner: group_owner.clone(),
                    }
                );
                recursive_find_group(
                    groups,
                    board,
                    row,
                    col,
                    (groups.len() - 1) as i32,
                    &group_owner,
                )
            }
        }
    }
}

struct GroupIndexWrapper {
    group_index: usize,
    order_number: i32,
}

fn remove_surrounded_pieces(
    groups: &mut Vec<Group>,
    board: &mut Vec<Vec<BoardSpace>>,
    last_move_row: i32,
    last_move_col: i32,
) {
    let mut group_index_values = Vec::<GroupIndexWrapper>::new();

    for i in 0..groups.len() {
        group_index_values.push(
            GroupIndexWrapper {
                group_index: i as usize,
                order_number: 0,
            }
        );
    }

    // If the player plays a move where they can take and be taken, then it is NOT suicide, it takes.
    if last_move_col > -1 && last_move_row > -1 {
        let row = last_move_row as usize;
        let col = last_move_col as usize;

        if 0 < row { // UP
            group_index_values[board[row - 1][col].group_index as usize].order_number = 1;
        }

        if row < (board.len() - 1) { // DOWN
            group_index_values[board[row + 1][col].group_index as usize].order_number = 1;
        }

        if 0 < col { // LEFT
            group_index_values[board[row][col - 1].group_index as usize].order_number = 1;
        }

        if col < (board[0].len() - 1) { // RIGHT
            group_index_values[board[row][col + 1].group_index as usize].order_number = 1;
        }

        group_index_values.sort_by(|a, b| b.order_number.partial_cmp(&a.order_number).unwrap());
    }
    // for (i, group) in groups.iter_mut().enumerate() {
    for group_index_wrapper in &group_index_values {
        let mut group = &mut groups[group_index_wrapper.group_index];
        if group.owner == Space::EMPTY {
            continue;
        }

        let group_index = group_index_wrapper.group_index as i32;
        let group_other_color = match group.owner {
            Space::BLACK => {
                Space::WHITE
            }
            _ => {
                Space::BLACK
            }
        };
        let group_owner_color = group.owner.clone();
        let mut group_surrounded = true;
        for member in group.locs.iter() {
            let row = member.row;
            let col = member.col;

            if 0 < row { // UP
                let eliminate_group = element_eliminates_group(
                    board,
                    row - 1,
                    col,
                    group_index,
                    &group_owner_color,
                    &group_other_color,
                );

                if !eliminate_group {
                    group_surrounded = false;
                    break;
                }
            }

            if row < (board.len() - 1) { // DOWN
                let eliminate_group = element_eliminates_group(
                    board,
                    row + 1,
                    col,
                    group_index,
                    &group_owner_color,
                    &group_other_color,
                );

                if !eliminate_group {
                    group_surrounded = false;
                    break;
                }
            }

            if 0 < col { // LEFT
                let eliminate_group = element_eliminates_group(
                    board,
                    row,
                    col - 1,
                    group_index,
                    &group_owner_color,
                    &group_other_color,
                );

                if !eliminate_group {
                    group_surrounded = false;
                    break;
                }
            }

            if col < (board[0].len() - 1) { // RIGHT
                let eliminate_group = element_eliminates_group(
                    board,
                    row,
                    col + 1,
                    group_index,
                    &group_owner_color,
                    &group_other_color,
                );

                if !eliminate_group {
                    group_surrounded = false;
                    break;
                }
            }
        }

        if !group_surrounded {
            continue;
        }

        println!("surrounded group_index: {group_index}");

        group.owner = Space::EMPTY;

        for member in group.locs.iter_mut() {
            let row = member.row;
            let col = member.col;

            board[row][col].element = Space::EMPTY;
        }
    }
}

fn recursive_find_group(
    groups: &mut Vec<Group>,
    board: &mut Vec<Vec<BoardSpace>>,
    row: usize,
    col: usize,
    group_index: i32,
    group_owner: &Space,
) {
    if 0 < row { // UP
        setup_space_and_find_group(
            groups,
            board,
            row - 1,
            col,
            group_index,
            group_owner,
        )
    }

    if row < (board.len() - 1) { // DOWN
        setup_space_and_find_group(
            groups,
            board,
            row + 1,
            col,
            group_index,
            group_owner,
        )
    }

    if 0 < col { // LEFT
        setup_space_and_find_group(
            groups,
            board,
            row,
            col - 1,
            group_index,
            group_owner,
        )
    }

    if col < (board[0].len() - 1) { // RIGHT
        setup_space_and_find_group(
            groups,
            board,
            row,
            col + 1,
            group_index,
            group_owner,
        )
    }
}

fn setup_space_and_find_group(
    groups: &mut Vec<Group>,
    board: &mut Vec<Vec<BoardSpace>>,
    row: usize,
    col: usize,
    group_index: i32,
    group_owner: &Space,
) {
    let space_has_group = set_space_if_no_group(
        board,
        row,
        col,
        group_index,
        group_owner,
    );

    if !space_has_group {
        let last_index = groups.len() - 1;
        groups[last_index].locs.push(Loc { row, col });
        recursive_find_group(
            groups,
            board,
            row,
            col,
            group_index,
            group_owner,
        )
    }
}

fn set_space_if_no_group(
    board: &mut Vec<Vec<BoardSpace>>,
    row: usize,
    col: usize,
    group_index: i32,
    group_owner: &Space,
) -> bool {
    let board_space = &board[row][col];

    if board_space.element == *group_owner
        && board_space.group_index == -1 {
        board[row][col].group_index = group_index;
        return false;
    }

    true
}

fn print_board(
    board: &Vec<Vec<BoardSpace>>,
    groups: &Vec<Group>,
) {
    for board_row in board {
        let mut row_str = String::new();
        for element in board_row {
            if !row_str.is_empty() {
                row_str.push(' ');
            }
            let mut return_str = String::new();
            row_str +=
                if element.group_index > -1 {
                    let element_str = element.group_index.to_string();
                    let spaces_vec = vec![b' '; groups.len().to_string().len() - element_str.len()];
                    let spaces = str::from_utf8(&spaces_vec).expect("fail to find utf8");
                    match groups[element.group_index as usize].owner {
                        Space::EMPTY => {
                            return_str.push('e');
                            return_str.push_str(element_str.as_str());
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                        Space::BLACK => {
                            return_str.push('b');
                            return_str.push_str(element_str.as_str());
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                        Space::WHITE => {
                            return_str.push('w');
                            return_str.push_str(element_str.as_str());
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                    }
                } else {
                    let spaces_vec = vec![b' '; groups.len()];
                    let spaces = str::from_utf8(&spaces_vec).expect("fail to find utf8");
                    match element.element {
                        Space::EMPTY => {
                            return_str.push('E');
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                        Space::WHITE => {
                            return_str.push('W');
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                        Space::BLACK => {
                            return_str.push('B');
                            return_str.push_str(spaces);
                            return_str.as_str()
                        }
                    }
                };
        }
        println!("{row_str}");
    }
}

fn element_eliminates_group(
    board: &mut Vec<Vec<BoardSpace>>,
    row: usize,
    col: usize,
    group_index: i32,
    group_owner_color: &Space,
    group_other_color: &Space,
) -> bool {
    board[row][col].element == *group_other_color
        || (board[row][col].element == *group_owner_color
        && board[row][col].group_index == group_index)
}
