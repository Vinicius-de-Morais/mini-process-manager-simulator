use std::{thread, time::Duration};
use taskmanager_emulator::objects::{
    resource::Resource,
    thread_spawner::ThreadPool,
};

fn main() {
    // Cria um pool de threads com 4 threads
    let thread_pool = ThreadPool::new(4);

    // Cria alguns recursos para os threads usarem
    let resource1 = Resource::new("Resource 1".to_string());
    let resource2 = Resource::new("Resource 2".to_string());

    // Adiciona tarefas ao pool de threads
    thread_pool.execute(
        || {
            println!("Thread 1 tentando acessar Resource 1...");
            thread::sleep(Duration::from_secs(1)); 
            println!("Thread 1 terminou.");
        },
        resource1.clone(),
    );

    thread_pool.execute(
        || {
            println!("Thread 2 tentando acessar Resource 2...");
            thread::sleep(Duration::from_secs(1)); 
            println!("Thread 2 terminou.");
        },
        resource2.clone(),
    );

    thread_pool.execute(
        || {
            println!("Thread 3 tentando acessar Resource 1 e Resource 2...");
            thread::sleep(Duration::from_secs(2));
            println!("Thread 3 terminou.");
        },
        resource1.clone(),
    );

    thread::sleep(Duration::from_secs(3)); // Aguarda as threads terminarem
    let resource_manager = thread_pool.resource_manager.lock().unwrap();

    if resource_manager.detect_deadlock() {
        println!("Deadlock detectado!");
    } else {
        println!("Nenhum deadlock detectado.");
    }
}