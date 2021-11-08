fn main() {
    println!("DFA in Rust 🦀 ");
    let input: &'static str = "aababcaaa";

    let mut dfa:i8 = 0;
    for c in input.chars()
    {
        if dfa == 0
        {
            dfa = start_state(c)
        }
        else if dfa == 1
        {
            dfa = q1(c)
        }
        else if dfa == 2
        {
            dfa = q2(c)
        }
        else if dfa == 3
        {
            dfa = accept()
        }
    }

    fn start_state(character:char)->i8
    {
        if character == 'a'
        {
              return 1;
        }
        else
        {
            return 0;
        }
    }

    fn q1(character:char)->i8
    {
        if character == 'a'
        {
              return 1;
        }
        else if character == 'b'
        {
              return 2;
        }
        else
        {
            return 0;
        }
    }

    fn q2(character:char)->i8
    {
        if character == 'c'
        {
              return 3;
        }
        else if character == 'a'
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }

    fn accept()->i8
    {
        return 3;
    }

    if dfa == 3
    {
        println!("The string was accepted");
    }
    else   
    {
        println!("The string was Not accepted");
    }

}
