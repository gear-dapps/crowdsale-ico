use gtest::System;

use ico_io::*;

mod init_ico;
pub use init_ico::*;

#[test]
fn balance_after_two_purchases() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2);

    balance_of(&ico, 0);

    let amount: u128 = 5;
    buy_tokens(&ico, amount, amount * START_PRICE);

    balance_of(&ico, amount);

    sys.spend_blocks(
        (TIME_INCREASE_STEP + 1)
            .try_into()
            .expect("Can't cast type"),
    );

    buy_tokens(&ico, amount, amount * (START_PRICE + PRICE_INCREASE_STEP));

    balance_of(&ico, amount * 2);
}

#[test]
fn owner_balance() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    let amount = 5;
    buy_tokens(&ico, amount, amount * START_PRICE);

    sys.spend_blocks(1001);

    let res: StateIcoReply = ico
        .meta_state(StateIco::BalanceOf(OWNER_ID.into()))
        .expect("Error in meta_state");

    if let StateIcoReply::BalanceOf { address, balance } = res {
        assert!(
            address == OWNER_ID.into() && balance == 0,
            "Error in balance_of()"
        );
    }

    end_sale(&ico);

    let res: StateIcoReply = ico
        .meta_state(StateIco::BalanceOf(OWNER_ID.into()))
        .expect("Error in meta_state");

    if let StateIcoReply::BalanceOf { address, balance } = res {
        assert!(
            address == OWNER_ID.into() && balance == TOKENS_CNT - amount,
            "Error in balance_of()"
        );
    }
}
