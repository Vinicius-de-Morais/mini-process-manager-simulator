use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::objects::resource_manager::ResourceManager;

use super::resource::Resource;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    pub resource_manager: Arc<Mutex<ResourceManager>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&resource_manager),
            ));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            resource_manager,
        }
    }

    pub fn execute<F>(&self, f: F )//, resource: Arc<Resource>)
    where
        F: FnOnce() + Send + 'static,
    {
        let thread_id = thread::current().id();

        // Adiciona o recurso ao ResourceManager
        // let mut resource_manager = self.resource_manager.lock().unwrap();

        // if let Err(err) = resource_manager.add_resource(resource.clone(), thread_id) {
        //     println!(
        //         "Thread {:?} aguardando recurso '{}': {}",
        //         thread_id, resource.name, err
        //     );

        //     // Simula a espera pelo recurso
        //     resource_manager
        //         .resources_tree
        //         .iter_mut()
        //         .find(|tree| tree.get_thread_id() == thread_id)
        //         .map(|tree| tree.add_waiting_thread(thread_id));

        //     return; // Não executa o trabalho até que o recurso esteja disponível
        // }

        // drop(resource_manager); 

        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn  shutdown(&mut self) {
        println!("Shutting down ThreadPool...");

        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                println!("Shutting thread {:?}", thread.thread().id());
                thread.join().unwrap();
            }
        }

        println!("All threads have been shut down.");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, resource_manager: Arc<Mutex<ResourceManager>>) -> Worker {
        let thread = thread::spawn(move || loop {
            //let message = receiver.lock().unwrap().recv();
            let message = receiver.lock().unwrap().try_recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();

                    // Libera os recursos após a execução
                    let thread_id = thread::current().id();
                    //resource_manager.lock().unwrap().release_resource(thread_id);
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}