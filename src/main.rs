use std::fs;
use std::process::Command;

fn main() {
    let args : Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        let paths = fs::read_dir("./").unwrap();

        //unwraps the item
        paths.map(|p| p.unwrap())
        //check if is a file and not a directory
        .filter(|e| e.metadata().unwrap().is_file())
        //get the file name
        .map(|x| x.file_name().into_string().unwrap())
        //only the .ts file 
        .filter(|s| s.ends_with(".ts"))
        //call tsc for each file 
        .for_each(|t|{
            let mut c = Command::new("tsc")
            .arg(t)
            .arg("--outDir")
            .arg("./src_js")
            .spawn()
            .expect("tsc command failed to start");
            let _ = c.wait();
        });

    //println!("Compiled to js successfully !")
    }
    else if (args[1] == "--clear") && (args.len() == 2){
            
            if let Ok(paths) = fs::read_dir("./src_js") {
    
            //unwraps the item
            paths.map(|p| p.unwrap())
            //check if is a file and not a directory
            .filter(|e| e.metadata().unwrap().is_file())
            //get the file name
            .map(|x| x.file_name().into_string().unwrap())
            //only the .ts file 
            .filter(|s| s.ends_with(".js"))
            //call tsc for each file 
            .for_each(|t|{
                let mut c =Command::new("rm")
                .arg(format!("./src_js/{}",t))
                .spawn()
                .expect("can not rm");

                let _ = c.wait();
            });

            
                let mut c = Command::new("rmdir")
                .arg("./src_js")
                .spawn()    
                .expect("can not rmdir");

                let _ = c.wait();
            }
            else {
                println!("Nothing to be done !");
            }

    }

    else {
        println!();
        println!("Improper use");
        println!();
        println!("ftsc \nto compile all .ts file in the current directory");
        println!();
        println!("ftsc --clear \nto clear all compiled .js file in the \"src_js\" directory");
    }
}
