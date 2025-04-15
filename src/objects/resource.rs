use std::sync::{Arc, Mutex};
use std::thread::{sleep, ThreadId};
use std::time::Duration;

use crate::DESLIGAR_SISTEMA;

#[derive(Clone)]
pub struct Resource {
    pub name: String,
    pub state: Arc<Mutex<ResourceState>>,
}

#[derive(Default)]
pub struct ResourceState {
    pub locked_by: Option<ThreadId>, // Thread que bloqueou o recurso
    pub waiting_threads: Vec<ThreadId>, // Threads esperando pelo recurso
}

impl ResourceState {
    pub fn copy(&self) -> Self {
        ResourceState {
            locked_by: self.locked_by,
            waiting_threads: self.waiting_threads.clone(),
        }
    }
}

impl Resource {
    pub fn new(name: String) -> Self {
        Resource {
            name,
            state: Arc::new(Mutex::new(ResourceState::default())),
        }
    }

    pub fn lock(&self) {
        let thread_id = std::thread::current().id();
        let mut state = self.state.lock().unwrap();

        if state.locked_by.is_none() {
            // Se o recurso não está bloqueado, bloqueia para a thread atual
            state.locked_by = Some(thread_id);
            println!("Recurso '{}' bloqueado pela thread {:?}", self.name, thread_id);
        } else if state.locked_by == Some(thread_id) {
            // Se a thread já possui o recurso, não faz nada
            println!(
                "Thread {:?} já possui o recurso '{}'.",
                thread_id, self.name
            );
        } else {
            // Se o recurso está bloqueado por outra thread, adiciona à lista de espera
            println!(
                "Thread {:?} aguardando pelo recurso '{}'.",
                thread_id, self.name
            );
            state.waiting_threads.push(thread_id);

            // Libera o lock no estado para evitar deadlock enquanto espera
            drop(state);

            // Aguarda até que o recurso seja liberado
            loop {
                {
                    let mut state = self.state.lock().unwrap();
                    if state.locked_by.is_none() {
                        // Quando o recurso for liberado, bloqueia para a thread atual
                        state.locked_by = Some(thread_id);
                        println!(
                            "Thread {:?} adquiriu o recurso '{}' após aguardar.",
                            thread_id, self.name
                        );                        
                        break;
                    }

                    if DESLIGAR_SISTEMA.load(std::sync::atomic::Ordering::SeqCst) {
                        println!("Sistema desligado. Thread {:?} não pode adquirir o recurso '{}'.", thread_id, self.name);
                        return;
                    }
                }                
                sleep(Duration::from_millis(1000));
            }
        }
    }

    pub fn unlock(&self) {
        let thread_id = std::thread::current().id();
        let mut state = self.state.lock().unwrap();

        if state.locked_by == Some(thread_id) {
            // Libera o recurso
            state.locked_by = None;
            println!("Recurso '{}' liberado pela thread {:?}", self.name, thread_id);

            // Notifica a próxima thread na fila de espera, se houver
            if let Some(next_thread) = state.waiting_threads.pop() {
                println!(
                    "Thread {:?} será notificada para adquirir o recurso '{}'.",
                    next_thread, self.name
                );
            }
        } else {
            println!(
                "Thread {:?} tentou liberar o recurso '{}' que não possui.",
                thread_id, self.name
            );
        }
    }

    pub fn copy(&self, new_name: String) -> Self {
        let state_copy = {
            let state = self.state.lock().unwrap();
            state.copy()
        };

        Resource {
            name: new_name,
            state: Arc::new(Mutex::new(state_copy)),
        }
    }
}