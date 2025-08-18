fn main() {
    let arr = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let sotni = arr[0] / 100;
    let des = (arr[0] / 10) % 10;
    let odin = arr[0] % 10;

    if des == 0 && odin == 0 {
        println!("{sotni}{odin}{des}");
    } else if sotni <= des && des <= odin && sotni != 0 {
        if des <= odin {
            println!("{sotni}{des}{odin}");
        } else {
            println!("{sotni}{odin}{des}");
        }
    } else if des <= sotni && des <= odin && des != 0 {
        if sotni <= odin {
            println!("{des}{sotni}{odin}");
        } else {
            println!("{des}{odin}{sotni}");
        }
    } else if odin <= sotni && odin <= des && odin != 0 {
        if sotni <= des {
            println!("{odin}{sotni}{des}");
        } else {
            println!("{odin}{des}{sotni}");
        }
    } else if sotni == 0 {
        if des <= odin {
            println!("{des}{sotni}{odin}");
        } else {
            println!("{odin}{sotni}{des}");
        }
    } else if des == 0 {
        if sotni <= odin {
            println!("{sotni}{des}{odin}");
        } else {
            println!("{odin}{des}{sotni}");
        }
    } else if sotni <= des {
        println!("{sotni}{odin}{des}");
    } else {
        println!("{des}{odin}{sotni}");
    }
}
