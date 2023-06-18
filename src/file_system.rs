use crate::node::Kfile;

use super::node::Knode;
use std::collections::VecDeque;
pub struct KfileSystem {
    root: Knode,
    pwd: String,
}

impl KfileSystem {
    pub fn new() -> KfileSystem {
        KfileSystem {
            root: Knode::new_kfolder(),
            pwd: String::new(),
        }
    }

    fn sanitize(s: &str) -> VecDeque<String> {
        let mut s: VecDeque<String> = s.trim().split("/").map(String::from).collect();

        let mut q = VecDeque::new();
        let mut n = 0;
        while let Some(v) = s.pop_back() {
            match v.as_str() {
                "" => continue,
                "." => continue,
                ".." => n += 1,
                _ => {
                    if n > 0 {
                        n -= 1;
                    } else {
                        q.push_front(v);
                    }
                }
            }
        }

        q
    }

    fn sanitize_str(s: String) -> String {
        let v: Vec<String> = KfileSystem::sanitize(&s).into_iter().collect();
        v.join("/")
    }

    fn get_mut(&mut self, s: &str) -> Option<&mut Knode> {
        let mut s = KfileSystem::sanitize(&self.to_abs_path(s));
        let mut ans = Some(&mut self.root);

        while let Some(token) = s.pop_front() {
            if let Some(Knode::Kfolder(v)) = ans {
                ans = v.children.get_mut(&token);
            } else {
                return None;
            }
        }

        ans
    }

    fn get(&self, s: &str) -> Option<&Knode> {
        let mut s = KfileSystem::sanitize(&self.to_abs_path(s));
        let mut ans = Some(&self.root);

        while let Some(token) = s.pop_front() {
            if let Some(Knode::Kfolder(v)) = ans {
                ans = v.children.get(&token);
            } else {
                return None;
            }
        }

        ans
    }

    pub fn pwd(&self) -> Result<String, String> {
        let mut s = String::from("/") + &self.pwd;
        Ok(s)
    }

    fn ls(&self, args: &Vec<&str>) -> Result<String, String> {
        let s = if args.len() >= 2 { args[1] } else { "" };

        if let Some(Knode::Kfolder(v)) = self.get(s) {
            Ok(v.ls())
        } else {
            unreachable!("Expected Kfolder variant");
        }
    }

    fn mkdir(&mut self, args: &Vec<&str>) -> Result<(), String> {
        let mut s = KfileSystem::sanitize(&self.to_abs_path(args[1]));
        let mut f = Some(&mut self.root);

        while let Some(token) = s.pop_front() {
            if let Some(Knode::Kfolder(v)) = f {
                match v.children.get(&token) {
                    Some(Knode::Kfile(p)) => {
                        return Err(format!("File '{}' exists!", token));
                    }
                    None => {
                        v.children.insert(token.clone(), Knode::new_kfolder());
                    }
                    _ => {}
                }
                f = v.children.get_mut(&token);
            } else {
                unreachable!("f should always be a folder!");
            };
        }

        Ok(())
    }

    fn cd(&mut self, args: &Vec<&str>) -> Result<(), String> {
        if args.len() == 1 {
            self.pwd.clear();
            return Ok(());
        }

        if self.get(args[1]).is_some() {
            self.pwd = self.to_abs_path(args[1]);
            return Ok(());
        } else {
            return Err(format!("cd: '{}': No such file or directory", args[1]));
        }
    }

    fn to_abs_path(&self, s: &str) -> String {
        if let Some(v) = s.chars().next() {
            if v == '/' {
                return String::from(s);
            } else {
                return KfileSystem::sanitize_str(String::from("/") + &self.pwd + &"/" + s);
            }
        } else {
            return KfileSystem::sanitize_str(String::from("/") + &self.pwd);
        }
    }

    fn tokenize(s: &str) -> Vec<&str> {
        s.trim().split_whitespace().collect()
    }

    fn call_none<F>(nargs: usize, args: &Vec<&str>, f: &mut F)
    where
        F: FnMut(&Vec<&str>) -> Result<(), String>,
    {
        if args.len() < nargs + 1 {
            eprintln!("{}: missing operand", args[0]);
            return;
        }

        if let Err(e) = f(args) {
            eprintln!("{}: {}", args[0], e);
        }
    }

    fn call_string<F>(nargs: usize, args: &Vec<&str>, f: &mut F)
    where
        F: FnMut(&Vec<&str>) -> Result<String, String>,
    {
        if args.len() < nargs + 1 {
            eprintln!("{}: missing operand", args[0]);
            return;
        }

        match f(args) {
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(e) => {
                eprintln!("{}: {}", args[0], e);
            }
        }
    }

    pub fn call(&mut self, cmd: &str) {
        let args = KfileSystem::tokenize(cmd);

        if args.is_empty() {
            return;
        }

        match args[0] {
            "ls" => KfileSystem::call_string(0, &args, &mut |args: &Vec<&str>| self.ls(args)),
            "pwd" => KfileSystem::call_string(0, &args, &mut |args: &Vec<&str>| self.pwd()),
            "mkdir" => KfileSystem::call_none(1, &args, &mut |args: &Vec<&str>| self.mkdir(args)),
            "cd" => KfileSystem::call_none(0, &args, &mut |args: &Vec<&str>| self.cd(args)),
            _ => eprintln!("Command '{}' not found", args[0]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Kfile;

    use super::super::node::Knode;
    use super::KfileSystem;
    use std::collections::VecDeque;

    #[test]
    fn test_sanitize() {
        assert_eq!(
            KfileSystem::sanitize("/../../pera/aieie/../asa/../.././miao"),
            VecDeque::from(vec![String::from("miao")])
        );
        assert_eq!(KfileSystem::sanitize(""), VecDeque::from(vec![]));
        assert_eq!(KfileSystem::sanitize("."), VecDeque::from(vec![]));
        assert_eq!(KfileSystem::sanitize("./."), VecDeque::from(vec![]));
        assert_eq!(KfileSystem::sanitize("./.././.."), VecDeque::from(vec![]));
        assert_eq!(KfileSystem::sanitize(".."), VecDeque::from(vec![]));
        assert_eq!(KfileSystem::sanitize("../../.."), VecDeque::from(vec![]));
    }

    #[test]
    fn test_to_abs_path() {
        let mut fs = KfileSystem::new();
        fs.pwd = String::from("/miao/bau");

        assert_eq!(fs.to_abs_path("one/two"), "/miao/bau/one/two");
        assert_eq!(fs.to_abs_path("/one/two"), "/one/two");
        assert_eq!(fs.to_abs_path("/miao/bau/bau.txt"), "/miao/bau/bau.txt");

        fs.pwd.clear();
        assert_eq!(fs.to_abs_path("bau"), "/bau");
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(KfileSystem::tokenize("mkdir bau"), vec!["mkdir", "bau"]);
        assert_eq!(KfileSystem::tokenize("mkdir  bau"), vec!["mkdir", "bau"]);
        assert_eq!(KfileSystem::tokenize("mkdir bau "), vec!["mkdir", "bau"]);
        assert_eq!(KfileSystem::tokenize("mkdir bau\n"), vec!["mkdir", "bau"]);
        assert_eq!(KfileSystem::tokenize("mkdir bau \n"), vec!["mkdir", "bau"]);
        assert_eq!(KfileSystem::tokenize(""), Vec::<&str>::new());
        assert_eq!(
            KfileSystem::tokenize("\n mkdir \n bau \n"),
            vec!["mkdir", "bau"]
        );
    }

    #[test]
    fn test_get_mut() {
        let mut fs = KfileSystem::new();
        if let Some(Knode::Kfolder(v)) = fs.get_mut("") {
            v.children
                .insert(String::from("miao"), Knode::new_kfolder());
            assert_eq!(v.ls(), "miao");
        } else {
            unreachable!("Expected Kfolder variant");
        }

        if let Some(Knode::Kfolder(v)) = fs.get_mut("miao") {
            v.children.insert(String::from("bau"), Knode::new_kfolder());
            assert_eq!(v.ls(), "bau");
        } else {
            unreachable!("Expected Kfolder variant");
        }

        if let Some(Knode::Kfolder(v)) = fs.get_mut("") {
            v.children.insert(String::from("bau"), Knode::new_kfolder());
            assert_eq!(v.ls(), "bau miao");
        } else {
            unreachable!("Expected Kfolder variant");
        }

        if let Some(Knode::Kfolder(v)) = fs.get_mut("miao/bau") {
            v.children
                .insert(String::from("bau.txt"), Knode::new_kfile());
            assert_eq!(v.ls(), "bau.txt");
        } else {
            unreachable!("Expected Kfolder variant");
        }

        if let Some(Knode::Kfile(v)) = fs.get_mut("miao/bau/bau.txt") {
        } else {
            unreachable!("Expected Kfile variant");
        }

        assert!(fs.get_mut("miao/bau.txt").is_none());
        assert!(fs.get_mut("/miao/bau/bau.txt").is_some());
        assert!(fs.get_mut("/miao/bau.txt/bau.txt").is_none());
    }
}
