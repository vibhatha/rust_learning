mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}


mod mansion {
    
    fn access_entrance_corridor() {}
    fn access_guest_house() {
        guest_house_1::room1();
        guest_house_2::room2();
    }

    mod guest_house_1 {
        use crate::mansion::access_entrance_corridor;
        fn room1() {}
        pub fn room2() {}
        fn room3() {
            access_entrance_corridor();
        }
    }

    pub mod guest_house_2 {
        use crate::mansion::access_entrance_corridor;
        fn room1() {}
        pub fn room2() {}
        fn room3() {
            access_entrance_corridor();
        }
    }

}


pub fn use_mansion() {
    crate::mansion::guest_house_1::room1();
    crate::mansion::guest_house_2::room2();
}

