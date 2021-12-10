use structopt::StructOpt; // biblioteca pra criar programa de linha de comando

#[derive(Debug, StructOpt)] // struct CmdLine herda de Debug e StrucOpt
struct CmdLine { // argumentos da linha de comando necessarios
    input: String
}

fn main() {
    let args = CmdLine::from_args(); // pega os argumentos da linha de comando
    let result = min_window(args.input, String::from("ABC")); // aplica min_window na string de entrada
    println!("{}", result); // printa o resultado
}

fn min_window(input: String, search_set: String) -> String {
    
    let maybe_result = (0..input.len()) // range de 0 ate tamanho da entrada
    .map(|i| { // executa a função lambda abaixo pros valores de 0 ate o tamanho da entrada
        
        // pega referencia de 'input'.
        // Isso é importante porque se não o rust move 'input' pra dentro da função lambda que vai ser usada abaixo
        // e então 'input' seria destruido no final da execução da lambda
        let input_reference = &input;
        
        (i+1..=input.len()) // range da posição atual de i ate o fim da string
        .map(move |j| // executa a função lambda abaixo pros valores de 0 ate o tamanho da entrada
            
          String::from(&input_reference[i..j]) // cria uma substring entre as posições i e j da string de entrada

        )
    }) // Vetor de vetor cotendo todas as combinações da string de entrada
    .flatten() // Tranforma vetor de vetores em um unico vetor de string contendo todas as combinações da string de entrada
    .filter(|word| search_set.chars().all(|c| word.contains(c))) // Filtra o vetor de palavras com todas as combinações pra so conter as que tem todos os carcteres de 'search_set'
    .min_by_key(|word| word.len()); // Retorna a menor palavra do vetor de palavras filtradas. SE HOUVER A MENOR PALAVRA RETORNA Some(filtered_min_word), SE NÃO HOUVER RETORNA None

    if let Some(filtered_min_word) = maybe_result { // Pattern Match, se 'maybe_result' for um Some(filtered_min_word) entra no if criando a variavel 'filtered_min_word'
        
      return filtered_min_word; // existe resultado e retorna ele
    
    }

    return String::from(""); // não existe resultado e retorna string vazia
}

// Testes unitarios
#[cfg(test)]
mod tests {
    use crate::min_window;

    #[test]
    fn case_1() {
      let result = min_window(String::from("abcd"), String::from("bd"));
      assert_eq!(result, "bcd");
    }

    #[test]
    fn case_2() {
      let result = min_window(String::from("ABFKHDFJSGCSKJHBAAC"), String::from("ABC"));
      assert_eq!(result, "BAAC");
    }

    #[test]
    fn case_3() {
      let result = min_window(String::from("BFKHDFJSGCSKJHBA"), String::from("ABC"));
      assert_eq!(result, "CSKJHBA");
    }

    #[test]
    fn case_4() {
      let result = min_window(String::from("BAAC"), String::from("ABC"));
      assert_eq!(result, "BAAC");
    }
}