pub fn limpar_terminal(){
    print!("{esc}c", esc = 27 as char);
}