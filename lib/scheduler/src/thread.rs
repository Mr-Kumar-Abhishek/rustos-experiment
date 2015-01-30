use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::default::Default;
use std::thunk::Thunk;
use StackPointer;

pub struct Thread {
    pub id: ThreadId,
    pub state: ThreadState,
}

#[derive(Debug)]
pub struct ThreadId(usize);

impl ThreadId {
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

pub enum ThreadState {
    New {
        function: Thunk,
    },
    Active {
        stack_pointer: StackPointer,
    },
    Running,
}

impl Thread {

    fn next_id() -> ThreadId {
        static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
        ThreadId(NEXT_ID.fetch_add(1, Ordering::Relaxed) + 2) // start at id 2
    }
    
    pub fn new<F>(f: F) -> Thread where F : FnOnce(), F: Send {
        Thread {
            id: Thread::next_id(),
            state: ThreadState::New {
                function: Thunk::new(f),
            }
        }
    }
}

impl Default for Thread {
    fn default() -> Thread {
        Thread::new(|| println!("default"))
    }
}
