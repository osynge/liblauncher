extern crate feaer;
extern crate libc;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::str;
use std::path::Path;

#[test]
fn test_launch_with_bash() {
    let foo = feaer::Ceaer::new();
    let mut bar = foo.unwrap();
    let pathname = String::from("/bin/bash");
    let tmp_path = Path::new(&pathname);
    let _ = bar.executable_set(&tmp_path);
    bar.arg("/bin/bash");
    let _ = bar.redirect_set(0, None, Some(feaer::RedirectType::RedirectWrite));
    let _ = bar.redirect_set(1, None, Some(feaer::RedirectType::RedirectRead));
    let _ = bar.redirect_set(2, None, Some(feaer::RedirectType::RedirectRead));
    let _ = bar.redirect_set(3, None, Some(feaer::RedirectType::RedirectRead));
    let mut process: feaer::Process;
    let rc = bar.launch();
    match rc {
        Ok(j) => process = j,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    let mut redirected_stdin: File;
    {
        match process.redirect_file(0) {
            Some(v) => {
                redirected_stdin = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let readrc = redirected_stdin.write(b"echo stdout >&1\n");
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
        match redirected_stdin.flush() {
            Ok(_) => {}
            Err(_) => {
                assert!(false);
                return;
            }
        };
    }

    let readrc = redirected_stdin.write(b"echo stderr 1>&2;\n");
    let readrc = redirected_stdin.write(b"echo fd3 1>&3;\n");
    let mut redirected_stdout: File;
    {
        match process.redirect_file(1) {
            Some(v) => {
                redirected_stdout = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let mut buf: [u8; 1024] = [0; 1024];
        let readrc = redirected_stdout.read(&mut buf);
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
        let test_data = "stdout\n".to_string();
        assert!(output == test_data);
    }
    let mut redirected_stderr: File;
    {
        match process.redirect_file(2) {
            Some(v) => {
                redirected_stderr = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let mut buf: [u8; 1024] = [0; 1024];
        let readrc = redirected_stderr.read(&mut buf);
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
        let test_data = "stderr\n".to_string();
        println!("output={:?}", output);
        assert!(output == test_data);
    }
    let mut redirected_fd3: File;
    {
        match process.redirect_file(3) {
            Some(v) => {
                redirected_fd3 = v;
            }
            None => {
                assert!(false);
                return;
            }
        }
        let mut buf: [u8; 1024] = [0; 1024];
        let readrc = redirected_fd3.read(&mut buf);
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
        let test_data = "fd3\n".to_string();
        println!("output={:?}", output);
        assert!(output == test_data);
    }
    let _result = process.wait();
}
