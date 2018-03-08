extern crate feaer;
extern crate libc;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use libc::c_int;
use std::io::Read;
use std::str;

#[test]
fn test_launch() {
    let foo = feaer::Launcher::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");

    let rc = bar.executable_set(&pathname);
    bar.argv.push(String::from("/bin/echo"));
    bar.argv.push(String::from("/bin/echo"));
    let rd1 = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));
    match rd1 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }

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
    {
        let mut redirect_file_id: File;
        match bar.redirect_file(1) {
            Some(v) => {
                redirect_file_id = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let mut buf: [u8; 1024] = [0; 1024];
        let readrc = redirect_file_id.read(&mut buf);
        let read_bytes = match readrc {
            Ok(v) => v,
            Err(e) => {
                panic!("Invalid read: {}", e);
            }
        };

        if read_bytes == 0 {
            println!("read_bytes={:?}", read_bytes);
            return;
        }
        let s = match str::from_utf8(&buf[0..read_bytes]) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let output = s.to_string();
        let test_data = "/bin/echo\n".to_string();
        assert!(output == test_data);
    }
    let _result = bar.wait();
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
    bar.argv.push(String::from("one"));
    bar.argv.push(String::from("two"));
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
    let mut bill: File;
    let mut buf: [u8; 1024] = [0; 1024];
    let read_bytes;
    unsafe {
        bill = File::from_raw_fd(fd_redirect);

        let readrc = bill.read(&mut buf);
        read_bytes = match readrc {
            Ok(v) => v,
            Err(e) => {
                panic!("Invalid read: {}", e);
            }
        };
    }
    if read_bytes == 0 {
        println!("read_bytes={:?}", read_bytes);
        return;
    }
    let s = match str::from_utf8(&buf[0..read_bytes]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let output = s.to_string();
    let test_data = "one two\n".to_string();
    assert!(output == test_data);
}
