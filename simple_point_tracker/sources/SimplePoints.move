module my_addrx::SimplePoint {
    use std::signer;

    const E_ALREADY_INITIALIZED: u64 = 0;

    struct Points has key {
        points: u64
    }

    struct Supply has key {
        total_supply: u64
    }

    fun init_module(admin: &signer) {
        let addrx = signer::address_of(admin);
        assert!(!exists<Supply>(addrx), E_ALREADY_INITIALIZED);

        move_to(admin, Supply {
            total_supply: 0
        });
    }

    public entry fun mint_to(admin: &signer, to: address, amount: u64) {}

    public entry fun burn_from(admin: &signer, from: address, amount: u64) {}

    public entry fun transfer(from: &signer, to: address, amount: u64) {}
}