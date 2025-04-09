struct RegistroVenda {
    mes: f64,
    valor: f64,
}

fn calcular_media (valores: &[f64]) -> f64 {
    let soma: f64 = valores.iter().sum::<f64>();
    let media: f64 = soma / valores.len() as f64;
    media
}

fn calcular_inclinacao(dados: &[RegistroVenda]) -> f64 {
    let meses: Vec<f64> = dados.iter().map(|d| d.mes).collect();
    let valores: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let media_meses = calcular_media(&meses);
    let media_valores = calcular_media(&valores);

    let numerador: f64 = dados.iter()
        .map(|d| (d.mes - media_meses) * (d.valor - media_valores))
        .sum();
    let denominador: f64 = dados.iter()
        .map(|d| (d.mes - media_meses).powi(2))
        .sum();
    numerador / denominador
}

fn calcular_intercepto(dados: &[RegistroVenda], inclinacao: f64) -> f64 {
    let media_meses = calcular_media(&dados.iter()
                .map(|d| d.mes).collect::<Vec<f64>>());
    let media_valores = calcular_media(&dados.iter()
                .map(|d| d.valor).collect::<Vec<f64>>());
    media_valores - (inclinacao * media_meses)
}

fn prever_valor(intercepto: f64, inclinacao: f64, mes: f64) -> f64 {
    intercepto + (inclinacao * mes)
}

fn calcular_r_quadrado(dados: &[RegistroVenda], intercepto: f64, inclinacao: f64) -> f64 {
    let valores_reais: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let valores_previstos: Vec<f64> = dados.iter().map(|d| prever_valor(intercepto, inclinacao, d.mes)).collect();
    let media_valores = calcular_media(&valores_reais);

    let ss_res: f64 = valores_reais.iter().zip(valores_previstos.iter())
        .map(|(&real, &previsto)| (real - previsto).powi(2))
        .sum();

    let ss_tot: f64 = valores_reais.iter()
        .map(|&real| (real - media_valores).powi(2))
        .sum();

    if ss_tot == 0.0 {
        if ss_res == 0.0 {
            1.0
        } else {
            0.0
        }
    } else {
        1.0 - (ss_res / ss_tot)
    }
}

fn calcular_erro_quadratico_medio(dados: &[RegistroVenda], intercepto: f64, inclinacao: f64) -> f64 {
    let valores_reais: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let valores_previstos: Vec<f64> = dados.iter().map(|d| prever_valor(intercepto, inclinacao, d.mes)).collect();
    let n = valores_reais.len() as f64;

    let soma_quadrados_erro: f64 = valores_reais.iter().zip(valores_previstos.iter())
        .map(|(&real, &previsto)| (real - previsto).powi(2))
        .sum();

    soma_quadrados_erro / n
}

fn main() {
    let dados_vendas = vec![
        RegistroVenda { mes: 1.0, valor: 100.0 },
        RegistroVenda { mes: 2.0, valor: 120.0 },
        RegistroVenda { mes: 3.0, valor: 140.0 },
    ];
    let inclinacao = calcular_inclinacao(&dados_vendas);
    let intercepto = calcular_intercepto(&dados_vendas, inclinacao);
    let previsao = prever_valor(intercepto, inclinacao, 6.0);
    let r_quadrado = calcular_r_quadrado(&dados_vendas, intercepto, inclinacao);
    let mse = calcular_erro_quadratico_medio(&dados_vendas, intercepto, inclinacao);

    println!("Inclinação: {:.2}, Intercepto: {:.2}", inclinacao, intercepto);
    println!("Previsão para mês 6: {:.2}", previsao);
    println!("Coeficiente de Determinação (R²): {:.3}", r_quadrado);
    println!("Erro Quadrático Médio (MSE): {:.2}", mse);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calcular_media() {
        let valores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 3.0)
    }

    #[test]
    fn test_calcular_media_iguais() {
        let valores = vec![9.0, 9.0, 9.0, 9.0, 9.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 9.0)
    }

    #[test]
    fn test_calcular_media_unico_valor() {
        let valores = vec![5.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 5.0)
    }

    #[test]
    fn test_calcular_media_negativos() {
        let valores = vec![-2.0, -4.0, -6.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, -4.0)
    }

    #[test]
    fn test_calcular_media_negativos_positivos() {
        let valores = vec![-2.0, 4.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 1.0)
    }
}