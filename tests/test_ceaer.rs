extern crate feaer;
extern crate libc;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::io::Read;
use std::str;
use std::path::Path;

#[test]
fn test_launch_echo() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let tmp_path = Path::new(&pathname);
    let rc = bar.executable_set(&tmp_path);
    bar.argv.push(String::from("/bin/echo"));
    bar.argv.push(String::from("lovely"));
    bar.argv.push(String::from("jovely"));
    let _ = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));

    match rc {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    {
        let mut redirect_file_id: File;
        match process.redirect_file(1) {
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
        let test_data = "lovely jovely\n".to_string();
        assert!(output == test_data);
    }
    let _result = process.wait();
}

#[test]
fn test_launch_cat_stdin_stdout() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/cat");
    let tmp_path = Path::new(&pathname);
    let _ = bar.executable_set(&tmp_path);
    bar.argv.push(String::from("/bin/cat"));
    let _ = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectWrite));
    let _ = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    {
        let mut redirect_file_id: File;
        match process.redirect_file(0) {
            Some(v) => {
                redirect_file_id = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let readrc = redirect_file_id.write(b"hello\n");
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
    }
    {
        let mut redirect_file_id: File;
        match process.redirect_file(1) {
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
        let test_data = "hello\n".to_string();
        assert!(output == test_data);
    }
    let _result = process.wait();
}

#[test]
fn test_ceaer_launch2() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let tmp_path = Path::new(&pathname);
    let rc = bar.executable_set(&tmp_path);
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
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    let result = process.wait();
    match result {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
fn test_launch_with_4() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/echo");
    let tmp_path = Path::new(&pathname);
    let rc = bar.executable_set(&tmp_path);
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
    let rd4 = bar.redirect_set(3, None, Some(feaer::RedirectType::RedirectRead));
    match rd4 {
        Ok(_) => {}
        Err(_) => {
            assert!(false);
        }
    }
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    let redirect_fd_rc = process.redirect_fd(1);
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

    assert!(read_bytes > 0);
    let s = match str::from_utf8(&buf[0..read_bytes]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let output = s.to_string();
    let test_data = "one two\n".to_string();
    assert!(output == test_data);
}



#[test]
fn test_launch_cat_stdin_stdout_kill() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/usr/bin/yes");
    let tmp_path = Path::new(&pathname);
    let _ = bar.executable_set(&tmp_path);
    bar.argv.push(String::from("/usr/bin/yes"));
    let _ = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectWrite));
    let _ = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    {
        let mut redirect_file_id: File;
        match process.redirect_file(0) {
            Some(v) => {
                redirect_file_id = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let readrc = redirect_file_id.write(b"hello\n");
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
    }
    {
        let mut redirect_file_id: File;
        match process.redirect_file(1) {
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
        assert!(read_bytes > 0);
    }
    let _result = process.signal(1);
}
