use std::net::TcpStream;
use ssh2::{Channel, Session};
use std::io::Read;
use std::io::Write;
use std::borrow::Cow;

pub struct SSHSession {
    inner: Session
}

pub struct SSHChannel {
    inner: Channel
}

impl SSHSession {
    fn authed_session(user : String, addr: String, port : u32) -> ssh2::Session {
        let socket = TcpStream::connect(&format!("{}:{}", &addr, &port)).unwrap();
        let mut sess = ssh2::Session::new().unwrap();
        sess.set_tcp_stream(socket);
        sess.handshake().unwrap();
        assert!(!sess.authenticated());

        {
            let mut agent = sess.agent().unwrap();
            agent.connect().unwrap();
            agent.list_identities().unwrap();
            let identities = agent.identities().unwrap();
            let identity = &identities[1];
            agent.userauth(&user, &identity).unwrap();
        }
        assert!(sess.authenticated());
        sess
    }

    pub fn new(user : &str, addr: &str, port : u32) -> SSHSession {
        SSHSession {
            inner: Self::authed_session(user.to_string(), addr.to_string(), port)
        }
    }

    pub fn create_channel(&self) -> SSHChannel {
        SSHChannel {
            inner: self.inner.channel_session().unwrap()
        }
    }

    pub fn run(&mut self, cmd : &str) {

        let mut chan = self.create_channel();

        chan.inner.flush().unwrap();
        chan.inner.exec(cmd).unwrap();

        let mut stdout = String::new();
        chan.inner.read_to_string(&mut stdout).unwrap();

        let mut stderr = String::new();
        chan.inner.stderr().read_to_string(&mut stderr).unwrap();

        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);

        chan.inner.wait_eof().unwrap();
    }

    pub fn run_args<T>(&mut self, args : &[T]) where T: Sized, T: std::string::ToString {
        let args = args.iter().map(|x| {
            ::shell_escape::unix::escape(Cow::Owned(x.to_string())).to_string()
        }).collect::<Vec<String>>();
        self.run(&args.join(" "));
    }
}