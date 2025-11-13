# Chapter 20 — Building a Multithreaded Web Server

This is the **capstone project** of the Rust Book — where you bring together *everything you’ve learned*:

* ownership & borrowing,
* traits & generics,
* smart pointers & concurrency,
* error handling,
* pattern matching,
* and module organization.

Let’s walk through it step by step — from **basic HTTP server** → **multithreaded version**.

---

## 🏗️ Overview

You’ll build a simple HTTP web server that:

1. Listens for TCP connections.
2. Parses incoming requests.
3. Returns a basic HTTP response.
4. Spawns a **thread pool** to handle multiple connections concurrently.
5. Gracefully shuts down when needed.

---

## 1️⃣ Starting Simple: A Single-Threaded Server

Let’s create a new project:

```bash
cargo new web_server
cd web_server
```

### `main.rs`

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // Bind to localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Handle each connection
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Now create a file called `hello.html`:

```html
<!DOCTYPE html>
<html>
  <body>
    <h1>Hello, Rust!</h1>
  </body>
</html>
```

Then run the server:

```bash
cargo run
```

And open your browser to [http://127.0.0.1:7878](http://127.0.0.1:7878) 🚀

✅ You’ve built your first minimal web server!

---

## 2️⃣ Handling Different Requests

Let’s make the server respond differently depending on the request path.

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
```

Also add `404.html`:

```html
<!DOCTYPE html>
<html>
  <body>
    <h1>404 - Not Found</h1>
  </body>
</html>
```

✅ Try `/` and `/sleep` in your browser.
The `/sleep` path simulates a long-running request — we’ll fix that next with threads.

---

## 3️⃣ Making It Multithreaded 🧵

Right now, one slow request (like `/sleep`) blocks all others.
Let’s add a **thread pool** to handle multiple connections concurrently.

We’ll create a new module for that.

---

### 🗂️ Project Structure

```
src/
 ├── main.rs
 └── lib.rs
```

---

### `src/lib.rs`

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool with `size` threads.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Box::new(|| {})).ok();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

✅ Here’s what’s happening:

* We create multiple **worker threads**.
* Each listens on a shared channel (`mpsc`) for jobs.
* `ThreadPool::execute()` sends jobs (closures) into the channel.
* Each worker pulls and runs jobs concurrently.

---

### `src/main.rs`

```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down server.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

✅ The `.take(2)` ensures the server shuts down after two requests (for demonstration).

---

## 🧠 How It Works

| Component                   | Responsibility                                |
| --------------------------- | --------------------------------------------- |
| **ThreadPool**              | Manages workers and job queue                 |
| **Worker**                  | Receives jobs and runs them in a loop         |
| **mpsc::channel**           | Sends jobs between threads                    |
| **Arc<Mutex<_>>**           | Shares safe access to receiver across threads |
| **Job (Box<dyn FnOnce()>)** | Represents any closure to run in a worker     |

---

## ⚙️ Why It’s Powerful

* Rust guarantees **safe concurrency** with `Arc` and `Mutex`.
* Each worker has **ownership** of its thread.
* The pool reuses threads instead of spawning new ones per request.
* Memory and thread safety are enforced at compile time.

---

## ✅ Try It Out

Run:

```bash
cargo run
```

Then visit:

* `http://127.0.0.1:7878/`
* `http://127.0.0.1:7878/sleep`

While `/sleep` is loading, refresh `/` — it should respond immediately now! 🎉
The thread pool is handling multiple requests concurrently.

---

## 🧩 Possible Extensions

* Add graceful shutdown signals (e.g., Ctrl+C handling).
* Add logging or timing for each request.
* Support more HTTP verbs (POST, PUT, etc.).
* Build a tiny templating system for dynamic HTML.
* Use `async` and `tokio` to make it non-blocking.

---

## 🧭 Key Takeaways

> You’ve now built a fully functional, multithreaded HTTP server *from scratch* in safe Rust.

| Concept               | Rust Feature Used                               |
| --------------------- | ----------------------------------------------- |
| Networking            | `std::net::TcpListener` / `TcpStream`           |
| File I/O              | `std::fs::read_to_string`                       |
| Concurrency           | Threads, `mpsc`, `Arc`, `Mutex`                 |
| Ownership & Borrowing | Thread-safe closure capture                     |
| Error Handling        | `unwrap()`, but could be improved with `Result` |
| Module Organization   | `lib.rs` + `main.rs`                            |

---

## 🏁 You Did It!

🎉 Congratulations — you’ve officially completed **The Rust Programming Language** book journey!
You now understand:

* Ownership, lifetimes, and borrowing
* Traits, generics, and smart pointers
* Concurrency and thread safety
* Patterns, macros, and modules
* Building and structuring real applications

---

Would you like me to continue with a **post-book guide** —
👉 *“Where to Go Next After the Rust Book”* — including key projects, crates, and advanced topics (like `tokio`, `serde`, `async/await`, `wasm`, and more)?
