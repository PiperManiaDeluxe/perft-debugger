//! Starts and communicates with a chess-engine subprocess.
//!
//! [`UCIEngine`] sends commands to the engine's standard input and forwards
//! lines from its standard output through an iterator. It provides the
//! process-level communication used by the UCI perft provider. It does not
//! itself implement the perft result parsing.

use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self},
    thread::{self, JoinHandle},
};

use anyhow::{Context, Result};

/// A chess engine process we can communicate with.
pub struct UCIEngine {
    /// Engine child process.
    process: Child,
    /// Stdin to the engine.
    tx: ChildStdin,
    /// Every line from the engine's stdout piped into a continuingly updating [`mpsc::Receiver`].
    rx: mpsc::Receiver<String>,
    /// `rx` updater thread.
    reader_thread: JoinHandle<()>,
}

impl UCIEngine {
    /// Opens a new [`UCIEngine`] process.
    ///
    /// The process runs infinitely until either the engine closes itself,
    /// or we close the engine with [`UCIEngine::close`].
    ///
    /// # Arguments
    ///
    /// * `engine_command`: Engine executable path/name.
    pub fn open_engine(engine_command: &str) -> Result<Self> {
        let mut engine_process = Command::new(engine_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| {
                format!("Failed to spawn engine with the command: `{engine_command}`")
            })?;

        let engine_in = engine_process
            .stdin
            .take()
            .context("Engine stdin is none")?;
        let engine_out = engine_process
            .stdout
            .take()
            .context("Engine stdout is none")?;

        // Forward each line from the engine to the main process
        let (tx, rx) = mpsc::channel::<String>();
        let reader_thread = thread::spawn(move || {
            let mut reader = BufReader::new(engine_out);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // Stockfish closed stdout
                    Ok(_) => {
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading stockfish stdout: {err}");
                        break;
                    }
                }
            }
        });

        Ok(Self {
            process: engine_process,
            tx: engine_in,
            rx,
            reader_thread,
        })
    }

    /// Sends a command to the engine via stdin.
    pub fn send_command(&mut self, command: &str) -> Result<()> {
        writeln!(self.tx, "{command}").context("Couldn't write to engine stdin")?;
        self.tx.flush().context("Couldn't flush engine stdin")
    }

    /// Returns an iterator over lines received from the engine's stdout.
    /// Blocks before every incoming message.
    pub fn iter_rx(&mut self) -> mpsc::Iter<'_, String> {
        self.rx.iter()
    }

    /// Exits the engine process.
    pub fn close(mut self) -> Result<()> {
        self.send_command("quit")
            .context("Sending quit command failed")?;
        let _ = self.process.wait();
        let _ = self.reader_thread.join();

        Ok(())
    }
}
