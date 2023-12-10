Install migration tool
```bash
$ cargo install sea-orm-cli
```

Create .env file
```dotenv
SERVER_URL=localhost:8080
CLIENT_URL=localhost:5173
DATABASE_URL=postgres://beer:beer@localhost:5432/beer
```

Run docker container
```bash
$ cd docker && docker compose up -d
```

Apply migration
```bash
$ sea-orm-cli migrate up
```

Run
```bash
$ cargo run
```