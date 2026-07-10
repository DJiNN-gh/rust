// Tipos de dados em Rust
/*
Rust possui diferentes tipos de dados primitivos, assim como as linguagens nas quais ele se baseia, com C e C++.

São eles:
    Inteiros (integer), sendo eles divididos nos tamanhos 
        i8/u8 (1 byte)
        i16/u16 (2 bytes)
        i32/u32 (4 bytes)
        i64/u64 (8 bytes)
        i128/u128 (16 bytes)
    
    Em Rust, assim como em C, inteiros possuem um sinal implícito em sua declaração, definindo-os como signed (+ e -) ou unsigned (+ apenas).

    Reais (float)

    Booleanos (bool)


 */

use std::mem;

 // Definindo os valores máximos para cada tipo de inteiro
const MAX_S_POS_INT8 : i8 = 127;
const MAX_S_NEG_INT8 : i8 = -128;
const MAX_U_INT8 : u8 = 255; 
const MAX_S_POS_INT16 : i16 = 32767;
const MAX_S_NEG_INT16 : i16 = -32768;
const MAX_U_INT16 : u16 = 65535;
const MAX_S_POS_INT32 : i32 = 214748364;
const MAX_S_NEG_INT32 : i32 = -214748365;
const MAX_U_INT32 : u32 = 4294967295;
const MAX_S_POS_INT64 : i64 = 9223372036854775807;
const MAX_S_NEG_INT64 : i64 = -9223372036854775808;
const MAX_U_INT64 : u64 = 18446744073709551615;
const MAX_S_POS_INT128 : i128 = 170141183460469231731687303715884105727;
const MAX_S_NEG_INT128 : i128 = -170141183460469231731687303715884105728;
const MAX_U_INT128 : u128 = 340282366920938463463374607431768211455;

fn main() {

    // Declaração e inicialização das variáveis
    let _i : i8 = 0;
    let _ui : u8 = 0;
    let _j : i16 = 0;
    let _uj : u16 = 0;
    let _k : i32 = 0;
    let _uk : u32 = 0;
    let _l : i64 = 0;
    let _ul : u64 = 0;
    let _m : i128 = 0;
    let _um : u128 = 0;

    // Testando um inteiro com sinal de 1 byte
    println!("Valor máximo de um inteiro de 1 byte com sinal: {_i}", _i = MAX_S_POS_INT8);
    println!("Valor mínimo de um inteiro de 1 byte com sinal: {_i}", _i = MAX_S_NEG_INT8);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_i), mem::size_of_val(&_i) * 8);

    print!("\n");

    // Testando um inteiro sem sinal de 1 byte
    println!("Valor máximo de um inteiro de 1 byte sem sinal: {_ui}", _ui = MAX_U_INT8);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_ui), mem::size_of_val(&_ui) * 8);

    print!("\n");

    // Testando um inteiro com sinal de 2 bytes
    println!("Valor máximo de um inteiro de 2 bytes com sinal: {_j}", _j = MAX_S_POS_INT16);
    println!("Valor mínimo de um inteiro de 2 bytes com sinal: {_j}", _j = MAX_S_NEG_INT16);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_j), mem::size_of_val(&_j) * 8);

    print!("\n");

    // Testando um inteiro sem sinal de 2 bytes
    println!("Valor máximo de um inteiro de 2 bytes sem sinal: {_uj}", _uj = MAX_U_INT16);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_uj), mem::size_of_val(&_uj) * 8);

    print!("\n");

    // Testando um inteiro com sinal de 4 bytes
    println!("Valor máximo de um inteiro de 4 bytes com sinal: {_k}", _k = MAX_S_POS_INT32);
    println!("Valor mínimo de um inteiro de 4 bytes com sinal: {_k}", _k = MAX_S_NEG_INT32);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_k), mem::size_of_val(&_k) * 8);

    print!("\n");

    // Testando um inteiro sem sinal de 4 bytes
    println!("Valor máximo de um inteito de 4 bytes sem sinal: {_uk}", _uk = MAX_U_INT32);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_uk), mem::size_of_val(&_uk) * 8);

    print!("\n");

    // Testando um inteiro com sinal de 8 bytes
    println!("Valor máximo de um inteiro de 8 bytes com sinal: {_l}", _l = MAX_S_POS_INT64);
    println!("Valor mínimo de um inteiro de 8 bytes com sinal: {_l}", _l = MAX_S_NEG_INT64);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_l), mem::size_of_val(&_l) * 8);

    print!("\n");

    // Testando um inteiro sem sinal de 8 bytes
    println!("Valor máximo de um inteiro de 8 bytes sem sinal: {_ul}", _ul = MAX_U_INT64);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_ul), mem::size_of_val(&_ul) * 8);

    print!("\n");

    // Testando um inteiro com sinal de 16 bytes
    println!("Valor máximo de um inteiro de 16 bytes com sinal: {_m}", _m = MAX_S_POS_INT128);
    println!("Valor mínimo de um inteiro de 16 bytes com sinal: {_m}", _m = MAX_S_NEG_INT128);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_m), mem::size_of_val(&_m) * 8);
    
    print!("\n");
    
    // Testando um inteiro sem sinal de 16 bytes
    println!("Valor máximo de um inteiro de 16 bytes sem sinal: {_um}", _um = MAX_U_INT128);
    println!("Tamanho em memória: {} bytes ou {} bits", mem::size_of_val(&_um), mem::size_of_val(&_um) * 8);
}