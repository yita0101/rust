

/******************* transaction sign *******************/



defineQueryObject!{ Q8375,
    // do sign
    prikey, Option<String>, None,
    // append
    pubkey, Option<String>, None,
    sigdts, Option<String>, None,
    //
    signature, Option<bool>, None,
    description, Option<bool>, None,
}


async fn transaction_sign(State(ctx): State<ApiCtx>, q: Query<Q8375>, body: Bytes) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_unit!(q, unit);
    q_must!(q, prikey, s!(""));
    q_must!(q, pubkey, s!(""));
    q_must!(q, sigdts, s!(""));
    q_must!(q, signature, false);
    q_must!(q, description, false);

    let lasthei = ctx.engine.latest_block().objc().height().uint();

    let txdts = q_body_data_may_hex!(q, body);
    let Ok((mut tx, _)) = transaction::create(&txdts) else {
        return api_error("transaction body error")
    };

    let (address, signobj) = match prikey.len() == 64 {
        true => {
            let Ok(prik) = hex::decode(&prikey) else {
                return api_error("prikey format error")
            };
            let Ok(acc) = Account::create_by_secret_key_value(prik.try_into().unwrap()) else {
                return api_error("prikey data error")
            };
            let fres = tx.fill_sign(&acc);
            if let Err(e) = fres {
                return api_error(&format!("fill sign error: {}", e))
            }
            (Address::cons(*acc.address()), fres.unwrap())
        },
        false => {
            // replace
            if pubkey.len() != 33*2 || sigdts.len() != 64*2 {
                return api_error("pubkey or signature data error")
            }
            let Ok(pbk) = hex::decode(&pubkey) else {
                return api_error("pubkey format error")
            };
            let Ok(sig) = hex::decode(&sigdts) else {
                return api_error("sigdts format error")
            };
            let pbk: [u8; 33] = pbk.try_into().unwrap();
            let sig: [u8; 64] = sig.try_into().unwrap();
            let signobj = Sign{
                publickey: Fixed33::cons( pbk ),
                signature: Fixed64::cons( sig ),
            };
            if let Err(e) = tx.push_sign(signobj.clone()) {
                return api_error(&format!("fill sign error: {}", e))
            }
            (Address::cons(Account::get_address_by_public_key(pbk)), signobj)
        },
    };

    // return info
    let mut data = render_tx_info(tx.as_read(), None, lasthei, &unit, true, signature, false, description);
    data.insert("sign_data", json!(jsondata!{
        "address", address.readable(),
        "pubkey", signobj.publickey.hex(),
        "sigdts", signobj.signature.hex(),
    }));
    api_data(data)

}





/******************* transaction build *******************/



defineQueryObject!{ Q2856,
    action, Option<bool>, None,
    signature, Option<bool>, None,
    description, Option<bool>, None,
}


async fn transaction_build(State(ctx): State<ApiCtx>, q: Query<Q2856>, body: Bytes) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_unit!(q, unit);
    q_must!(q, action, false);
    q_must!(q, signature, false);
    q_must!(q, description, false);

    let txjsonobj = q_body_data_may_hex!(q, body);
    
    


    let Ok(txp) = transaction::create_pkg(BytesW4::from_vec(txjsonobj)) else {
        return api_error("transaction body error")
    };

    // return info
    api_data(
        render_tx_info(txp.objc().as_read(), None, 0, &unit, true, signature, action, description)
    )

}






/******************* transaction check *******************/



defineQueryObject!{ Q9764,
    signature, Option<bool>, None,
    description, Option<bool>, None,
}


async fn transaction_check(State(ctx): State<ApiCtx>, q: Query<Q9764>, body: Bytes) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_unit!(q, unit);
    q_must!(q, signature, false);
    q_must!(q, description, false);

    let txdts = q_body_data_may_hex!(q, body);
    let Ok(txp) = transaction::create_pkg(BytesW4::from_vec(txdts)) else {
        return api_error("transaction body error")
    };

    // return info
    api_data(
        render_tx_info(txp.objc().as_read(), None, 0, &unit, false, signature, true, description)
    )

}



/******************* transaction exist *******************/



defineQueryObject!{ Q3457,
    hash, Option<String>, None,
    body, Option<bool>, None,
    action, Option<bool>, None,
    signature, Option<bool>, None,
    description, Option<bool>, None,
}


async fn transaction_exist(State(ctx): State<ApiCtx>, q: Query<Q3457>) -> impl IntoResponse {
    ctx_store!(ctx, store);
    ctx_state!(ctx, state);
    q_unit!(q, unit);
    q_must!(q, hash, s!(""));
    q_must!(q, body, false);
    q_must!(q, action, false);
    q_must!(q, signature, false);
    q_must!(q, description, false);

    let lasthei = ctx.engine.latest_block().objc().height().uint();

    let Ok(hx) = hex::decode(&hash) else {
        return api_error("transaction hash format error")
    };
    if hx.len() != 32 {
        return api_error("transaction hash format error")
    }
    let txhx = Hash::must(&hx);
    let Some(txp) = state.txexist(&txhx) else {
        return api_error("transaction not find")
    };
    // read block
    let bkey = txp.height.to_string();
    let blkpkg = ctx.load_block(&store, &bkey);
    if let Err(_) = blkpkg {
        return api_error("cannot find block by transaction ptr")
    }
    let blkpkg = blkpkg.unwrap();
    let blkobj = blkpkg.objc();
    let blktrs = blkobj.transactions();

    // search tx hash
    let tx = match (|hx: &Hash|{
        let txnum = blkobj.transaction_count().uint() as usize; // drop coinbase
        for i in 1..txnum {
            if *hx == blktrs[i].hash() {
                return Some(blktrs[i].clone())
            }
        }
        None
    })(&txhx) {
        None => return api_error("transaction not find in the block"),
        Some(tx) => tx,
    };

    // return info
    api_data(
        render_tx_info(tx.as_read(), Some(blkobj.as_read()), lasthei, &unit, 
            body, signature, action, description
        )
    )
}





////////////////////////////////////


/*
* params: belong_block_obj, 
*/
fn render_tx_info(tx: &dyn TransactionRead, 
    blblk: Option<&dyn BlockRead>, lasthei: u64, unit: &String, 
    body: bool, signature: bool, action: bool, description: bool,
) -> JsonObject {


    let fee_str = tx.fee().to_unit_string(unit);
    let main_addr = tx.address().unwrap().readable();
    let mut data = jsondata!{
        // tx
        "hash", tx.hash().hex(),
        "hash_with_fee", tx.hash_with_fee().hex(),
        "type", tx.timestamp().uint(),
        "timestamp", tx.timestamp().uint(),
        "fee", fee_str,
        "fee_got", tx.fee_got().to_unit_string(unit),
        "main_address", main_addr,
        "action", tx.action_count(),
    };

    if body {
        data.insert("body", json!(tx.serialize().hex()));
    }

    if signature {
        check_signature(&mut data, tx);
    }

    if description {
        data.insert("description", json!(format!(
            "Main addr {} pay {} HAC tx fee",
            main_addr, fee_str
        )));
    }

    if let Some(blkobj) = blblk {
        let txblkhei = blkobj.height().uint();
        // belong block info
        data.insert("block", json!(jsondata!{
            "height", txblkhei,
            "timestamp", blkobj.timestamp().uint(),
        }));
        // confirm block height
        data.insert("confirm", json!(lasthei - txblkhei));
    }

    if action {
        let acts = tx.actions();
        let mut actobjs = Vec::with_capacity(acts.len());
        for act in acts {
            actobjs.push( action_json_desc(tx, act.as_ref(), unit, true, description) );
        }
        data.insert("actions", json!(actobjs));
    }

    data
}



fn check_signature(data: &mut JsonObject, tx: &dyn TransactionRead) {
    let Ok(sigstats) = check_tx_signature(tx) else {
        return
    };
    let mut sigchs = vec![];
    for (adr, sg) in sigstats {
        sigchs.push(jsondata!{
            "address", *adr,
            "complete", sg, // is sign ok
        });
    }
    data.insert("signatures", json!(sigchs));
}


