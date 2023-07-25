#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract(env=ecies_extension::EciesEnvironment)]
mod ink_ecies {
    use ink::prelude::vec::Vec;

    use ecies_extension::EciesErr;

    #[ink(storage)]
    pub struct InkEcies {
        pub public_key: Vec<u8>,
        pub encrypted: Vec<u8>,
    }

    impl InkEcies {
        #[ink(constructor)]
        pub fn new(public_key: Vec<u8>) -> Self {
            Self {
                public_key,
                encrypted: Vec::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get_public_key(&self) -> Vec<u8> {
            self.public_key.clone()
        }

        #[ink(message)]
        pub fn get_encrypted(&self) -> Vec<u8> {
            self.encrypted.clone()
        }

        #[ink(message)]
        pub fn set_public_key(&mut self, public_key: Vec<u8>) {
            // pk needs to be valid
            // e.g. "0x02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5" (sk is 2)
            self.public_key = public_key;
        }

        #[ink(message)]
        pub fn encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, EciesErr> {
            self.env().extension().encrypt(self.public_key.clone(), msg)
        }

        #[ink(message)]
        pub fn encrypt_and_save(&mut self, msg: Vec<u8>) -> Result<Vec<u8>, EciesErr> {
            let encrypted = self.encrypt(msg)?;
            self.encrypted = encrypted;
            Ok(self.encrypted.clone())
        }
    }
}
