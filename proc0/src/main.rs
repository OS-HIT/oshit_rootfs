// user/src/bin/initproc.rs

#![no_std]
#![no_main]

#[macro_use]
extern crate oshit_usrlib;


use oshit_usrlib::{print, sys_exec, sys_exit, sys_fork, sys_getpid, sys_waitpid, sys_yield};

const busybox_commands : &'static [&'static [u8]] = &[
    b"echo \"#### independent command test\"\0",
    b"ash -c exit\0",
    b"sh -c exit\0",
    b"basename /aaa/bbb\0",
    b"cal\0",
    b"clear\0",
    b"date \0",
    b"df \0",
    b"dirname /aaa/bbb\0",
    b"dmesg \0",
    b"du\0",
    b"expr 1 + 1\0",
    b"false\0",
    b"true\0",
    b"which ls\0",
    b"uname\0",
    b"uptime\0",
    b"printf \"abc\n\"\0",
    b"ps\0",
    b"pwd\0",
    b"free\0",
    b"hwclock\0",
    b"kill 10\0",
    b"ls\0",
    b"sleep 1\0",
    b"echo \"#### file opration test\"\0",
    b"touch test.txt\0",
    b"echo \"hello world\" > test.txt\0",
    b"cat test.txt\0",
    b"cut -c 3 test.txt\0",
    b"od test.txt\0",
    b"head test.txt\0",
    b"tail test.txt \0",
    b"hexdump -C test.txt \0",
    b"md5sum test.txt\0",
    b"echo \"ccccccc\" >> test.txt\0",
    b"echo \"bbbbbbb\" >> test.txt\0",
    b"echo \"aaaaaaa\" >> test.txt\0",
    b"echo \"2222222\" >> test.txt\0",
    b"echo \"1111111\" >> test.txt\0",
    b"echo \"bbbbbbb\" >> test.txt\0",
    b"sort test.txt | ./busybox uniq\0",
    b"stat test.txt\0",
    b"strings test.txt \0",
    b"wc test.txt\0",
    b"[ -f test.txt ]\0",
    b"more test.txt\0",
    b"rm test.txt\0",
    b"mkdir test_dir\0",
    b"mv test_dir test\0",
    b"rmdir test\0",
    b"grep hello busybox_cmd.txt\0",
    b"cp busybox_cmd.txt busybox_cmd.bak\0",
    b"rm busybox_cmd.bak\0",
    b"find -name \"busybox_cmd.txt\"\0",
    ];

#[no_mangle]
fn main() -> i32 {
    println!("[proc0] Started.");
    if sys_fork() == 0 {
        // let rst = "result.txt";
        // println!("If the CMD runs incorrectly, return value will put in {}", rst);
        // println!("Else nothing will put in {}", rst);
        // println!("TEST START");
        // for command in busybox_commands {
        //     let child = sys_fork();
        //     let args = [
        //         command.as_ptr(),
        //         0 as *const u8
        //     ];
        //     if child == 0 {
        //         let ret = sys_exec(b"/busybox".as_ptr(), &args, &[0 as *const u8]);
        //         if ret != 0 && (command.ne(b"false\0")) {
        //             println!("testcase busybox {} fail", core::str::from_utf8(command).unwrap());
        //             println!("return: {}, cmd: {}", ret, core::str::from_utf8(command).unwrap());   // TODO: Write to file
        //             sys_exit(-1);
        //         }
        //     } else {
        //         let mut exit_code: i32 = 0;
        //         sys_waitpid(child, &mut exit_code);
        //         println!("testcase busybox {} success", core::str::from_utf8(command).unwrap());
        //     }
        // }
        let child = sys_fork();
        if child == 0 {
            let ret = sys_exec(b"/shell\0".as_ptr(), &[0 as *const u8], &[0 as *const u8]);
            if ret != 0  {
                sys_exit(-1);
            }
        } else {
            let mut exit_code: i32 = 0;
            sys_waitpid(child, &mut exit_code);
        }
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = sys_waitpid(-1, &mut exit_code);
            if pid == -1 || pid == -2{
                sys_yield();
                continue;
            } 
            println!("TEST END");   // TODO: Write to file
        }
    }
    0
}
