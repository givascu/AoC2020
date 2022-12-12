use std::collections::{HashMap, HashSet, VecDeque};

type Range = (usize, usize);
type Ranges = HashMap<&'static str, (Range, Range)>;
type Ticket = Vec<usize>;

fn parse_input() -> (Ranges, Vec<Ticket>, Ticket) {
    let mut is_my_ticket = false;
    let mut is_other_ticket = false;

    let mut ranges = Ranges::new();
    let mut tickets = Vec::new();
    let mut my_ticket = Ticket::new();

    for line in include_str!("../input/day16.txt")
        .lines()
        .filter(|&x| !x.is_empty())
    {
        if line.starts_with("your") {
            is_my_ticket = true;
        } else if line.starts_with("nearby") {
            is_my_ticket = false;
            is_other_ticket = true;
        } else if is_my_ticket {
            my_ticket = line.split(',').map(|x| x.parse().unwrap()).collect();
        } else if is_other_ticket {
            tickets.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        } else {
            let parts = line.split_once(": ").unwrap();
            let (r1, r2) = parts
                .1
                .split_once(" or ")
                .map(|(r1, r2)| {
                    (
                        r1.split_once('-')
                            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                            .unwrap(),
                        r2.split_once('-')
                            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                            .unwrap(),
                    )
                })
                .unwrap();
            ranges.insert(parts.0, (r1, r2));
        }
    }

    (ranges, tickets, my_ticket)
}

pub fn solve_1() -> usize {
    let (ranges, tickets, _) = parse_input();
    tickets.iter().fold(0, |acc, ticket| {
        acc + ticket
            .iter()
            .filter(|&num| {
                ranges
                    .values()
                    .all(|(r1, r2)| !(r1.0..=r1.1).contains(num) && !(r2.0..=r2.1).contains(num))
            })
            .sum::<usize>()
    })
}

pub fn solve_2() -> usize {
    let (ranges, tickets, my_ticket) = parse_input();

    // Filter out invalid tickets.
    let valid_tickets = tickets
        .iter()
        .filter(|&ticket| {
            !ticket.iter().any(|num| {
                ranges
                    .values()
                    .all(|(r1, r2)| !(r1.0..=r1.1).contains(num) && !(r2.0..=r2.1).contains(num))
            })
        })
        .collect::<Vec<_>>();

    let mut idx2token = HashMap::new(); // map num index to rule
    let mut queue = (0..my_ticket.len()).collect::<VecDeque<_>>();
    let mut occupied = HashSet::new(); // rules already matched

    while !queue.is_empty() {
        let idx = queue.pop_front().unwrap();
        let mut num_matches = 0;
        let mut found = None;
        for (&token, &(r1, r2)) in &ranges {
            if occupied.contains(token) {
                continue;
            }
            if valid_tickets.iter().all(|&ticket| {
                (r1.0..=r1.1).contains(&ticket[idx]) || (r2.0..=r2.1).contains(&ticket[idx])
            }) {
                num_matches += 1;
                found = Some(token);
            }
        }
        assert!(num_matches > 0, "could not find match for idx={idx}");
        if num_matches == 1 {
            let token = found.unwrap();
            idx2token.insert(idx, token);
            occupied.insert(token);
        } else {
            queue.push_back(idx); // schedule for retry
        }
    }

    idx2token
        .iter()
        .filter(|&x| x.1.starts_with("departure"))
        .map(|x| my_ticket[*x.0])
        .product::<usize>()
}
