module my_addrx::HelloWorld {
    use std::vector;
    use std::signer;
    use std::event;

    struct Message has key {
        message: vector<u8>
    }

    #[event]
    struct MessageSet has drop, store {
        message: vector<u8>
    }

    public entry fun set_message(account: &signer, message: vector<u8>) acquires Message {
        let addrx = signer::address_of(account);

        if (!exists<Message>(addrx)) {
            move_to(account, Message {
                message
            });
        };

        let msg = borrow_global_mut<Message>(addrx);
        msg.message = message;

        event::emit(MessageSet {
            message
        });
    }

    #[view]
    public fun get_message(account: address) : vector<u8> acquires Message {
        if (!exists<Message>(account)) {
            vector::empty<u8>()
        } else {
            borrow_global<Message>(account).message
        }
    }

}