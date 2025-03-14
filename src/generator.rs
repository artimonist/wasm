use crate::encrypt;
use artimonist::{ComplexDiagram, GenericDiagram, Password, SimpleDiagram, ToMatrix, Xpriv, BIP85};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simple_init(values: Vec<String>, salt: String) -> String {
    let mx = values
        .into_iter()
        .map(|v| v.chars().next())
        .collect::<Vec<_>>()
        .to_matrix();
    let master = SimpleDiagram(mx)
        .bip32_master(salt.as_bytes())
        .expect("failed to create master")
        .to_string();
    encrypt(&master, true)
}

#[wasm_bindgen]
pub fn complex_init(values: Vec<String>, salt: String) -> String {
    let master = ComplexDiagram(values.to_matrix())
        .bip32_master(salt.as_bytes())
        .expect("failed to create master")
        .to_string();
    encrypt(&master, true)
}

#[wasm_bindgen]
pub fn generate(master: &str, target: &str, min: u32, max: u32) -> Vec<String> {
    if !["mnemonic", "xpriv", "wif", "pwd", "emoji"].contains(&target) {
        crate::log!("unkown target: {target}, possible values: [mnemonic, wif, xpriv, pwd, emoji]");
        return vec![];
    }
    let master_str = encrypt(master, false);
    let master = Xpriv::from_str(&master_str).expect("failed to parse master");
    (min..=max)
        .map(|i| {
            match target {
                "mnemonic" => master.bip85_mnemonic(Default::default(), 24, i),
                "xpriv" => master.bip85_xpriv(i),
                "wif" => master.bip85_wif(i).map(|w| [w.addr, w.pk].join(" ")),
                "pwd" => master.bip85_pwd(Password::Distinct, 20, i),
                "emoji" => master.bip85_pwd(Password::Emoji, 20, i),
                _ => Ok("".to_string()),
            }
            .expect("failed to generate")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram() {
        const MASTER_SIMPLE: &str = "xprv9s21ZrQH143K3DsMn27o9Dw3iwDWJa6ztqdbeyVoMm1UjeK4PQYZPqxpyu5hYGm3qzzB2p1HvZHGoEK1Vwu84SvbcuygptA9kguvhXfDVYN";
        const MASTER_COMPLEX: &str = "xprv9s21ZrQH143K2n99UMaa9UBh8XsecHCxbVus8dTCKV61jm5CfTwS1G2aXVfnu9Z1h7C2tAgmAJyUegPCMgcVmJx5pVwSvBupHJWiRfbcTpb";

        let values: [String; 15] = core::array::from_fn(|_| "abc".to_string());
        let simple = simple_init(values.to_vec(), "salt".to_string());
        assert_eq!(encrypt(&simple, false), MASTER_SIMPLE);
        let complex = complex_init(values.to_vec(), "salt".to_string());
        assert_eq!(encrypt(&complex, false), MASTER_COMPLEX);
    }

    #[test]
    fn test_generate() {
        const MASTER_KEY: &str = "xprv9s21ZrQH143K3DsMn27o9Dw3iwDWJa6ztqdbeyVoMm1UjeK4PQYZPqxpyu5hYGm3qzzB2p1HvZHGoEK1Vwu84SvbcuygptA9kguvhXfDVYN";
        const MNEMONIC: &str = "blast maximum essay guard host shaft tone obscure auction buddy slush teach impulse leave spike lizard spray silver crack february ride color interest february";
        const XPRIV: &str = "xprv9s21ZrQH143K4au5psAVwa5oKSJx7Anzzecurk5CRpaoJ4nBYhFhjPSpjQP59pT9qwCV3uyXN3zucrzBDQ2vzG5LUdf91URk73L56VDmVPg";
        const WIF: &str = "37rCnHecnr8HLcZ75NLc1y2VsTirMuhG4D L1VB5cWqQ1MFidkimPbCknAuEXk2QHGTYmNuvSdyDzfXAsJ9FCPV";
        const PWD: &str = "%pKsb#E*tZV1HRfPSZz@";
        const EMOJI: &str = "⚡🚀🐶🐷🚗🍉🍺🔔🔔🍕🍒☀🍕💊🍟🏠🚀👻☔🐴";

        let master = encrypt(MASTER_KEY, true);
        let mnemonic = generate(&master, "mnemonic", 1, 1);
        assert_eq!(mnemonic[0], MNEMONIC);
        let xpriv = generate(&master, "xpriv", 1, 1);
        assert_eq!(xpriv[0], XPRIV);
        let wif = generate(&master, "wif", 1, 1);
        assert_eq!(wif[0], WIF);
        let pwd = generate(&master, "pwd", 2, 2);
        assert_eq!(pwd[0], PWD);
        let emoji = generate(&master, "emoji", 1, 1);
        assert_eq!(emoji[0], EMOJI);
    }
}
