use helpers::HashMap;

#[derive(Default)]
struct DirEntry<'a> {
  dirs: HashMap<&'a str, DirEntry<'a>>,
  files: HashMap<&'a str, FileEntry>,
}

struct FileEntry {
  size: u32,
}

const ROOT_DIR: &str = "/";

fn parse_entries(s: &str) -> DirEntry<'_> {
  let mut cur = vec![(ROOT_DIR, DirEntry::default())];
  let mut inside_ls = false;
  for line in s.lines() {
    match line.strip_prefix("$ ") {
      None => {
        assert!(inside_ls);
        let (fst, snd) = line.split_once(' ').unwrap();
        // no need to explicitly record dir entries from ls info
        if fst != "dir" {
          let size = fst.parse::<u32>().unwrap();
          let (_, parent) = cur.last_mut().unwrap();
          assert!(parent.files.insert(snd, FileEntry { size }).is_none());
        }
      }
      Some(line) => {
        if line == "ls" {
          assert!(!inside_ls, "should not ls twice in a row");
          inside_ls = true;
        } else {
          inside_ls = false;
          let (cd, dir) = line.split_once(' ').unwrap();
          assert_eq!(cd, "cd");
          if dir == ".." {
            let (name, done) = cur.pop().unwrap();
            let (_, parent) = cur.last_mut().unwrap();
            assert!(parent.dirs.insert(name, done).is_none());
          } else {
            cur.push((dir, DirEntry::default()));
          }
        }
      }
    }
  }
  loop {
    let (name, done) = cur.pop().unwrap();
    #[allow(clippy::single_match_else)]
    match cur.last_mut() {
      Some((_, parent)) => assert!(parent.dirs.insert(name, done).is_none()),
      None => {
        assert_eq!(name, ROOT_DIR);
        return done;
      }
    }
  }
}

fn go<F>(dir: &DirEntry<'_>, f: &mut F) -> u32
where
  F: FnMut(u32),
{
  let size: u32 = dir
    .dirs
    .values()
    .map(|x| go(x, f))
    .chain(dir.files.values().map(|x| x.size))
    .sum();
  f(size);
  size
}

pub fn p1(s: &str) -> u32 {
  let e = parse_entries(s);
  let mut ret = 0u32;
  go(&e, &mut |size| {
    if size <= 100_000 {
      ret += size;
    }
  });
  ret
}

pub fn p2(_: &str) -> u32 {
  0
}

#[test]
fn t() {
  let s = include_str!("input/d07.txt");
  assert_eq!(p1(s), 1_428_881);
  assert_eq!(p2(s), 0);
}
