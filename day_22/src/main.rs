use std::collections::VecDeque;
use utils::*;

use Action::*;
#[derive(Copy, Clone)]
enum Action {
    Deal,
    DealN(usize),
    Cut(i64),
}

struct PosDeck {
    len: u64,
    pos: u64,
}

impl PosDeck {
    fn new(len: u64, pos: u64) -> PosDeck {
        PosDeck { len, pos }
    }
    fn do_action(&mut self, action: Action) {
        match action {
            Deal => self.pos = (self.len - 1) - self.pos,
            DealN(n) => self.pos = (self.pos * n as u64) % self.len,
            Cut(n) => {
                if n > 0 {
                    let n = n as u64;
                    if n <= self.pos {
                        self.pos = self.pos - n;
                    } else {
                        let x = (n - 1) - self.pos;
                        self.pos = (self.len - 1) - x;
                    }
                } else {
                    let n = n.abs() as u64;
                    //println!("n:{}  selflen:{} pos:{}",n,self.len-n,self.pos);
                    if self.pos <= self.len - 1 - n {
                        self.pos = self.pos + n;
                    } else {
                        let offset = (self.len - 1) - self.pos;
                        //  println!("n:{} offset:{}",n,offset);
                        self.pos = (n - 1) - offset;
                    }
                }
                //0123456789
                //cut 2
                //2345678901
            }
        }
    }
}

struct Deck {
    cards: VecDeque<u64>,
    table: Vec<u64>,
}
impl Deck {
    fn new(size: usize) -> Deck {
        let mut cards = VecDeque::with_capacity(size);
        for i in 0..size {
            cards.push_back(i as u64);
        }
        let table = vec![0; cards.len()];
        Deck { cards, table }
    }

    fn do_action(&mut self, action: Action) {
        match action {
            Deal => self.deal(),
            DealN(n) => self.deal_n(n),
            Cut(n) => self.cut(n),
        }
    }

    fn deal(&mut self) {
        let mut s = 0;
        let mut e = self.cards.len() - 1;

        while s < e {
            let t = self.cards[s];
            self.cards[s] = self.cards[e];
            self.cards[e] = t;
            s += 1;
            e -= 1;
        }
    }

    fn deal_n(&mut self, n: usize) {
        let mut i = 0;
        let mut j = 0;
        while i < self.cards.len() {
            self.table[j] = self.cards[i];
            i += 1;
            j = (j + n) % self.cards.len();
        }
        for (i, card) in self.table.iter().enumerate() {
            self.cards[i] = *card;
        }
    }

    fn cut(&mut self, n: i64) {
        if n > 0 {
            for _ in 0..n {
                let card = self.cards.pop_front().unwrap();
                self.cards.push_back(card);
            }
        } else {
            for _ in 0..n.abs() {
                let card = self.cards.pop_back().unwrap();
                self.cards.push_front(card);
            }
        }
    }

    fn print(&self) {
        for card in &self.cards {
            print!("{}", card);
        }
        println!("");
    }
}

fn main() {
    let mut deck = Deck::new(10007);
    let mut actions = Vec::new();
    for line in read_file("input.txt") {
        if line.starts_with("deal into") {
            actions.push(Deal)
        } else if line.starts_with("deal with") {
            let split = line.split(' ');
            let last = split.last().unwrap();
            let num = last.parse::<usize>().unwrap();
            actions.push(DealN(num));
        } else if line.starts_with("cut") {
            let split = line.split(' ');
            let last = split.last().unwrap();
            let num = last.parse::<i64>().unwrap();
            actions.push(Cut(num));
        }
    }

    for action in &actions {
        deck.do_action(*action);
    }

    let pos = deck.cards.iter().position(|n| *n == 2019).unwrap();
    println!("part 1 pos:{}", pos);

    let mut deck = PosDeck::new(119315717514047, 2020);
    let end : u64 = 101741582076661;
    for i in 1..end {
        for action in &actions {
            deck.do_action(*action);
        }
        if deck.pos == 2020 {
            println!("repeat at {}",i);
            break;
        }
        if i % 1_000_000 == 0 {
            println!("{}%",i as f64/end as f64);
        }
    }
   // println!("part2 test pos:{}", deck.pos);
}
