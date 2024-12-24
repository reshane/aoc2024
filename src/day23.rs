
pub fn solve() {
    let contents = std::fs::read_to_string("input_23.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(contents: String) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    contents.lines()
        .for_each(|line| {
            if let Some((l, r)) = line.split_once("-") {
                map.entry(l.to_string())
                    .and_modify(|nbors|{
                        nbors.push(r.to_string());
                    }).or_insert(
                        vec![r.to_string()]
                    );
                map.entry(r.to_string())
                    .and_modify(|nbors|{
                        nbors.push(l.to_string());
                    }).or_insert(
                        vec![l.to_string()]
                    );
            }
        });
    map
}

fn solve_p1(contents: String) -> i64 {
    let netw = parse_input(contents);
    let mut triplets = Vec::<HashSet<String>>::new();
    netw.iter()
        .filter(|host| {
            host.0[0..1] == *"t"
        })
        .for_each(|host| {
            for i in 0..host.1.len()-1 {
                for j in i+1..host.1.len() {
                    if netw.get(&host.1[j]).expect("where is it").contains(&host.1[i])
                    && netw.get(&host.1[i]).expect("where is it").contains(&host.1[j]) {
                        let triplet = HashSet::from_iter(
                            vec![host.0.to_owned(), host.1[i].clone(), host.1[j].clone()].into_iter()
                        );
                        if !triplets.contains(&triplet) {
                            triplets.push(triplet);
                        }
                    }
                }
            }
        });
    // println!("{:?}", triplets);
    triplets.len() as i64
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_23.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 7);
}

fn find_maximal_cliques(
    r: HashSet<String>, 
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    map: &HashMap<String, Vec<String>>,
) -> Vec<HashSet<String>> {
    let mut res = Vec::<HashSet<String>>::new();
    if p.is_empty() && x.is_empty() {
        res.push(r.clone());
    }

    let p_v = p.iter().cloned().collect::<Vec<String>>();
    for v in p_v {
        let mut ruv = r.clone();
        ruv.insert(v.to_string());

        let pinv = p.clone().intersection(
            &HashSet::from_iter(map.get(&v).unwrap().clone().into_iter())
        ).map(|s| { s.to_string() }).collect::<HashSet<String>>();

        let xinv = x.clone().intersection(
            &HashSet::from_iter(map.get(&v).unwrap().clone().into_iter())
        ).map(|s| { s.to_string() }).collect::<HashSet<String>>();

        res.append(&mut find_maximal_cliques(
            ruv,
            pinv,
            xinv,
            map,
        ));
        p.remove(&v);
        x.insert(v.to_string());
    }
    res
}

fn solve_p2(contents: String) -> String {
    let netw = parse_input(contents);
    let p = netw.keys()
        .map(|k| { k.to_string() })
        .collect::<HashSet<String>>();
    let mut cliques = find_maximal_cliques(
        HashSet::new(), 
        p, 
        HashSet::new(), 
        &netw
    );
    cliques.sort_by(|a,b| {
        b.len().cmp(&a.len())
    });
    let mut max = cliques[0].clone()
        .into_iter()
        .collect::<Vec<String>>();
    max.sort();
    max.join(",")
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_23.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == String::from("co,de,ka,ta"));
}
