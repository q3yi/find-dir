use std::{
    path::PathBuf,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};

use crate::args::Args;

#[derive(Debug, Clone)]
pub struct Options {
    parellel: usize,
    chan_size: usize,
    recursive: bool,
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        let threads = value.parallel.unwrap_or(num_cpus::get() as usize);
        Options {
            parellel: threads,
            chan_size: 1024, // change to a reasonable value?
            recursive: value.recursive,
        }
    }
}

pub struct Finder {
    walk: Option<Box<dyn FnOnce()>>,
    collector: Receiver<String>,
}

impl Finder {
    pub fn new<F>(entries: Vec<String>, filter: Arc<F>, opts: Options) -> Finder
    where
        F: Fn(&PathBuf) -> bool + Send + Sync + 'static,
    {
        let (sender, collector) = sync_channel(opts.chan_size);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(opts.parellel)
            .build()
            .unwrap();
        let pool = Arc::new(pool);

        let walk = move || {
            let walker = Walker::new(pool.clone(), sender, filter.clone(), opts.recursive);
            for entry in entries {
                let walker = walker.clone();
                pool.spawn(move || {
                    walker.scan(PathBuf::from(entry));
                })
            }
        };

        Finder {
            walk: Some(Box::new(walk)),
            collector,
        }
    }
}

impl Iterator for Finder {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if let Some(walk) = self.walk.take() {
            walk();
        }
        self.collector.recv().ok()
    }
}

#[derive(Clone)]
struct Walker {
    pool: Arc<rayon::ThreadPool>,
    chan: SyncSender<String>,
    filter: Arc<dyn Fn(&PathBuf) -> bool + Send + Sync + 'static>,
    recursive: bool,
}

impl Walker {
    fn new<F>(
        pool: Arc<rayon::ThreadPool>,
        chan: SyncSender<String>,
        filter: Arc<F>,
        recursive: bool,
    ) -> Self
    where
        F: Fn(&PathBuf) -> bool + Send + Sync + 'static,
    {
        Walker {
            pool,
            chan,
            filter,
            recursive,
        }
    }

    fn scan(&self, path: PathBuf) {
        let flag = (self.filter)(&path);

        if flag {
            let path = path.to_str().unwrap().to_owned();
            _ = self.chan.send(path);
        }

        if !path.is_dir() || (flag && !self.recursive) {
            return;
        }

        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            if !entry.metadata().unwrap().is_dir() {
                continue;
            }
            let walker = self.clone();
            self.pool.spawn(move || {
                walker.scan(entry.path());
            });
        }
    }
}
