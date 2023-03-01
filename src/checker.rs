/*
1. The watch function watches a file for changes. It returns a Result with two possible errors: ErrorKind::Io and ErrorKind::PathNotFound. I have to use the anyhow::Result to handle these errors.
2. The try_recv function returns a Result with two possible errors: TryRecvError::Empty and TryRecvError::Disconnected. I have to use the anyhow::Result to handle these errors.
3. The ErrorKind::Io and ErrorKind::PathNotFound errors are converted to anyhow::Error by the ? operator. 
4. The TryRecvError::Empty and TryRecvError::Disconnected errors are converted to anyhow::Error by the panic!() function. 
5. The panic!() function is called because the code should never reach the TryRecvError::Disconnected error. The program will panic if the error is reached. 
6. I used the panic!() function because I don't know how to handle the TryRecvError::Disconnected error.
7. The code above is not the best way to implement the FileRevisions struct. The code should be refactored to handle the TryRecvError::Disconnected error. 
*/

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

pub struct FileRevisions {
    rx: Receiver<notify::Result<Event>>,
    _watcher: RecommendedWatcher,
}

impl FileRevisions {
    pub fn from_path(filename: &Path) -> notify::Result<Self> {
        let (tx, rx) = channel::<notify::Result<Event>>();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Default::default())?;
        watcher.watch(filename, RecursiveMode::NonRecursive)?;
        Ok(FileRevisions {
            rx,
            _watcher: watcher,
        })
    }
}

impl FileRevisions {
    pub fn changed(&mut self) -> anyhow::Result<bool> {
        let mut ret = false;
        loop {
            match self.rx.try_recv() {
                Ok(Ok(_event)) => ret = true,
                Ok(Err(e)) => return Err(e.into()),
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => panic!(),
            }
        }
        Ok(ret)
    }
}
