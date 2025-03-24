
#[derive(Debug)]
pub struct Vmconfig {
    file_path : String,
    dump : bool,
    version : String
}

impl Vmconfig {
    pub fn build(args: Vec<String>) -> Result<Vmconfig, &'static str> {
        
        let mut d = false;
        let mut s: String = String::new();
        let mut ver : String = String::from("5.1");
        
        for i in 1..args.len() 
        {
            match args[i].as_str() {
                "-dump" | "-d" if d == false => { d = true; }
                "ver=5.1" | "ver=5.3" => { ver = String::from(args[i].as_str()).split_off(4); }
                other /* if other.ends_with(".out") */ => 
                {
                    if s.is_empty() {
                        s = String::from(other);
                    } else {
                        return Err("program takes a single .out files");
                    }
                }
                //_ => { return Err("program argument not reconized"); }
            }
        }
        
        Ok(Vmconfig {
            file_path : s,
            dump : d,
            version : ver
        })
    }

    pub fn get_dump(&self) -> bool {
        self.dump
    }

    pub fn get_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn get_ver(&self) -> u32 {
        let mut chrs = self.version.chars();
        const RADIX: u32 = 10;
        chrs.nth(0).unwrap().to_digit(RADIX).unwrap() * 16 + chrs.nth(1).unwrap().to_digit(RADIX).unwrap()
    }

}

