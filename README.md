# Something About Us

A Rust web application providing user management system with OAuth authentication and JWT token-based authorization.

## Features

- **OAuth Authentication**: User authentication via GitHub OAuth
- **JWT Tokens**: Secure JWT token issuance and verification using EdDSA keys
- **User Management**: PostgreSQL-based user data management
- **Session Management**: Session caching with Memcached
- **RESTful API**: Web API built with Axum framework
- **API Documentation**: API documentation via Swagger UI

## Tech Stack

- **Language**: Rust (Edition 2021)
- **Web Framework**: Axum
- **Database**: PostgreSQL (SeaORM)
- **Cache**: Memcached
- **Authentication**: OAuth2, JWT (EdDSA)
- **Containerization**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- PostgreSQL
- Memcached

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd something_about_us
   ```

2. **Setup configuration**
   ```bash
   cp config.toml.example config.toml
   ```

3. **Run with Docker Compose**
   ```bash
   docker-compose up -d
   ```

4. **Run in local development**
   ```bash
   cargo run --bin migration
   cargo run --bin something_about_us
   ```

### Configuration

Configure the following in `config.toml`:

- **Server**: Domain, port, User-Agent
- **Database**: PostgreSQL connection settings
- **Cache**: Memcached connection settings
- **JWT**: Key paths, TTL, issuer information
- **OAuth**: GitHub OAuth client credentials
- **Security**: Session cookie settings

## API Documentation

Access Swagger UI at:

```
http://localhost:3000/swagger-ui/
```

## Development

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

## Project Structure

```
├── migration/          # Database migrations
├── something_about_us/ # Main application
├── jwks/              # JWT key storage
├── config.toml        # Configuration file
└── docker-compose.yaml # Docker configuration
```

## License

This project is distributed under the MIT License.