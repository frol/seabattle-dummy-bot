use std::io::{BufRead, Write};

#[derive(Debug, Copy, Clone)]
enum GameCell {
    Unknown,
    Missing,
    Hit,
}

impl From<GameCell> for char {
    fn from(value: GameCell) -> Self {
        match value {
            GameCell::Unknown => '_',
            GameCell::Missing => 'O',
            GameCell::Hit => '#',
        }
    }
}

fn main() -> std::io::Result<()> {
    print!(
        "\
        _####_###_\n\
        __________\n\
        #________#\n\
        #_____#__#\n\
        _________#\n\
        ##________\n\
        ___#_____#\n\
        _________#\n\
        #_________\n\
        ___#______\n\
        "
    );
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    let stdin = std::io::stdin();
    let mut stdin_reader = stdin.lock();
    let mut x = 1;
    let mut y = 1;
    let mut enemy_map = vec![vec![GameCell::Unknown; 10]; 10];
    loop {
        println!("{} {}", x, y);
        let _ = std::io::stdout().flush();
        if stdin_reader.read_line(&mut line).is_err() {
            break;
        }
        enemy_map[y-1][x-1] = match line.trim() {
            "miss" => GameCell::Missing,
            "hit" | "sunk" => GameCell::Hit,
            "" => break,
            unexpected_response => panic!("unknown response '{}'", unexpected_response),
        };
        line.clear();
        //eprintln!("{}", enemy_map.iter().map(|line| line.iter().copied().map(|cell| cell.into()).chain("\n".chars()).collect::<String>()).collect::<String>());
        x += 1;
        if x >= 11 {
            x = 1;
            y += 1;
        }
    }
    Ok(())
}
