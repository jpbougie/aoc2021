use anyhow::Result;
use regex::Regex;
fn main() -> Result<()> {
    let str = include_str!("../../inputs/04.txt");
    let mut lines = str.lines();
    let picks: Vec<u32> = lines.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    let mut cards = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    // skip a line
    while lines.next().is_some() {
        let mut card = Vec::with_capacity(4);
        for _i in 0..5 {
            let l = lines.next().unwrap();
            let mut numbers: Vec<u32> = re.find_iter(l).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
            card.append(&mut numbers);
        }
        cards.push(Board{cells: card.into_iter().map(Cell::new).collect(), size: 5});
    }

    let mut boards = cards.clone();
    'picks: for pick in picks.iter() {
        for card in boards.iter_mut() {
            card.mark(*pick);
            if card.has_won() {
                println!("Part 1: {}", card.unmarked_sum() * pick);
                break 'picks;
            }
        }
    }

    let mut boards = cards.clone();
    for pick in picks.iter() {
        for card in boards.iter_mut() {
            card.mark(*pick);
        }

        let non_winners = boards.iter().filter(|b| !b.has_won()).cloned().collect::<Vec<Board>>();

        if !non_winners.is_empty() {
            boards = non_winners;
        } else {
            println!("Part 2: {}", boards.last().unwrap().unmarked_sum() * pick);
            break
        }
    }

    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct Cell {
    val: u32,
    marked: bool
}

impl Cell {
    fn new(val: u32) -> Self {
        Self{val, marked: false}
    }

    fn mark(&mut self) {
        self.marked = true
    }
}

#[derive(Debug, Default, Clone)]
struct Board {
    cells: Vec<Cell>,
    size: usize
}

impl Board {
    fn has_won(&self) -> bool {
        self.row_has_won() || self.col_has_won()
    }

    fn unmarked_sum(&self) -> u32 {
        self.cells.iter().filter(|c| !c.marked).map(|c| c.val).sum()
    }

    fn row_has_won(&self) -> bool {
        for i in 0..self.size {
            if self.cells[i*self.size..i*self.size+self.size].iter().all(|c| c.marked) {
                return true
            }
        }

        false
    }

    fn col_has_won(&self) -> bool {
        'a: for i in 0..self.size {
            for j in 0..self.size {
                if !(self.cells[j*self.size + i].marked) {
                    continue 'a;
                }
            }
            return true
        }
        false
    }

    fn mark(&mut self, pick: u32) {
        self.cells.iter_mut().for_each(|c| if c.val == pick { c.mark() })
    }
}