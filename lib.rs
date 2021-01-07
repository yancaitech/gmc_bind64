#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod gmc_bind64 {
    pub type Bind64 = [u8; 64];

    #[ink(storage)]
    pub struct GmcBind64 {
        map1: ink_storage::collections::HashMap<AccountId, Hash>,
        map2: ink_storage::collections::HashMap<AccountId, Hash>,
    }

    impl GmcBind64 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                map1: ink_storage::collections::HashMap::new(),
                map2: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn get_hash(&self, accid: AccountId) -> Bind64 {
            let zero_val: Hash = Hash::from([0; 32]);
            let r1 = self.map1.get(&accid).unwrap_or(&zero_val).as_ref();
            let r2 = self.map2.get(&accid).unwrap_or(&zero_val).as_ref();
            let cid: Bind64 = Bind64::from([
                r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7], r1[8], r1[9], r1[10],
                r1[11], r1[12], r1[13], r1[14], r1[15], r1[16], r1[17], r1[18], r1[19], r1[20],
                r1[21], r1[22], r1[23], r1[24], r1[25], r1[26], r1[27], r1[28], r1[29], r1[30],
                r1[31], r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7], r2[8], r2[9],
                r2[10], r2[11], r2[12], r2[13], r2[14], r2[15], r2[16], r2[17], r2[18], r2[19],
                r2[20], r2[21], r2[22], r2[23], r2[24], r2[25], r2[26], r2[27], r2[28], r2[29],
                r2[30], r2[31],
            ]);
            cid
        }

        #[ink(message)]
        pub fn set_hash(&mut self, s: Bind64) -> bool {
            let account_id = self.env().caller();
            let h1: Hash = Hash::from([
                s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9], s[10], s[11], s[12],
                s[13], s[14], s[15], s[16], s[17], s[18], s[19], s[20], s[21], s[22], s[23], s[24],
                s[25], s[26], s[27], s[28], s[29], s[30], s[31],
            ]);
            let h2: Hash = Hash::from([
                s[32], s[33], s[34], s[35], s[36], s[37], s[38], s[39], s[40], s[41], s[42], s[43],
                s[44], s[45], s[46], s[47], s[48], s[49], s[50], s[51], s[52], s[53], s[54], s[55],
                s[56], s[57], s[58], s[59], s[60], s[61], s[62], s[63],
            ]);
            let ro = self.map1.insert(account_id, h1);
            if let None = ro {
                return false;
            }
            let ro = self.map2.insert(account_id, h2);
            if let None = ro {
                return false;
            }
            true
        }
    }
}
