use nix::mount::MsFlags;
use nix::mount::{mount, umount};
use nix::sched;
use nix::sched::CloneFlags;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "run" => run()?,
        "child" => child()?,
        _ => println!("Invalid command"),
    };

    Ok(())
}

fn command_spawn(command: &String, args: &[String]) -> isize {
    Command::new(&command)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
        .unwrap() as isize
}

fn run() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    println!("Running command: {:?}", args[2..].join(" "));

    const STACK_SIZE: usize = 1024 * 1024;
    let ref mut stack = [0; STACK_SIZE];

    let mut child_args = vec![String::from("child")];
    child_args.extend_from_slice(&args[2..]);

    dbg!(&child_args);

    let sandbox = || command_spawn(&String::from("/proc/self/exe"), child_args.as_slice());

    sched::clone(
        Box::new(sandbox),
        stack,
        CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS,
        None,
    )
    .unwrap();
    Ok(())
}

fn child() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    println!("Running command: {:?}", args[2..].join(" "));

    cg()?;

    nix::unistd::sethostname("container").unwrap();
    nix::unistd::chroot("/home/myuser/ubuntufs").unwrap();
    nix::unistd::chdir("/").unwrap();
    mount(
        Some("proc"),
        "proc",
        Some("proc"),
        MsFlags::empty(),
        Some(""),
    )
    .unwrap();

    command_spawn(&args[2], &args[3..]);

    umount("proc").unwrap();
    Ok(())
}

fn cg() -> Result<(), io::Error> {
    let pids: &Path = Path::new("/sys/fs/cgroup/pids");
    match fs::create_dir(pids.join("myuser")) {
        _ => (),
    };

    fs::write(pids.join("myuser/pids.max"), b"20").unwrap();
    fs::write(pids.join("myuser/notify_on_release"), b"1").unwrap();
    fs::write(
        pids.join("myuser/cgroup.procs"),
        format!("{}", nix::unistd::getpid().as_raw()),
    )
    .unwrap();

    Ok(())
}
