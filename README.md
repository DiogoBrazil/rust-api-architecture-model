# Rust API Architecture Model

[![Language Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

Um projeto esqueleto para construir APIs RESTful em Rust, apresentando uma implementação de Clean Architecture para promover um código modular, testável e escalável.

Este repositório serve como um ponto de partida robusto para novos projetos, fornecendo uma estrutura de diretórios clara, um CRUD de usuário completo e autenticação JWT pronta para uso.

## ✨ Recursos

-   **Clean Architecture:** Separação de responsabilidades em camadas (`core`, `services`, `controllers`, `repositories`).
-   **Autenticação JWT:** Geração e validação de tokens para proteger rotas.
-   **CRUD de Usuário Completo:** Operações de Criar, Ler, Atualizar e Deletar usuários.
-   **Configuração Centralizada:** Gerenciamento de configurações de ambiente e banco de dados.
-   **Tratamento de Erros:** Um sistema de erros customizado para respostas de API consistentes.
-   **Containerização com Docker:** Arquivo `docker-compose.yml` para configurar facilmente um banco de dados PostgreSQL.
-   **Framework Web Atix:** Utiliza o poderoso e seguro framework Actix Web.

## 🏛️ Arquitetura

A arquitetura é projetada para isolar a lógica de negócios das dependências externas, como o banco de dados ou o framework web. Isso torna o sistema mais fácil de manter, testar e evoluir.

-   **`src/core` (Camada de Domínio):** Contém as `entities` (lógica de negócio e estruturas de dados) e os `contracts` (traits que definem o comportamento esperado das camadas externas, como repositórios). Esta é a camada mais interna e não depende de nenhuma outra.
-   **`src/services` (Camada de Aplicação):** Orquestra a lógica de negócio. Os serviços utilizam os contratos do `core` para realizar suas tarefas, coordenando a interação entre os `controllers` e os `repositories`.
-   **`src/controllers` (Camada de Apresentação):** Responsável por lidar com as requisições HTTP. Recebe os dados, os valida e chama os `services` apropriados. Não contém lógica de negócio.
-   **`src/routes` (Camada de Apresentação):** Define os endpoints da API e os associa aos seus respectivos `controllers`.
-   **`src/repositories` & `src/adapters` (Camada de Infraestrutura):** Implementa os detalhes técnicos. Os `repositories` implementam os `contracts` do `core` para interagir com o banco de dados. Os `adapters` contêm outras implementações, como o hasher de senhas.

## 🚀 Como Começar

Siga os passos abaixo para configurar e executar o projeto localmente.

### Pré-requisitos

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Docker](https://www.docker.com/get-started) e [Docker Compose](https://docs.docker.com/compose/install/)

### 1. Clone o Repositório

```bash
git clone https://github.com/DiogoBrazil/rust-api-architecture-model.git
cd rust-api-architecture-model
```

### 2. Configure as Variáveis de Ambiente

Copie o arquivo de exemplo `.env` e preencha com suas configurações.

```bash
cp example.env .env
```

Abra o arquivo `.env` e configure as seguintes variáveis:

-   `DATABASE_URL`: A URL de conexão com o banco de dados. Se estiver usando o Docker Compose abaixo, o valor padrão deve funcionar.
-   `JWT_SECRET`: Uma chave secreta para assinar os tokens JWT.
-   `SERVER_ADDR`: O endereço onde a API será executada.

Exemplo de `.env`:

```env
DATABASE_URL="postgres://postgres:postgres123@localhost:5432/db_model"
SERVER_ADDR=0.0.0.0:8080
JWT_SECRET="seu-segredo-super-secreto"
API_KEY="sua-api-key"
RUST_LOG=info
```

### 3. Inicie o Banco de Dados com Docker

O `docker-compose.yml` incluído irá iniciar um container PostgreSQL.

```bash
docker-compose up -d
```

### 4. Execute as Migrações do Banco (se aplicável)

Este esqueleto não inclui um sistema de migração, mas se você adicionar um (como o `sqlx-cli`), este seria o momento de executá-lo. Para o esquema inicial, você pode precisar criar as tabelas manualmente.

### 5. Execute a Aplicação

```bash
cargo run
```

A API estará disponível em `http://localhost:8080`.

## Endpoints da API

Aqui estão os endpoints disponíveis e exemplos de como usá-los com `curl`.

### Autenticação

#### `POST /auth/login`

Autentica um usuário e retorna um token JWT.

```bash
curl -X POST http://localhost:8080/auth/login \
-H "Content-Type: application/json" \
-d '{
  "email": "user@example.com",
  "password": "password123"
}'
```

### Usuários

#### `POST /users`

Cria um novo usuário.

```bash
curl -X POST http://localhost:8080/users \
-H "Content-Type: application/json" \
-d '{
  "name": "Test User",
  "email": "user@example.com",
  "password": "password123"
}'
```

#### `GET /users`

Retorna uma lista de todos os usuários. (Requer token de autenticação)

```bash
curl -X GET http://localhost:8080/users \
-H "Authorization: Bearer <seu-token-jwt>"
```

#### `GET /users/{id}`

Busca um usuário pelo seu ID. (Requer token de autenticação)

```bash
curl -X GET http://localhost:8080/users/<user-id> \
-H "Authorization: Bearer <seu-token-jwt>"
```

#### `PUT /users/{id}`

Atualiza os dados de um usuário. (Requer token de autenticação)

```bash
curl -X PUT http://localhost:8080/users/<user-id> \
-H "Content-Type: application/json" \
-H "Authorization: Bearer <seu-token-jwt>" \
-d '{
  "name": "New Name"
}'
```

#### `DELETE /users/{id}`

Deleta um usuário pelo seu ID. (Requer token de autenticação)

```bash
curl -X DELETE http://localhost:8080/users/<user-id> \
-H "Authorization: Bearer <seu-token-jwt>"
```

## 🛠️ Tecnologias Utilizadas

-   **Framework:** [Actix Web](https://actix.rs/)
-   **Banco de Dados:** [SQLx](https://github.com/launchbadge/sqlx) para comunicação com PostgreSQL
-   **Autenticação:** [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
-   **Hashing de Senhas:** [argon2](https://crates.io/crates/argon2)
-   **Assíncrono:** [Tokio](https://tokio.rs/)
-   **Validação:** [Regex](https://crates.io/crates/regex) e lógica customizada.

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue para discutir uma nova funcionalidade ou reportar um bug. Pull requests também são muito bem-vindos.

## 📄 Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE). Sinta-se à vontade para adicionar um arquivo de licença ao seu projeto.