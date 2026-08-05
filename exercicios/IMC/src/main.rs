// Exercício para calcular o índice de massa corporal de uma pessoa e retornar seu IMC e a categoria na qual ela se enquadra, de acordo com a definição da OMS

use std::io; // Importando a biblioteca std::io para interação com o usuário

// Usando constantes para verificação (flags) das categorias de índice de massa corporal
const ABAIXO_PESO: f32 = 18.5;  // Definindo a constante para verificação da categoria Abaixo do peso
const SOBRE_PESO: f32 = 25.0;   // Definindo a constante para verificação da categoria Sobrepeso
const OBESO_PESO: f32 = 30.0;   // Definindo a constante para verificação da categoria Obeso
const CONVERSOR: f32 = 100.0;   // Definindo a constante para conversão de metros para centímetros

fn main() {

    //Interface com o usuário
    println!("Digite sua altura, em centímetros, e seu peso, em quilogramas:");

    // Capturando o primeiro dado: altura

    let mut altura_s: String = String::new();   // Declaração da variável que vai armazenar a entrada de dado do usuário
                                                // Para utilizar uma string de tamanho variável, apenda-se o tipo mut
    io::stdin() // Método para captura de dados a partir de uma interação
        .read_line(&mut altura_s)   // Aqui é lida a linha digitada pelo usuário
        .expect("Não foi possível ler a entrada de dados"); // Aqui é tratado o caso de erro, quando não é possível ler a linha digitada
    
    let mut peso_s: String = String::new();
    io::stdin()
        .read_line(&mut peso_s)
        .expect("Não foi possível ler a entrada de dados");

    // Conversão da string literal para um tipo real
    let altura_f: f32 = altura_s
        .trim() // Método trim() para remover o \n ao final da string
        .parse()    // Método parse() para remover espaços excessivos
        .expect("Valor inválido");  // Método expect() para tratar erro de entrada

    let peso_f: f32 = peso_s
        .trim()
        .parse()
        .expect("Valor inválido");

    // Calculando o índice de massa corporal e armazenando em uma variável, evitando assim a repetição do cálculo
    let imc_f: f32 = peso_f / (altura_f / CONVERSOR).powi(2);

    // Usando a função print! para manter o IMC e a categoria na mesma linha, separando as informações com uma tabulação
    print!("Seu IMC é: {:.2}\t", imc_f);

    // Estrutura de decisão que verifica o IMC e define uma categoria baseada no mesmo
    if imc_f < ABAIXO_PESO{
        println!("Você está categoria 1 - Abaixo do peso");
    }
        else if imc_f >= ABAIXO_PESO && imc_f < SOBRE_PESO{
            println!("Você está na categoria 2 - Peso ideal");
        }
            else if imc_f >= SOBRE_PESO && imc_f <= OBESO_PESO{
                println!("Você está na categoria 3 - Sobrepeso");
            }
                else{
                    println!("Você está na categoria 4 - Obeso");
                }
}