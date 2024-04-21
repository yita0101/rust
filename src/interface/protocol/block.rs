


pub trait BlockRead : Serialize + Send + dyn_clone::DynClone {

    fn hash(&self) -> Hash { panic_never_call_this!() }

    fn height(&self) -> &BlockHeight { panic_never_call_this!() }
    fn timestamp(&self) -> &Timestamp { panic_never_call_this!() }
    fn prevhash(&self) -> &Hash { panic_never_call_this!() }
    fn mrklroot(&self) -> &Hash { panic_never_call_this!() }
    
    fn transaction_count(&self) -> &Uint4 { panic_never_call_this!() }
    fn transactions(&self) -> &Vec<Box<dyn Transaction>> { panic_never_call_this!() }
    fn transaction_hash_list(&self, hash_with_fee: bool) -> Vec<Hash> { panic_never_call_this!() }

}

pub trait Block : BlockRead + Parse + Send + dyn_clone::DynClone {

    fn update_mrklroot(&mut self) { panic_never_call_this!() }
    fn set_mrklroot(&mut self, _: Hash) { panic_never_call_this!() }
    fn push_transaction(&mut self, _: &dyn Transaction) -> RetErr { panic_never_call_this!() }

}


dyn_clone::clone_trait_object!(BlockRead);
dyn_clone::clone_trait_object!(Block);