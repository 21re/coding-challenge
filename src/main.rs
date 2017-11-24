
#![feature(iterator_step_by)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate rand;

use rand::{ ThreadRng, Rng };
use rand::distributions::{IndependentSample, Range};
use rocket::response::Stream;
use std::io::{ self, Read };

pub struct ChallengeGenerator<R> {
  rng: R,
  ranges: u32,
  letter_remaining: usize,
  letter: u8,
  new_line_pending: bool
}

const GEN_ASCII_STR_CHARSET: &'static [u8] =
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl <R: Rng> ChallengeGenerator<R> {
  fn next_letter(&mut self) {
    let block_size_range = Range::new(1, 15);
    self.letter_remaining = block_size_range.ind_sample(&mut self.rng);
    self.letter = self.rng.choose(GEN_ASCII_STR_CHARSET).unwrap().clone();
    self.ranges = self.ranges - 1;
  }

}
impl <R: Rng> Read for ChallengeGenerator<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    if self.new_line_pending {
      self.new_line_pending = false;
      buf[0] = 10 as u8;
      return Ok(1);
    }

    if self.letter_remaining == 0 {
      self.next_letter();
    }

    if self.ranges < 1 {
      return Ok(0);
    }

    if buf.len() == 1 {
      buf[0] = self.letter;
      self.new_line_pending = true;
      self.letter_remaining = self.letter_remaining - 1;
      return Ok(1);
    }

    let out_size = std::cmp::min(buf.len() / 2, self.letter_remaining);
    for i in (0 .. (out_size * 2)).step_by(2) {
      buf[i] = self.letter;
      buf[i + 1] = 10 as u8;
    }
    self.letter_remaining = self.letter_remaining - out_size;
    return Ok(out_size * 2);
  }
}


#[get("/")]
fn index() -> io::Result<Stream<ChallengeGenerator<ThreadRng>>> {
  let mut rng = rand::thread_rng();
  let ranges_range = Range::new(2, 5);
  let ranges: u32 = ranges_range.ind_sample(&mut rng);
  let gen = ChallengeGenerator{rng: rng, ranges: ranges, letter_remaining: 0, letter: 0 as u8, new_line_pending: false};
  let s: Stream<ChallengeGenerator<ThreadRng>> = Stream::from(gen);
  Ok(s)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}



