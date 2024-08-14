use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {

    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
    vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
    vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
    vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    sort_works(&mut table);
    show(&table);
    show(&table);


    // P.93
    let mut y = 32;
    {
        let m = &mut y;
        *m += 32;
        assert!(*m == 64);
    }
    assert!(y == 64);

    struct Anime { name: &'static str, bechdel_pass: bool}
    let aria = Anime { name:  "Aria: The Animation", bechdel_pass: true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // 参照を明示するとこうなる
    assert_eq!((*anime_ref).name, "Aria: The Animation");


    let mut v = vec![1973, 1968, 2001];
    v.sort();
    eprintln!("sorted vec: {:?}", v);
    let mut v2 = vec![1973, 1968, 2001];
    (&mut v2).sort(); // 3行上と等価。
    eprintln!("sorted v2: {:?}", v2);

    let x=10;
    let y =20;
    let mut r = &x;
    if true {r = &y;}
    assert!(*r == 20);

    struct Point {x: i32, y: i32}
    let point = Point {x: 1000, y: 729};
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);
    assert!(!std::ptr::eq(rx, ry));

    let r = &fractorial(6);
    assert_eq!(r + &1009, 1729);

    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        // 以下のassert_eq!はコンパイル通らない。xが生きてないところでxへの参照（r）を使おうとしているため。
        // assert_eq!(*r, 1);

        // r = &x、つまり「xに対して借用し、rに格納する」こと自体はエラーにならない。
        // rに格納したxの借用がxのスコープ外で使われていないので、ダングリングポインタにはならない。
    }

    {
        let v0 = vec![99, 100, 10];
        let mut r = &v0[1];

        {
            let v = vec![1, 2, 3];
            r = &v[1];

            eprintln!("inner r is {}", r);
        }
        // あー。最初に "let mut r="で代入した値に戻るとかそういうことはなくて、
        // 借りて持ち出した後に使おうとすると、その時点でアウトになるんだ。
        // eprintln!("outer r is {}", r);
        //そうね、88行目でrは書き換えられてしまうから、ここでrを参照したときにスコープ外だから値が100に戻るとか、そんなことないんだ。


        // つづきは5.3.2「仮引数として参照を受け取る場合」。
    }

}

fn fractorial(n: usize) -> usize {
    (1..n+1).product()
}

#[test]
fn test_fractorial() {
    let f1 = fractorial(1);
    assert!(f1 == 1);

    let f2 = fractorial(2);
    assert!((f2 == 2));

    let f3 = fractorial(3);
    assert!((f3 == 6));
}


fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work)
        }
    }
}

fn sort_works(table: &mut Table) {
    for(_artist, works) in table {
        works.sort();
    }
}