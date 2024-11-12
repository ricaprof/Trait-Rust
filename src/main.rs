use std::ops::{Mul, Add};

// Define o trait genérico Calculavel<T> que funciona com qualquer tipo numérico que satisfaça as restrições
trait Calculavel<T> {
    fn area(&self) -> Result<T, String>;
    fn perimetro(&self) -> Result<T, String>;
    fn calcular(&self); //Exibe uma mensagem com os valores das funções acima
}

// Definição da struct genérica Circulo<T>
struct Circulo<T> {
    raio: T,
}

// Estrutura Retangulo, que implementará o trait Calculavel<T>
struct Retangulo<T> {
    largura: T,
    altura: T,
}

// Implementa o trait Calculavel para Retangulo<T>
impl<T> Calculavel<T> for Retangulo<T>
where
    T: Copy + PartialOrd + Mul<Output = T> + Add<Output = T> + From<f32> + std::fmt::Display,
{
    fn area(&self) -> Result<T, String> {
        //Verifica se ambos os valorese são positivos
        if self.largura < T::from(0.0) || self.altura < T::from(0.0) {
            return Err("Erro: Largura e altura devem ser valores positivos.".to_string());
        }
        Ok(self.largura * self.altura)
    }
    
    fn perimetro(&self) -> Result<T, String> {
        //Verifica se ambos os valores são positivos
        if self.largura < T::from(0.0) || self.altura < T::from(0.0) {
            return Err("Erro: Largura e altura devem ser valores positivos.".to_string());
        }
        Ok((self.largura + self.altura) * T::from(2.0))
    }
    
    fn calcular(&self) {
        //Calcula a área
        match (self.area(), self.perimetro()) {
            (Ok(area), Ok(perimetro)) => println!(
                "Retângulo de largura {} e altura {}\nÁrea: {}\nPerímetro: {}",
                self.largura, self.altura, area, perimetro
            ), //Exibe a mensagem de erro
            (Err(e), _) | (_, Err(e)) => println!("{}", e),
        }
    }
}

// Implementa o trait Calculavel para Circulo<T>
impl<T> Calculavel<T> for Circulo<T>
where
    T: Copy + PartialOrd + Mul<Output = T> + Add<Output = T> + From<f32> + std::fmt::Display,
{
    fn area(&self) -> Result<T, String> {
        //Verifica se ambos os valores são positivos
        if self.raio < T::from(0.0) {
            return Err("Erro: O raio deve ser um valor positivo.".to_string());
        }
        let pi = T::from(3.1415);
        Ok(self.raio * self.raio * pi)
    }

    fn perimetro(&self) -> Result<T, String> {
        //Verifica se ambos os valores são positivos
        if self.raio < T::from(0.0) {
            return Err("Erro: O raio deve ser um valor positivo.".to_string());
        }
        let pi = T::from(3.1415);
        Ok(self.raio * T::from(2.0) * pi)
    }

    fn calcular(&self) {
        //Calcula a área
        match (self.area(), self.perimetro()) {
            (Ok(area), Ok(perimetro)) => println!(
                "Círculo de raio {}\nÁrea: {}\nPerímetro: {}",
                self.raio, area, perimetro
            ),
            //Exibe a mensagem de erro, caso tenha uma
            (Err(e), _) | (_, Err(e)) => println!("{}", e),
        }
    }
}
#[test]
fn valores_invalidos(){

    // Teste com Retangulo<f32> com valor inválido
    let retangulo_f32 = Retangulo {
        largura: -5.0_f32,
        altura: 3.0_f32,
    };
    retangulo_f32.calcular();

    // Teste com Retangulo<f64> com valor inválido
    let retangulo_f64 = Retangulo {
        largura: 5.0,
        altura: -3.0,
    };
    retangulo_f64.calcular();

    // Teste com Circulo<f32> com valor inválido
    let circulo_f32 = Circulo { raio: -5.0_f32 };
    circulo_f32.calcular();

    // Teste com Circulo<f64> com valor inválido
    let circulo_f64 = Circulo { raio: -5.0 };
    circulo_f64.calcular();    
    
}

fn main() {
    //Os testes com valores inválidos estão na função valores_invalidos
    // Teste com Retangulo<f32> com valores válidos
    let retangulo_f32 = Retangulo {
        largura: 5.0_f32,
        altura: 3.0_f32,
    };
    retangulo_f32.calcular();

    // Teste com Retangulo<f32> com valores inválidos
    let retangulo_f64 = Retangulo {
        largura: 5.0,
        altura: 3.0,
    };
    retangulo_f64.calcular();

    // Teste com Circulo<f32> com valor válido
    let circulo_f32 = Circulo { raio: 5.0_f32 };
    circulo_f32.calcular();

    // Teste com Circulo<f64> com valor inválido
    let circulo_f64 = Circulo { raio: 5.0 };
    circulo_f64.calcular();
}
