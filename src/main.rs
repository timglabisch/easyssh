extern crate ssh2;
extern crate shell_escape;

use crate::ssh::SSHSession;

mod ssh;

fn main() {

    let mut  server1 = SSHSession::new("root", "49.12.74.151", 22);
    server1.run("dpkg -l");
    server1.run("env");
    server1.run_args(&["echo", "\"<>{}'foo"]);

}
