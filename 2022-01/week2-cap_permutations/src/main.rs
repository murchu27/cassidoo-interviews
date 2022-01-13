fn main() {
    println!("Running cap_permutations(\"ab2\")");
    println!("{:?}", cap_permutations("ab2"));

    println!("Running cap_permutations(\"35p\")");
    println!("{:?}", cap_permutations("35p"));

    println!("Running cap_permutations(\"6xo9xo\")");
    println!("{:?}", cap_permutations("6xo9xo"));
}

fn cap_permutations(s: &str) -> Vec<String> {
    let mut perms: Vec<String> = vec![String::from("")];

    for c in s.chars() {
        // if character is an ascii alphabetic character,
        // we add uppercase and lowercase permutations
        if c.is_ascii_alphabetic() {
            // make new Vector to hold lowercase permutations
            let mut perms_l: Vec<String> = Vec::new();

            for p_u in &mut perms {
                let mut p_l: String = p_u.clone();

                p_u.push(c.to_ascii_uppercase());

                p_l.push(c.to_ascii_lowercase());
                perms_l.push(p_l);
            }

            perms.append(&mut perms_l);
        }
        // otherwise, we just add the character
        else {
            for p in &mut perms {
                p.push(c);
            }
        }
    }

    return perms;
}
