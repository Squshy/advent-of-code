// Five of a kind (all same)
// Four of a kind (four save)
// Full house (3 cards have same label, remaining 2 share a label)
// Three of a kind (3 cards have same label, remaining 2 do not share a label)
// Two pair (2 cards share a label and another 2 share a label)
// One pair (2 cards share a label, other 3 cards do not share anythign with anyone)
// High card (all cards are distinct)
//
// These are ranked in order shown above (first is highest)

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Tiger = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl From<&u8> for Card {
    fn from(value: &u8) -> Self {
        match *value as char {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Tiger,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[u8]> for HandType {
    fn from(value: &[u8]) -> Self {
        let mut chars: Vec<(Card, u8)> = vec![];

        value.iter().for_each(|ch| {
            let ch: Card = ch.into();

            if let Some(idx) = chars.iter().position(|(c, _)| c == &ch) {
                chars[idx] = (ch, chars[idx].1 + 1);
            } else {
                chars.push((ch, 1));
            }
        });

        let max_val = chars.iter().max_by_key(|&(_, val)| val).unwrap().1;
        match chars.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if max_val == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if max_val == 3 {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u32,
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            for idx in 0..5 {
                let my_card = &self.cards[idx];
                let their_card = &other.cards[idx];

                if my_card != their_card {
                    return my_card.cmp(&their_card);
                }
            }

            std::cmp::Ordering::Equal
        }
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            Some(self.hand_type.cmp(&other.hand_type))
        } else {
            for idx in 0..5 {
                let my_card = &self.cards[idx];
                let their_card = &other.cards[idx];

                if my_card != their_card {
                    return Some(my_card.cmp(&their_card));
                }
            }

            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Hand {
    fn new(cards: &[u8], bid: u32) -> Self {
        Self {
            hand_type: cards.into(),
            cards: [
                Card::from(&cards[0]),
                Card::from(&cards[1]),
                Card::from(&cards[2]),
                Card::from(&cards[3]),
                Card::from(&cards[4]),
            ],
            bid,
        }
    }
}

fn process() -> u32 {
    let input = include_bytes!("../../data/input.txt");
    let sep = input.iter().position(|&b| b == b' ').unwrap();

    let mut hands = input
        .split(|&b| b == b'\n')
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let bid = atoi::atoi::<u32>(&line[sep + 1..]).unwrap();
            let hand = Hand::new(&line[..sep], bid);

            hand
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum::<u32>()
}

fn main() {
    println!("{}", process());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks() {
        let lines = r#"AAAAA
AA8AA
23332
TTT98
23432
A23A4
23456"#
            .as_bytes();

        let hands = lines
            .split(|&b| b == b'\n')
            .take_while(|line| !line.is_empty())
            .map(|line| Hand::new(&line[..5], 1))
            .collect::<Vec<Hand>>();

        assert_eq!(hands.len(), 7);
        assert_eq!(hands[0].hand_type, HandType::FiveOfAKind);
        assert_eq!(hands[1].hand_type, HandType::FourOfAKind);
        assert_eq!(hands[2].hand_type, HandType::FullHouse);
        assert_eq!(hands[3].hand_type, HandType::ThreeOfAKind);
        assert_eq!(hands[4].hand_type, HandType::TwoPair);
        assert_eq!(hands[5].hand_type, HandType::OnePair);
        assert_eq!(hands[6].hand_type, HandType::HighCard);
    }

    #[test]
    fn cards() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Tiger);
        assert!(Card::Tiger > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
    }

    #[test]
    fn types() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_eq() {
        let h1 = Hand {
            hand_type: HandType::TwoPair,
            cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
            bid: 1,
        };
        let h2 = Hand {
            hand_type: HandType::TwoPair,
            cards: [Card::King, Card::Tiger, Card::Jack, Card::Jack, Card::Tiger],
            bid: 1,
        };

        assert!(h1 > h2);
    }
}
