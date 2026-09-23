fn main() {
    // array constante do tipo i32 com 3 itens
    // const precisa ter um tipo declarado
    const ARRAY_EXAMPLE: [i32; 3] = [0; 3];
    //    │              │          │
    //    │              │          └── valor: três zeros
    //    │              └───────────── tipo: array de 3 i32 (o tamanho faz parte do tipo)
    //    └──────────────────────────── nome

    for x in ARRAY_EXAMPLE {
        print!("{x} ");
    }
    print!("\n");

    // outro modo de criar uma array é iniciar todos os seus elementos
    const ARRAY_EXAMPLE_2: [i32; 3] = [10, 20, 30];

    for x in ARRAY_EXAMPLE_2 {
        print!("{x} ");
    }
    print!("\n");

    let array_example_3 = ARRAY_EXAMPLE_2.clone();


    for x in array_example_3 {
        print!("{x} ");
    }
    print!("\n");

}