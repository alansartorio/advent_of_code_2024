use std::{
    env,
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

use petgraph::{dot::Dot, EdgeType, Graph};

pub trait OpenGraph {
    fn open_graph(&self);
}

impl<N, E, D: EdgeType> OpenGraph for Graph<N, E, D>
where
    N: Display,
    E: Display,
{
    fn open_graph(&self) {
        if env::var("OPEN_GRAPH").is_ok() {
            let graph_dot = Dot::new(self).to_string();

            let mut dot = Command::new("neato")
                .arg("-Tpdf")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();

            dot.stdin
                .as_mut()
                .unwrap()
                .write_all(graph_dot.as_bytes())
                .unwrap();

            Command::new("zathura")
                .arg("-")
                .stdin(Stdio::from(dot.stdout.unwrap()))
                .spawn()
                .unwrap();
        }
    }
}
