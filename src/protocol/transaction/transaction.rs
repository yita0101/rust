// Action list
StructFieldDynList!{
    DynListVMAction, 
    Uint2, VMAction, vm::action::create
}


// TransactionType1
DefineCommonTransaction!{
    TX_TYPE_1_DEPRECATED, TransactionType1
}


// TransactionType2
DefineCommonTransaction!{
    TX_TYPE_2, TransactionType2
}



pubFnTransactionsTypeDefineCreate!{

    TX_TYPE_0_COINBASE   , 0u8, TransactionCoinbase
    TX_TYPE_1_DEPRECATED , 1u8, TransactionType1
    TX_TYPE_2            , 2u8, TransactionType2

}




