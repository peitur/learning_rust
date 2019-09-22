
fn create_data( s: &str ) -> String {
    let s = String::from( s );
    return s;
}

fn data_work( s: String, l: usize ) -> (String, usize ) {
    let (a, _) = s.split_at( l );
    println!( "{} [{}]", a, a.len() );
    let l = a.len();
    return ( s, l );
}

/*
fn xtest( s: String ) -> (String, usize) {
    let l = s.len();
    return (s, l);
}

fn print_greeting_trim( greeting: String, l: usize ) -> usize {
    let (a, _) = greeting.split_at( l );
    println!( "{} [{}]", a, a.len() );
    return a.len();
}
*/

fn do_and_done( s: &String ) -> usize {
    let l = s.len();
    return l;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn main() {
    let g1 = create_data("Hello, world!");


//    let (g2, a) = xtest( g1 );
    let (g2, a) = data_work( g1, 10 );
//     println!( "oooo {}", g2 );
    println!( "aaaa {}: {}", g2, a );

    let g3 = g2.clone();
    let l3 = do_and_done( &g3 );
    println!( "bbbb {}: {}", g3, l3 );

    let mut g4 = g2.clone();

    change(&mut g4);
    println!( "cccc {}: {}", g4, l3 );
}
