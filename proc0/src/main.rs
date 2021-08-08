// user/src/bin/initproc.rs

#![no_std]
#![no_main]

#[macro_use]
extern crate oshit_usrlib;
extern crate alloc;
use alloc::vec::Vec;

use oshit_usrlib::{print, sys_exec, sys_exit, sys_fork, sys_getpid, sys_waitpid, sys_yield};

// const busybox_commands : &'static [&'static [u8]] = &[
//     b"echo \"#### independent command test\"\0",
//     b"ash -c exit\0",
//     b"sh -c exit\0",
//     b"basename /aaa/bbb\0",
//     b"cal\0",
//     b"clear\0",
//     b"date \0",
//     b"df \0",
//     b"dirname /aaa/bbb\0",
//     b"dmesg \0",
//     b"du\0",
//     b"expr 1 + 1\0",
//     b"false\0",
//     b"true\0",
//     b"which ls\0",
//     b"uname\0",
//     b"uptime\0",
//     b"printf \"abc\n\"\0",
//     b"ps\0",
//     b"pwd\0",
//     b"free\0",
//     b"hwclock\0",
//     b"kill 10\0",
//     b"ls\0",
//     b"sleep 1\0",
//     b"echo \"#### file opration test\"\0",
//     b"touch test.txt\0",
//     b"echo \"hello world\" > test.txt\0",
//     b"cat test.txt\0",
//     b"cut -c 3 test.txt\0",
//     b"od test.txt\0",
//     b"head test.txt\0",
//     b"tail test.txt \0",
//     b"hexdump -C test.txt \0",
//     b"md5sum test.txt\0",
//     b"echo \"ccccccc\" >> test.txt\0",
//     b"echo \"bbbbbbb\" >> test.txt\0",
//     b"echo \"aaaaaaa\" >> test.txt\0",
//     b"echo \"2222222\" >> test.txt\0",
//     b"echo \"1111111\" >> test.txt\0",
//     b"echo \"bbbbbbb\" >> test.txt\0",
//     b"sort test.txt | ./busybox uniq\0",
//     b"stat test.txt\0",
//     b"strings test.txt \0",
//     b"wc test.txt\0",
//     b"[ -f test.txt ]\0",
//     b"more test.txt\0",
//     b"rm test.txt\0",
//     b"mkdir test_dir\0",
//     b"mv test_dir test\0",
//     b"rmdir test\0",
//     b"grep hello busybox_cmd.txt\0",
//     b"cp busybox_cmd.txt busybox_cmd.bak\0",
//     b"rm busybox_cmd.bak\0",
//     b"find -name \"busybox_cmd.txt\"\0",
//     ];

const commands: &'static[&str] = &[
    "busybox echo \"#### independent command test\"",
    "busybox ash -c exit",
    "busybox sh -c exit",
    "busybox basename /aaa/bbb",
    "busybox cal",
    "busybox clear",
    "busybox date ",
    "busybox df",
    "busybox dirname /aaa/bbb",
    "busybox dmesg ",
    "busybox du",
    "busybox expr 1 + 1",
    "busybox false",
    "busybox true",
    "busybox which ls",
    "busybox uname",
    "busybox uptime",
    "busybox printf \"abc\n\"",
    "busybox ps",
    "busybox pwd",
    "busybox free",
    "busybox hwclock",
    "busybox kill 10",
    "busybox ls",
    "busybox sleep 1", 
    "busybox echo \"#### file opration test\"",
    "busybox touch test.txt",
    "busybox echo \"hello world\" > test.txt",
    "busybox cat test.txt",
    "busybox cut -c 3 test.txt",
    "busybox od test.txt",
    "busybox head test.txt",
    "busybox tail test.txt ",
    "busybox hexdump -C test.txt ",
    "busybox md5sum test.txt",
    "busybox echo \"ccccccc\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox echo \"aaaaaaa\" >> test.txt",
    "busybox echo \"2222222\" >> test.txt",
    "busybox echo \"1111111\" >> test.txt",
    "busybox echo \"bbbbbbb\" >> test.txt",
    "busybox sort test.txt | ./busybox uniq",
    "busybox stat test.txt",
    "busybox strings test.txt ",
    "busybox wc test.txt",
    "busybox [ -f test.txt ]",
    "busybox more test.txt",
    "busybox rm test.txt",
    "busybox mkdir test_dir",
    "busybox mv test_dir test",
    "busybox rmdir test",
    "busybox grep hello busybox_cmd.txt",
    "busybox cp busybox_cmd.txt busybox_cmd.bak",
    "busybox rm busybox_cmd.bak",
    "busybox find -name \"busybox_cmd.txt\"",
];

#[no_mangle]
fn main() -> i32 {
    let busybox_argv: &[*const u8] = &[
        b"busybox\0".as_ptr(),
        // b"ls\0".as_ptr(),
        b"sh\0".as_ptr(),
        b"-c\0".as_ptr(),
        // b"./busybox cat ./busybox_cmd.txt\0".as_ptr(),
        b"./busybox cat ./busybox_cmd.txt | while read line
        do
            eval \"./busybox $line\"
            RTN=$?
            if [[ $RTN -ne 0 && $line != \"false\" ]] ;then
                echo \"testcase busybox $line fail\"
                # echo \"return: $RTN, cmd: $line\" >> $RST
            else
                echo \"testcase busybox $line success\"
            fi
        done\0".as_ptr(),
        // b"echo hello world\0".as_ptr(),
        0usize as *const u8
    ];

    let busybox_envp: &[*const u8] = &[
        // b"SHELL=/bin/bash\0".as_ptr(),
        b"PWD=/busybox\0".as_ptr(),
        b"LOGNAME=root\0".as_ptr(),
        b"_=busybox\0".as_ptr(),
        b"MOTD_SHOWN=pam\0".as_ptr(),
        b"LINES=67\0".as_ptr(),
        b"HOME=/\0".as_ptr(),
        b"LANG=zh_CN.UTF-8\0".as_ptr(),
        b"COLUMNS=138\0".as_ptr(),
        b"TERM=xterm-256color\0".as_ptr(),
        b"USER=root\0".as_ptr(),
        b"SHLVL=1\0".as_ptr(),
        b"PATH=/\0".as_ptr(),
        // b"SSH_TTY=/dev/tty0\0".as_ptr(),
        b"OLDPWD=/\0".as_ptr(),
        0usize as *const u8
    ];

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
        // let child = sys_fork();
        // if child == 0 {
        //     let ret = sys_exec(b"/shell\0".as_ptr(), &[0 as *const u8], &[0 as *const u8]);
        //     if ret != 0  {
        //         sys_exit(-1);
        //     }
        // } else {
        //     let mut exit_code: i32 = 0;
        //     sys_waitpid(child, &mut exit_code);
        // }
        for command in commands {
            println!("Now executing: {}", command);
            let words: Vec<&str> = command.split(" ").collect();
            let words: Vec<Vec<u8>> = words.iter().map(
                |word| -> Vec<u8> {
                    let mut res = word.as_bytes().to_vec();
                    res.push(b'\0');
                    res
                }
            ).collect();
            let words: Vec<*const u8> = words.iter().map(|word| word.as_ptr()).collect();
            if sys_fork() == 0 {
                sys_exec(b"/busybox\0".as_ptr(), &words, busybox_envp);
            } else {
                let mut exit_code: i32 = 0;
                let pid = sys_waitpid(-1, &mut exit_code);
                if pid == -1 || pid == -2 {
                    sys_yield();
                    continue;
                } 
            }
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
