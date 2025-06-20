use std::hash::{Hash};
use std::ops::{Add, Sub};
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
//use ascii::*;

#[derive(Clone)]
pub struct CharGrid<T> where T: AsRef<AsciiStr> {
    pub chars: T,
    pub bounds: [usize; 2],
    pub newline_lengh: usize,
}



impl<T: AsRef<AsciiStr>> CharGrid<T> {

    pub fn vec_index_to_uquard(&self, index: usize) -> Ucoord {
        Ucoord(index % (self.bounds[0]+self.newline_lengh), index / (self.bounds[0]+self.newline_lengh))
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        //#[cfg(test)]
        {
            println!();
            for y in 0..self.bounds[1] {
                for x in 0..self.bounds[0] {
                    print!("{}", { self.chars.as_ref()[y*(self.bounds[0]) + x] });
                }
                println!();
            }
        }
    }


}

impl CharGrid<AsciiString> {
    pub fn index(&self, quard: Ucoord) -> Option<&AsciiChar> {
        self.index_usize(quard).map(|index| &self.chars[index])
    }

    pub fn index_usize(&self, quard: Ucoord) -> Option<usize> {
        if quard.0>=self.bounds[0] || quard.1>=self.bounds[1] {None} else {Some(quard.1*self.bounds[0] + quard.0)}
    }
    pub fn index_mut(&mut self, quard: Ucoord) -> Option<&mut AsciiChar> {
        self.index_usize(quard).map(|index| &mut self.chars[index])
    }
    pub fn new(input: &str) -> CharGrid<AsciiString> {
        let input = input.as_ascii_str().unwrap();
        let mut chars = AsciiString::with_capacity(input.len());
        chars.extend(input.chars().filter(|char| !(char ==&'\n'|| char ==&'\r')));
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().count(), input.lines().count()],
            chars,
            newline_lengh: 0,
        }
    }
}

impl CharGrid<&AsciiStr> {
    pub fn index(&self, quard: Ucoord) -> Option<&AsciiChar> {
        self.index_usize(quard).map(|index| &self.chars[index])
    }

    pub fn index_usize(&self, quard: Ucoord) -> Option<usize> {
        if quard.0>=self.bounds[0] || quard.1>=self.bounds[1] {None} else {Some(quard.1*(self.bounds[0]+self.newline_lengh) + quard.0)}
    }
    
    pub fn new(input: &str) -> CharGrid<&AsciiStr> {
        let input = input.as_ascii_str().unwrap();
        //let mut chars = AsciiString::with_capacity(input.len());
        //chars.extend(input.chars().filter(|char| !(char ==&'\n'|| char ==&'\r')));
        let mut newline_lengh = 0;
        if input.chars().any(|char| char == '\n') {newline_lengh += 1}
        if input.chars().any(|char| char == '\r') {newline_lengh += 1}
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().count(), input.lines().count()],
            chars: input,
            newline_lengh,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ucoord(pub usize, pub usize);
// #[derive(Debug, Clone, Copy, Hash)]
// pub struct Iquard(pub i32, pub i32);

impl Add for Ucoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Ucoord(self.0+rhs.0, self.1+rhs.1)
    }

}

// impl Hash for Uquard {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         (self.0*(self.1*1000)).hash(state);
//     }
// }

impl Sub for Ucoord {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0>=rhs.0 && self.1>=rhs.1 {
            Some(Ucoord(self.0-rhs.0, self.1-rhs.1))
        } else { None }
    }
}

// impl PartialEq<Self> for Uquard {
//     fn eq(&self, other: &Self) -> bool {
//         self.0==other.0 && self.1==other.1
//     }
// }
// impl Add for Iquard {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Iquard(self.0+rhs.0, self.1+rhs.1)
//     }
//
// }