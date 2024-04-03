
pub struct BlockChainKernel {

    cnf: KernelConf,

    store: Arc<BlockStore>,

    klctx: Mutex<StateRoller>, // change

    mintk: Box<dyn MintChecker>,
    vmobj: Box<dyn VM>,
    // actns: Box<dyn >,

    // insert lock
    isrlck: Mutex<bool>,
    // updlck: RwLock<bool>,
}

impl BlockChainKernel {

    pub fn create(ini: &IniObj) -> BlockChainKernel {
        let cnf = NewKernelConf(ini);
        // data dir
        std::fs::create_dir_all(&cnf.store_data_dir);
        std::fs::create_dir_all(&cnf.state_data_dir);
        std::fs::create_dir_all(&cnf.ctrkv_data_dir);
        // block store
        let stoldb = BlockStore::create(&cnf.store_data_dir);
        // kernel
        let kernel = BlockChainKernel{
            cnf: cnf,
            store: Arc::new(stoldb),
        };

        kernel
    }

    pub fn start(&mut self) -> RetErr {


        Ok(())
    }

    pub fn get_latest_state(&self) -> Option<Arc<dyn State>> {
        let ctx = self.klctx.try_lock();
        if let Err(_) = ctx {
            return None // state busy !!!
        }
        let ctx = ctx.unwrap();
        if let Some(st) = ctx.state.upgrade() {
            return Some(st)
        }
        if let Some(sc) = ctx.scusp.upgrade() {
            return Some(sc.state.clone())
        }
        // base
        Some(ctx.sroot.state.clone())
    }
}





