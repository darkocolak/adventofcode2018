use std::collections::HashMap;
use std::string::String;

fn main() {
    let ids: [&str;  250] = ["uqcipadzntnheslgvjjozmkfyr", "uqcipadzwtnhexlzvxjobmkfkr", "cqcipadpwtnheslgyxjobmkfyr", "ubnipadzwtnheslgvxjobmkfyw", "uqcisadzwtnheslgvxjfbmkfor", "uqcisaezwtnheslgvxkobmkfyr", "uqcguadzwtnheslgvxjobmkfir", "uqcipadzmtnhesldvxdobmkfyr", "uqcipadzwtzheslgdxjtbmkfyr", "uquipadzwtcheslgvxjobmkfbr", "uqctpadzwtnhesjbvxjobmkfyr", "ueciparzwtnheslgvxjobmkfyx", "uqcipadzwtnhessgvxjkbmkfkr", "uqcipamzwtnheslgvxioamkfyr", "uciizadzwtnheslgvxjobmkfyr", "uqcieadzwtnhesfgvxeobmkfyr", "fqcipadzwtnreslgvkjobmkfyr", "uqcipadzrtnherlgvxjobmklyr", "uqcypadzwtnheslgvxjobmkxfr", "uqcipadzwtnhemlgvxjobmvfur", "uwciuadzwwnheslgvxjobmkfyr", "uqcipadzwtnhcscgvxjobmkuyr", "upripadzwtnheslovxjobmkfyr", "uqcipadzltnheslgvxjobmkftc", "uqcipadzwtnheslgvgjobmifsr", "uqoipadzwtnheslgvxjosmkfkr", "uqcipadzwtbhesrqvxjobmkfyr", "uqcipadzwtnheslpvxjobmhfyx", "uhcinadzwtnheslgvxjybmkfyr", "uqcipadzwtnhhslgvxjabmkbyr", "uecipadzwtnheslgvxjobqyfyr", "uqcipadfwtnheslwvxjobgkfyr", "uqcipadzvtnheshgvxzobmkfyr", "fqcipadzwtcheslgvxjobmkfyt", "uecipadzwtnheslgpxjbbmkfyr", "uqclpadzwtnheslgvnjobukfyr", "qqciprdzetnheslgvxjobmkfyr", "uqcipahpwtnheslgvxjtbmkfyr", "uqcidadzwtnhesljvxyobmkfyr", "uqciradswtnqeslgvxjobmkfyr", "uqcipadzwtrhmslgvxjobmkfyf", "urcipadzjtnheslgvxfobmkfyr", "uqcipadzwznheslgvxjobmkfcv", "uqcipadowtnheslgyxjobmkfym", "uqcigadzwtnheslgvxjoomkmyr", "uqjipafzwtnheslgvejobmkfyr", "uqcioadzwtnhhslgvxzobmkfyr", "uqcgpadkwtnheslgvxjobhkfyr", "ufciiadewtnheslgvxjobmkfyr", "uqoipadzwtnheslgvxjllmkfyr", "uqcipadzutnheslgwxxobmkfyr", "uqcipadzwtlheslgaxjobmkfwr", "uqcbpadzutnheslgvxjbbmkfyr", "uucipadzwvnhesngvxjobmkfyr", "uqcifadzwtnceslgvxjoumkfyr", "ujcipadzwteheslgvxjobmkfyj", "uqcipadzwtnheslqvxjobmkuyp", "uqcipadzwtnheslgvxjoxmkxyw", "uqcipaduwtnheslgvujmbmkfyr", "uicipadnwtnheslgvxjobmbfyr", "uqcipadzwteheslgvxjobbmfyr", "uqcipadzwgnneslgvxjobmklyr", "uqcipadzxtnhwslgvbjobmkfyr", "uqcipaxwwtnheslxvxjobmkfyr", "uocipadzwtnheslgvxjobqdfyr", "uqciaauzwtnheslgtxjobmkfyr", "uncipagzwtnkeslgvxjobmkfyr", "uqcipadzwtnhehlgvxjohdkfyr", "uqcipadzwtnheslgvxjobmspyz", "uccipadzwtnhvsltvxjobmkfyr", "uacipagzwtnheslgvxjoqmkfyr", "tqcipaduwtnheslgvxjobmmfyr", "uqcipadzwtnheslgvxqebmifyr", "uecipadthtnheslgvxjobmkfyr", "uocipadzwtnhdslgvkjobmkfyr", "uqcipadtwtnheslgvxhobmufyr", "uqkipadzwtnleslgtxjobmkfyr", "uqcipadzjunheslgvxjobmnfyr", "ubcipadzwtvheslgvxjobmkfyf", "uqcipadzwpfheslgvxjsbmkfyr", "uocipadzwtndeslgvxjobmmfyr", "uqcipadzwtnheslgtxjobhkfyq", "uqcipadzwtrheslgvxjobmyfya", "uqcipadzwtvheslgvxjolgkfyr", "uqcipidzwtaheslgvxjobmkfxr", "uyzixadzwtnheslgvxjobmkfyr", "uqyihadzwtnhedlgvxjobmkfyr", "uqcipadzwtnhesltvejobqkfyr", "uqciptdzwtnheslgyxlobmkfyr", "uqcipzdzwtnhzslgvxjosmkfyr", "uqcipadzwtnbeslgexjobmkfvr", "uqcipadzwtnheslcwxjobmkkyr", "uqcapadzwcnheslgvxjolmkfyr", "uqcjpadzwtnhejlgvxjxbmkfyr", "uqcipadwwtxweslgvxjobmkfyr", "uqmipadzwtnhezlgvxjobmkyyr", "uqcipubzwtnpeslgvxjobmkfyr", "uecvpadzwtnheslgvxjocmkfyr", "uqcipadzwfnheslgvxjibmkdyr", "uqcipadzwtnheslgvxvfbykfyr", "uqcipadzwtnheslgvgjoimkfyt", "dqcqpaqzwtnheslgvxjobmkfyr", "uqcipbdzwtnheslgvxjobmkghr", "jqcipadzwtnheslgvxjgbmkzyr", "uqcipadzwtnheslgvxqkqmkfyr", "uqcipadzptnheslgvxjxbokfyr", "uucijadzwtwheslgvxjobmkfyr", "uccfpadzwtnheslgvxjobpkfyr", "uqcipadzwtnheslgvxjobakeyq", "uqcipadzwtnheolgvxqobjkfyr", "imiipadzwtnheslgvxjobmkfyr", "uqcehadzwtnheslgvxjobmkuyr", "uqcipadzztnheslgvxjorokfyr", "rqcixadzwtnheelgvxjobmkfyr", "uqcipadzwtzheslgvxjodmkfyi", "uqcipaezwtnwuslgvxjobmkfyr", "uqcipadzwtnheslggxjobjkfyq", "uqcipadzwkghesagvxjobmkfyr", "uqcypqdzwtnheslgvxjobakfyr", "iqcipadzwtnhezltvxjobmkfyr", "uxcimadzwtnheslgvxjobmxfyr", "uqcipaizwtvhwslgvxjobmkfyr", "uqcipafzwtnheslgvxjpbmkfym", "uqcipadzwinheslgvxlobmpfyr", "uqcupadzwtnheslkvxmobmkfyr", "uqcapadzwtnhesrgvxjobmkfsr", "urcipafzwtnheslgvxjobmkfur", "uqcipaczwtnheslgvbjobmknyr", "uqcizadzztgheslgvxjobmkfyr", "uqcipfdzwtnhesxgvxjobmkfyw", "uqcipbdzwtnhyslgvxjobmcfyr", "uqcipadzwanhezlgvxjobmkfwr", "uvcipadzwtnheslgvxjbkmkfyr", "uqcipajzwtnseslgvxjobmkfyq", "uqcipvdzwtnheslgvmlobmkfyr", "uqcipadzdgnheslgmxjobmkfyr", "uqcipddzwtnhestgvpjobmkfyr", "umcipadzwtdheslgvxjzbmkfyr", "uqciuwdzwtnheslgvxjobmkflr", "uqcipadzwtnheslgsxabbmkfyr", "uceipadzwtnheslgvxjobgkfyr", "mqcipadzwtnhesrgvxjobmjfyr", "aqcipadvwtnheslgvxjobmkryr", "uqsipadzwtnofslgvxjobmkfyr", "uqcixadzwtfheslgvxjzbmkfyr", "uqcipadnwfnheslgvxjohmkfyr", "uqcivadzwtnheslfvxjobmkfyz", "uqciprdzwtnheslgvxjobmkjir", "uqcipadhbtnheslgvxjoxmkfyr", "fqcipadzwtnhesfgvxjobmkfye", "uqoipqdzwtnheqlgvxjobmkfyr", "uqcipadzwtnhesltvxmobmkzyr", "uqcipadzwtnhebqgvsjobmkfyr", "uqcipadzwtnheslglxjobmfbyr", "gqcipadzwtgheslgvxjobwkfyr", "uqcipadzwtnheslgfxjzbmlfyr", "ujcnpadzwtnheslrvxjobmkfyr", "ujcivadzwtnheglgvxjobmkfyr", "uqcitadzwgnheslgvxjofmkfyr", "uqcipahzatnhmslgvxjobmkfyr", "uqzipaizwtnheslgvujobmkfyr", "uqcipadzltnheylgvnjobmkfyr", "uqcidadzwtnhwsljvxyobmkfyr", "uqcipadzwtihetlgvxjobhkfyr", "oqcipabzwtnheslgvfjobmkfyr", "uqcipadzwtnveslgvxjobzkfzr", "uqcipadzwtjheslgqxjobmlfyr", "uqcnpadzztnheslgvxjobmkoyr", "uqciuadzwonheslgvxjobmkfyz", "tqcipadzwtnheslgvxaobmqfyr", "uqcipadtwtnhqslgvxjobmkeyr", "uqcipadzwbnheslgvajobmsfyr", "ubcopadzwtnhgslgvxjobmkfyr", "uqcipydzwtwheslgvxjobakfyr", "cqbijadzwtnheslgvxjobmkfyr", "uscipadowtnheslgvxjobmkfcr", "uqcipadzwtgheslnvxjobskfyr", "uqcipzdzwtnzeslgkxjobmkfyr", "uqcipawzwtnhrslgbxjobmkfyr", "uqcipadzatchyslgvxjobmkfyr", "uqcipadzotnpeslgvxjobmjfyr", "uqcipagzwtnheslgvxjobmvfyt", "uqcipadzwhnheslgvxyobmkfyo", "uqcipadzwtnheslgmqjobmkfyc", "uqcupadzwgnheslgvcjobmkfyr", "uqcipabzwbnheslgvxjobmkwyr", "uqciiadzwtnheslgvxjobmkfmz", "uqkipauzwtnheslgvxjjbmkfyr", "uqcipidzetnheslgvxjobmkfyi", "uqcipadzwtnheslgqxjokmkfmr", "uqcipadzqtnhesllvxjobmkfyk", "uqccpadzwtnheslgmxsobmkfyr", "uqcipadzwteheslgvljfbmkfyr", "uqcipadxwinheslgaxjobmkfyr", "uqcipadzwtnheslhvxyobmkfjr", "aqcipadzwnnheslgvxjqbmkfyr", "uvcipadzwtnheszgvxjobmkfyg", "uqcipahzmtnheslgvxjobmkfir", "ukcipadzbtnheslgvxjobmkfyb", "uqcipadzwtnhemlgvqjobmkfpr", "uqcipadzwtnheslgvmeobmkfpr", "uqciphdrwtnheslgvxjobmkfyw", "uqcipadzwtnheslevxqobzkfyr", "uqcipadzwknzeslgvxnobmkfyr", "wqcipadzwjnheslgvxjobbkfyr", "uqcipadzwtdheslgvmjobmkjyr", "uqvipadzwtnhextgvxjobmkfyr", "uqhipadzwtnheslwvxjzbmkfyr", "uqcipadzwtnherlgsxjobmksyr", "uqcipadzwtnhesqgvxjotmvfyr", "udcipadzwtnhekwgvxjobmkfyr", "uqcjprdzwtnheslgvxjobmkfpr", "uqcipadzatnheclgvqjobmkfyr", "uqcbpadzctnheslqvxjobmkfyr", "uqcipadzqtnhesluvxjobrkfyr", "uqcipadzwtnhcslgvxjoomwfyr", "uqcppadzwxnheslgwxjobmkfyr", "uqcipadcwtnheslrvxjdbmkfyr", "ukcipadzwtnhhslgvxjobmkgyr", "uqckpadzwtnheslgvxjokmkiyr", "uqcspadzwtjheslgvxjobmkfjr", "uqcipadpwtnhsslgvxjobmkfyu", "uqcepadzwtnheilgvbjobmkfyr", "jqcipadiwtnheslgvxjobmkjyr", "uqcipadzrtnseslgqxjobmkfyr", "sqmipadzwtnhewlgvxjobmkfyr", "uqcieadzhtnheslgvgjobmkfyr", "uqcipadzwkwhewlgvxjobmkfyr", "uqcipadzwtzheslgvxjpbqkfyr", "uzcipadzjtnheslgvxjobmlfyr", "uqcipadzwtnheslnvxjobmkfee", "uqciyanzwtnheslgvxjoimkfyr", "uqcipadqwtnheswghxjobmkfyr", "uycipadzwtnheslovxjobmofyr", "uqcipadzwtnheslgvxcozmxfyr", "uqmipadzwtnxezlgvxjobmkfyr", "uqcipadzftnheslgvxjotmkffr", "aqcipaizwtnhesagvxjobmkfyr", "uqcipcdzwtnheslgoajobmkfyr", "uqcypadgwtnhesbgvxjobmkfyr", "uqcipcdzwtnheslgvxjebmkfyb", "uhcvpadzwtnheslgvxjobzkfyr", "uqcipadzwtnpesagvxmobmkfyr", "uqcipadzwtnidslgvxjobmkfor", "uqcipadkwtnhesigvxjzbmkfyr", "uqcypadlwtnheslsvxjobmkfyr", "qqcipadzwtnheswgvxjobmkoyr", "uqcipadzwtnheslgvxjhbmmcyr", "uqcipadzwtnhesogvxjormkfmr", "uqcipadzwtnhetcgvxgobmkfyr"];
    let mut count_of_twos = 0;
    let mut count_of_threes = 0;    
    for id in ids.iter() {
        let count_map = count_all_in_string(id);
        count_of_threes += get_count_for_nubmer(&count_map, &3);
        count_of_twos += get_count_for_nubmer(&count_map, &2)
    }
    println!("{:?}", count_of_threes * count_of_twos);  
}

fn get_count_for_nubmer(map: &HashMap<char, i32>, number: &i32) -> i32 {
    for (key, val) in map.iter() {
        if val == number {
            return 1;
        }        
    }
    0
}

fn count_all_in_string(s: &str) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();
    for c in s.chars() {
        if !char_counts.contains_key(&c) {
            let char_count = count_in_string(s, c);
            char_counts.insert(c, char_count);
        }
    }
    char_counts
}

fn count_in_string(s: &str, character: char) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        if c == character {
            count +=1;
        }
    }
    count
}
