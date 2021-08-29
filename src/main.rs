const ANSWER: [[&str; 4]; 4] = [["0","1","2","3"],["4","5","6","7"],["8","9","10","11"],["12","13","14","15"]];
use rand::seq::SliceRandom;
use std::io;
fn out_of_range_check<'a>(i_tmp:i32,j_tmp:i32,moved_blank_tmp:[i32;2], puzzle_board_tmp:[[&'a str;4];4])-> ([[&'a str;4];4],  [i32; 2]){
    let moved_blank = [moved_blank_tmp[0] as usize, moved_blank_tmp[1] as usize];
    let mut moved_blank_i32 = moved_blank_tmp;
    let mut puzzle_board = puzzle_board_tmp;
    let i = (moved_blank_tmp[0] + i_tmp) as usize;
    let j = (moved_blank_tmp[1] + j_tmp) as usize;
    // println!("{} {}",moved_blank_tmp[0], i_tmp);
    if moved_blank_tmp[0] + i_tmp >= 0 && moved_blank_tmp[1] + j_tmp >= 0 && moved_blank_tmp[0] + j_tmp <= 3 && moved_blank_tmp[1] + j_tmp <= 3{
        let tmp = puzzle_board[moved_blank[0] as usize][moved_blank[1] as usize];
        puzzle_board[moved_blank[0] as usize][moved_blank[1]] = puzzle_board[i][j];
        puzzle_board[i][j] = tmp;
        moved_blank_i32[0] = moved_blank_i32[0] + i_tmp;
        moved_blank_i32[1] = moved_blank_i32[1] + j_tmp;
    } else {
        println!("そっちには動かせません");
    }
    return (puzzle_board,moved_blank_i32);
}
fn make_puzzle<'a>() -> [[&'a str; 4]; 4]{
    let mut puzzle_board:[[&str;4];4] = Default::default();
    let mut basic_num = ["0","1","2","3","4","5","6","7","8","9","10","11","12","13","14","15"];
    let mut rng = rand::thread_rng();
    basic_num.shuffle(&mut rng);
    let mut num = 0;
    for i in 0..4{
        for j in 0..4{
            puzzle_board[i as usize][j as usize] = basic_num[num as usize];
            num = num + 1;
        }
    }
    return puzzle_board;
}

fn main() {
    let mut puzzle_board = make_puzzle();
    let mut moved_blank:[i32;2] = [0,0];
    for i in 0..4{
        for j in 0..4{
            match puzzle_board[i][j] {
                "0" => {
                    moved_blank[0] = i as i32;
                    moved_blank[1] = j as i32;
                    // println!("found {:?}",  moved_blank);
                }
                _ => {
                    // println!("found something else!");
                }
            }
        }
    }
    while puzzle_board != ANSWER {
        for a in puzzle_board{
            print!("{:?}\n",a);
        }
        println!("W(上), A(左), S(下),D(右)で入力してください(0が移動します)");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line.");
        let word = &word.trim();
        let mut tmp_i = 0;
        let mut tmp_j = 0;
        println!("入力は{}",word);
        if word == &"w" || word == &"W" {
            // println!("test");
            tmp_i = -1;
            tmp_j = 0;
        }else if word == &"a" || word == &"A" {
            tmp_i = 0;
            tmp_j = -1;
        }else if word == &"s" || word == &"S" {
            tmp_i = 1;
            tmp_j = 0;
        }else if word == &"d" || word == &"D" {
            tmp_i = 0;
            tmp_j = 1;
        }
        let result = out_of_range_check(tmp_i, tmp_j, moved_blank,puzzle_board);
        puzzle_board = result.0;
        moved_blank = result.1;

    }
    println!("揃いました。おめでとう");
}
