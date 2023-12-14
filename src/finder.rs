use std::{
    path::PathBuf,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};

#[derive(Debug)]
pub struct Filter {
    has_file: Option<String>,
}

impl Filter {
    pub fn new() -> Self {
        Self { has_file: None }
    }

    pub fn has(mut self, file: String) -> Self {
        self.has_file = Some(file);
        self
    }

    fn do_filter(&self, path: &PathBuf) -> bool {
        if let Some(ref file) = self.has_file {
            path.join(file).exists()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Config {
    recursive: bool,
}

impl Config {
    pub fn new(recursive: bool) -> Self {
        Config { recursive }
    }
}

#[derive(Debug, Clone)]
pub struct Finder {
    pool: Arc<rayon::ThreadPool>,
    filter: Arc<Filter>,
    config: Arc<Config>,
}

impl Finder {
    pub fn new(pool: Arc<rayon::ThreadPool>, filter: Filter, config: Config) -> Finder {
        Finder {
            pool,
            filter: Arc::new(filter),
            config: Arc::new(config),
        }
    }

    pub fn scan(&self, start_paths: Vec<String>) -> Receiver<String> {
        let (tx, rx) = sync_channel(1000);
        for path in start_paths.iter() {
            let tx = tx.clone();
            let path = PathBuf::from(path);
            let walker = self.clone();
            self.pool.spawn(move || walker.scan_path(path, tx))
        }

        rx
    }

    fn scan_path(&self, entry: PathBuf, tx: SyncSender<String>) {
        let flag = self.filter.do_filter(&entry);

        if flag {
            let path = entry.to_str().unwrap().to_owned();
            _ = tx.send(path);
        }

        if !entry.is_dir() || (flag && !self.config.recursive) {
            return;
        }

        for entry in entry.read_dir().unwrap() {
            let entry = entry.unwrap();
            if !entry.metadata().unwrap().is_dir() {
                continue;
            }
            let tx = tx.clone();
            let walker = self.clone();
            self.pool.spawn(move || {
                walker.scan_path(entry.path(), tx);
            })
        }
    }
}
