
// fork temp 
pub fn fork_temp_state(base: Arc<ChainState>) -> ChainState {
    ChainState{
        // memdb
        db: DB::Memory(MemoryDB::new()),
        base: Some(Arc::<ChainState>::downgrade(&base))
    }
}