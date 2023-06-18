use std::collections::HashMap;
#[derive(Debug)]
pub enum Knode {
    Kfolder(Kfolder),
    Kfile(Kfile),
}

impl Knode {
    pub fn new_kfolder() -> Knode {
        Knode::Kfolder(Kfolder::new())
    }
    pub fn new_kfile() -> Knode {
        Knode::Kfile(Kfile::new())
    }

    pub fn is_kfolder(&self) -> bool {
        match self {
            Kfolder => return true,
            _ => return false,
        }
    }
    pub fn is_kfile(&self) -> bool {
        match self {
            Kfile => return true,
            _ => return false,
        }
    }

    pub fn as_kfolder_mut(&mut self) -> Option<&mut Kfolder> {
        match self {
            Knode::Kfolder(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_kfile_mut(&mut self) -> Option<&mut Kfile> {
        match self {
            Knode::Kfile(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_kfolder(&self) -> Option<&Kfolder> {
        match self {
            Knode::Kfolder(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_kfile(&self) -> Option<&Kfile> {
        match self {
            Knode::Kfile(f) => Some(f),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Kfile {
    text: String,
}

impl Kfile {
    pub fn new() -> Kfile {
        Kfile {
            text: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Kfolder {
    pub children: HashMap<String, Knode>,
}

impl Kfolder {
    pub fn new() -> Kfolder {
        Kfolder {
            children: HashMap::new(),
        }
    }

    pub fn ls(&self) -> String {
        let mut v = Vec::new();
        v.extend(self.children.keys().cloned());
        v.sort();
        v.join(" ")
    }

    pub fn insert(&mut self, k: String, v: Knode) -> bool {
        let ans = self.children.contains_key(&k);
        if !ans {
            self.children.insert(k, v);
        }
        !ans
    }
}

#[cfg(test)]
mod tests {
    use super::Kfolder;
    use super::Knode;

    #[test]
    fn test_insert() {
        let mut f = Kfolder::new();
        assert!(f.insert(String::from("bau"), Knode::new_kfolder()));
        assert!(f.insert(String::from("zio"), Knode::new_kfolder()));
        assert!(!f.insert(String::from("zio"), Knode::new_kfolder()));
        assert!(!f.insert(String::from("zio"), Knode::new_kfile()));
        assert!(f.insert(String::from("miao"), Knode::new_kfile()));
    }

    #[test]
    fn test_ls() {
        let mut f = Kfolder::new();
        assert!(f.insert(String::from("bau"), Knode::new_kfolder()));
        assert!(f.insert(String::from("zio"), Knode::new_kfolder()));
        assert!(!f.insert(String::from("zio"), Knode::new_kfolder()));
        assert!(!f.insert(String::from("zio"), Knode::new_kfile()));
        assert!(f.insert(String::from("miao"), Knode::new_kfile()));
        assert_eq!(f.ls(), "bau miao zio");
    }
}
