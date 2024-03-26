

fn _locate_chunk(rtck: &Arc<ChunkRoller>, hx: &Hash) -> Option<Arc<ChunkRoller>> {
    let chk = rtck;
    if chk.hash == *hx {
        return Some(rtck.clone())
    }
    for a in chk.childs.borrow().iter() {
        let res = _locate_chunk(a, hx);
        if let Some(ck) = res {
            return Some(ck.clone())
        }
    }
    // not find
    None
}


// find
fn locate_base_chunk(this: &KernelCtx, hx: &Hash) -> Option<Arc<ChunkRoller>> {
    _locate_chunk(&this.sroot, hx)
}
