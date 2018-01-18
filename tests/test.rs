extern crate process_watcher;



#[test]
fn test_launch() {
    let foo = process_watcher::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/ls");
    let rc = bar.executable_set(&pathname);
    match rc {
    	Ok(_) => {
    		
    	}
    	Err(_) => {
            assert!(false);
        }
    }
    let rc = bar.launch();
    match rc {
    	Ok(_) => {
    		
    	}
    	Err(_) => {
            assert!(false);
        }
    }
    let result = bar.wait();
    println!("{:?}", result);
}
