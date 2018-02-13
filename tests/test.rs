extern crate feaer;
extern crate libc;
use std::fs::File;
use libc::read;
use libc::c_void;
use libc::c_char;
use libc::size_t;


#[test]
fn test_launch() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/ls");

    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rc = bar.launch();
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let result = bar.wait();
    println!("{:?}", result);
}





#[test]
fn test_launch2() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
    bar.argv.push(String::from("x"));
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rd1 = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectWrite));
    match rd1 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }

    let rd2 = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));
    match rd2 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rd3 = bar.redirect_set(2, None, Some(feaer::RedirectType::RedirectRead));
    match rd3 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rc = bar.launch();
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rdfd1 = bar.redirect_fd(1);
    assert!(rdfd1 != None);
    match rdfd1 {
        Some(x) => {
            assert!(x != 0);
            let bill: File;
            let mut buff: [char; 1000] = ['\0'; 1000];
            let buff_ptr = buff.as_mut_ptr() as *mut libc::c_void;
            let fd = x as i32;
            //println!("{:?}", x);
            let count: libc::size_t = 1000;
            unsafe {
                let read_bytes = read(fd, buff_ptr, count);
            }
            println!("{:?}", buff[0]);
            assert!(buff[0] == '\n');
        }
        None => {}
    }
    let result = bar.wait();
    println!("{:?}", result);
}
