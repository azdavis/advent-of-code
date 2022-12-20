use helpers::static_regex;
use helpers::topo_sort::{self, Graph};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

static_regex!(RE = r#"^Step (\w) must be finished before step (\w) can begin.$"#);

type JobId = u8;

fn parse(s: &str) -> impl Iterator<Item = (JobId, JobId)> + '_ {
  s.lines().map(|line| {
    let caps = RE.captures(line).unwrap();
    let a = caps[1].bytes().next().unwrap();
    let b = caps[2].bytes().next().unwrap();
    (a, b)
  })
}

pub fn p1(s: &str) -> String {
  let mut graph = Graph::<JobId>::new();
  for (a, b) in parse(s) {
    graph.entry(a).or_default().insert(b);
  }
  let sorted = topo_sort::get(&graph).unwrap();
  String::from_utf8(sorted.into_iter().rev().collect()).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Job {
  finish_time: Reverse<usize>,
  id: Reverse<JobId>,
}

impl Job {
  fn new(id: JobId, finish_time: usize) -> Job {
    Job {
      finish_time: Reverse(finish_time),
      id: Reverse(id),
    }
  }
}

fn p2_help(s: &str, mut workers: usize, extra_duration: usize) -> usize {
  let mut dependents = Graph::<JobId>::new();
  let mut dependencies = Graph::<JobId>::new();
  for (a, b) in parse(s) {
    dependents.entry(a).or_default().insert(b);
    dependents.entry(b).or_default();
    dependencies.entry(b).or_default().insert(a);
    dependencies.entry(a).or_default();
  }
  let mut pq = BinaryHeap::<Job>::new();
  let mut cur_time = 0usize;
  loop {
    let can_start: Vec<_> = dependencies
      .iter()
      .filter_map(|(&id, ds)| ds.is_empty().then_some(id))
      .take(workers)
      .collect();
    workers -= can_start.len();
    for id in can_start {
      dependencies.remove(&id);
      let job_duration = usize::from(id - b'A') + 1 + extra_duration;
      pq.push(Job::new(id, cur_time + job_duration));
    }
    let finished_job = match pq.pop() {
      None => return cur_time,
      Some(j) => {
        cur_time = j.finish_time.0;
        j.id.0
      }
    };
    workers += 1;
    for job in dependents[&finished_job].iter() {
      dependencies.get_mut(job).unwrap().remove(&finished_job);
    }
  }
}

pub fn p2(s: &str) -> usize {
  p2_help(s, 5, 60)
}

#[test]
fn t() {
  let s = include_str!("input/d07.txt");
  assert_eq!(p1(s), "GNJOCHKSWTFMXLYDZABIREPVUQ");
  assert_eq!(p2(s), 886);
}

#[test]
fn ex1_p1() {
  let s = include_str!("input/d07_ex1.txt");
  assert_eq!(p1(s), "CABDFE");
}

#[test]
fn ex1_p2() {
  let s = include_str!("input/d07_ex1.txt");
  assert_eq!(p2_help(s, 2, 0), 15);
}
