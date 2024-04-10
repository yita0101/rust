
pub struct ChainState {

    // db: DB,
    memk: MemoryDB,
    disk: Arc<LevelDB>,
    base: Weak<ChainState>,
}

impl ChainState {

    pub fn copy_ldb(&self) -> Arc<LevelDB> {
        self.disk.clone()
    }
    
}


impl StoreDB for ChainState {

    fn get_at(&self, key: &[u8]) -> Option<Bytes> {
        // check mem
        if let Some(dt) = self.memk.get(key) {
            // find the key
            if let MemdbItem::Delete = dt {
                return None // delete mark
            }else if let MemdbItem::Value(v) = dt {
                return Some(Bytes::Mem(v.clone())) // find
            }
        }
        // is have base db
        let basedb = self.base.upgrade();
        if let None = basedb {
            // no base ptr, check disk db // search disk final
            return match self.disk.get_at(key) {
                Some(rb) => Some(Bytes::Raw(rb)),
                _ => None, // not find
            }
        }
        // must have base ptr, check base
        basedb.unwrap().get_at(key) // search from base ptr
    }
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Bytes> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn put(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.memk.set(&key, &vdt); // local mem
    }

    fn rm(&mut self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.memk.del(&key); // local mem
    }

}


impl State for ChainState {
    fn as_db(&mut self) -> &mut dyn StoreDB {
        self
    }
}


impl ChainState {

    pub fn open(dir: &Path) -> ChainState {
        let ldb = LevelDB::open(dir);
        ChainState{
            memk: MemoryDB::new(),
            disk: Arc::new(ldb),
            base: Weak::new().into(), // no base
        }
    }

    pub fn flush_disk(&mut self) {
        impl_flush_disk(self)
    }

    pub fn merge_copy(&self, src: &dyn State) -> RetErr {
        Ok(())
    }
}

