pub mod event;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use tui::widgets::ListState;

#[derive(Clone)]
pub struct RandomSignal {
  distribution: Uniform<u64>,
  rng: ThreadRng
}

impl RandomSignal {
  pub fn new(lower: u64, upper: u64) -> RandomSignal {
    RandomSignal {
      distribution: Uniform::new(lower, upper),
      rng: rand::thread_rng(),
    }
  }
}

impl Iterator for RandomSignal {
  type Item = u64;
  fn next(&mut self) -> Option<u64> {
    Some(self.distribution.sample(&mut self.rng))
  }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub trait AsVec<T> {
    fn as_vec() -> Vec<T>;
}

pub struct TabsState<E: AsVec<E> + Clone + Copy> {
    pub titles: Vec<E>,
    pub index: usize,
    pub current: E,
}

impl<E: AsVec<E> + Clone + Copy> TabsState<E> {
    pub fn new(default: E) -> TabsState<E> {
        TabsState { titles: E::as_vec(), index: 0, current: default }
    }
    pub fn from(titles: Vec<E>, default: E) -> TabsState<E> {
        TabsState { titles, index: 0, current: default }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
        self.current = self.titles[self.index];
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
        self.current = self.titles[self.index];
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
