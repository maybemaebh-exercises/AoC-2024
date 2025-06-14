use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

pub struct CharGrid {
    pub chars: Vec<char>,
    pub bounds: [usize; 2]
}

impl CharGrid {
    pub fn index(&self, quard: Uquard) -> Option<&char> {
        if quard.0>=self.bounds[0] {None} else {self.chars.get(quard.1*(self.bounds[0]) + quard.0)}
    }

    pub fn index_mut(&mut self, quard: Uquard) -> Option<&mut char> {
        if quard.0>=self.bounds[0] {None} else {self.chars.get_mut(quard.1*(self.bounds[0]) + quard.0)}
    }

    pub fn vec_index_to_uquard(&self, index: usize) -> Uquard {
        Uquard(index % self.bounds[0], index / self.bounds[0])
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        #[cfg(test)]
        {
            println!();
            for y in 0..self.bounds[1] {
                for x in 0..self.bounds[0] {
                    print!("{}", { self.chars[y*(self.bounds[0]) + x] });
                }
                println!();
            }
        }
    }

    pub fn new(input: &str) -> CharGrid {
        let mut chars: Vec<char> = Vec::with_capacity(input.len());
        chars.extend(input.chars().filter(|x| !(x==&'\n'||x==&'\r')));
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().filter(|x| !(x==&'\n'||x==&'\r')).count(), input.lines().count()],
            chars
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uquard(pub usize, pub usize);
#[derive(Debug, Clone, Copy, Hash)]
pub struct Iquard(pub i32, pub i32);

impl Add for Uquard {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Uquard(self.0+rhs.0, self.1+rhs.1)
    }

}

// impl Hash for Uquard {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         (self.0*(self.1*1000)).hash(state);
//     }
// }

impl Sub for Uquard {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0>=rhs.0 && self.1>=rhs.1 {
            Some(Uquard(self.0-rhs.0, self.1-rhs.1))
        } else { None }
    }
}

// impl PartialEq<Self> for Uquard {
//     fn eq(&self, other: &Self) -> bool {
//         self.0==other.0 && self.1==other.1
//     }
// }
impl Add for Iquard {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Iquard(self.0+rhs.0, self.1+rhs.1)
    }

}