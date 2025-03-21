# Web Crawler em Rust

![Rust](https://img.shields.io/badge/Rust-1.XX-orange?style=for-the-badge&logo=rust)
![Tokio](https://img.shields.io/badge/Tokio-async-blue?style=for-the-badge)

Um pequeno **web crawler** desenvolvido em **Rust**, que verifica a disponibilidade de APIs externas. Este projeto nasceu da necessidade de testar rapidamente se todas as APIs externas que utilizo estavam funcionando corretamente.

## 🚀 Funcionalidades

- 🚀 **Verifica a disponibilidade de URLs/API endpoints**
- 🔄 **Executa verificações assíncronas utilizando Tokio**
- ⚡ **Relata falhas de conexão e status HTTP das requisições**
- 🔐 **Possível aplicação na área de segurança para análise de endpoints vulneráveis**

## 🛠 Tecnologias Utilizadas

- **Rust** 🦀
- **Tokio** (para execução assíncrona)
- **reqwest** (para requisições HTTP)
- **serde** (para manipulação de JSON, se necessário)

## 📦 Como Usar

### 1️⃣ Clonar o Repositório
```sh
 git clone https://github.com/2APF/Check.git
 cd Check
```

### 2️⃣ Instalar Dependências
Certifique-se de ter o **Rust** e o **Cargo** instalados.
```sh
 cargo build
```

### 3️⃣ Executar o Web Crawler
```sh
 cargo run
```

### 4️⃣ Personalizar URLs a serem verificadas
Edite a lista de **URLs** no código para verificar diferentes APIs.

## 📜 Exemplo de Uso
```rust
use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://api.exemplo1.com/status",
        "https://api.exemplo2.com/health",
    ];
    
    for url in urls {
        match reqwest::get(url).await {
            Ok(resp) => println!("{} - Status: {}", url, resp.status()),
            Err(err) => println!("Erro ao acessar {}: {}", url, err),
        }
    }
}
```

## 🔥 Possíveis Melhorias
- Adicionar suporte a **proxy** para anonimização.
- Melhorar o **tratamento de erros** para casos específicos.
- Criar um **log de execuções** para análise posterior.
- Suporte a **parâmetros via linha de comando**.

## 🤝 Contribuindo
Sinta-se livre para contribuir! Faça um **fork**, crie um **branch** com sua melhoria e envie um **pull request**.

## 📄 Licença
Este projeto está sob a licença **MIT**. Sinta-se à vontade para utilizá-lo e modificá-lo! 🚀

