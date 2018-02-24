extern crate feaer;
extern crate libc;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use libc::c_int;
use std::io::Read;

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


    let jon = bar.redirect_fd(0);
    let redirect_file_id: c_int;
    match jon {
        Some(v) => {
            redirect_file_id = v as c_int;
        }
        None => {
            assert!(false);
            return;
        }
    }
    let result = bar.wait();
}





#[test]
fn test_launch2() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
    bar.argv.push(String::from("test_launch2"));
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rd1 = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectMirror));
    match rd1 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }

    let rd2 = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectMirror));
    match rd2 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rd3 = bar.redirect_set(2, None, Some(feaer::RedirectType::RedirectMirror));
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
    let result = bar.wait();
    println!("result={:?}=result", result);
}




#[test]
fn test_launch3() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
    bar.argv.push(String::from("xjjjjjjklk"));
    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let rd1 = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectMirror));
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
    let rd3 = bar.redirect_set(2, None, Some(feaer::RedirectType::RedirectMirror));
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
    let redirect_fd_rc = bar.redirect_fd(1);
    let fd_redirect: i32;
    match redirect_fd_rc {
        Some(v) => {
            fd_redirect = v as i32;
        }
        None => {
            assert!(false);
            return;
        }
    }
    println!("result={:?}=result", fd_redirect);


    let mut contents = String::new();

    let mut george: std::fs::File;

    unsafe {
        let mut bill = File::from_raw_fd(fd_redirect);
        for c in bill.bytes() {
            println!("line:{}", c.unwrap());
        }
        println!("nodata:");
    }
    println!("With text:\n{}", contents);

}
