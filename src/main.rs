use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep, ThreadId};
use std::time::Duration;

use taskmanager_emulator::objects::resource::{Resource};
use taskmanager_emulator::objects::resource_manager::ResourceManager;
use taskmanager_emulator::objects::resource_tree::ResourceTree;
use taskmanager_emulator::objects::thread_spawner::ThreadPool;
use taskmanager_emulator::DESLIGAR_SISTEMA;

fn main() {
    // Cria o ResourceManager compartilhado entre threads
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));

    // Cria dois recursos
    let resource1 = Arc::new(Resource::new("Recurso 1".to_string()));
    let resource2 = Arc::new(Resource::new("Recurso 2".to_string()));

    // Clona os recursos e o ResourceManager para a Thread A
    let rm_a = Arc::clone(&resource_manager);
    let r1_a = Arc::clone(&resource1);
    let r2_a = Arc::clone(&resource2);
    
    // Clona os recursos e o ResourceManager para a Thread B
    let rm_b = Arc::clone(&resource_manager);
    let r1_b = Arc::clone(&resource1);
    let r2_b = Arc::clone(&resource2);


    let mut pool = ThreadPool::new(4);

    pool.execute(
        move || {
            let thread_id = thread::current().id();
        
            println!("---Thread A tenta adquirir Recurso 1 e depois Recurso 2---");
            {   
                let mut rm = resource_manager.lock().unwrap();
                rm.add_resource(r1_a.clone(), thread_id).unwrap();
            }
            
            r1_a.lock();
            // Simula algum trabalho antes de tentar adquirir o próximo recurso
            thread::sleep(Duration::from_millis(100));

            println!("Thread A tenta adquirir Recurso 2");
            {
                let mut rm = resource_manager.lock().unwrap();
                rm.add_resource(r2_a.clone(), thread_id).unwrap();
            }
            
            r2_a.lock();
            thread::sleep(Duration::from_secs(10));

            println!("----FIM THREAD A----");
        }
    );//, r1_a);


    pool.execute(
        move || {
            let thread_id = thread::current().id();

        println!("---Thread B tenta adquirir Recurso 2 e depois Recurso 1---");
        {
            let mut rm = rm_b.lock().unwrap();
            rm.add_resource(r2_b.clone(), thread_id).unwrap();
        }
        
        r2_b.lock();
        // Simula algum trabalho antes de tentar adquirir o próximo recurso
        thread::sleep(Duration::from_millis(100));

        println!("Thread B tenta adquirir Recurso 1");
        {
            let mut rm = rm_b.lock().unwrap();
            rm.add_resource(r1_b.clone(), thread_id).unwrap();
        }
        
        r1_b.lock();
        thread::sleep(Duration::from_secs(10));

        println!("----FIM THREAD B----");
    });


    sleep(Duration::from_secs(10));

    println!("Tenta validar a existência de deadlock");
    let mut rm = rm_a.lock().unwrap();

    if rm.detect_deadlock() {
        DESLIGAR_SISTEMA.store(true, std::sync::atomic::Ordering::SeqCst);
        pool.shutdown();
    } 
}