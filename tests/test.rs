extern crate feaer;



#[test]
fn test_launch() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/ls");

    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
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
