module my_addrx::Counter {
    const E_NUM_DOESNT_EXIST: u64 = 0;
    const E_UNDERFLOW: u64 = 1;

    use std::signer;

    struct Number has key {
        num: u64
    }

    public entry fun increment(account: &signer, number: u64) acquires Number {
        let addrx = signer::address_of(account);
        if (!exists<Number>(addrx)) {
            move_to(account, Number {
                num: 0
            });
        };

        let nums = borrow_global_mut<Number>(addrx);
        nums.num += number;
    }

    public entry fun decrement(account: &signer, number: u64) acquires Number {
        let addrx = signer::address_of(account);
        assert!(exists<Number>(addrx), E_NUM_DOESNT_EXIST);

        let nums = borrow_global_mut<Number>(addrx);
        assert!(nums.num >= number, E_UNDERFLOW);
        nums.num -= number;
    }

    #[view]
    public fun get_number(account: address) : u64 acquires Number {
        if (!exists<Number>(account)) {
            0
        } else {
            borrow_global<Number>(account).num
        }
    }
}