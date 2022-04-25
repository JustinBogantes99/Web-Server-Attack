use std::{sync::mpsc,thread};
use std::sync::Arc;
use std::sync::Mutex;

/*
Estructura para la creacion del hilo
*/
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
/*
Estructura que tendra el job
*/
type Job = Box<dyn FnOnce() + Send + 'static>;
/*
Implementacion de los hilos
*/
impl ThreadPool {
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
/*
Estructura con la cual el hilo es ejecutado utiliza el tamaño de este 
*/
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
/*
Implementacion del worker el cual crea el hilo que se utiliza
*/
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}