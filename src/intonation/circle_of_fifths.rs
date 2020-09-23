/// Tonal Octave Space
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CircleOfFifths {
    tone_count: usize,
    tone: usize,
    starting_tone: usize,
    sent_root: bool,
}

impl CircleOfFifths {
    pub fn new(tone_count: usize, starting_tone: usize) -> Self {
        Self {
            tone_count,
            starting_tone,
            tone: starting_tone,
            sent_root: false,
        }
    }

    pub fn fifth(&self) -> usize {
        self.tone_count - 5
    }

    pub fn tone_count(&self) -> usize {
        self.tone_count
    }

    pub fn tone(&self, index: usize) -> usize {
        (self.starting_tone + (index * self.fifth())) % self.tone_count()
    }

    pub fn index(&self, tone: usize) -> usize {
        let copy = Self {
            tone_count: self.tone_count,
            starting_tone: self.starting_tone,
            tone: self.starting_tone,
            sent_root: false,
        };

        let m = tone % self.tone_count();
        for (i, t) in copy.enumerate() {
            if t == m {
                return i
            }
        }
        panic!("the index {} cannot be reached, but this should be impossible!?");
    }
}

impl Iterator for CircleOfFifths {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if !self.sent_root {
            self.sent_root = true;
            return Some(self.tone);
        }

        self.tone = (self.tone + self.fifth()) % self.tone_count;
        if self.tone != self.starting_tone {
            return Some(self.tone);
        }
        return Option::None;
    }
}

#[test]
fn cof_test() {
    let circle = CircleOfFifths::new(12, 0);
    println!("circle {} has an interval of {}",circle.tone_count(), circle.fifth());
    for i in circle {
        println!("{}", i);
    }

    let tone = circle.tone(9);
    let index = circle.index(8);
    println!("tone at 9 = {}", tone);
    println!("index at 8 = {}", index);
}